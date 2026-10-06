import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { buildEnvironment } from "./environment.mjs";

const cwd = fileURLToPath(new URL("../src-tauri/", import.meta.url));
const checks = [
  ["fmt", "--all", "--", "--check"],
  ["test", "--locked"],
  ["clippy", "--locked", "--all-targets", "--", "-D", "warnings"],
];

for (const args of checks) {
  const result = spawnSync("cargo", args, {
    cwd,
    stdio: "inherit",
    env: buildEnvironment(),
  });
  if (result.error || result.status !== 0) {
    console.error(result.error?.message ?? `cargo ${args[0]} 未通过`);
    process.exit(result.status ?? 1);
  }
}
