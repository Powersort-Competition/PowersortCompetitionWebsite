import { fileURLToPath, URL } from "node:url";
import { defineConfig, loadEnv } from "vite";
import vue from "@vitejs/plugin-vue";

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
    plugins: [vue()],
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
