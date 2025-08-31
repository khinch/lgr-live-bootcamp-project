import BasePage from "../common/BasePage";
import LoginPage from "./LoginPage";

export default class TwoFaPage extends BasePage {
  /*
   *********************
   *** Page Elements ***
   *********************
   */
  twoFaForm() {
    const twoFaForm = this.page.locator("section[id='2fa-section']");
    return {
      heading: () => twoFaForm.getByTestId("heading"),
      twoFaInput: () => twoFaForm.getByTestId("twoFaInput"),
      verifyButton: () => twoFaForm.locator("button[id='2fa-form-submit']"),
      goBacklabel: () => twoFaForm.getByTestId("noAccountLabel"),
      loginLink: () => twoFaForm.locator("a[id='2fa-login-link']"),
      error: () => twoFaForm.locator("div[id='2fa-err-alert']"),
    };
  }

  /*
   ********************
   *** Page Actions ***
   ********************
   */
  async waitForPageLoad() {
    let locators = [this.navbar.title, this.twoFaForm().verifyButton()];
    super.waitForPageLoad(locators);
  }

  async goToLoginPage() {
    await this.twoFaForm().loginLink().click();
    let loginPage: LoginPage = new LoginPage(this.page);
    loginPage.waitForPageLoad();
  }

  async submitCode(code: string) {
    await this.twoFaForm().twoFaInput().fill(code);
    await this.twoFaForm().verifyButton().click();
  }
}
