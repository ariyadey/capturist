import { inject, Injectable } from "@angular/core";
import { MatIconRegistry } from "@angular/material/icon";
import { DomSanitizer } from "@angular/platform-browser";

/**
 * Service to manage and register custom icons with Angular Material's MatIconRegistry.
 */
@Injectable({
  providedIn: "root",
})
export class IconService {
  private readonly domSanitizer = inject(DomSanitizer);
  private readonly matIconRegistry = inject(MatIconRegistry);

  setUpMatIconRegistry() {
    this.matIconRegistry.addSvgIconResolver((name, namespace) =>
      this.domSanitizer.bypassSecurityTrustResourceUrl(
        namespace.length > 0 ? `icon/${namespace}/${name}.svg` : `icon/${name}.svg`,
      ),
    );
  }
}
