import { test, expect } from "@playwright/test";
import path from "path";

test.describe("Corresponding Individual Hamburger Tests", () => {
  test("[Hamburger-ID-001] Confirm it has project details", async ({
    page,
  }, testInfo) => {
    const projectName = testInfo.project.name;
    const screenshotBase = path.join(
      "screenshots",
      "users",
      projectName,
      "hamburger",
    );

    await page.goto(`/en/`);
    await page.screenshot({
      path: `${screenshotBase}/01-page-entered.png`,
      fullPage: true,
    });

    const hamburger = page.getByRole('button').filter({ hasText: /^$/ }).first();
    await expect(hamburger).toBeVisible();
    await hamburger.click();
    await page.screenshot({
      path: `${screenshotBase}/02-hamburger-clicked.png`,
      fullPage: true,
    });

    const login = page.getByRole('button', { name: 'Login' })
    await expect(login).toBeVisible();
    await page.screenshot({
      path: `${screenshotBase}/03-main-page-clicked.png`,
      fullPage: true,
    });

    await hamburger.click();
    await page.screenshot({
      path: `${screenshotBase}/04-hamburger-clicked.png`,
      fullPage: true,
    });
  });
});
