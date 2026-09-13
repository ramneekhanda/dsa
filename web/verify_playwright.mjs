import { chromium } from 'playwright';

(async () => {
  console.log('Launching Chromium...');
  const browser = await chromium.launch({ headless: true, channel: 'chrome' });
  const page = await browser.newPage();

  const consoleErrors = [];
  page.on('console', msg => {
    if (msg.type() === 'error') {
      console.log('[Browser Error]:', msg.text());
      consoleErrors.push(msg.text());
    } else {
      console.log('[Browser Log]:', msg.text());
    }
  });

  page.on('pageerror', err => {
    console.error('[Page Exception]:', err);
    consoleErrors.push(err.toString());
  });

  console.log('Navigating to http://localhost:5173...');
  await page.goto('http://localhost:5173', { waitUntil: 'networkidle' });

  // Wait for canvas to load
  await page.waitForSelector('canvas', { timeout: 15000 });
  console.log('Canvas found and loaded.');

  // Switch to "Learn" tab
  console.log('Switching to Learn tab...');
  const learnTab = page.getByText('Learn', { exact: true });
  if (await learnTab.isVisible()) {
    await learnTab.click();
    await page.waitForTimeout(500);
  }

  console.log('Finding Lesson 3.6 in Learn panel...');
  const lessonItem = page.locator('li.lesson', { hasText: '3.6 Remote Imports & Presets' });
  await lessonItem.waitFor({ timeout: 5000 });
  await lessonItem.click();
  console.log('Clicked Lesson 3.6.');

  await page.waitForTimeout(1000);

  const loadBtn = page.locator('button.load-btn');
  await loadBtn.waitFor({ timeout: 5000 });
  console.log('Found load button, clicking...');
  await loadBtn.click();
  console.log('Clicked "Load this example into editor".');

  // Wait 4 seconds for Bevy simulation to run multiple ticks and exchange messages
  await page.waitForTimeout(4000);

  const screenshotPath = '/Users/ramneekhanda/.gemini/antigravity/brain/e8ef25d3-f55b-4c9e-8692-cd4a0b98744c/playwright_lesson3_6.png';
  await page.screenshot({ path: screenshotPath, fullPage: true });
  console.log('Saved screenshot to:', screenshotPath);

  // Filter known non-fatal Wasm control flow message
  const fatalErrors = consoleErrors.filter(e => !e.includes("Using exceptions for control flow"));
  console.log('Fatal browser errors count:', fatalErrors.length);

  await browser.close();
  if (fatalErrors.length > 0) {
    console.error('Test failed with errors:', fatalErrors);
    process.exit(1);
  } else {
    console.log('ALL PLAYWRIGHT CHECKS PASSED!');
    process.exit(0);
  }
})();
