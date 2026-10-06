import { existsSync } from "node:fs";

export function buildEnvironment() {
  const env = { ...process.env };
  const commandLineTools = "/Library/Developer/CommandLineTools";

  // Prefer a separately installed CLT toolchain without changing xcode-select.
  if (
    process.platform === "darwin" &&
    !env.DEVELOPER_DIR &&
    existsSync(`${commandLineTools}/SDKs/MacOSX.sdk`)
  ) {
    env.DEVELOPER_DIR = commandLineTools;
  }

  return env;
}
