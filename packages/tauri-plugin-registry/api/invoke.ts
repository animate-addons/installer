import { invoke } from "@tauri-apps/api";
import { IRegistryNode } from "./registry";


export const isSupported = (): Promise<boolean> => invoke("plugin:registry|is_supported");


export const open = (path: string, depth?: number): Promise<IRegistryNode> => invoke("plugin:registry|open", { path, depth });

