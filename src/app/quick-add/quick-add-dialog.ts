import { CdkTextareaAutosize } from "@angular/cdk/text-field";
import { CommonModule } from "@angular/common";
import {
  ChangeDetectionStrategy,
  Component,
  computed,
  effect,
  ElementRef,
  inject,
  signal,
  viewChild,
} from "@angular/core";
import { NonNullableFormBuilder, ReactiveFormsModule, Validators } from "@angular/forms";
import { MatButton } from "@angular/material/button";
import { MatDialogActions, MatDialogClose, MatDialogContent } from "@angular/material/dialog";
import { MatDivider } from "@angular/material/divider";
import { MatFormField, MatSuffix } from "@angular/material/form-field";
import { MatInput } from "@angular/material/input";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { Todoist } from "@cpt/shared/external/todoist";
import { NativeNotification } from "@cpt/shared/ipc/native-notification";
import { invoke } from "@tauri-apps/api/core";
import { TodoistRequestError } from "@doist/todoist-api-typescript";
import { MatIcon } from "@angular/material/icon";
import { MatTooltip } from "@angular/material/tooltip";
import { toSignal } from "@angular/core/rxjs-interop";
import { from } from "rxjs";

@Component({
  selector: "cpt-quick-add-dialog",
  templateUrl: "./quick-add-dialog.html",
  styleUrl: "./quick-add-dialog.scss",
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    MatDialogContent,
    MatFormField,
    MatDialogActions,
    MatInput,
    ReactiveFormsModule,
    MatButton,
    MatDialogClose,
    CdkTextareaAutosize,
    MatDivider,
    MatProgressSpinnerModule,
    CommonModule,
    MatIcon,
    MatTooltip,
    MatSuffix,
  ],
})
export class QuickAddDialog {
  protected readonly todoistApi = inject(Todoist).api;
  protected readonly notification = inject(NativeNotification);
  protected readonly form = inject(NonNullableFormBuilder).group({
    name: ["", Validators.required],
    description: [""],
  });
  protected readonly isAdding = signal(false);
  protected readonly isWayland = toSignal(from(invoke("is_wayland_session")));
  protected readonly shortcutTooltipText = computed(() => {
    const commonHint = `
      You can manually assign one by setting a script shortcut in OS settings
      executing 'capturist --quick-add' command.
    `;
    return this.isWayland()
      ? `Currently, global shortcut is not working for wayland automatically. ${commonHint}`
      : `The default global shortcut is "Ctrl+Space". ${commonHint}`;
  });
  protected readonly taskNameTextArea = viewChild("taskNameTextArea", {
    read: ElementRef<HTMLTextAreaElement>,
  });

  constructor() {
    effect(() => {
      if (this.isAdding()) {
        this.form.disable();
      } else {
        this.form.enable();
        this.taskNameTextArea()?.nativeElement.focus();
      }
    });
  }

  onSubmit() {
    if (this.form.invalid || this.isAdding()) return;

    this.isAdding.set(true);
    this.todoistApi
      .quickAddTask({
        text: this.form.getRawValue().name,
        note: this.form.getRawValue().description,
        autoReminder: true,
      })
      .then(async (task) => {
        this.form.reset();
        await this.notification.send({ title: "Task added", body: task.url });
      })
      .catch(async (error: TodoistRequestError) => {
        await this.notification.send({ title: "Failed to add task", body: error.message });
        console.error(error);
      })
      .finally(() => {
        this.isAdding.set(false);
      });
  }
}
