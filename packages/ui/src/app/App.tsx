import { appWindow } from "@tauri-apps/api/window";
import { Component, For, Show, createEffect, createResource, createSignal, onCleanup, onMount, untrack } from "solid-js";

import "./index.css";

const App = () => {
  onMount(() => {
    appWindow.show();
  });

  return (
    <>

    </>
  )
}

export default App;
