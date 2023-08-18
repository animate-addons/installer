#![cfg(target_os = "windows")]

use std::{collections::HashMap, path::PathBuf};
use winreg::{enums::*, RegKey};




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
    let path: PathBuf = path.components().into_iter().collect();

    let mut parts = path.components().collect::<Vec<_>>();
    if parts.len() == 0 {
      return Err(());
    }
    
    let root_path = parts.remove(0).as_os_str().to_str().unwrap().to_uppercase();
    let sub_path: PathBuf = parts.iter().collect();
    let name = if parts.len() > 0 { parts.remove(parts.len() - 1).as_os_str().to_str().unwrap().to_string() } else { root_path.clone() };

    let hkey: isize = match root_path.as_str() {
      "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
      "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
      "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
      "HKEY_USERS" => HKEY_USERS,
      "HKEY_CURRENT_CONFIG" => HKEY_CURRENT_CONFIG,
      _ => return Err(())
    };

    let root_key = RegKey::predef(hkey);
    match root_key.open_subkey(sub_path) {
      Ok(sub_key) => {
        let mut node = Self {
          name,
          path: PathBuf::from(root_path).join(parts.into_iter().collect::<PathBuf>()),
          full_path: path.clone(),
          ..Default::default()
        };

        for pair in sub_key.enum_values() {
          let (value_name, value_info) = match pair {
            Ok(pair) => pair,
            Err(_) => continue
          };
          match value_info.vtype {
            REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
              let value: String = sub_key.get_value(value_name.as_str()).unwrap();
              node.values.insert(value_name, RegistryValue::String(value));
            },
            REG_DWORD => {
              let value: u32 = sub_key.get_value(value_name.as_str()).unwrap();
              node.values.insert(value_name, RegistryValue::U32(value));
            },
            REG_QWORD => {
              let value: u64 = sub_key.get_value(value_name.as_str()).unwrap();
              node.values.insert(value_name, RegistryValue::U64(value));
            },
            _ => {}
          }
        }
        for child_key in sub_key.enum_keys() {
          let Ok(child_name) = child_key else { continue };
          let child_path = path.join(child_name.as_str());
          match RegistryNode::open(child_path) {
            Ok(child) => node.children.insert(child_name, child),
            Err(_) => continue
          };
        }

        Ok(node)
      },
      Err(_) => Err(())
    }
  }
}
