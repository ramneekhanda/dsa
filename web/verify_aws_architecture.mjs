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

  const yaml = `imports:
  - from: "plibs:themes/cloud"
  - from: "plibs:aws/all"

graph_defn:
  graph_attrs:
    title: "AWS Serverless Microservices with plibs:aws/all"
  graph:
    - name: api_gw
      node_type: api_gateway
      links: ["order_processor"]
    - name: order_processor
      node_type: lambda_func
      links: ["orders_table", "events_queue"]
    - name: orders_table
      node_type: dynamodb
      links: []
    - name: events_queue
      node_type: sqs_queue
      links: []

  node_types:
    - id: api_gateway
      attrs:
        ticks: 2
        template_ref: aws_api_gw_card
        params:
          route_path: "POST /v1/orders"
      fn: |
        fn on_init() {
          globals.req_id = 0;
        }
        fn on_timer() {
          globals.req_id += 1;
          if globals.req_id == 1 {
            explain("aws_overview", "AWS Serverless Architecture imported from **plibs:aws/all**!\\n\\n• API Gateway routes HTTP traffic\\n• Lambda processes order events\\n• DynamoDB stores persistent records\\n• SQS buffers async notifications");
          }
          send(links[0], "ORDER #" + globals.req_id);
        }

    - id: lambda_func
      attrs:
        template_ref: aws_lambda_card
        params:
          status_text: "LAMBDA // ORDERS"

    - id: dynamodb
      attrs:
        template_ref: aws_dynamo_card
        params:
          table_name: "OrdersTable"

    - id: sqs_queue
      attrs:
        template_ref: aws_sqs_card
        params:
          queue_type: "SQS // ORDERS_FIFO"
`;

  console.log('Compiling AWS simulation...');
  await page.evaluate(async (code) => {
    if (window.__procsimCompile) {
      await window.__procsimCompile(code);
    }
  }, yaml);

  await page.waitForTimeout(2500);

  const shotPath = '/Users/ramneekhanda/.gemini/antigravity/brain/e8ef25d3-f55b-4c9e-8692-cd4a0b98744c/aws_serverless_architecture.png';
  await page.screenshot({ path: shotPath, fullPage: true });
  console.log('Saved AWS Serverless Architecture screenshot:', shotPath);

  await browser.close();
  console.log('AWS verification test complete!');
})();
