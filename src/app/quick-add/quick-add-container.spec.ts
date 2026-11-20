import { ComponentFixture, TestBed } from "@angular/core/testing";

import { beforeEach, describe, expect, it } from "vitest";
import { QuickAddContainer } from "./quick-add-container";

describe("QuickAddContainer", () => {
  let component: QuickAddContainer;
  let fixture: ComponentFixture<QuickAddContainer>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [QuickAddContainer],
    }).compileComponents();

    fixture = TestBed.createComponent(QuickAddContainer);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it("should create", () => {
    expect(component).toBeTruthy();
  });
});
