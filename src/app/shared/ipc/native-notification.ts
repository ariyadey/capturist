import { Injectable } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";

@Injectable({
  providedIn: "root",
})
export class NativeNotification {
  async send(options: { title: string; body?: string }) {
    await invoke("send_notification", options);
  }
}
