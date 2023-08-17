import { invoke } from "@tauri-apps/api";
import { IRegistryNode } from "./registry";



export const open = (path: string): Promise<IRegistryNode> => invoke("plugin:registry|open", { path });
