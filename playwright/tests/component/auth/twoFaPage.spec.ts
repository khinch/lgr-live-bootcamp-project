import { test, expect, Page } from "@playwright/test";
import AppPage from "../../../model/app/AppPage";
import LoginPage from "../../../model/auth/LoginPage";
import TwoFaPage from "../../../model/auth/TwoFaPage";

import twoFaContent from "../../../resources/content/auth/twoFaPage.json";

let appPage: AppPage;
let loginPage: LoginPage;
let twoFaPage: TwoFaPage;

test.beforeEach(async ({ page }) => {
  loginPage = new LoginPage(page);
  await loginPage.navigateToPage();
  await loginPage.waitForPageLoad();

  await page.route("**/auth/login", (route) =>
    route.fulfill({
      status: 206,
      json: {
        message: "2FA required",
        loginAttemptId: "this-is-a-uuid",
      },
    })
  );

  await completeLogin("foo@bar.com", "password");

  twoFaPage = new TwoFaPage(page);
  await twoFaPage.waitForPageLoad();
});

test.describe("Two FA Page - Content", () => {
  test("Should have correct page title", async ({ page }) => {
    await expect(page).toHaveTitle(twoFaContent.pageTitle);
  });

  test("Should have correct labels", async () => {
    await expect(twoFaPage.navbar.title).toHaveText(twoFaContent.navbar.title);
    await expect(twoFaPage.twoFaForm().heading()).toHaveText(
      twoFaContent.twoFaForm.heading
    );
    await expect(twoFaPage.twoFaForm().twoFaInput()).toHaveAttribute(
      "placeholder",
      twoFaContent.twoFaForm.twoFaPlaceholder
    );
    await expect(twoFaPage.twoFaForm().verifyButton()).toHaveText(
      twoFaContent.twoFaForm.verifyButtonText
    );
    await expect(twoFaPage.twoFaForm().goBacklabel()).toHaveText(
      twoFaContent.twoFaForm.goBackLabel
    );
    await expect(twoFaPage.twoFaForm().loginLink()).toHaveText(
      twoFaContent.twoFaForm.loginLinkText
    );
  });

  test("Should load in correct state", async () => {
    await expect(twoFaPage.twoFaForm().heading()).toBeVisible();
    await expect(twoFaPage.twoFaForm().twoFaInput()).toBeVisible();
    await expect(twoFaPage.twoFaForm().twoFaInput()).toHaveText("");
    await expect(twoFaPage.twoFaForm().verifyButton()).toBeVisible();
    await expect(twoFaPage.twoFaForm().verifyButton()).toBeEnabled();
    await expect(twoFaPage.twoFaForm().goBacklabel()).toBeVisible();
    await expect(twoFaPage.twoFaForm().loginLink()).toBeVisible();
    await expect(twoFaPage.twoFaForm().loginLink()).toBeEnabled();
    await expect(twoFaPage.twoFaForm().error()).not.toBeVisible();
  });
});

test.describe("Two FA Page - Functions", () => {
  test("Log in button should navigate to Login page", async ({ page }) => {
    await twoFaPage.twoFaForm().loginLink().click();
    await loginPage.waitForPageLoad();
    await expect(loginPage.loginForm().loginButton()).toBeVisible();
  });

  test("Successful two FA submission should load App page", async ({
    page,
  }) => {
    let email: string = "foo@bar.com";

    await page.route("**/auth/verify-2fa", async (route) => {
      const request = route.request();
      const body = request.postData();
      const json = JSON.parse(body);
      expect(json.email).toBe(email);
      expect(json.loginAttemptId).toBe("this-is-a-uuid");
      expect(json["2FACode"]).toBe("654321");

      await route.fulfill({
        status: 200,
      });
    });

    await page.route("**/protected", (route) =>
      route.fulfill({
        status: 200,
        json: {
          img_url:
            "https://i.ibb.co/YP90j68/Light-Live-Bootcamp-Certificate.png",
        },
      })
    );

    await twoFaPage.twoFaForm().twoFaInput().fill("654321");
    await twoFaPage.twoFaForm().verifyButton().click();

    appPage = new AppPage(page);
    await appPage.waitForPageLoad();
    await expect(appPage.imageContainer().certificateImage()).toBeVisible();
  });
});

test.describe("Two FA Page - Error Handling", () => {
  test("Should handle HTTP400 errors", async ({ page }) => {
    await page.route("**/auth/verify-2fa", (route) =>
      route.fulfill({
        status: 400,
        json: { error: "Invalid input" },
      })
    );

    await submitTwoFa("1");

    await expect(twoFaPage.twoFaForm().error()).toBeVisible();
    await expect(twoFaPage.twoFaForm().error()).toHaveText(
      twoFaContent.errorMessages.invalidCode
    );
  });

  test("Should handle HTTP401 errors", async ({ page }) => {
    await page.route("**/auth/verify-2fa", (route) =>
      route.fulfill({
        status: 401,
        json: { error: "Incorrect credentials" },
      })
    );

    await submitTwoFa("654321");

    await expect(twoFaPage.twoFaForm().error()).toBeVisible();
    await expect(twoFaPage.twoFaForm().error()).toHaveText(
      twoFaContent.errorMessages.wrongCode
    );
  });
});

async function completeLogin(email: string, password: string) {
  await loginPage.loginForm().emailInput().fill(email);
  await loginPage.loginForm().passwordInput().fill(password);
  await loginPage.loginForm().loginButton().click();
}

async function submitTwoFa(twoFaCode: string) {
  await twoFaPage.twoFaForm().twoFaInput().fill(twoFaCode);
  await twoFaPage.twoFaForm().verifyButton().click();
}
