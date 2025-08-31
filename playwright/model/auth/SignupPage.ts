import BasePage from "../common/BasePage";
import LoginPage from "../auth/LoginPage";

export default class SignupPage extends BasePage {
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
    let locators = [this.navbar.title, this.signupForm().signupButton()];
    super.waitForPageLoad(locators);
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
