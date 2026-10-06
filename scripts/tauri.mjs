import { spawn } from "node:child_process";
import { fileURLToPath } from "node:url";
import { buildEnvironment } from "./environment.mjs";

const executable = fileURLToPath(
  new URL("../node_modules/@tauri-apps/cli/tauri.js", import.meta.url),
);

const child = spawn(process.execPath, [executable, ...process.argv.slice(2)], {
  cwd: fileURLToPath(new URL("../", import.meta.url)),
  stdio: "inherit",
  env: buildEnvironment(),
});

child.on("error", (error) => {
  console.error(`无法启动 Tauri：${error.message}`);
  process.exitCode = 1;
});

child.on("exit", (code) => {
  process.exitCode = code ?? 1;
});
