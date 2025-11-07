import { APP_BASE_HREF } from "@angular/common";
import { provideHttpClient, withFetch } from "@angular/common/http";
import {
  ApplicationConfig,
  DOCUMENT,
  inject,
  provideAppInitializer,
  provideBrowserGlobalErrorListeners,
  provideZonelessChangeDetection,
} from "@angular/core";
import { provideRouter, withInMemoryScrolling, withRouterConfig } from "@angular/router";
import { Todoist } from "@cpt/shared/external/todoist";
import { forwardConsole } from "@cpt/shared/ipc/app-log";
import { AppWindowLabel } from "@cpt/shared/ipc/app-window-label";
import { window } from "@tauri-apps/api";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Window } from "@tauri-apps/api/window";
import { routes } from "./app.routes";

export const appConfig: ApplicationConfig = {
  providers: [
    provideAppInitializer(() => {
      forwardConsole();
      switch (getCurrentWebviewWindow().label as AppWindowLabel) {
        case AppWindowLabel.QUICK_ADD:
          return inject(Todoist).initialize();
        case AppWindowLabel.AUTHENTICATION:
          return Promise.resolve();
      }
    }),
    {
      provide: APP_BASE_HREF,
      useFactory: () => inject(DOCUMENT).querySelector("base[href]")?.getAttribute("href") ?? "/",
    },
    provideBrowserGlobalErrorListeners(),
    provideZonelessChangeDetection(),
    provideRouter(
      routes,
      withRouterConfig({
        onSameUrlNavigation: "reload",
      }),
      withInMemoryScrolling({
        anchorScrolling: "enabled",
        scrollPositionRestoration: "enabled",
      }),
    ),
    provideHttpClient(withFetch()),
    { provide: Window, useValue: window },
  ],
};
