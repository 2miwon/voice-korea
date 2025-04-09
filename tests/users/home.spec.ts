import { test, expect } from "@playwright/test";
import path from "path";

test.describe("Home Page Tests", () => {
    test("[Home-ID-001] Confirm it has project details", async ({
        page,
    }, testInfo) => {
        const projectName = testInfo.project.name;
        const screenshotBase = path.join(
            "screenshots",
            "users",
            projectName,
            "home",
        );
        await page.goto(`http://dev.voice-korea.com/en/`);
        const goToConsole = page.getByRole('button', { name: 'Create a public opinion survey' })
        await expect(goToConsole).toBeVisible();
        await goToConsole.click();
        // await page.waitForTimeout(2000);
        // await expect(page).toHaveURL('https://console.dev.voice-korea.com/');
        await page.screenshot({
            path: `${screenshotBase}/01-go-to-console.png`,
            fullPage: true,
        });

        await page.goto(`http://dev.voice-korea.com/en/`);
        await page.screenshot({
            path: `${screenshotBase}/02-page-entered.png`,
            fullPage: true,
        });

        const hoverOnProject = page.locator('.h-260').first()
        await hoverOnProject.hover();

        await page.screenshot({
            path: `${screenshotBase}/03-hover-on-project.png`,
            fullPage: true,
        });

        const description = page.getByText('test description', { exact: true }).first()
        await expect(description).toBeVisible()

        const projectButton = page.getByRole('button', { name: 'See Details' })
        await expect(projectButton).toBeVisible();
        await projectButton.click();
        await expect(page).toHaveURL(/.*\/projects\/\d+/);

        await page.screenshot({
            path: `${screenshotBase}/04-project-details.png`,
            fullPage: true,
        });

        await page.goto(`http://dev.voice-korea.com/en/`);

        const nameInInquiry = page.getByText('Name')
        await expect(nameInInquiry).toBeVisible();
        const nameInInquiryInput = page.getByRole('textbox', { name: 'Please Enter Your Name' })
        await expect(nameInInquiryInput).toBeVisible();
        await nameInInquiryInput.fill('test name')

        await page.screenshot({
            path: `${screenshotBase}/05-inquiry-name.png`,
            fullPage: true,
        });

        const emailInInquiry = page.getByText('Email')
        await expect(emailInInquiry).toBeVisible();
        const emailInInquiryInput = page.getByRole('textbox', { name: 'Please Enter Your Email' })
        await emailInInquiryInput.fill('test-email@gmail.com')

        await page.screenshot({
            path: `${screenshotBase}/06-inquiry-email.png`,
            fullPage: true,
        });

        const messageInInquiry = page.getByText('Message')
        await expect(messageInInquiry).toBeVisible();
        const messageInInquiryInput = page.getByRole('textbox', { name: 'Please enter the information' })
        await messageInInquiryInput.fill('test message')

        await page.screenshot({
            path: `${screenshotBase}/07-inquiry-message.png`,
            fullPage: true,
        });

        const inquiryButton = page.getByRole('button', { name: 'Inquiry' })
        await expect(inquiryButton).toBeVisible();
        await inquiryButton.click();


    });


    // test("[Home-ID-002] Validate Inquiry Form", async ({
    //     page,
    // }, testInfo) => {
    //     const projectName = testInfo.project.name;
    //     const screenshotBase = path.join(
    //         "screenshots",
    //         "users",
    //         projectName,
    //         "home",
    //     );

    //     await page.goto(`http://dev.voice-korea.com/en/`);

    //     await page.screenshot({
    //         path: `${screenshotBase}/01-page-entered.png`,
    //         fullPage: true,
    //     });

    //     const nameInInquiryInput = page.getByRole('textbox', { name: 'Please Enter Your Name' })
    //     await expect(nameInInquiryInput).toBeVisible();
    //     await nameInInquiryInput.fill('test name')
    //     await page.screenshot({
    //         path: `${screenshotBase}/02-inquiry-name.png`,
    //         fullPage: true,
    //     });


    //     const inquiryButton = page.getByRole('button', { name: 'Inquiry' })
    //     await expect(inquiryButton).toBeVisible();
    //     await inquiryButton.click();

    //     const emailError = page.getByText('Please enter your email')
    //     await expect(emailError).toBeVisible();

    //     const messageError = page.getByText('Please enter your inquiry')
    //     await expect(messageError).toBeVisible();

    //     await page.screenshot({
    //         path: `${screenshotBase}/03-inquiry-error.png`,
    //         fullPage: true,
    //     });
    // })

    // test("[Home-ID-003] Navigate To Governance.", async ({
    //     page,
    // }, testInfo) => {
    //     const projectName = testInfo.project.name;
    //     const screenshotBase = path.join(
    //         "screenshots",
    //         "users",
    //         projectName,
    //         "home",
    //     );

    //     await page.goto(`http://dev.voice-korea.com/en/`);

    //     await page.screenshot({
    //         path: `${screenshotBase}/01-page-entered.png`,
    //         fullPage: true,
    //     });

    //     const goToGovernance = page.getByRole('link', { name: 'user-86569e21-e852-48e7-92b9-' });
    //     await expect(goToGovernance).toBeVisible()
    //     await goToGovernance.click()
    //     await expect(page).toHaveURL("http://dev.voice-korea.com/en/governance/181");

    //     await page.screenshot({
    //         path: `${screenshotBase}/02-governance-page.png`,
    //         fullPage: true,
    //     });
    // })


    // test("[Home-ID-004] Test Pricing.", async ({
    //     page,
    // }, testInfo) => {
    //     const projectName = testInfo.project.name;
    //     const screenshotBase = path.join(
    //         "screenshots",
    //         "users",
    //         projectName,
    //         "home",
    //     );

    //     await page.goto(`http://dev.voice-korea.com/en/`);

    //     await page.screenshot({
    //         path: `${screenshotBase}/01-page-entered.png`,
    //         fullPage: true,
    //     });

    //     const free = page.locator('#price div').filter({ hasText: 'FreeParticipate in public' }).nth(1)
    //     await expect(free).toBeVisible()
    //     await page.screenshot({
    //         path: `${screenshotBase}/02-free-page.png`,
    //         fullPage: true,
    //     });

    //     const freeHeader = page.getByText('Free', { exact: true })
    //     await expect(freeHeader).toBeVisible()

    //     const freeButton = page.getByRole('button', { name: 'Start' }).first()
    //     await expect(freeButton).toBeVisible();
    //     await freeButton.click()


    //     const premium = page.locator('#price div').filter({ hasText: 'FreeParticipate in public' }).nth(1)
    //     await expect(premium).toBeVisible()
    //     await page.screenshot({
    //         path: `${screenshotBase}/03-premium-page.png`,
    //         fullPage: true,
    //     });

    //     const premiumHeader = page.getByText('Premium', { exact: true })
    //     await expect(premiumHeader).toBeVisible()

    //     const premiumButton = page.getByRole('button', { name: 'Start' }).nth(1)
    //     await expect(premiumButton).toBeVisible();
    //     await premiumButton.click()


    //     const guide = page.locator('div').filter({ hasText: /^Public Opinion Participation Guide$/ }).nth(1)
    //     await expect(guide).toBeVisible()
    //     await guide.click()

    //     const consoleGuide = page.locator('div').filter({ hasText: /^Public Opinion Survey Design Console Guide$/ }).nth(1)
    //     await expect(guide).toBeVisible()
    //     await consoleGuide.click()

    //     await page.screenshot({
    //         path: `${screenshotBase}/04-guides-page.png`,
    //         fullPage: true,
    //     });

    // })


});
