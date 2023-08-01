import { Component, onMount } from "solid-js";

export const WebApp: Component = () => {
  onMount(() => {
    window.location.replace("https://github.com/animate-addons/installer");
  });

  return (
    <>
      Web
    </>
  )
}
