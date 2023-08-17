use std::path::PathBuf;

use crate::registry::RegistryNode;

#[tauri::command]
pub async fn open(path: PathBuf) -> Result<RegistryNode, ()> {
  let node = RegistryNode::open(path);
  node
}
