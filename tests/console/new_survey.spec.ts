import { test, expect } from '@playwright/test';
import path from 'path';

const timeouts = {
    wait: parseInt(process.env.WAIT_TIMEOUT || "2000", 10),
    visible: parseInt(process.env.VISIBLE_TIMEOUT || "5000", 10),
    url: parseInt(process.env.URL_TIMEOUT || "7000", 10)
};



test.describe('New Survey Page', () => {

    test('[Survey-003] Verify Fields, Errors, and Interactions', async ({
        page,
      }, testInfo) => {
        const projectName = testInfo.project.name;
        const screenshotBase = path.join(
          "screenshots",
          "console",
          "surveys-003",
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

        // await expect(page).toHaveURL("https://console.dev.voice-korea.com/en/surveys", );
        await page.screenshot({ 
            path: `${screenshotBase}/05-survey-page.png`, 
            fullPage: true 
        });

        const startSurveyButton = page.getByRole('link', { name: "Start Survey" });
        await expect(startSurveyButton).toBeVisible();
        await startSurveyButton.click();
        await page.screenshot({ path: 'screenshots/console/NewSurvey-001/06-survey-started.png', fullPage: true });

        await page.waitForLoadState('networkidle');
        await page.waitForTimeout(timeouts.wait);

        await expect(page).toHaveURL('https://console.dev.voice-korea.com/en/surveys/new', );
        await page.screenshot({ path: 'screenshots/console/NewSurvey-001/07-survey-questions.png', fullPage: true });

        await page.screenshot({ 
            path: `${screenshotBase}/06-new-survey-page.png`, 
            fullPage: true 
        });

        // const back = page.getByRole('link').filter({ hasText: /^$/ })
        // await expect(back).toBeVisible();

        const categoryDropdown = await page.getByRole('combobox')
        await expect(categoryDropdown).toBeVisible();
        await categoryDropdown.selectOption('environment')

        const titleInput = page.getByRole('textbox', { name: 'Please enter a description' }).first()
        await expect(titleInput).toBeVisible();

        const startDatePicker = page.getByRole('button', { name: "/04/16" })
        await expect(startDatePicker).toBeVisible();

        const endDatePicker = page.getByRole('button', { name: "/04/17" })
        await expect(endDatePicker).toBeVisible();

        const descriptionInput = page.getByRole('textbox', { name: 'Please enter a description' }).nth(1)
        await expect(descriptionInput).toBeVisible();

        await page.screenshot({ 
            path: `${screenshotBase}/07-all-fields-visible.png`, 
            fullPage: true 
        });


        const costAndReward = page.getByText('Expected Cost and RewardsPlease enter the estimated time it will take to')
        await expect(costAndReward).toBeVisible()
        await costAndReward.click();

        const estimatedTime = page.getByText('Estimated Time', { exact: true })
        await expect(estimatedTime).toBeVisible()

        const estimatedPaymentPoints = page.getByText('Enter payment points when')
        await expect(estimatedPaymentPoints).toBeVisible()

        const estimatedTimeInput = page.getByRole('textbox', { name: 'Enter estimated time' })
        await expect(estimatedTimeInput).toBeVisible()
        await estimatedTimeInput.fill('4')

        const estimatedPaymentPointsInput = page.getByRole('textbox', { name: 'Enter point' })
        await expect(estimatedPaymentPointsInput).toBeVisible()
        await estimatedPaymentPointsInput.fill('4')

        const toAddNewQuestionButton = page.getByRole('button', { name: 'Please add a new question.' })
        await expect(toAddNewQuestionButton).toBeVisible()
        await toAddNewQuestionButton.click()


        const firstQuestionType = page.locator('div').filter({ hasText: /^Single ChoiceMultiple ChoiceShort AnswerSubjective$/ }).getByRole('combobox')
        await expect(firstQuestionType).toBeVisible()
        await firstQuestionType.selectOption('Multiple Choice')

        const firstQuestionTypeDescriptionInput = page.getByRole('textbox', { name: 'Please Input Description' }).nth(2)
        await expect(firstQuestionTypeDescriptionInput).toBeVisible()
        await firstQuestionTypeDescriptionInput.fill('A Multiple Question Type')

        const firstQuestionTypeTitleInput = page.getByRole('textbox', { name: 'Please enter a title.' })
        await expect(firstQuestionTypeTitleInput).toBeVisible()
        await firstQuestionTypeTitleInput.fill('My Multiple Question')

        const initiateAddOptions = page.getByRole('button', { name: 'Add Option' })
        await expect(initiateAddOptions).toBeVisible()
        await initiateAddOptions.click()

        const firstMultipleOption = page.getByRole('textbox', { name: 'Option' })
        await expect(firstMultipleOption).toBeVisible()
        await firstMultipleOption.fill("So happy")

        await initiateAddOptions.click()

        const secondMultipleOption = page.getByRole('textbox', { name: 'Option 2' })
        await expect(secondMultipleOption).toBeVisible()
        await secondMultipleOption.fill("still happy")

        await toAddNewQuestionButton.click()

        const secondQuestionType = page.getByRole('combobox').nth(2)
        await expect(secondQuestionType).toBeVisible()
        await secondQuestionType.selectOption('Single Choice')

        const secondQuestionTypeDescriptionInput = page.getByRole('textbox', { name: 'Please Input Description' }).nth(3)
        await expect(secondQuestionTypeDescriptionInput).toBeVisible()
        await secondQuestionTypeDescriptionInput.fill('A Single Question Type')

        const secondQuestionTypeTitleInput = page.getByRole('textbox', { name: 'Please enter a title.' }).nth(1)
        await expect(secondQuestionTypeTitleInput).toBeVisible()
        await secondQuestionTypeTitleInput.fill('My Single Question')

        const secondInitiateAddOptions = page.getByRole('button', { name: 'Add Option' }).nth(1)
        await expect(secondInitiateAddOptions).toBeVisible()
        await secondInitiateAddOptions.click()

        const firstSingleOption =  page.getByRole('textbox', { name: 'Option' }).nth(2)
        await expect(firstSingleOption).toBeVisible()
        await firstSingleOption.fill("So happy")


        await toAddNewQuestionButton.click()

        const thirdQuestionType = page.getByRole('combobox').nth(3)
        await expect(thirdQuestionType).toBeVisible()
        await thirdQuestionType.selectOption('Short Answer')

        const thirdQuestionTypeDescriptionInput = page.getByRole('textbox', { name: 'Please Input Description' }).nth(4)
        await expect(thirdQuestionTypeDescriptionInput).toBeVisible()
        await thirdQuestionTypeDescriptionInput.fill('A Short Answer Type')

        const thirdQuestionTypeTitleInput = page.getByRole('textbox', { name: 'Please enter a title.' }).nth(2)
        await expect(thirdQuestionTypeTitleInput).toBeVisible()
        await thirdQuestionTypeTitleInput.fill('My Short Answer')

        await toAddNewQuestionButton.click()

        const fourthQuestionType = page.getByRole('combobox').nth(4)
        await expect(fourthQuestionType).toBeVisible()
        await fourthQuestionType.selectOption('Subjective')

        const fourthQuestionTypeDescriptionInput = page.getByRole('textbox', { name: 'Please Input Description' }).nth(5)
        await expect(fourthQuestionTypeDescriptionInput).toBeVisible()
        await fourthQuestionTypeDescriptionInput.fill('A Subjective Type')

        const fourthQuestionTypeTitleInput = page.getByRole('textbox', { name: 'Please enter a title.' }).nth(3)
        await expect(fourthQuestionTypeTitleInput).toBeVisible()
        await fourthQuestionTypeTitleInput.fill('My Subjective')


        const toNextPage = page.getByRole('button', { name: 'Next' })
        await expect(toNextPage).toBeVisible()
        await toNextPage.click()


        const propositionPage = page.getByText('Participant Attribute Setting')
        await expect(propositionPage).toBeVisible()

        const totalNumberOfPeople = page.getByText('Total Number of People')
        await expect(totalNumberOfPeople).toBeVisible()

        const totalNumberOfPeopleInput = page.getByPlaceholder('Input Total Number of People')
        await expect(totalNumberOfPeopleInput).toBeVisible()
        await totalNumberOfPeopleInput.fill('9')

        const attributeGroup = page.getByText('Attribute Group').first()
        await expect(attributeGroup).toBeVisible()

        const attributeGroupOptions = page.getByRole('button', { name: 'Enter Contents' })
        await expect(attributeGroupOptions).toBeVisible()



        const errorMessages = [
            'Input introduction is required.',
            'Title is required.',
            'Start date is required.',
            'End date is required.',
            'Description is required.'
        ];

        await page.waitForLoadState('networkidle');
        await page.screenshot({ 
            path: `${screenshotBase}/10-survey-submitted.png`, 
            fullPage: true 
        });

        // await expect(page).toHaveURL(/.*surveys$/, );
    });

});