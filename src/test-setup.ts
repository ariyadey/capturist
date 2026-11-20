import { mockIPC, mockWindows } from "@tauri-apps/api/mocks";

mockWindows("main");

mockIPC(() => {
  // Generic happy path for any Tauri IPC command during unit tests.
  return Promise.resolve(undefined);
});
