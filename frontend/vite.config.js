import { fileURLToPath, URL } from "node:url";
import { defineConfig, loadEnv } from "vite";
import vue from "@vitejs/plugin-vue";

import Icons from 'unplugin-icons/vite'
import Components from 'unplugin-vue-components/vite'
import IconsResolve from 'unplugin-icons/resolver'

import vuetify from 'vite-plugin-vuetify'

// https://vitejs.dev/config/

export default ({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");
  return defineConfig({
    build: {
      target: 'esnext'
    },
    define: {
      "process.env": env,
    },
    plugins: [vue(), vuetify(),
      Components({
        resolvers: [IconsResolve()],
        dts: true,
      }),
      Icons({
        compiler: 'vue3',
        autoInstall: true,
      }),
    ],
    base: "https://powersort-competition.github.io/PowersortCompetitionWebsite/",
    server: {
      host: true,
      port: 8080,
    },
    resolve: {
      alias: {
        "@": fileURLToPath(new URL("./src", import.meta.url)),
      },
    },
  });
};
