use std::path::PathBuf;

use super::util::RegistryNode;

#[tauri::command]
pub async fn get_tree(path: PathBuf) -> Result<RegistryNode, String> {
  let node = RegistryNode::open(path);
  print!("{:?}", node);
  node
}
