// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Compile contracts to wasm and RISC-V binaries.
use anyhow::Result;
use parity_wasm::elements::{deserialize_file, serialize_to_file, Internal};
use std::{
	env, fs,
	hash::Hasher,
	path::{Path, PathBuf},
	process::Command,
};
use twox_hash::XxHash32;

/// Read the file at `path` and return its hash as a hex string.
fn file_hash(path: &Path) -> String {
	let data = fs::read(path).expect("file exists; qed");
	let mut hasher = XxHash32::default();
	hasher.write(&data);
	hasher.write(include_bytes!("build.rs"));
	let hash = hasher.finish();
	format!("{:x}", hash)
}

/// A contract entry.
struct Entry {
	/// The path to the contract source file.
	path: PathBuf,
	/// The hash of the contract source file.
	hash: String,
}

impl Entry {
	/// Create a new contract entry from the given path.
	fn new(path: PathBuf) -> Self {
		let hash = file_hash(&path);
		Self { path, hash }
	}

	/// Return the path to the contract source file.
	fn path(&self) -> &str {
		self.path.to_str().expect("path is valid unicode; qed")
	}

	/// Return the name of the contract.
	fn name(&self) -> &str {
		self.path
			.file_stem()
			.expect("file exits; qed")
			.to_str()
			.expect("name is valid unicode; qed")
	}

	/// Return the name of the output wasm file.
	fn out_wasm_filename(&self) -> String {
		format!("{}.wasm", self.name())
	}
}

/// Collect all contract entries from the given source directory.
/// Contracts that have already been compiled are filtered out.
fn collect_entries(contracts_dir: &Path, out_dir: &Path) -> Vec<Entry> {
	fs::read_dir(&contracts_dir)
		.expect("src dir exists; qed")
		.filter_map(|file| {
			let path = file.expect("file exists; qed").path();
			if path.extension().map_or(true, |ext| ext != "rs") {
				return None;
			}

			let entry = Entry::new(path);
			if out_dir.join(&entry.hash).exists() {
				None
			} else {
				Some(entry)
			}
		})
		.collect::<Vec<_>>()
}

/// Create a `Cargo.toml` to compile the given contract entries.
fn create_cargo_toml<'a>(
	fixtures_dir: &Path,
	entries: impl Iterator<Item = &'a Entry>,
	output_dir: &Path,
) -> Result<()> {
	let uapi_path = fixtures_dir.join("../uapi").canonicalize()?;
	let common_path = fixtures_dir.join("./contracts/common").canonicalize()?;
	let mut cargo_toml: toml::Value = toml::from_str(&format!(
		"
[package]
name = 'contracts'
version = '0.1.0'
edition = '2021'

# Binary targets are injected below.
[[bin]]

[dependencies]
uapi = {{ package = 'pallet-contracts-uapi', default-features = false,  path = {uapi_path:?}}}
common = {{ package = 'pallet-contracts-fixtures-common',  path = {common_path:?}}}

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
"
	))?;

	let binaries = entries
		.map(|entry| {
			let name = entry.name();
			let path = entry.path();
			toml::Value::Table(toml::toml! {
				name = name
				path = path
			})
		})
		.collect::<Vec<_>>();

	cargo_toml["bin"] = toml::Value::Array(binaries);
	let cargo_toml = toml::to_string_pretty(&cargo_toml)?;
	fs::write(output_dir.join("Cargo.toml"), cargo_toml).map_err(Into::into)
}

/// Invoke `cargo fmt` to check that fixtures files are formatted.
fn invoke_fmt(current_dir: &Path, contracts_dir: &Path) -> Result<()> {
	let fmt_res = Command::new("rustup")
		.current_dir(current_dir)
		.args(&["run", "nightly", "cargo", "fmt", "--check"])
		.output()
		.unwrap();

	if fmt_res.status.success() {
		return Ok(())
	}

	let stdout = String::from_utf8_lossy(&fmt_res.stdout);
	eprintln!("{}", stdout);
	eprintln!(
		"Fixtures files are not formatted.\nPlease run `rustup run nightly rustfmt {}/*.rs`",
		contracts_dir.display()
	);
	anyhow::bail!("Fixtures files are not formatted")
}

/// Invoke `cargo build` to compile the contracts.
fn invoke_build(current_dir: &Path) -> Result<()> {
	let encoded_rustflags = [
		"-Clink-arg=-zstack-size=65536",
		"-Clink-arg=--import-memory",
		"-Clinker-plugin-lto",
		"-Ctarget-cpu=mvp",
		"-Dwarnings",
	]
	.join("\x1f");

	let build_res = Command::new(env::var("CARGO")?)
		.current_dir(current_dir)
		.env("CARGO_ENCODED_RUSTFLAGS", encoded_rustflags)
		.args(&["build", "--release", "--target=wasm32-unknown-unknown"])
		.output()
		.unwrap();

	if build_res.status.success() {
		return Ok(())
	}

	let stderr = String::from_utf8_lossy(&build_res.stderr);
	eprintln!("{}", stderr);
	anyhow::bail!("Failed to build contracts");
}

/// Post-process the compiled wasm contracts.
fn post_process_wasm(input_path: &Path, output_path: &Path) -> Result<()> {
	let mut module = deserialize_file(input_path)?;
	if let Some(section) = module.export_section_mut() {
		section.entries_mut().retain(|entry| {
			matches!(entry.internal(), Internal::Function(_)) &&
				(entry.field() == "call" || entry.field() == "deploy")
		});
	}

	serialize_to_file(output_path, module).map_err(Into::into)
}

/// Write the compiled contracts to the given output directory.
fn write_output(build_dir: &Path, out_dir: &Path, entries: Vec<Entry>) -> Result<()> {
	for entry in entries {
		let wasm_output = entry.out_wasm_filename();
		post_process_wasm(
			&build_dir.join("target/wasm32-unknown-unknown/release").join(&wasm_output),
			&out_dir.join(&wasm_output),
		)?;
		fs::write(out_dir.join(&entry.hash), "")?;
	}

	Ok(())
}

/// Returns the root path of the wasm workspace.
fn find_workspace_root(current_dir: &Path) -> Option<PathBuf> {
	let mut current_dir = current_dir.to_path_buf();

	while current_dir.parent().is_some() {
		if current_dir.join("Cargo.toml").exists() {
			let cargo_toml_contents =
				std::fs::read_to_string(current_dir.join("Cargo.toml")).ok()?;
			if cargo_toml_contents.contains("[workspace]") {
				return Some(current_dir);
			}
		}

		current_dir.pop();
	}

	None
}

fn main() -> Result<()> {
	let fixtures_dir: PathBuf = env::var("CARGO_MANIFEST_DIR")?.into();
	let contracts_dir = fixtures_dir.join("contracts");
	let out_dir: PathBuf = env::var("OUT_DIR")?.into();
	let workspace_root = find_workspace_root(&fixtures_dir).expect("workspace root exists; qed");

	let entries = collect_entries(&contracts_dir, &out_dir);
	if entries.is_empty() {
		return Ok(());
	}

	let tmp_dir = tempfile::tempdir()?;
	let tmp_dir_path = tmp_dir.path();
	fs::copy(workspace_root.join(".rustfmt.toml"), tmp_dir_path.join(".rustfmt.toml"))?;
	create_cargo_toml(&fixtures_dir, entries.iter(), tmp_dir.path())?;
	invoke_fmt(tmp_dir_path, &contracts_dir)?;
	invoke_build(tmp_dir_path)?;
	write_output(tmp_dir_path, &out_dir, entries)?;

	Ok(())
}
