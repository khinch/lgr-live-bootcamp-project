import { test, expect, Page } from "@playwright/test";
import LoginPage from "../../../model/auth/LoginPage";
import SignupPage from "../../../model/auth/SignupPage";

import signupContent from "../../../resources/content/auth/signupPage.json";

let loginPage: LoginPage;
let signupPage: SignupPage;

test.beforeEach(async ({ page }) => {
  signupPage = new SignupPage(page);
  loginPage = new LoginPage(page);
  await loginPage.navigateToPage();
  await loginPage.waitForPageLoad();
  await loginPage.loginForm().signupLink().click();
  await signupPage.waitForPageLoad();
});

test.describe("Signup Page - Content", () => {
  test("Should have correct page title", async ({ page }) => {
    await expect(page).toHaveTitle(signupContent.pageTitle);
  });

  test("Should have correct labels", async () => {
    await expect(signupPage.navbar.title).toHaveText(
      signupContent.navbar.title
    );
    await expect(signupPage.signupForm().heading()).toHaveText(
      signupContent.signupForm.heading
    );
    await expect(signupPage.signupForm().emailInput()).toHaveAttribute(
      "placeholder",
      signupContent.signupForm.emailPlaceholder
    );
    await expect(signupPage.signupForm().passwordInput()).toHaveAttribute(
      "placeholder",
      signupContent.signupForm.passwordPlaceholder
    );
    await expect(signupPage.signupForm().twoFaLabel()).toHaveText(
      signupContent.signupForm.twoFaLabel
    );
    await expect(signupPage.signupForm().signupButton()).toHaveText(
      signupContent.signupForm.signupButtonText
    );
    await expect(signupPage.signupForm().alreadyHaveAccountlabel()).toHaveText(
      signupContent.signupForm.alreadyHaveAccountLabel
    );
    await expect(signupPage.signupForm().loginLink()).toHaveText(
      signupContent.signupForm.LoginLinkText
    );
  });

  test("Should load in correct state", async () => {
    await expect(signupPage.signupForm().heading()).toBeVisible();
    await expect(signupPage.signupForm().emailInput()).toBeVisible();
    await expect(signupPage.signupForm().emailInput()).toHaveText("");
    await expect(signupPage.signupForm().passwordInput()).toBeVisible();
    await expect(signupPage.signupForm().passwordInput()).toHaveText("");
    await expect(signupPage.signupForm().signupButton()).toBeVisible();
    await expect(signupPage.signupForm().signupButton()).toBeEnabled();
    await expect(
      signupPage.signupForm().alreadyHaveAccountlabel()
    ).toBeVisible();
    await expect(signupPage.signupForm().loginLink()).toBeVisible();
    await expect(signupPage.signupForm().loginLink()).toBeEnabled();
    await expect(signupPage.signupForm().error()).not.toBeVisible();
  });
});

test.describe("Signup Page - Functions", () => {
  test("Log in link should navigate to Login page", async ({ page }) => {
    await signupPage.signupForm().loginLink().click();
    await loginPage.waitForPageLoad();
    await expect(loginPage.loginForm().loginButton()).toBeVisible();
  });

  for (const twoFaValue of [true, false]) {
    test(`Successful sign up should navigate to Login page: 2fa ${twoFaValue}`, async ({
      page,
    }) => {
      let email: string = "foo@bar.com";
      let password: string = "password";

      await page.route("**/auth/signup", async (route) => {
        const request = route.request();
        const body = request.postData();
        const json = JSON.parse(body);
        expect(json.email).toBe(email);
        expect(json.password).toBe(password);
        expect(json.requires2FA).toBe(twoFaValue);

        await route.fulfill({
          status: 201,
          json: { message: "User created successfully!" },
        });
      });

      await signupPage.signupForm().emailInput().fill(email);
      await signupPage.signupForm().passwordInput().fill(password);
      if (twoFaValue) {
        await signupPage.signupForm().twoFaCheckbox().check();
      }
      await signupPage.signupForm().signupButton().click();

      await loginPage.waitForPageLoad();
      await expect(loginPage.loginForm().loginButton()).toBeVisible();
    });
  }
});

test.describe("Signup Page - Error Handling", () => {
  test("Should handle HTTP400 errors", async ({ page }) => {
    await page.route("**/auth/signup", (route) =>
      route.fulfill({
        status: 400,
        json: { error: "Invalid input" },
      })
    );

    await completeForm("foo", "password", false);

    await expect(signupPage.signupForm().error()).toBeVisible();
    await expect(signupPage.signupForm().error()).toHaveText(
      signupContent.errorMessages.invalidCredentials
    );
  });

  test("Should handle HTTP409 errors", async ({ page }) => {
    await page.route("**/auth/signup", (route) =>
      route.fulfill({
        status: 409,
        json: { error: "User already exists" },
      })
    );

    await completeForm("foo@bar.com", "password", false);

    await expect(signupPage.signupForm().error()).toBeVisible();
    await expect(signupPage.signupForm().error()).toHaveText(
      signupContent.errorMessages.userExists
    );
  });
});

async function completeForm(email: string, password: string, twoFa: boolean) {
  await signupPage.signupForm().emailInput().fill(email);
  await signupPage.signupForm().passwordInput().fill(password);
  if (twoFa) {
    signupPage.signupForm().twoFaCheckbox().check();
  }
  await signupPage.signupForm().signupButton().click();
}
