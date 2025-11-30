import { ChangeDetectionStrategy, Component, inject, OnDestroy, OnInit } from "@angular/core";
import { MatDialog, MatDialogState } from "@angular/material/dialog";
import { QuickAddDialog } from "@cpt/quick-add/quick-add-dialog";
import { UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { exhaustMap, from, tap } from "rxjs";
import { WindowLabel } from "@cpt/shared/ipc/window-label";

@Component({
  selector: "cpt-quick-add-container",
  template: ``,
  styles: ``,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [],
})
export class QuickAddContainer implements OnInit, OnDestroy {
  protected readonly dialog = inject(MatDialog);
  protected readonly currentWebviewWindow = getCurrentWebviewWindow();
  protected readonly unlistenFns = Array<UnlistenFn>();

  async ngOnInit() {
    this.openQuickAdd();

    const quickAddUnlistenFn = await this.currentWebviewWindow.listen("tauri://move", (event) => {
      console.log(`Event ${event} received.`);
      this.openQuickAdd();
    });
    this.unlistenFns.push(quickAddUnlistenFn);

    const blurUnlistenFn = await this.currentWebviewWindow.listen("tauri://blur", (event) => {
      console.log(`Event ${event} received.`);
      this.dialog.getDialogById(WindowLabel.QUICK_ADD)?.close();
    });
    this.unlistenFns.push(blurUnlistenFn);
  }

  ngOnDestroy() {
    this.unlistenFns.forEach((unlistenFn) => unlistenFn());
  }

  protected openQuickAdd() {
    console.log("Opening Quick-Add dialog...");

    if (this.dialog.getDialogById(WindowLabel.QUICK_ADD)?.getState() === MatDialogState.OPEN) {
      console.log("Quick-Add dialog is already open.");
      return;
    }

    this.dialog
      .open<QuickAddDialog, null, void>(QuickAddDialog, {
        id: WindowLabel.QUICK_ADD,
        panelClass: "quick-add-panel",
      })
      .afterClosed()
      .pipe(
        exhaustMap(() => from(getCurrentWebviewWindow().hide())),
        tap(() => console.log("Quick-Add dialog closed.")),
      )
      .subscribe();
  }
}
