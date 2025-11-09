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
import { forwardConsole } from "@cpt/shared/ipc/ipc-log";
import { WindowLabel } from "@cpt/shared/ipc/window-label";
import { window } from "@tauri-apps/api";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Window } from "@tauri-apps/api/window";
import { routes } from "./app.routes";
import { IconService } from "@cpt/shared/theme/icon-service";

export const appConfig: ApplicationConfig = {
  providers: [
    provideAppInitializer(() => {
      forwardConsole();
      inject(IconService).setUpMatIconRegistry();
      switch (getCurrentWebviewWindow().label as WindowLabel) {
        case WindowLabel.QUICK_ADD:
          return inject(Todoist).initialize();
        case WindowLabel.AUTHENTICATION:
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
