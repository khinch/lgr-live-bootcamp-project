import { defineConfig, devices } from "@playwright/test";

/**
 * Read environment variables from file.
 * https://github.com/motdotla/dotenv
 */
import dotenv from "dotenv";
import path from "path";
dotenv.config({ path: path.resolve(__dirname, "../.env") });

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
  fullyParallel: true,
  // workers: process.env.CI ? 1 : 1,
  forbidOnly: !!process.env.CI,
  reporter: "html",
  testDir: "./tests",
  use: {
    baseURL: "http://localhost:5000",
  },
  snapshotPathTemplate:
    "{testDir}/__snapshots__/{projectName}/{testFilePath}/{arg}{ext}",
  projects: [
    {
      name: "component",
      testDir: "./tests/component",
      use: {
        ...devices["Desktop Firefox"],
      },
    },
    {
      name: "system",
      testDir: "./tests/system",
      use: {
        ...devices["Desktop Firefox"],
      },
    },
    {
      name: "visual-chrome",
      testDir: "./tests/visual",
      use: {
        ...devices["Desktop Chrome"],
        viewport: { width: 577, height: 522 },
      },
    },
    {
      name: "visual-firefox",
      testDir: "./tests/visual",
      use: {
        ...devices["Desktop Firefox"],
        viewport: { width: 577, height: 522 },
      },
    },
    {
      name: "visual-webkit",
      testDir: "./tests/visual",
      use: {
        ...devices["Desktop Safari"],
        viewport: { width: 577, height: 522 },
      },
    },
  ],
});
