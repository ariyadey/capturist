import { Injectable } from "@angular/core";
import { TodoistApi, TodoistRequestError } from "@doist/todoist-sdk";
import { invoke } from "@tauri-apps/api/core";

/**
 * Wraps the Todoist SDK client so every API call goes through a single point
 * that refreshes an expired access token and retries once.
 *
 * Only the short-lived access token is exposed here (fetched from the backend
 * keyring via IPC); the long-lived refresh token is never sent to the webview.
 */
@Injectable({ providedIn: "root" })
export class Todoist {
  /** The underlying SDK client. `undefined` until {@link initialize} runs. */
  private apiClient!: TodoistApi;

  /** In-flight refresh, deduplicating concurrent refreshes. `null` when idle. */
  private refreshing: Promise<void> | null = null;

  /**
   * A single proxy over the API client, so {@link api} has a stable identity
   * and every method call is routed through {@link call}.
   */
  private readonly apiProxy = new Proxy({} as TodoistApi, {
    get:
      (_, prop: keyof TodoistApi) =>
      (...args: Array<never>) =>
        this.call(prop, args),
  });

  /** The proxied Todoist API client for making authorized requests. */
  get api(): TodoistApi {
    return this.apiProxy;
  }

  /**
   * Fetches a valid access token from the backend and builds the SDK client.
   * Call once at startup (or after signing in) before using {@link api}.
   */
  async initialize(): Promise<void> {
    const token = await invoke<string>("get_todoist_access_token");
    this.apiClient = new TodoistApi(token);
  }

  /**
   * Runs one method on the live client. If the call fails with a 401 the access
   * token has expired: it refreshes (deduplicated) and retries once against the
   * freshly rebuilt client, so the retry never reuses the stale token.
   *
   * Non-401 errors propagate unchanged.
   */
  private async call(method: keyof TodoistApi, args: Array<never>): Promise<unknown> {
    const run = () => {
      const client = this.apiClient;
      const apiMethod = Reflect.get(client, method, client) as ApiMethod;
      return apiMethod.apply(client, args);
    };

    try {
      return await run();
    } catch (error) {
      // Access token expired mid-session: refresh (once, deduped) and retry
      // against the freshly rebuilt client.
      if (error instanceof TodoistRequestError && error.httpStatusCode === 401) {
        console.warn("Hit error 401; Refreshing the token manually...");
        await this.refresh();
        return await run();
      }
      throw error;
    }
  }

  /**
   * Refreshes the access token, deduplicating concurrent refreshes so the
   * rotated refresh token is only ever used once (two races would revoke it).
   */
  private async refresh(): Promise<void> {
    this.refreshing ??= this.doRefresh().finally(() => (this.refreshing = null));
    await this.refreshing;
  }

  /** Performs a single backend refresh and rebuilds the client with the new token. */
  private async doRefresh(): Promise<void> {
    await invoke("refresh_todoist_access_token");
    await this.initialize();
  }
}

/** An SDK method taking zero or more arguments and returning a promise. */
type ApiMethod = (...args: Array<never>) => Promise<unknown>;
