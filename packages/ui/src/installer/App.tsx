import { Component, For, Show, createEffect, createResource, createSignal, onCleanup, onMount, untrack } from "solid-js";
import { RegistryNode } from "tauri-plugin-registry";

export const App: Component = () => {
  const [tree, setTree] = createSignal<object>({});

  return (
    <>
      <button onClick={() => RegistryNode.open("HKEY_LOCAL_MACHINE/SOFTWARE/Adobe").then(setTree)}>Click</button>
      <p>{JSON.stringify(tree(), null, 2)}</p>
    </>
  )
}

