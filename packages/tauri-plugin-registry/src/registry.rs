#![cfg(target_os = "windows")]

use std::{collections::HashMap, path::PathBuf};




#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum RegistryValue {
  String(String),
  U32(u32),
  U64(u64)
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryNode {
  name: String,
  path: PathBuf,
  full_path: PathBuf,
  values: HashMap<String, RegistryValue>,
  children: HashMap<String, RegistryNode>
}

impl RegistryNode {
  pub fn open(path: PathBuf) -> Result<Self, ()> {
    Ok(Self::default())
  }
}
