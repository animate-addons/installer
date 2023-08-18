import * as invoke from "./invoke";

export interface IRegistryNode {
  name: string;
  path: string,
  fullPath: string;
  values: Record<string, string | number>;
  children: Record<string, IRegistryNode>;
}

export class RegistryNode {
  
  
  public static async open(path: string, depth?: number): Promise<RegistryNode> {
    const data = await invoke.open(path, depth);
    return new RegistryNode(path, data);
  }

  private constructor(public readonly path: string, public readonly data: IRegistryNode) {
    
  }
}
