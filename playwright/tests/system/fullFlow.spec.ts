import { test } from "@playwright/test";
import AppPage from "../../model/app/AppPage";
import LoginPage from "../../model/auth/LoginPage";
import Navbar from "../../model/common/Navbar";
import SignupPage from "../../model/auth/SignupPage";
import { getRandomEmail } from "../../utils/dataUtils";

let appPage: AppPage;
let loginPage: LoginPage;
let navbar: Navbar;
let signupPage: SignupPage;

test.beforeEach(async ({ page }) => {
  appPage = new AppPage(page);
  loginPage = new LoginPage(page);
  navbar = new Navbar(page);
  signupPage = new SignupPage(page);

  await appPage.navigateToPage();
  await appPage.waitForPageLoad();
});

const twoFaValues = [false, true];

for (const twoFaValue of twoFaValues) {
  test(`Signup, login and logout - 2FA=${twoFaValue}`, async ({ request }) => {
    await appPage.assertLoggedOut();

    await appPage.goToLoginPage();
    await loginPage.goToSignupPage();

    let email = getRandomEmail("testwebsitepleaseignore.uk");
    let password = "password";

    await signupPage.doSignup(email, password, twoFaValue);
    await loginPage.doLogin(request, email, password, twoFaValue);
    await appPage.assertLoggedIn();

    await navbar.logoutButton.click();
    await appPage.assertLoggedOut();
  });
}
