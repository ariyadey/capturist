import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";

@Component({
  selector: "cpt-root",
  template: `<router-outlet />`,
  styles: ``,
  imports: [RouterOutlet],
})
export class App {}

// TODO: 07/11/2025 Add a global shortcut hint for wayland users
