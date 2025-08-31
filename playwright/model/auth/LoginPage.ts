import { type APIRequestContext, type Page } from "@playwright/test";
import { waitForAllToBeVisible } from "../../utils/playwrightUtils";
import AppPage from "../app/AppPage";
import Navbar from "../common/Navbar";
import SignupPage from "./SignupPage";
import TwoFaPage from "./TwoFaPage";
import { retrieveTwoFaCode } from "../../utils/apiUtils";

export default class LoginPage {
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
  loginForm() {
    const loginForm = this.page.locator("#login-section");
    return {
      heading: () => loginForm.getByTestId("heading"),
      emailInput: () => loginForm.getByTestId("emailInput"),
      passwordInput: () => loginForm.getByTestId("passwordInput"),
      loginButton: () => loginForm.locator("#login-form-submit"),
      noAccountlabel: () => loginForm.getByTestId("noAccountLabel"),
      signupLink: () => loginForm.locator("#signup-link"),
      error: () => loginForm.locator("#login-err-alert"),
    };
  }

  /*
   ********************
   *** Page Actions ***
   ********************
   */
  async navigateToPage() {
    await this.page.goto("http://localhost:5000/auth/");
  }

  async waitForPageLoad() {
    await this.page.waitForLoadState("networkidle");
    let locators = [this.navbar.title, this.loginForm().loginButton()];
    await waitForAllToBeVisible(locators);
  }

  async goToSignupPage() {
    await this.loginForm().signupLink().click();
    let signupPage: SignupPage = new SignupPage(this.page);
    signupPage.waitForPageLoad();
  }

  async doLogin(
    request: APIRequestContext,
    username: string,
    password: string,
    twoFa: boolean
  ) {
    await this.loginForm().emailInput().fill(username);
    await this.loginForm().passwordInput().fill(password);
    await this.loginForm().loginButton().click();

    if (twoFa) {
      let twoFaPage: TwoFaPage = new TwoFaPage(this.page);
      await twoFaPage.waitForPageLoad();
      const twoFaCode = await retrieveTwoFaCode(request, username);
      await twoFaPage.submitCode(twoFaCode);
    }

    let appPage: AppPage = new AppPage(this.page);
    await appPage.waitForPageLoad();
  }
}
