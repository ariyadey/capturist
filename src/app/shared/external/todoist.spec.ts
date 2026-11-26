import { TestBed } from "@angular/core/testing";
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { TodoistApi, TodoistRequestError } from "@todoist/sdk";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { Todoist } from "./todoist";

describe("TodoistApiService", () => {
  let service: Todoist;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(Todoist);
  });

  afterEach(() => {
    vi.restoreAllMocks();
    clearMocks();
  });

  it("should be created", () => {
    expect(service).toBeTruthy();
  });

  it("initializes the SDK client from the backend access token", async () => {
    const quickAddSpy = vi
      .spyOn(TodoistApi.prototype, "quickAddTask")
      .mockResolvedValue({ id: "1", url: "https://app.todoist.com/task/1" } as never);
    mockIPC((cmd) => {
      if (cmd === "get_todoist_access_token") return "initial-token";
      return undefined;
    });

    await service.initialize();
    const result = await service.api.quickAddTask({ text: "Buy milk" });

    expect(quickAddSpy).toHaveBeenCalledWith({ text: "Buy milk" });
    expect(result).toEqual({ id: "1", url: "https://app.todoist.com/task/1" });
  });

  it("refreshes once (deduplicated) and retries on 401 responses", async () => {
    let tokenCounter = 0;
    let refreshCount = 0;
    mockIPC(async (cmd) => {
      if (cmd === "get_todoist_access_token") return `token-${++tokenCounter}`;
      if (cmd === "refresh_todoist_access_token") {
        refreshCount++;
        await Promise.resolve();
      }
      return undefined;
    });

    let attempts = 0;
    vi.spyOn(TodoistApi.prototype, "quickAddTask").mockImplementation(async () => {
      attempts++;
      if (attempts <= 2) {
        throw new TodoistRequestError("Unauthorized", 401);
      }
      return { id: String(attempts), url: "https://app.todoist.com/task/ok" } as never;
    });

    await service.initialize();
    const [first, second] = await Promise.all([
      service.api.quickAddTask({ text: "Task 1" }),
      service.api.quickAddTask({ text: "Task 2" }),
    ]);

    expect(refreshCount).toBe(1);
    expect(first.url).toBe("https://app.todoist.com/task/ok");
    expect(second.url).toBe("https://app.todoist.com/task/ok");
  });

  it("propagates non-401 errors without refreshing", async () => {
    let refreshCount = 0;
    mockIPC((cmd) => {
      if (cmd === "get_todoist_access_token") return "valid-token";
      if (cmd === "refresh_todoist_access_token") refreshCount++;
      return undefined;
    });

    vi.spyOn(TodoistApi.prototype, "quickAddTask").mockRejectedValue(
      new TodoistRequestError("Server error", 500),
    );

    await service.initialize();
    await expect(service.api.quickAddTask({ text: "Fail" })).rejects.toThrow("Server error");
    expect(refreshCount).toBe(0);
  });
});
