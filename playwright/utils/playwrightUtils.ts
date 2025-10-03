import { Locator } from "@playwright/test";

export async function waitForAllToBeVisible(
  locators: Locator[],
  timeout: number = 5000
): Promise<void> {
  await Promise.all(
    locators.map((locator) => locator.waitFor({ state: "visible", timeout }))
  );
}

export async function isVisible(locator: Locator): Promise<boolean> {
  return locator.isVisible();
}
