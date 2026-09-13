import { chromium } from 'playwright';

(async () => {
  console.log('Launching Chromium...');
  const browser = await chromium.launch({ headless: true, channel: 'chrome' });
  const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });

  page.on('console', msg => console.log(`[Browser ${msg.type()}]:`, msg.text()));
  page.on('pageerror', err => console.error('[Page Exception]:', err));

  console.log('Navigating to http://localhost:5173...');
  await page.goto('http://localhost:5173', { waitUntil: 'networkidle' });
  await page.waitForSelector('#bevy-canvas', { timeout: 15000 });
  await page.waitForTimeout(1000);

  // Switch to Learn tab
  const learnTab = page.getByText('Learn', { exact: true });
  if (await learnTab.isVisible()) {
    await learnTab.click();
    await page.waitForTimeout(500);
  }

  console.log('Selecting Lesson 3.7...');
  const lessonItem = page.locator('li.lesson', { hasText: '3.7 Remote Theme Packages' });
  await lessonItem.waitFor({ timeout: 5000 });
  await lessonItem.click();
  await page.waitForTimeout(1000);

  // Screenshot of Lesson 3.7 in the Learn panel
  const ss1 = '/Users/ramneekhanda/.gemini/antigravity/brain/e8ef25d3-f55b-4c9e-8692-cd4a0b98744c/tutorial_3_7_learn_panel.png';
  await page.screenshot({ path: ss1, fullPage: true });
  console.log('Saved Learn panel screenshot:', ss1);

  const loadBtn = page.locator('button.load-btn');
  await loadBtn.waitFor({ timeout: 5000 });
  await loadBtn.click();
  console.log('Loaded Lesson 3.7 into editor.');

  // Switch to Editor tab
  const editorTab = page.getByText('Editor', { exact: true });
  if (await editorTab.isVisible()) {
    await editorTab.click();
    await page.waitForTimeout(2500);
  }

  // Screenshot of the active simulation running with the synthwave theme and explain bubble
  const ss2 = '/Users/ramneekhanda/.gemini/antigravity/brain/e8ef25d3-f55b-4c9e-8692-cd4a0b98744c/tutorial_3_7_running.png';
  await page.screenshot({ path: ss2, fullPage: true });
  console.log('Saved running simulation screenshot:', ss2);

  await browser.close();
  console.log('Done!');
})();
