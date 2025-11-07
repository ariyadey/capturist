import { bootstrapApplication } from "@angular/platform-browser";
import { App } from "@cpt/app";
import { appConfig } from "@cpt/app.config";

bootstrapApplication(App, appConfig).catch((err) => console.error(err));
