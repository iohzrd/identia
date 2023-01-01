import adapter from "@sveltejs/adapter-static";
import sveltePreprocess from "svelte-preprocess";
import { optimizeImports } from "carbon-preprocess-svelte";
import { vitePreprocess } from "@sveltejs/kit/vite";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: [sveltePreprocess(), optimizeImports()],
  kit: {
    adapter: adapter({
      fallback: "index.html",
    }),
    prerender: { entries: [] },
  },
};

export default config;
