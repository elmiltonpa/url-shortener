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
      PUBLIC_GOOGLE_CLIENT_ID: envField.string({
        context: "server",
        access: "public",
      }),
      PUBLIC_GOOGLE_REDIRECT_URI: envField.string({
        context: "server",
        access: "public",
        optional: true,
      }),
      PUBLIC_APP_DOMAIN: envField.string({
        context: "server",
        access: "public",
        optional: true,
      }),
    },
  },
  vite: {
    plugins: [tailwindcss()],
  },
  integrations: [icon(), vue()],
});