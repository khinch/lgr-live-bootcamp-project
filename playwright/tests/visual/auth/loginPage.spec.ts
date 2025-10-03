import { test, expect } from "@playwright/test";
import LoginPage from "../../../model/auth/LoginPage";

let loginPage: LoginPage;

test.beforeEach(async ({ page }) => {
  loginPage = new LoginPage(page);
});

test.describe("Login Page", () => {
  test(`Loaded`, async ({ page }) => {
    await loginPage.navigateToPage();
    await loginPage.waitForPageLoad();
    await expect(page).toHaveScreenshot();
  });
});
