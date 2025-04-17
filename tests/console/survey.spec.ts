import { test, expect } from '@playwright/test';
import path from 'path'

const timeouts = {
  wait: parseInt(process.env.WAIT_TIMEOUT || "5000", 10),
  visible: parseInt(process.env.VISIBLE_TIMEOUT || "10000", 10),
  url: parseInt(process.env.URL_TIMEOUT || "15000", 10)
};



test.describe('Survey Page Flow', () => {

  test('[Survey-001] Login and Interact with Surveys', async ({
    page,
  }, testInfo) => {
    const projectName = testInfo.project.name;
    const screenshotBase = path.join(
      "screenshots",
      "console",
      "surveys-001",
      projectName,
      "begin",
    );

    await page.goto('https://console.dev.voice-korea.com/en/');
    await page.screenshot({
      path: `${screenshotBase}/01-login-page.png`,
      fullPage: true
    });

    await page.waitForLoadState('domcontentloaded');

    const emailInput = page.getByRole('textbox').first();
    await expect(emailInput).toBeVisible();
    await emailInput.fill('jesuswrites20043@gmail.com');
    await page.screenshot({
      path: `${screenshotBase}/02-email-filled.png`,
      fullPage: true
    });

    const passwordInput = page.getByRole('textbox').nth(1);
    await expect(passwordInput).toBeVisible();
    await passwordInput.fill('12345678A#');
    await page.screenshot({
      path: `${screenshotBase}/03-password-filled.png`,
      fullPage: true
    });


    const loginButton = page.getByRole('button', { name: "Login" });
    await expect(loginButton).toBeVisible();
    await loginButton.click();
    await page.screenshot({
      path: `${screenshotBase}/04-login-clicked.png`,
      fullPage: true
    });

    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(timeouts.wait);

    // await expect(page).toHaveURL(/.*surveys/, );
    await page.screenshot({
      path: `${screenshotBase}/05-survey-page.png`,
      fullPage: true
    });

    const startSurveyButton = page.getByRole('link', { name: "Start Survey" });
    await expect(startSurveyButton).toBeVisible();
    await startSurveyButton.click();

    await page.screenshot({
      path: `${screenshotBase}/06-survey-started.png`,
      fullPage: true
    });


    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(timeouts.wait);

    await expect(page).toHaveURL('https://console.dev.voice-korea.com/en/surveys/new', );
    await page.screenshot({
      path: `${screenshotBase}/07-survey-questions.png`,
      fullPage: true
    });

  });

  test('[Survey-002] Login, Go to Surveys, and Logout', async ({
    page,
  }, testInfo) => {
    const projectName = testInfo.project.name;
    const screenshotBase = path.join(
      "screenshots",
      "console",
      "surveys-002",
      projectName,
      "begin",
    );
    await page.goto('https://console.dev.voice-korea.com/en/');
    await page.screenshot({
      path: `${screenshotBase}/01-login-page.png`,
      fullPage: true
    });

    await page.waitForLoadState('domcontentloaded');

    const emailInput = page.getByRole('textbox').first();
    await expect(emailInput).toBeVisible();
    await emailInput.fill('jesuswrites20043@gmail.com');
    await page.screenshot({
      path: `${screenshotBase}/02-email-filled.png`,
      fullPage: true
    });

    const passwordInput = page.getByRole('textbox').nth(1);
    await expect(passwordInput).toBeVisible();
    await passwordInput.fill('12345678A#');
    await page.screenshot({
      path: `${screenshotBase}/03-password-filled.png`,
      fullPage: true
    });

    const loginButton = page.getByRole('button', { name: "Login" });
    await expect(loginButton).toBeVisible();
    await loginButton.click();
    await page.screenshot({
      path: `${screenshotBase}/04-login-clicked.png`,
      fullPage: true
    });

    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(timeouts.wait);

    // await expect(page).toHaveURL(/.*surveys/, );
    await page.screenshot({
      path: `${screenshotBase}/05-survey-page.png`,
      fullPage: true
    });

    const logoutButton = page.getByRole('link', { name: "Logout" });
    await expect(logoutButton).toBeVisible();
    await logoutButton.click();
    await page.screenshot({
      path: `${screenshotBase}/06-logout-clicked.png`,
      fullPage: true
    });

    await expect(page).toHaveURL('https://console.dev.voice-korea.com/en/', );
    await page.screenshot({
      path: `${screenshotBase}/07-logged-out.png`,
      fullPage: true
    });
  });

});