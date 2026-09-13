import { chromium } from 'playwright';

const plibsThemes = [
  { id: 'synthwave', tmpl: 'synth_card' },
  { id: 'nordic', tmpl: 'nord_card' },
  { id: 'dracula', tmpl: 'dracula_node' },
  { id: 'matrix', tmpl: 'matrix_blade' },
  { id: 'solarized_light', tmpl: 'solar_card' }
];

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

  for (const { id: theme, tmpl } of plibsThemes) {
    console.log(`Testing plibs theme: ${theme}...`);
    const yaml = `theme: ${theme}

graph_defn:
  graph:
    - name: ingress
      node_type: sender_node
      links: ["cluster_core"]
    - name: cluster_core
      node_type: target_node
      links: []
  node_types:
    - id: sender_node
      attrs:
        ticks: 1
        template_ref: ${tmpl}
      fn: |
        fn on_init() {
          globals.t = 0;
        }
        fn on_timer() {
          globals.t += 1;
          if globals.t == 1 {
            explain("plibs_theme_${theme}", "Theme **${theme.toUpperCase()}** loaded from **plibs/themes/${theme}.yml**!\\n\\nComplete package with matching node template, message bubble, and custom explainer.");
          }
          send("cluster_core", "MSG #" + globals.t);
        }
    - id: target_node
      attrs:
        ticks: 5
        template_ref: ${tmpl}
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

    const shotPath = `/Users/ramneekhanda/.gemini/antigravity/brain/e8ef25d3-f55b-4c9e-8692-cd4a0b98744c/plibs_theme_${theme}.png`;
    await page.screenshot({ path: shotPath, fullPage: true });
    console.log(`Saved screenshot for ${theme}: ${shotPath}`);

    // Dismiss callout to prepare for next
    const canvas = page.locator('#bevy-canvas');
    const box = await canvas.boundingBox();
    if (box) {
      await page.mouse.click(box.x + box.width / 2 + 100, box.y + box.height / 2 - 30);
    }
    await page.waitForTimeout(500);
  }

  await browser.close();
  console.log('All plibs theme tests complete!');
})();
