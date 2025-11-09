import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";

@Component({
  selector: "cpt-root",
  template: `<router-outlet />`,
  styles: ``,
  imports: [RouterOutlet],
})
export class App {}
