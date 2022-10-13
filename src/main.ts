import "carbon-components-svelte/css/g100.css";
import "@splidejs/splide/dist/css/splide.min.css";
import App from "./App.svelte";

const app = new App({
  target: document.getElementById("app"),
});

export default app;
