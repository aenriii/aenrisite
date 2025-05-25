import { mdsvex } from "mdsvex";
import adapter from "svelte-adapter-bun";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { createHighlighter } from "@bitmachina/highlighter";
import { frappe } from "@catppuccin/vscode";

import { join } from "path";
import { cwd } from "process";
export function pathFromProjectRoot(...args) {
  return join(cwd(), ...args);
}

const config = {
  preprocess: [
    vitePreprocess(),
    mdsvex({
      extensions: [".svx", ".md"],
      layout: {
        blog: pathFromProjectRoot("/src/lib/layouts/BlogLayout.svelte"),
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
