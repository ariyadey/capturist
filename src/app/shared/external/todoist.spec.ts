import { TestBed } from "@angular/core/testing";

import { beforeEach, describe, expect, it } from "vitest";
import { Todoist } from "./todoist";

describe("TodoistApiService", () => {
  let service: Todoist;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(Todoist);
  });

  it("should be created", () => {
    expect(service).toBeTruthy();
  });
});
