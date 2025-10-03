import { test, expect } from "@playwright/test";
import SignupPage from "../../../model/auth/SignupPage";

let signupPage: SignupPage;

test.beforeEach(async ({ page }) => {
  signupPage = new SignupPage(page);
});

test.describe("Signup Page", () => {
  test(`Loaded`, async ({ page }) => {
    await signupPage.navigateToPage();
    await expect(page).toHaveScreenshot();
  });
});
