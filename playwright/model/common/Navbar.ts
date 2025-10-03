import { type Locator, type Page } from "@playwright/test";

export default class Navbar {
  /**
   * @param {import('playwright').Page} page
   */

  readonly page: Page;
  readonly navbar: Locator;
  readonly title: Locator;
  readonly loginButton: Locator;
  readonly logoutButton: Locator;

  constructor(page: Page) {
    this.page = page;
    this.navbar = page.getByTestId("navbar");
    this.title = this.navbar.getByTestId("title");
    this.loginButton = this.navbar.locator("#login-link");
    this.logoutButton = this.navbar.locator("#logout-link");
  }
}
