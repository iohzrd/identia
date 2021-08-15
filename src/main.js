import App from "./App.svelte";
import "carbon-components-svelte/css/g100.css";

const app = new App({
  target: document.body,
  props: {
    name: "world",
  },
});

export default app;
