// Reproduce calibration with physical canvas input; retain Lantern observations.
import {chromium} from '/nvme/development/polyorama/node_modules/playwright/index.mjs';
import {execFileSync} from 'node:child_process';
import {writeFileSync} from 'node:fs';
const evidence = `${process.cwd()}/evidence/calibration`;
const browser = await chromium.connectOverCDP('http://127.0.0.1:9317');
const page = browser.contexts()[0].pages()[0];
function lantern(command, args, output) {
  const result = execFileSync('lantern', [command,'--endpoint','http://127.0.0.1:9317',...args,'--json'], {encoding:'utf8'});
  writeFileSync(`${evidence}/${output}.json`, result);
  if (!JSON.parse(result).ok) throw new Error(result);
}
async function capture(name) {
  await page.waitForTimeout(250);
  lantern('screenshot',['--output',`${evidence}/${name}.png`,'--overwrite'],`${name}-capture`);
}
async function click(x,y) { await page.mouse.click(x,y); await page.waitForTimeout(250); }
await page.setViewportSize({width:1100,height:657});
lantern('flow',['--open','http://127.0.0.1:3817/','--timeout-ms','10000','--quiet-ms','600'],'flow');
await page.waitForSelector('body[data-ready="true"]');
await capture('home');
await click(120,195); await capture('reading');
lantern('wheel',['--selector','#reader','--dy','3400','--timeout-ms','3000','--strict'],'scroll');
await capture('scrolled');
await click(265,41); await capture('unicode');
await click(40,195); await capture('back-home');
await click(408,195); await capture('missing');
await page.setViewportSize({width:390,height:700}); await capture('narrow');
await click(268,213); await capture('unsupported');
await click(150,41);
lantern('hover',['--selector','#reader','--timeout-ms','3000','--strict'],'hover-reader');
lantern('wheel',['--selector','#reader','--dy','-10000','--timeout-ms','3000','--strict'],'scroll-top');
await capture('narrow-reading');
lantern('layout',['--container-selector','body'],'narrow-layout');
lantern('accessibility',['--depth','5','--max-nodes','60'],'accessibility');
const state = await page.evaluate(() => ({ready:document.body.dataset.ready,injected:document.body.dataset.injected ?? null,url:location.href,viewport:{width:innerWidth,height:innerHeight}}));
writeFileSync(`${evidence}/browser-state.json`, JSON.stringify(state,null,2)+'\n');
if (state.injected !== null || state.ready !== 'true') throw new Error('Unexpected document state');
console.log('PASS: capture journey completed; pixels require separate visual assessment');
await browser.close();
