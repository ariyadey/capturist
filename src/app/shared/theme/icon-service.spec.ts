import { TestBed } from "@angular/core/testing";

import { beforeEach, describe, expect, it } from "vitest";
import { IconService } from "./icon-service";

describe("IconService", () => {
  let service: IconService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(IconService);
  });

  it("should be created", () => {
    expect(service).toBeTruthy();
  });
});
