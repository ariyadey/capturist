import { CdkTextareaAutosize } from "@angular/cdk/text-field";
import { CommonModule } from "@angular/common";
import {
  ChangeDetectionStrategy,
  Component,
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
import { MatFormField } from "@angular/material/form-field";
import { MatInput } from "@angular/material/input";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { Todoist } from "@cpt/shared/external/todoist";
import { NativeNotification } from "@cpt/shared/ipc/native-notification";
import { TodoistRequestError } from "@doist/todoist-api-typescript";

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
  protected readonly taskNameTextArea = viewChild("taskNameTextArea", {
    read: ElementRef<HTMLTextAreaElement>,
  });

  constructor() {
    effect(() => {
      if (this.isAdding()) {
        this.form.disable();
      } else {
        this.form.enable();
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
        setTimeout(() => this.taskNameTextArea()?.nativeElement.focus());
        await this.notification.send({ title: "Task added", body: task.url });
      })
      .catch(async (error: TodoistRequestError) => {
        await this.notification.send({ title: "Failed to add task", body: error.message });
        console.error(error.message);
      })
      .finally(() => {
        this.isAdding.set(false);
      });
  }
}
