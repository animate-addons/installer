import { appWindow } from "@tauri-apps/api/window";
import { Component, For, Show, createEffect, createResource, createSignal, onCleanup, onMount, untrack } from "solid-js";

const App = () => {
  window.addEventListener("DOMContentLoaded", () => appWindow.show());

  return (
    <>

    </>
  )
}

export default App;
