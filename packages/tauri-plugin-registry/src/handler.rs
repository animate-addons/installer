use std::path::PathBuf;

use crate::registry::RegistryNode;

#[tauri::command]
pub fn is_supported() -> bool {
  #[cfg(target_os = "windows")]
  return true;

  #[cfg(not(target_os = "windows"))]
  return false;
}

#[tauri::command]
pub async fn open(path: PathBuf, depth: Option<u32>) -> Result<RegistryNode, ()> {
  let node = RegistryNode::open(path, depth);
  node
}
