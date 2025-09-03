import { test, expect, Page } from "@playwright/test";
import AppPage from "../../../model/app/AppPage";
import LoginPage from "../../../model/auth/LoginPage";

import appContent from "../../../resources/content/app/appPage.json";
import loginContent from "../../../resources/content/auth/loginPage.json";

let appPage: AppPage;
let loginPage: LoginPage;

test.beforeEach(async ({ page }) => {
  appPage = new AppPage(page);
});

test.describe("App Page Content - Logged Out", () => {
  test.beforeEach(async () => {
    await appPage.navigateToPage();
    await appPage.waitForPageLoad();
  });

  test("Should have correct page title", async ({ page }) => {
    await expect(page).toHaveTitle(appContent.pageTitle);
  });

  test("Should have correct labels", async () => {
    await expect(appPage.navbar.title).toHaveText(appContent.navbar.title);
    await expect(appPage.navbar.loginButton).toHaveText(
      appContent.navbar.loginButton
    );
  });

  test("Should load in correct state", async () => {
    await expect(appPage.navbar.loginButton).toBeVisible();
    await expect(appPage.navbar.logoutButton).not.toBeVisible();
    await expect(appPage.imageContainer().defaultImage()).toBeVisible();
    await expect(appPage.imageContainer().certificateImage()).not.toBeVisible();
  });

  test("Log in button should navigate to Login page", async ({ page }) => {
    await appPage.navbar.loginButton.click();
    loginPage = new LoginPage(page);
    await loginPage.waitForPageLoad();
    await expect(page).toHaveTitle(loginContent.pageTitle);
  });
});

test.describe("App Page Content - Logged In", () => {
  test.beforeEach(async () => {
    await appPage.mockLogin();
  });

  test("Should have correct labels", async () => {
    await expect(appPage.navbar.title).toHaveText(appContent.navbar.title);
    await expect(appPage.navbar.logoutButton).toHaveText(
      appContent.navbar.logoutButton
    );
  });

  test("Should load in correct state", async () => {
    await expect(appPage.navbar.loginButton).not.toBeVisible();
    await expect(appPage.navbar.logoutButton).toBeVisible();
    await expect(appPage.imageContainer().defaultImage()).not.toBeVisible();
    await expect(appPage.imageContainer().certificateImage()).toBeVisible();
  });

  test("Log out button should navigate to App page", async ({ page }) => {
    await page.route("**/auth/logout", (route) =>
      route.fulfill({
        status: 200,
      })
    );

    await appPage.navbar.logoutButton.click();
    await appPage.waitForPageLoad();

    await expect(appPage.navbar.loginButton).toBeVisible();
    await expect(appPage.navbar.logoutButton).not.toBeVisible();
    await expect(appPage.imageContainer().defaultImage()).toBeVisible();
    await expect(appPage.imageContainer().certificateImage()).not.toBeVisible();
  });
});
