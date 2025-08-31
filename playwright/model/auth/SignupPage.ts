import { expect, type Locator, type Page } from "@playwright/test";
import { waitForAllToBeVisible } from "../../utils/playwrightUtils";
import LoginPage from "../auth/LoginPage";
import Navbar from "../common/Navbar";

export default class SignupPage {
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
  signupForm() {
    const signupForm = this.page.locator("#signup-section");
    return {
      heading: () => signupForm.getByTestId("heading"),
      emailInput: () => signupForm.getByTestId("emailInput"),
      passwordInput: () => signupForm.getByTestId("passwordInput"),
      twoFaCheckbox: () => signupForm.locator("input[id='2FA-checkbox']"),
      twoFaLabel: () => signupForm.locator("label[for='2FA-checkbox']"),
      signupButton: () => signupForm.locator("#signup-form-submit"),
      alreadyHaveAccountlabel: () =>
        signupForm.getByTestId("alreadyHaveAccountLabel"),
      loginLink: () => signupForm.locator("#signup-login-link"),
      error: () => signupForm.locator("#login-err-alert"),
    };
  }

  /*
   ********************
   *** Page Actions ***
   ********************
   */
  async waitForPageLoad() {
    await this.page.waitForLoadState("networkidle");
    let locators = [this.navbar.title, this.signupForm().signupButton()];
    await waitForAllToBeVisible(locators);
  }

  async goToLoginPage() {
    await this.signupForm().loginLink().click();
    let loginPage: LoginPage = new LoginPage(this.page);
    await loginPage.waitForPageLoad();
  }

  async doSignup(username: string, password: string, twoFa: boolean) {
    await this.signupForm().emailInput().fill(username);
    await this.signupForm().passwordInput().fill(password);
    if (twoFa) {
      await this.signupForm().twoFaCheckbox().check();
    }
    await this.signupForm().signupButton().click();
    let loginPage: LoginPage = new LoginPage(this.page);
    await loginPage.waitForPageLoad();
  }
}
