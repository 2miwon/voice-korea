import { test, expect } from '@playwright/test';
import path from 'path';

const timeouts = {
  wait: parseInt(process.env.WAIT_TIMEOUT || "5000", 10),
  visible: parseInt(process.env.VISIBLE_TIMEOUT || "10000", 10),
  url: parseInt(process.env.URL_TIMEOUT || "15000", 10)
};



test.describe('Deliberations Page Flow', () => {

  test('[Deliberations-001] Login and Create A Deliberation', async ({
    page,
  }, testInfo) => {
    const projectName = testInfo.project.name;
    const screenshotBase = path.join(
      "screenshots",
      "console",
      "delib-001",
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

    await page.screenshot({
      path: `${screenshotBase}/05-survey-page.png`,
      fullPage: true
    });

    const deliberationSidebarButton = page.getByRole('link', { name: 'Deliberation' });
    await expect(deliberationSidebarButton).toBeVisible();
    await deliberationSidebarButton.click();

    await page.screenshot({
      path: `${screenshotBase}/06-deliberation-page.png`,
      fullPage: true
    });


    expect(page).toHaveURL('https://console.dev.voice-korea.com/en/deliberations')
    const startDeliberationButton = page.getByRole('link', { name: 'Start Deliberation' })
    await expect(startDeliberationButton).toBeVisible()
    await startDeliberationButton.click()

    expect(page).toHaveURL('https://console.dev.voice-korea.com/en/deliberations/new')

    const firstTab = page.getByText('1')
    const secondTab = page.getByText('2')
    const thirdTab = page.getByText('3')
    const fourthTab = page.getByText('4')
    const fifthTab = page.getByText('5')

    await expect(firstTab).toBeVisible()

    const firstTabPageTitle = page.getByText('Deliberation outline', { exact: true })
    await expect(firstTabPageTitle).toBeVisible()

    const projectNameLabel = page.getByText('Project name')
    await expect(projectNameLabel).toBeVisible()

    const projectNameInput = page.getByRole('textbox', { name: 'Please enter the project name.' })
    await expect(projectNameInput).toBeVisible()
    await projectNameInput.fill('New Project 1')


    const introductionLabel = page.getByText('Brief introduction')
    await expect(introductionLabel).toBeVisible()


    const introductionInput = page.getByRole('textbox', { name: 'Please enter a brief' })
    await expect(introductionInput).toBeVisible()
    await introductionInput.fill('First new project')


    const delibFieldLabel = page.getByText('Deliberation field')
    await expect(delibFieldLabel).toBeVisible()


    const delibFieldSelect = page.locator('div').filter({ hasText: /^You can select more than one\.$/ }).first()
    await expect(delibFieldSelect).toBeVisible()
    await delibFieldSelect.click()


    const delibOptionOne = page.getByRole('button', { name: 'Society' })
    await expect(delibOptionOne).toBeVisible()
    await delibOptionOne.click()

    const postClickDelibSelect = page.locator('div').filter({ hasText: /^Society$/ }).nth(1)
    await postClickDelibSelect.click()


    const delibOptionTwo = page.getByRole('button', { name: 'Environment' })
    await expect(delibOptionTwo).toBeVisible()
    await delibOptionTwo.click()


    const clearDelibOptions = page.getByRole('button').nth(3)
    await expect(clearDelibOptions).toBeVisible()
    await clearDelibOptions.click()

    await delibFieldSelect.click()

    await delibOptionOne.click()


    const thumbnailLabel = page.getByText('Thumbnail', { exact: true })
    await expect(thumbnailLabel).toBeVisible()


    const tempSave = page.getByText('Temporary Save')
    await expect(tempSave).toBeVisible()


    const goBackToDelibMainPage = page.getByText('To deliberation management')
    await expect(goBackToDelibMainPage).toBeVisible()


    const toNextPageButton = page.getByText('Next')
    await expect(toNextPageButton).toBeVisible()
    await expect(toNextPageButton).toBeDisabled()

    const emptyThumbnailState = page.getByRole('textbox', { name: 'No file' })
    await expect(emptyThumbnailState).toBeVisible()

    const thumbnailFilePicker = page.getByRole('button', { name: 'Upload directly' })
    await expect(thumbnailFilePicker).toBeVisible()

    await thumbnailFilePicker.setInputFiles('stamp.png')

    await page.waitForTimeout(200000);

    const thumbnailPreview = page.getByRole('img', { name: 'thumbnail preview' })
    await expect(thumbnailPreview).toBeVisible()

    await toNextPageButton.click()



    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(timeouts.wait);
  });


  test('[Deliberations-002] Test Negatives And Errors When Creating A Deliberation', async ({
    page,
  }, testInfo) => {
    const projectName = testInfo.project.name;
    const screenshotBase = path.join(
      "screenshots",
      "console",
      "delib-002",
      projectName,
      "negatives",
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

    await page.screenshot({
      path: `${screenshotBase}/05-survey-page.png`,
      fullPage: true
    });

    const deliberationSidebarButton = page.getByRole('link', { name: 'Deliberation' });
    await expect(deliberationSidebarButton).toBeVisible();
    await deliberationSidebarButton.click();

    await page.screenshot({
      path: `${screenshotBase}/06-deliberation-page.png`,
      fullPage: true
    });


    expect(page).toHaveURL('https://console.dev.voice-korea.com/en/deliberations')
    const startDeliberationButton = page.getByRole('link', { name: 'Start Deliberation' })
    await expect(startDeliberationButton).toBeVisible()
    await startDeliberationButton.click()

    expect(page).toHaveURL('https://console.dev.voice-korea.com/en/deliberations/new')

    const firstTab = page.getByText('1')
    const secondTab = page.getByText('2')
    const thirdTab = page.getByText('3')
    const fourthTab = page.getByText('4')
    const fifthTab = page.getByText('5')

    await expect(firstTab).toBeVisible()

    const firstTabPageTitle = page.getByText('Deliberation outline', { exact: true })
    await expect(firstTabPageTitle).toBeVisible()

    const projectNameLabel = page.getByText('Project name')
    await expect(projectNameLabel).toBeVisible()

    const projectNameInput = page.getByRole('textbox', { name: 'Please enter the project name.' })
    await expect(projectNameInput).toBeVisible()


    const introductionLabel = page.getByText('Brief introduction')
    await expect(introductionLabel).toBeVisible()


    const introductionInput = page.getByRole('textbox', { name: 'Please enter a brief' })
    await expect(introductionInput).toBeVisible()


    const delibFieldLabel = page.getByText('Deliberation field')
    await expect(delibFieldLabel).toBeVisible()


    const delibFieldSelect = page.locator('div').filter({ hasText: /^You can select more than one\.$/ }).first()
    await expect(delibFieldSelect).toBeVisible()

    const thumbnailLabel = page.getByText('Thumbnail', { exact: true })
    await expect(thumbnailLabel).toBeVisible()


    const tempSave = page.getByText('Temporary Save')
    await expect(tempSave).toBeVisible()


    const goBackToDelibMainPage = page.getByText('To deliberation management')
    await expect(goBackToDelibMainPage).toBeVisible()


    const toNextPageButton = page.getByText('Next')
    await expect(toNextPageButton).toBeVisible()
    await expect(toNextPageButton).toBeDisabled()

    const emptyThumbnailState = page.getByRole('textbox', { name: 'No file' })
    await expect(emptyThumbnailState).toBeVisible()

    const thumbnailFilePicker = page.getByRole('button', { name: 'Upload directly' })
    await expect(thumbnailFilePicker).toBeVisible()

    await toNextPageButton.click()

    const projectTitleError = page.getByText('Please enter the project')
    await expect(projectTitleError).toBeVisible()



    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(timeouts.wait);
  });


});