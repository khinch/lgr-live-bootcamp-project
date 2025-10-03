import { test, expect, Page } from "@playwright/test";
import AppPage from "../../../model/app/AppPage";
import LoginPage from "../../../model/auth/LoginPage";
import SignupPage from "../../../model/auth/SignupPage";

import loginContent from "../../../resources/content/auth/loginPage.json";

let appPage: AppPage;
let loginPage: LoginPage;
let signupPage: SignupPage;

test.beforeEach(async ({ page }) => {
  loginPage = new LoginPage(page);
  await loginPage.navigateToPage();
  await loginPage.waitForPageLoad();
});

test.describe("Login Page - Content", () => {
  test("Should have correct page title", async ({ page }) => {
    await expect(page).toHaveTitle(loginContent.pageTitle);
  });

  test("Should have correct labels", async () => {
    await expect(loginPage.navbar.title).toHaveText(loginContent.navbar.title);
    await expect(loginPage.loginForm().heading()).toHaveText(
      loginContent.loginForm.heading
    );
    await expect(loginPage.loginForm().emailInput()).toHaveAttribute(
      "placeholder",
      loginContent.loginForm.emailPlaceholder
    );
    await expect(loginPage.loginForm().passwordInput()).toHaveAttribute(
      "placeholder",
      loginContent.loginForm.passwordPlaceholder
    );
    await expect(loginPage.loginForm().loginButton()).toHaveText(
      loginContent.loginForm.loginButtonText
    );
    await expect(loginPage.loginForm().noAccountlabel()).toHaveText(
      loginContent.loginForm.noAccountLabel
    );
    await expect(loginPage.loginForm().signupLink()).toHaveText(
      loginContent.loginForm.SignUpLinkText
    );
  });

  test("Should load in correct state", async () => {
    await expect(loginPage.loginForm().heading()).toBeVisible();
    await expect(loginPage.loginForm().emailInput()).toBeVisible();
    await expect(loginPage.loginForm().emailInput()).toHaveText("");
    await expect(loginPage.loginForm().passwordInput()).toBeVisible();
    await expect(loginPage.loginForm().passwordInput()).toHaveText("");
    await expect(loginPage.loginForm().loginButton()).toBeVisible();
    await expect(loginPage.loginForm().loginButton()).toBeEnabled();
    await expect(loginPage.loginForm().noAccountlabel()).toBeVisible();
    await expect(loginPage.loginForm().signupLink()).toBeVisible();
    await expect(loginPage.loginForm().signupLink()).toBeEnabled();
    await expect(loginPage.loginForm().error()).not.toBeVisible();
  });
});

test.describe("Login Page - Functions", () => {
  test("Sign up button should navigate to Signup page", async ({ page }) => {
    await loginPage.loginForm().signupLink().click();
    signupPage = new SignupPage(page);
    await signupPage.waitForPageLoad();
    await expect(signupPage.signupForm().signupButton()).toBeVisible();
  });

  test("Successful log in should load App page", async ({ page }) => {
    let email: string = "foo@bar.com";
    let password: string = "password";

    await page.route("**/auth/login", async (route) => {
      const request = route.request();
      const body = request.postData();
      const json = JSON.parse(body);
      expect(json.email).toBe(email);
      expect(json.password).toBe(password);

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

    await loginPage.loginForm().emailInput().fill(email);
    await loginPage.loginForm().passwordInput().fill(password);
    await loginPage.loginForm().loginButton().click();

    appPage = new AppPage(page);
    await appPage.waitForPageLoad();
    await expect(appPage.imageContainer().certificateImage()).toBeVisible();
  });
});

test.describe("Login Page - Error Handling", () => {
  test("Should handle HTTP400 errors", async ({ page }) => {
    await page.route("**/auth/login", (route) =>
      route.fulfill({
        status: 400,
        json: { error: "Invalid input" },
      })
    );

    await completeLogin("foo", "password");

    await expect(loginPage.loginForm().error()).toBeVisible();
    await expect(loginPage.loginForm().error()).toHaveText(
      loginContent.errorMessages.invalidCredentials
    );
  });

  test("Should handle HTTP401 errors", async ({ page }) => {
    await page.route("**/auth/login", (route) =>
      route.fulfill({
        status: 401,
        json: { error: "Incorrect credentials" },
      })
    );

    await completeLogin("foo@bar.com", "drowssap");

    await expect(loginPage.loginForm().error()).toBeVisible();
    await expect(loginPage.loginForm().error()).toHaveText(
      loginContent.errorMessages.wrongCredentials
    );
  });
});

async function completeLogin(email: string, password: string) {
  await loginPage.loginForm().emailInput().fill(email);
  await loginPage.loginForm().passwordInput().fill(password);
  await loginPage.loginForm().loginButton().click();
}
