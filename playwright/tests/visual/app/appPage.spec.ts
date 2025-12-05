import { test, expect } from "@playwright/test";
import AppPage from "../../../model/app/AppPage";

let appPage: AppPage;

test.beforeEach(async ({ page }) => {
  appPage = new AppPage(page);
});

const viewportWidths = [575, 576];

for (const width of viewportWidths) {
  test.describe("App Page - Narrow viewport boundary", () => {
    test.use({ viewport: { width, height: 522 } });

    test(`Logged Out - Width: ${width}`, async ({ page }) => {
      await appPage.mockLogout();
      await expect(page).toHaveScreenshot();
    });

    test(`Logged In - Width: ${width}`, async ({ page }) => {
      await appPage.mockLogin();
      await expect(page).toHaveScreenshot();
    });
  });
}
