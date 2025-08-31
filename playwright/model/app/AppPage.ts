import { expect, type Page } from "@playwright/test";
import { waitForAllToBeVisible } from "../../utils/playwrightUtils";
import LoginPage from "../auth/LoginPage";
import Navbar from "../common/Navbar";

export default class AppPage {
  /**
   * @param {import('playwright').Page} page
   */

  readonly page: Page;
  readonly navbar: Navbar;

  constructor(page: Page) {
    this.page = page;
    this.navbar = new Navbar(page);
  }

  /*
   *********************
   *** Page Elements ***
   *********************
   */
  imageContainer() {
    const container = this.page.getByTestId("imageContainer");
    return {
      container: () => container,
      defaultImage: () => container.getByTestId("defaultImage"),
      certificateImage: () => container.getByTestId("certificateImage"),
    };
  }

  /*
   ********************
   *** Page Actions ***
   ********************
   */
  async navigateToPage() {
    await this.page.goto("http://localhost:5000/app/");
  }

  async waitForPageLoad() {
    await this.page.waitForLoadState("networkidle");
    let locators = [this.navbar.title, this.imageContainer().container()];
    await waitForAllToBeVisible(locators);
    await this.page.waitForLoadState("networkidle");
  }

  async goToLoginPage() {
    await this.navbar.loginButton.click();
    let loginPage = new LoginPage(this.page);
    await loginPage.waitForPageLoad();
  }

  async assertLoggedOut() {
    await expect(this.imageContainer().defaultImage()).toBeVisible();
    await expect(this.imageContainer().certificateImage()).not.toBeVisible();
    await expect(this.navbar.logoutButton).not.toBeVisible();
    await this.page.waitForLoadState("networkidle");
  }

  async assertLoggedIn() {
    await expect(this.imageContainer().defaultImage()).not.toBeVisible();
    await expect(this.imageContainer().certificateImage()).toBeVisible();
    await expect(this.navbar.logoutButton).toBeVisible();
    await this.page.waitForLoadState("networkidle");
  }
}
