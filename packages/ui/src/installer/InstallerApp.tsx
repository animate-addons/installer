import { Component, For, createSignal, onMount } from "solid-js";

export const InstallerApp: Component = () => {
  const [tauries, setTauries] = createSignal<Record<string, any>>({});

  onMount(() => {
    setTauries(Object.fromEntries(Object.entries(window).filter(([key, value]) => /TAURI/i.test(key))));
    // setTauries(Object.fromEntries(Object.entries(window).filter(([key, value]) => key === "__TAURI__")));
    // setTauries(Object.fromEntries(Object.entries(window)));
  });

  return (
    <>
      <For each={Object.entries(tauries())}>
        {([key, value]) => (
          <p><b>{key}</b>{`: ${value}`}</p>
        )}
      </For>
    </>
  )
}
