import { expect } from "@playwright/test";
import BasePage from "../common/BasePage";
import LoginPage from "../auth/LoginPage";

export default class AppPage extends BasePage {
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
    await this.page.goto("/app/");
  }

  async waitForPageLoad() {
    let locators = [this.navbar.title, this.imageContainer().container()];
    await super.waitForPageLoad(locators);
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

  async mockLogin() {
    await this.page.route("**/protected", (route) =>
      route.fulfill({
        status: 200,
        json: {
          img_url:
            "https://i.ibb.co/YP90j68/Light-Live-Bootcamp-Certificate.png",
        },
      })
    );

    await this.navigateToPage();
    await this.waitForPageLoad();
  }

  async mockLogout() {
    await this.page.route("**/protected", (route) =>
      route.fulfill({
        status: 401,
      })
    );

    await this.navigateToPage();
    await this.waitForPageLoad();
  }
}
