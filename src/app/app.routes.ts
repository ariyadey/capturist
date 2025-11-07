import { inject } from "@angular/core";
import { Route, Router, Routes, UrlSegment } from "@angular/router";
import { AppPath } from "@cpt/app.path";
import { AppWindowLabel } from "@cpt/shared/ipc/app-window-label";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

export const routes: Routes = [
  {
    path: AppPath.ROOT,
    pathMatch: "full",
    redirectTo: _ => {
      switch (getCurrentWebviewWindow().label as AppWindowLabel) {
        case AppWindowLabel.AUTHENTICATION:
          return "landing";
        case AppWindowLabel.QUICK_ADD:
          return "quick-add";
      }
    },
  },
  {
    path: AppPath.LANDING,
    loadComponent: () => import("@cpt/landing-page/landing-page").then(x => x.LandingPage),
    canMatch: [
      (_: Route, __: Array<UrlSegment>) =>
        (getCurrentWebviewWindow().label as AppWindowLabel) === AppWindowLabel.AUTHENTICATION
          ? true
          : inject(Router).parseUrl(`/${AppPath.ROOT}`),
    ],
  },
  {
    path: AppPath.QUICK_ADD,
    loadComponent: () =>
      import("@cpt/quick-add/quick-add-container").then(x => x.QuickAddContainer),
    canMatch: [
      (_: Route, __: Array<UrlSegment>) =>
        (getCurrentWebviewWindow().label as AppWindowLabel) === AppWindowLabel.QUICK_ADD
          ? true
          : inject(Router).parseUrl(`/${AppPath.ROOT}`),
    ],
  },
];
