use std::{path::PathBuf, collections::HashMap};

use winreg::{enums::*, RegKey};

pub fn get_hkey(string: String) -> Result<isize, ()> {
  match string.to_uppercase().as_str() {
    "HKEY_CLASSES_ROOT" => Ok(HKEY_CLASSES_ROOT),
    "HKEY_CURRENT_CONFIG" => Ok(HKEY_CURRENT_CONFIG),
    "HKEY_CURRENT_USER" => Ok(HKEY_CURRENT_USER),
    "HKEY_CURRENT_USER_LOCAL_SETTINGS" => Ok(HKEY_CURRENT_USER_LOCAL_SETTINGS),
    "HKEY_DYN_DATA" => Ok(HKEY_DYN_DATA),
    "HKEY_LOCAL_MACHINE" => Ok(HKEY_LOCAL_MACHINE),
    "HKEY_PERFORMANCE_DATA" => Ok(HKEY_PERFORMANCE_DATA),
    "HKEY_PERFORMANCE_NLSTEXT" => Ok(HKEY_PERFORMANCE_NLSTEXT),
    "HKEY_PERFORMANCE_TEXT" => Ok(HKEY_PERFORMANCE_TEXT),
    "HKEY_USERS" => Ok(HKEY_USERS),
    &_ => Err(())
  }
}

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum RegistryValue {
  String(String),
  U32(u32),
  U64(u64),
}

#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryNode {
  name: String,
  full_path: PathBuf,
  values: HashMap<String, RegistryValue>,
  children: HashMap<String, RegistryNode>,
}

impl RegistryNode {
  pub fn open(path: PathBuf) -> Result<Self, String> {
    let mut parts: Vec<std::path::Component<'_>> = path.components().collect::<Vec<_>>();

    if parts.len() == 0 {
      return Err("length".into())
    }



    let hkey = parts.remove(0).as_os_str().to_str().unwrap().to_string();
    let sub_path: PathBuf = parts.iter().collect();
    let name = if parts.len() > 0 {
      parts.remove(parts.len() - 1).as_os_str().to_str().unwrap().to_string()
    } else {
      hkey.clone()
    };

    println!("HKEY: {:?}; NAME: {:?}; SUB PATH {:?};", hkey, name, sub_path);
    let hkey: Option<isize> = match hkey.as_str() {
      "HKEY_CLASSES_ROOT" => Some(HKEY_CLASSES_ROOT),
      "HKEY_LOCAL_MACHINE" => Some(HKEY_LOCAL_MACHINE),
      "HKEY_CURRENT_USER" => Some(HKEY_CURRENT_USER),
      "HKEY_USERS" => Some(HKEY_USERS),
      "HKEY_CURRENT_CONFIG" => Some(HKEY_CURRENT_CONFIG),
      _ => None
    };

    let root_key = match hkey {
      Some(hkey) => RegKey::predef(hkey),
      None => return Err("hkey".into())
    };
    match root_key.open_subkey(sub_path) {
      Ok(sub_key) => {
        let mut node = RegistryNode::default();

        node.name = name;
        node.full_path = path.components().collect::<PathBuf>();
        
        for value in sub_key.enum_values() {
          let (value_name, value_info) = value.unwrap();
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
            _ => continue
          };
        }

        for key in sub_key.enum_keys() {
          let key = match key {
            Ok(key) => key,
            Err(_) => continue
          };
          match RegistryNode::open(path.into_iter().collect::<PathBuf>().join(key.as_str())) {
            Ok(child) => node.children.insert(key, child),
            Err(_) => continue
          };
        }

        Ok(node)
      },
      Err(error) => Err(error.to_string())
    }
  }
}
