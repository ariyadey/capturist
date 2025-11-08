import { debug, error, info, trace, warn } from "@tauri-apps/plugin-log";

export function forwardConsole() {
  const logLevelMap = new Map([
    ["log", trace],
    ["debug", debug],
    ["info", info],
    ["warn", warn],
    ["error", error],
  ] as const);
  logLevelMap.forEach((logger, fnName, _) => {
    const original = console[fnName];
    console[fnName] = message => {
      original(message);
      logger(message);
    };
  });
}
