// Replay the retained physical reading journey against a freshly loaded candidate.
import {chromium} from '/nvme/development/polyorama/node_modules/playwright/index.mjs';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
const evidence = process.env.PAGEFOLD_EVIDENCE;
if (!evidence) throw Error('Set PAGEFOLD_EVIDENCE to the current evidence directory');
const browser = await chromium.connectOverCDP('http://127.0.0.1:9317');
const page = browser.contexts()[0].pages()[0];
await page.reload();
await page.waitForFunction(() => document.body.dataset.ready === 'true');
await browser.close();
for (const step of JSON.parse(readFileSync(evidence+'/reader-actions.json','utf8'))) {
  const args = ['tools/qualification-action.mjs',step.action];
  for (const field of ['value','extra']) if (step[field] !== undefined) args.push(String(step[field]));
  execFileSync('node',args,{env:process.env,stdio:'pipe'});
}
console.log('PASS: replayed physical reading, links, keyboard focus, scrolling, narrow layout and search journey');
