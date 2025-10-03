import { test, expect } from "@playwright/test";
import LoginPage from "../../../model/auth/LoginPage";
import TwoFaPage from "../../../model/auth/TwoFaPage";

let loginPage: LoginPage;
let twoFaPage: TwoFaPage;

test.beforeEach(async ({ page }) => {
  loginPage = new LoginPage(page);
  await loginPage.navigateToPage();

  await page.route("**/auth/login", (route) =>
    route.fulfill({
      status: 206,
      json: {
        message: "2FA required",
        loginAttemptId: "this-is-a-uuid",
      },
    })
  );

  await loginPage.loginForm().emailInput().fill("foo@bar.com");
  await loginPage.loginForm().passwordInput().fill("password");
  await loginPage.loginForm().loginButton().click();
  twoFaPage = new TwoFaPage(page);
  await twoFaPage.waitForPageLoad();
});

test.describe("Two FA Page", () => {
  test(`Loaded`, async ({ page }) => {
    await expect(page).toHaveScreenshot();
  });
});
