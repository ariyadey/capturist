import { CdkTextareaAutosize } from "@angular/cdk/text-field";
import { ChangeDetectionStrategy, Component, inject } from "@angular/core";
import { NonNullableFormBuilder, ReactiveFormsModule, Validators } from "@angular/forms";
import { MatButton } from "@angular/material/button";
import { MatDialogActions, MatDialogClose, MatDialogContent } from "@angular/material/dialog";
import { MatDivider } from "@angular/material/divider";
import { MatFormField } from "@angular/material/form-field";
import { MatInput } from "@angular/material/input";
import { Todoist } from "@cpt/shared/external/todoist";
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
  ],
})
export class QuickAddDialog {
  protected readonly todoistApi = inject(Todoist).api;
  protected readonly form = inject(NonNullableFormBuilder).group({
    name: ["", Validators.required],
    description: [""],
  });

  onSubmit() {
    this.todoistApi
      .quickAddTask({
        text: this.form.getRawValue().name,
        note: this.form.getRawValue().description,
        autoReminder: true,
      })
      .catch((error: TodoistRequestError) => {
        console.error(error.isAuthenticationError());
        // TODO: 05/09/2025 Show error message
        // TODO: 08/11/2025 ctrl+enter handling
      })
      .then(_ => this.form.reset());
  }
}
