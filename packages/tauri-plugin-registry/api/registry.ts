import * as invoke from "./invoke";
import * as path from "@tauri-apps/api/path";

export interface IRegistryNode {
  name: string;
  path: string,
  fullPath: string;
  values: Record<string, string | number>;
  children: Record<string, IRegistryNode>;
}

export class RegistryNode {
  public static open(path: string): RegistryNode {
    return new RegistryNode(path);
  }

  private constructor(path: string) {

  }

  public async load(): Promise<this> {
    return this;
  }

  public value<Type extends string | number>(name: string): Type;
  public value(name: string, value: string | number):
  RegistryActionQueue<[{
    type: RegistryActionType.Write,
    kind: RegistryActionKind.Value,
  }]>;
  public value(name: string, value: string | number | null = null): any {
    if(value == null) {
      return "";
    } else {
      return new RegistryActionQueue(this);
    }
  }
}


enum RegistryActionType {
  Read,
  Write,
  Delete
}
enum RegistryActionKind {
  Key,
  Value
}

interface RegistryAction {
  type: RegistryActionType;
  kind: RegistryActionKind;
}

class RegistryActionQueue<Actions extends RegistryAction[]> {
  public constructor(private node: RegistryNode) {
    
  }

  public setValue<ValueType extends string | number>(name: string, value: ValueType): 
  RegistryActionQueue<[
    ...Actions,
    {
      type: RegistryActionType.Write,
      kind: RegistryActionKind.Value
    }
  ]> {
    return this;
  }
  public async execute(): Promise<RegistryNode> {
    return "" as any;
  }
}



RegistryNode.open("test")
  .value("test", 1);
