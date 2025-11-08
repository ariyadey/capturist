import { ChangeDetectionStrategy, Component, inject, OnDestroy, OnInit } from "@angular/core";
import { MatDialog } from "@angular/material/dialog";
import { QuickAddDialog } from "@cpt/quick-add/quick-add-dialog";
import { IpcEvent } from "@cpt/shared/ipc/ipc-event";
import { UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { exhaustMap, from, tap } from "rxjs";

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
  protected isQuickAddOpen = false;

  async ngOnInit() {
    this.openQuickAdd();
    const quickAddUnlistenFn = await this.currentWebviewWindow.listen(IpcEvent.QUICK_ADD, (_) =>
      this.openQuickAdd(),
    );
    const blurUnlistenFn = await this.currentWebviewWindow.listen("tauri://blur", (_) =>
      this.currentWebviewWindow.hide(),
    );
    this.unlistenFns.push(quickAddUnlistenFn, blurUnlistenFn);
  }

  ngOnDestroy() {
    this.unlistenFns.forEach((unlistenFn) => unlistenFn());
  }

  protected openQuickAdd() {
    if (this.isQuickAddOpen) {
      return;
    }

    this.isQuickAddOpen = true;
    this.dialog
      .open<QuickAddDialog, null, void>(QuickAddDialog, {
        panelClass: "quick-add-panel",
      })
      .afterClosed()
      .pipe(
        exhaustMap(() => from(getCurrentWebviewWindow().hide())),
        tap(() => (this.isQuickAddOpen = false)),
      )
      .subscribe();
  }
}
