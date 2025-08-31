import { test, expect } from '@playwright/test';
import AppPage from '../../model/app/AppPage';
import LoginPage from '../../model/auth/LoginPage';
import Navbar from '../../model/common/Navbar';
import SignupPage from '../../model/auth/SignupPage';
import { getRandomEmail } from '../../utils/dataUtils';

import appContent from "../../resources/content/app/appPage.json" with { type: "json" };
import loginPageContent from "../../resources/content/auth/loginPage.json" with { type: "json" };
import signupPageContent from "../../resources/content/auth/signupPage.json" with { type: "json" };
import modalContent from "../../resources/content/auth/modal.json" with { type: "json" };

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

test('Signup, login and logout - no 2FA', async ({ request }) => {
  await appPage.assertLoggedOut();

  await appPage.goToLoginPage();
  await loginPage.goToSignupPage();

  let email = getRandomEmail("testwebsitepleaseignore.uk");
  let password = "password";
  let twoFa = false;

  await signupPage.doSignup(email, password, twoFa);
  await loginPage.doLogin(request, email, password, twoFa);
  await appPage.assertLoggedIn();

  await navbar.logoutButton.click();
  await appPage.assertLoggedOut();
});

test('Signup, login and logout - 2FA enabled', async ({ request }) => {
  test.setTimeout(120_000); // Postmark takes time to log 2FA requests

  await appPage.assertLoggedOut();

  await appPage.goToLoginPage();
  await loginPage.goToSignupPage();

  let email = getRandomEmail("testwebsitepleaseignore.uk");
  let password = "password";
  let twoFa = true;

  await signupPage.doSignup(email, password, twoFa);
  await loginPage.doLogin(request, email, password, twoFa);
  await appPage.assertLoggedIn();

  await navbar.logoutButton.click();
  await appPage.assertLoggedOut();
});