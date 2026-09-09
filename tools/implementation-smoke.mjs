// Physical input against the actual filesystem-backed client; observations are read-only.
import {chromium} from '/nvme/development/polyorama/node_modules/playwright/index.mjs';
import {execFileSync} from 'node:child_process';
import {writeFileSync} from 'node:fs';
const evidence = process.cwd() + '/evidence/implementation';
const endpoint = 'http://127.0.0.1:9317';
const browser = await chromium.connectOverCDP(endpoint);
const page = browser.contexts()[0].pages()[0];
page.setDefaultTimeout(15000);
function lantern(command, args, name) {
  const output = execFileSync('lantern', [command, '--endpoint', endpoint, ...args, '--json'], {encoding:'utf8'});
  writeFileSync(evidence + '/' + name + '.json', output);
  if (!JSON.parse(output).ok) throw new Error(output);
}
async function state() { return page.evaluate(() => window.pagefoldObservation()); }
async function click(id) {
  const node = (await state()).nodes.find(n => n.id === id);
  if (!node) throw new Error('Missing current target: ' + id);
  const [x,y,w,h] = node.rect;
  await page.mouse.click(x+w/2,y+h/2);
  await page.waitForTimeout(200);
}
async function capture(name) {
  await page.waitForTimeout(250);
  lantern('screenshot', ['--output', evidence + '/' + name + '.png', '--overwrite'], name + '-capture');
  writeFileSync(evidence + '/' + name + '-state.json', JSON.stringify(await state(), null, 2));
}
await page.setViewportSize({width:1100,height:800});
lantern('flow', ['--open','http://127.0.0.1:3817/','--timeout-ms','10000','--quiet-ms','500'], 'flow');
await page.waitForFunction(() => document.body.dataset.ready === 'true' && window.pagefoldObservation().nodes.length);
await capture('empty');
await click('directory');
await page.keyboard.insertText('/nvme/development/pagefold-supervision-20260909-a/knowledge');
await page.waitForTimeout(100);
await click('pagefold.open');
await page.waitForFunction(() => window.pagefoldObservation().root && !window.pagefoldObservation().pending);
await capture('home');
await click('page:Unicode.md');
await page.waitForFunction(() => window.pagefoldObservation().page === 'Unicode.md');
await capture('unicode');
await click('page:guides/Reading.md');
await page.waitForFunction(() => window.pagefoldObservation().page === 'guides/Reading.md');
await capture('reading');
await click('search');
await page.keyboard.insertText('café');
await page.waitForTimeout(150);
const filtered = await state();
if (!filtered.nodes.some(n => n.id === 'page:Unicode.md')) throw new Error('Unicode search did not return page');
await click('page:Unicode.md');
await capture('search');
await click('pagefold.refresh');
await page.waitForFunction(() => !window.pagefoldObservation().pending && window.pagefoldObservation().status.includes('refreshed'));
await capture('refresh');
await page.setViewportSize({width:390,height:760});
await capture('narrow');
lantern('layout', ['--container-selector','body'], 'narrow-layout');
const documentState = await page.evaluate(() => ({url:location.href, injected:document.body.dataset.injected ?? null, ready:document.body.dataset.ready}));
if (documentState.injected !== null) throw new Error('Content executed');
writeFileSync(evidence + '/document-state.json', JSON.stringify(documentState,null,2));
console.log('PASS: physical directory selection, browsing, Unicode, search result and refresh smoke; screenshots need separate assessment');
await browser.close();
