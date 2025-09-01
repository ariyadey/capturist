import { bootstrapApplication } from "@angular/platform-browser";
import { AppComponent } from "@cpt/app.component";
import { appConfig } from "@cpt/app.config";

bootstrapApplication(AppComponent, appConfig).catch((err) => console.error(err));
