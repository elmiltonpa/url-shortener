import { defineConfig, envField } from "astro/config";
import tailwindcss from "@tailwindcss/vite";
import icon from "astro-icon";
import vercel from "@astrojs/vercel";

import vue from "@astrojs/vue";

export default defineConfig({
  output: "server",
  adapter: vercel(),
  env: {
    schema: {
      API_URL: envField.string({ context: "server", access: "secret" }),
    },
  },
  vite: {
    plugins: [tailwindcss()],
  },
  integrations: [icon(), vue()],
});