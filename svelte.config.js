import { mdsvex } from "mdsvex";
import adapter from "svelte-adapter-bun";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { createHighlighter } from "@bitmachina/highlighter";
import { frappe } from "@catppuccin/vscode";
const config = {
  preprocess: [
    vitePreprocess(),
    mdsvex({
      extensions: [".svx", ".md"],
      layout: {
        blog: "./src/lib/layouts/BlogLayout.svelte",
      },
      highlight: {
        highlighter: await createHighlighter({
          theme: frappe,
        }),
      },
    }),
  ],
  kit: {
    adapter: adapter(),
    alias: {
      $remote: "./remote",
      $static: "./static",
    },
  },
  extensions: [".svelte", ".svx", ".md", ".svg"],
};

export default config;
