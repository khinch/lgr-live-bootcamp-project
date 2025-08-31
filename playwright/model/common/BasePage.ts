import { Locator, Page } from "@playwright/test";
import Navbar from "./Navbar";
import { waitForAllToBeVisible } from "../../utils/playwrightUtils";

export default abstract class BasePage {
  /**
   * @param {import('playwright').Page} page
   */

  protected readonly page: Page;
  protected readonly navbar: Navbar;

  constructor(page: Page) {
    this.page = page;
    this.navbar = new Navbar(page);
  }

  async waitForPageLoad(locators: Locator[]): Promise<void> {
    await this.page.waitForLoadState("networkidle");
    await waitForAllToBeVisible(locators);
    await this.page.waitForLoadState("networkidle");
  }
}
