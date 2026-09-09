// Physical canvas inputs; the application hook is used only for observation.
import {chromium} from '/nvme/development/polyorama/node_modules/playwright/index.mjs';
import {execFileSync} from 'node:child_process';
import {writeFileSync, appendFileSync} from 'node:fs';
const endpoint = 'http://127.0.0.1:9317';
const evidence = process.env.PAGEFOLD_EVIDENCE || process.cwd() + '/evidence/qualification';
const browser = await chromium.connectOverCDP(endpoint);
const page = browser.contexts()[0].pages()[0];
const [action, value, extra] = process.argv.slice(2);
const state = () => page.evaluate(() => window.pagefoldObservation());
await page.waitForFunction(() => document.body.dataset.ready === 'true' && window.pagefoldObservation().nodes.length);
const before = await state();
async function target(id) {
  const n = (await state()).nodes.find(n => n.id === id);
  if (!n) throw Error('Missing observed target: ' + id);
  const [x,y,w,h] = n.rect;
  await page.mouse.click(x+w/2,y+h/2);
}
if (action === 'click') await target(value);
else if (action === 'point') await page.mouse.click(Number(value),Number(extra));
else if (action === 'fill') {
  await target(value);
  await page.keyboard.press('Control+A');
  await page.keyboard.press('Backspace');
  await page.keyboard.insertText(extra);
} else if (action === 'key') await page.keyboard.press(value);
else if (action === 'wheel') {
  const n = before.nodes.find(n => n.id === 'reader');
  await page.mouse.move(n.rect[0]+n.rect[2]/2,n.rect[1]+n.rect[3]/2);
  await page.mouse.wheel(0,Number(value));
} else if (action === 'viewport') await page.setViewportSize({width:Number(value),height:Number(extra)});
else if (!['capture','state'].includes(action)) throw Error('Unknown action');
await page.waitForTimeout(400);
const after = await state();
appendFileSync(evidence+'/actions.jsonl',JSON.stringify({action,value,extra,before,after})+'\n');
if (action === 'capture') {
  const output = execFileSync('lantern',['screenshot','--endpoint',endpoint,'--output',evidence+'/'+value+'.png','--overwrite','--json'],{encoding:'utf8'});
  writeFileSync(evidence+'/'+value+'-capture.json',output);
  if (!JSON.parse(output).ok) throw Error(output);
  writeFileSync(evidence+'/'+value+'-state.json',JSON.stringify(after,null,2)+'\n');
}
console.log(JSON.stringify(after));
await browser.close();
