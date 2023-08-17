import * as invoke from "./invoke";

export interface IRegistryNode {
  name: string;
  path: string,
  fullPath: string;
  values: Record<string, string | number>;
  children: Record<string, IRegistryNode>;
}

export class RegistryNode {
  
  
  public static async open(path: string): Promise<RegistryNode> {
    const data = await invoke.open(path);
    return new RegistryNode(path, data);
  }

  private constructor(public readonly path: string, public readonly data: IRegistryNode) {
    
  }
}
