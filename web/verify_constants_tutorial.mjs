import { chromium } from 'playwright';

(async () => {
  console.log('Launching Chromium...');
  const browser = await chromium.launch({ headless: true, channel: 'chrome' });
  const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });

  page.on('console', msg => {
    console.log(`[Browser ${msg.type()}]:`, msg.text());
  });

  page.on('pageerror', err => {
    console.error('[Page Exception]:', err);
  });

  console.log('Navigating to http://localhost:5173...');
  await page.goto('http://localhost:5173', { waitUntil: 'networkidle' });

  // Wait for canvas
  await page.waitForSelector('canvas', { timeout: 15000 });
  console.log('Canvas loaded.');

  // Switch to Learn tab
  const learnTab = page.getByText('Learn', { exact: true });
  if (await learnTab.isVisible()) {
    await learnTab.click();
    await page.waitForTimeout(500);
  }

  console.log('Selecting Lesson 1.8...');
  const lessonItem = page.locator('li.lesson', { hasText: '1.8 Constants & State Persistence' });
  await lessonItem.waitFor({ timeout: 5000 });
  await lessonItem.click();
  await page.waitForTimeout(1000);

  const loadBtn = page.locator('button.load-btn');
  await loadBtn.waitFor({ timeout: 5000 });
  await loadBtn.click();
  console.log('Loaded Lesson 1.8 into editor.');

  // Switch to Editor tab
  const editorTab = page.getByText('Editor', { exact: true });
  if (await editorTab.isVisible()) {
    await editorTab.click();
    await page.waitForTimeout(1000);
  }

  console.log('Waiting for orchestrator and workers to run...');
  for (let i = 1; i <= 3; i++) {
    await page.waitForTimeout(1500);
    const screenshotPath = `/Users/ramneekhanda/.gemini/antigravity/brain/e8ef25d3-f55b-4c9e-8692-cd4a0b98744c/constants_and_scope_${i}.png`;
    await page.screenshot({ path: screenshotPath, fullPage: true });
    console.log(`Captured screenshot ${i}: ${screenshotPath}`);
  }

  await browser.close();
  console.log('Verification finished successfully.');
})();
