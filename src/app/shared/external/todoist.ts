import { Injectable } from "@angular/core";
import { TodoistApi } from "@doist/todoist-api-typescript";
import { invoke } from "@tauri-apps/api/core";

@Injectable({ providedIn: "root" })
export class Todoist {
  private todoistApi!: TodoistApi;

  get api(): TodoistApi {
    return this.todoistApi;
  }

  async initialize() {
    const token = await invoke<string>("get_todoist_access_token");
    this.todoistApi = new TodoistApi(token);
  }
}
