import { NgOptimizedImage } from "@angular/common";
import { ChangeDetectionStrategy, Component, signal } from "@angular/core";
import { MatButton } from "@angular/material/button";
import { invoke } from "@tauri-apps/api/core";

@Component({
  selector: "cpt-landing-page",
  templateUrl: "./landing-page.html",
  styleUrls: ["./landing-page.scss"],
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [NgOptimizedImage, MatButton],
})
export class LandingPage {
  protected readonly authenticationClicked = signal(false);

  async authenticate() {
    this.authenticationClicked.set(true);
    await invoke("start_authentication");
  }
}
