import { ComponentFixture, TestBed } from "@angular/core/testing";

import { beforeEach, describe, expect, it } from "vitest";
import { QuickAddDialog } from "./quick-add-dialog";

describe("QuickAdd", () => {
  let component: QuickAddDialog;
  let fixture: ComponentFixture<QuickAddDialog>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [QuickAddDialog],
    }).compileComponents();

    fixture = TestBed.createComponent(QuickAddDialog);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it("should create", () => {
    expect(component).toBeTruthy();
  });
});
