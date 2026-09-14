// Owner harness: viewport setup, measured canvas focus, and read-only observation.
import { chromium } from '/nvme/development/polyorama/node_modules/playwright/index.mjs';
import { writeFileSync } from 'node:fs';
const browser = await chromium.connectOverCDP('http://127.0.0.1:9317');
try {
  const page = browser.contexts()[0].pages().find(p => p.url().startsWith('http://127.0.0.1:3827/'));
  if (!page) throw new Error('Calibration page unavailable');
  const [action, arg, out] = process.argv.slice(2);
  await page.waitForFunction(() => document.body.dataset.ready === 'true');
  if (action === 'viewport') await page.setViewportSize({width:Number(arg),height:900});
  if (action === 'focus' || action === 'fill') {
    const node = await page.evaluate(() => window.pagefoldObservation().nodes.find(n => n.id === 'search'));
    await page.mouse.click(node.rect[0] + 20, node.rect[1] + node.rect[3] / 2);
    await page.keyboard.press('Control+a');
    await page.keyboard.press('Backspace');
    if (action === 'fill') await page.keyboard.insertText(arg);
  }
  if (action === 'state') {
    await page.waitForFunction(q => window.pagefoldObservation().query === q, arg);
    // Wait until egui's bounded scroll animation has settled.
    await page.waitForTimeout(500);
    const state = await page.evaluate(() => ({viewport:{width:innerWidth,height:innerHeight}, ...window.pagefoldObservation()}));
    if (out) writeFileSync(out, JSON.stringify(state, null, 2) + '\n');
    console.log(JSON.stringify(state));
  }
} finally { await browser.close(); }
