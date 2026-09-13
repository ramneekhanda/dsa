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

  // Switch back to Editor tab
  const editorTab = page.getByText('Editor', { exact: true });
  if (await editorTab.isVisible()) {
    await editorTab.click();
    await page.waitForTimeout(1000);
  }

  // 1. Verify Cyberpunk Theme Screenshot
  await page.waitForTimeout(3000);
  await page.screenshot({ path: '/Users/ramneekhanda/.gemini/antigravity/brain/e8ef25d3-f55b-4c9e-8692-cd4a0b98744c/theme_1_cyberpunk.png', fullPage: true });
  console.log('Saved cyberpunk screenshot.');

  // 2. Change to Cloud Theme via codeEditor
  console.log('Testing live theme switch to "theme: cloud"...');
  const cloudYaml = `theme: cloud

imports:
  - from: "stdlib:load_balancer"

graph_defn:
  graph_attrs:
    title: "Cloud Cluster with stdlib Load Balancer"
  graph:
    - name: client
      node_type: client
      links: ["lb"]
    - name: lb
      node_type: round_robin_lb
      links: ["srv1", "srv2"]
    - name: srv1
      node_type: worker
      links: []
    - name: srv2
      node_type: worker
      links: []

  node_types:
    - id: client
      attrs:
        ticks: 2
      fn: |
        fn on_timer() {
          send(links[0], "GET /api/v1/data");
        }
    - id: worker
      attrs:
        ticks: 1
      fn: |
        fn on_message(msg) {
          log("Worker processing: " + msg);
        }
`;

  await page.evaluate(async (code) => {
    // @ts-ignore
    window.__procsimCodeEditor?.setCode(code);
    // @ts-ignore
    await window.__procsimCompile?.(code);
  }, cloudYaml);

  await page.waitForTimeout(3000);
  await page.screenshot({ path: '/Users/ramneekhanda/.gemini/antigravity/brain/e8ef25d3-f55b-4c9e-8692-cd4a0b98744c/theme_2_cloud.png', fullPage: true });
  console.log('Saved cloud screenshot.');

  // 3. Change to Datacenter Theme
  console.log('Testing live theme switch to "theme: datacenter"...');
  const datacenterYaml = `theme: datacenter

imports:
  - from: "stdlib:load_balancer"

graph_defn:
  graph_attrs:
    title: "Datacenter Rack with stdlib Load Balancer"
  graph:
    - name: client
      node_type: client
      links: ["lb"]
    - name: lb
      node_type: round_robin_lb
      links: ["srv1", "srv2"]
    - name: srv1
      node_type: worker
      links: []
    - name: srv2
      node_type: worker
      links: []

  node_types:
    - id: client
      attrs:
        ticks: 2
      fn: |
        fn on_timer() {
          send(links[0], "GET /api/v1/data");
        }
    - id: worker
      attrs:
        ticks: 1
      fn: |
        fn on_message(msg) {
          log("Worker processing: " + msg);
        }
`;

  await page.evaluate(async (code) => {
    // @ts-ignore
    window.__procsimCodeEditor?.setCode(code);
    // @ts-ignore
    await window.__procsimCompile?.(code);
  }, datacenterYaml);

  await page.waitForTimeout(3000);
  await page.screenshot({ path: '/Users/ramneekhanda/.gemini/antigravity/brain/e8ef25d3-f55b-4c9e-8692-cd4a0b98744c/theme_3_datacenter.png', fullPage: true });
  console.log('Saved datacenter screenshot.');

  // 4. Change to Minimal Theme
  console.log('Testing live theme switch to "theme: minimal"...');
  const minimalYaml = `theme: minimal

imports:
  - from: "stdlib:load_balancer"

graph_defn:
  graph_attrs:
    title: "Minimal Cluster with stdlib Load Balancer"
  graph:
    - name: client
      node_type: client
      links: ["lb"]
    - name: lb
      node_type: round_robin_lb
      links: ["srv1", "srv2"]
    - name: srv1
      node_type: worker
      links: []
    - name: srv2
      node_type: worker
      links: []

  node_types:
    - id: client
      attrs:
        ticks: 2
      fn: |
        fn on_timer() {
          send(links[0], "GET /api/v1/data");
        }
    - id: worker
      attrs:
        ticks: 1
      fn: |
        fn on_message(msg) {
          log("Worker processing: " + msg);
        }
`;

  await page.evaluate(async (code) => {
    // @ts-ignore
    window.__procsimCodeEditor?.setCode(code);
    // @ts-ignore
    await window.__procsimCompile?.(code);
  }, minimalYaml);

  await page.waitForTimeout(3000);
  await page.screenshot({ path: '/Users/ramneekhanda/.gemini/antigravity/brain/e8ef25d3-f55b-4c9e-8692-cd4a0b98744c/theme_4_minimal.png', fullPage: true });
  console.log('Saved minimal screenshot.');

  // Filter known non-fatal Wasm control flow message
  const fatalErrors = consoleErrors.filter(e => !e.includes("Using exceptions for control flow"));
  console.log('Fatal browser errors count:', fatalErrors.length);

  await browser.close();
  if (fatalErrors.length > 0) {
    console.error('Test failed with errors:', fatalErrors);
    process.exit(1);
  } else {
    console.log('ALL PLAYWRIGHT THEME SWITCHING CHECKS PASSED!');
    process.exit(0);
  }
})();
