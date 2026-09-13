import { chromium } from 'playwright';

const themes = ['cyberpunk', 'cloud', 'datacenter', 'minimal'];

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

  for (const theme of themes) {
    console.log(`Testing theme: ${theme}...`);
    const yaml = `theme: ${theme}

graph_defn:
  graph:
    - name: client
      node_type: client
      links: ["server"]
    - name: server
      node_type: server
      links: []
  node_types:
    - id: client
      attrs:
        ticks: 1
      fn: |
        fn on_init() {
          globals.t = 0;
        }
        fn on_timer() {
          globals.t += 1;
          if globals.t == 1 {
            explain("theme_demo_${theme}", "Interactive Narration Callout bubble themed for **${theme.toUpperCase()}**.\\n\\nNotice the custom shape, neon/border strokes, typography, and button accents!");
          }
        }
    - id: server
      attrs:
        ticks: 5
      fn: |
        fn on_init() {}
        fn on_timer() {}
        fn on_msg(from, msg) {}
`;

    await page.evaluate(async (code) => {
      if (window.__procsimCompile) {
        await window.__procsimCompile(code);
      }
    }, yaml);

    // Wait for simulation tick and explain bubble pop-in
    await page.waitForTimeout(2000);

    const shotPath = `/Users/ramneekhanda/.gemini/antigravity/brain/e8ef25d3-f55b-4c9e-8692-cd4a0b98744c/explain_theme_${theme}.png`;
    await page.screenshot({ path: shotPath, fullPage: true });
    console.log(`Saved screenshot for ${theme}: ${shotPath}`);

    // Click Got It / Next button or dismiss to continue smoothly
    const canvas = page.locator('#bevy-canvas');
    const box = await canvas.boundingBox();
    if (box) {
      // Click near center of canvas to click the action button
      await page.mouse.click(box.x + box.width / 2 + 100, box.y + box.height / 2 - 30);
    }
    await page.waitForTimeout(600);
  }

  await browser.close();
  console.log('All theme tests complete!');
})();
