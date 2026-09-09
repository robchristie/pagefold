// Owner harness for physical canvas input and viewport setup; Lantern observes results.
import {chromium} from '/nvme/development/polyorama/node_modules/playwright/index.mjs';
const browser = await chromium.connectOverCDP('http://127.0.0.1:9317');
const page = browser.contexts()[0].pages()[0];
const [action, a, b] = process.argv.slice(2);
if (action === 'click') await page.mouse.click(Number(a), Number(b));
else if (action === 'viewport') await page.setViewportSize({width:Number(a), height:Number(b)});
else if (action === 'state') console.log(JSON.stringify(await page.evaluate(() => ({ready:document.body.dataset.ready, injected:document.body.dataset.injected ?? null, url:location.href, viewport:{width:innerWidth,height:innerHeight}}))));
else throw new Error('Expected click, viewport or state');
console.log(JSON.stringify({action,a,b}));
await browser.close();
