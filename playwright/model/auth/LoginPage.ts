import { type APIRequestContext } from "@playwright/test";
import AppPage from "../app/AppPage";
import BasePage from "../common/BasePage";
import SignupPage from "./SignupPage";
import TwoFaPage from "./TwoFaPage";
import { retrieveTwoFaCode } from "../../utils/apiUtils";

export default class LoginPage extends BasePage {
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
    await this.page.goto("/auth/");
    await this.waitForPageLoad();
  }

  async waitForPageLoad() {
    let locators = [this.navbar.title, this.loginForm().loginButton()];
    await super.waitForPageLoad(locators);
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
