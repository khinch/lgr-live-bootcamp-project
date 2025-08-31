import { type Page } from "@playwright/test";
import { waitForAllToBeVisible } from "../../utils/playwrightUtils";
import LoginPage from "./LoginPage";
import Navbar from "../common/Navbar";

export default class TwoFaPage {
  /**
   * @param {import('playwright').Page} page
   */

  readonly page: Page;

  constructor(page: Page) {
    this.page = page;
  }

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
    await this.page.waitForLoadState("networkidle");
    let navbar: Navbar = new Navbar(this.page);
    let locators = [navbar.title, this.twoFaForm().verifyButton()];
    await waitForAllToBeVisible(locators);
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
