import { chromium } from 'playwright';

(async () => {
  console.log('Launching Chromium...');
  const browser = await chromium.launch({ headless: true, channel: 'chrome' });
  const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });

  const errors = [];
  const warnings = [];
  page.on('console', msg => {
    const txt = msg.text();
    if (msg.type() === 'error') errors.push(txt);
    if (msg.type() === 'warning') warnings.push(txt);
    console.log(`[Browser ${msg.type()}]:`, txt);
  });

  page.on('pageerror', err => {
    console.error('[Page Exception]:', err);
    errors.push(err.toString());
  });

  console.log('Navigating to http://localhost:5173...');
  await page.goto('http://localhost:5173', { waitUntil: 'networkidle' });

  // Wait for canvas & editor
  await page.waitForSelector('canvas', { timeout: 15000 });
  await page.waitForTimeout(2000);

  // Switch to Learn tab and load a tutorial into editor
  const learnTab = page.getByText('Learn', { exact: true });
  if (await learnTab.isVisible()) {
    await learnTab.click();
    await page.waitForTimeout(500);
  }

  const lessonItem = page.locator('li.lesson', { hasText: '1.8 Constants & State Persistence' });
  await lessonItem.waitFor({ timeout: 5000 });
  await lessonItem.click();
  await page.waitForTimeout(500);

  const loadBtn = page.locator('button.load-btn');
  await loadBtn.waitFor({ timeout: 5000 });
  await loadBtn.click();
  console.log('Loaded lesson into editor.');

  // Switch back to editor tab
  const editorTab = page.getByText('Editor', { exact: true });
  if (await editorTab.isVisible()) {
    await editorTab.click();
    await page.waitForTimeout(1000);
  }

  // Take screenshot of editor with syntax highlighting
  const screenshotPath = '/Users/ramneekhanda/.gemini/antigravity/brain/e8ef25d3-f55b-4c9e-8692-cd4a0b98744c/monaco_syntax_highlighted.png';
  await page.screenshot({ path: screenshotPath, fullPage: true });
  console.log('Saved screenshot:', screenshotPath);

  // Check that no editorWorkerService warning occurred
  const hasWorkerWarning = warnings.some(w => w.includes('editorWorkerService') || w.includes('Could not create web worker'));
  console.log('Has worker warning:', hasWorkerWarning);

  await browser.close();
  console.log('Done!');
})();
