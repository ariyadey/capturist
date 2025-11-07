import { TestBed } from "@angular/core/testing";

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
