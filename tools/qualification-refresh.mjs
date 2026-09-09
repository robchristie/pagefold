// Isolated synthetic filesystem changes, followed by physical UI refresh.
import {chromium} from '/nvme/development/polyorama/node_modules/playwright/index.mjs';
import {execFileSync} from 'node:child_process';
import {mkdirSync, mkdtempSync, cpSync, writeFileSync, readFileSync, renameSync, unlinkSync} from 'node:fs';
import assert from 'node:assert/strict';
const root = process.cwd();
const scratch = mkdtempSync(root+'/.runtime-scratch/qualification-journey-');
const knowledge = scratch+'/knowledge';
cpSync('/nvme/development/pagefold-supervision-20260909-a/knowledge',knowledge,{recursive:true});
mkdirSync(scratch+'/empty');
writeFileSync(knowledge+'/Change.md','# Change original\n\nqual-original\n\n![Missing](absent.png)\n');
const evidence = process.env.PAGEFOLD_EVIDENCE || root+'/evidence/qualification';
const browser = await chromium.connectOverCDP('http://127.0.0.1:9317');
const page = browser.contexts()[0].pages()[0];
const events=[];
const state=()=>page.evaluate(()=>window.pagefoldObservation());
async function click(id) {
  const before=await state(); const n=before.nodes.find(n=>n.id===id); assert(n,id);
  const [x,y,w,h]=n.rect; await page.mouse.click(x+w/2,y+h/2); await page.waitForTimeout(150);
  events.push({action:'click',id,before,after:await state()});
}
async function fill(id,text) {await click(id);await page.keyboard.press('Control+A');await page.keyboard.press('Backspace');await page.keyboard.insertText(text);await page.waitForTimeout(150);}
async function capture(name) {
  await page.waitForTimeout(250);
  const out=execFileSync('lantern',['screenshot','--endpoint','http://127.0.0.1:9317','--output',evidence+'/'+name+'.png','--overwrite','--json'],{encoding:'utf8'});
  writeFileSync(evidence+'/'+name+'-capture.json',out); assert(JSON.parse(out).ok);
  writeFileSync(evidence+'/'+name+'-state.json',JSON.stringify(await state(),null,2)+'\n');
}
async function settled(){await page.waitForFunction(()=>!window.pagefoldObservation().pending);}
async function refresh(changed=true){const old=(await state()).generation;await click('pagefold.refresh');await settled();assert((await state()).status.includes('refreshed'));if(changed)assert.notEqual((await state()).generation,old);}
function index(){return JSON.parse(readFileSync(root+'/.runtime-scratch/qualification-state/index.json','utf8'));}
try {
  await fill('directory',knowledge);
  // Delay the real response solely to observe the pending UI; do not replace data.
  await page.route('**/api/snapshot',async route=>{await new Promise(r=>setTimeout(r,1800));await route.continue();});
  await click('pagefold.open');
  assert((await state()).pending); await page.waitForTimeout(100);
  await capture('loading');
  assert((await state()).nodes.filter(n=>n.id.startsWith('pagefold.')).every(n=>!n.enabled));
  await settled(); await page.unroute('**/api/snapshot');
  assert.equal((await state()).root,knowledge);
  await fill('search','qual-original'); await click('page:Change.md'); await capture('second-directory');
  writeFileSync(knowledge+'/Change.md','# Change edited\n\nqual-edited\n');
  writeFileSync(knowledge+'/Added.md','# Added\n\nqual-added\n');
  await refresh(); await capture('edited-open');
  assert.equal((await state()).page,'Change.md');
  assert(!(await state()).nodes.some(n=>n.id.startsWith('page:')));
  assert(JSON.stringify(index()).includes('qual-edited'));
  assert(!JSON.stringify(index()).includes('qual-original'));
  await fill('search','qual-added'); await click('page:Added.md'); await capture('added-result');
  renameSync(knowledge+'/Added.md',knowledge+'/Renamed.md'); await refresh(); await capture('renamed-open');
  assert.equal((await state()).page,'Added.md');
  assert((await state()).nodes.some(n=>n.id==='page:Renamed.md'));
  assert(!(await state()).nodes.some(n=>n.id==='page:Added.md'));
  await click('page:Renamed.md'); unlinkSync(knowledge+'/Renamed.md'); await refresh(); await capture('deleted-open');
  assert(!(await state()).nodes.some(n=>n.id.startsWith('page:')));
  await fill('search',''); await click('page:Home.md');
  // A different existing synthetic PNG replaces the isolated attachment.
  cpSync(evidence+'/empty.png',knowledge+'/attachments/gradient.png');
  await refresh(); await capture('attachment-replaced');
  unlinkSync(knowledge+'/attachments/gradient.png'); await refresh(); await capture('attachment-removed');
  unlinkSync(root+'/.runtime-scratch/qualification-state/index.json'); await refresh(false);
  assert(JSON.stringify(index()).includes('qual-edited'));
  const beforeFailure=await state();
  writeFileSync(root+'/.runtime-scratch/qualification-state/index.json','{corrupt index');
  await refresh(false);
  assert(JSON.stringify(index()).includes('qual-edited'));
  renameSync(knowledge,scratch+'/temporarily-unavailable');
  await click('pagefold.refresh');await settled(); await capture('failed-refresh');
  assert.equal((await state()).generation,beforeFailure.generation);
  assert((await state()).status.includes('Previous snapshot retained'));
  assert((await state()).snapshot_stale);
  // Follow Home's relative reading link using its opened-image position in the reader.
  const reader = (await state()).nodes.find(n=>n.id==='reader');
  await page.mouse.click(110,reader.rect[1]+130); await page.waitForTimeout(300);
  assert.equal((await state()).page,'guides/Reading.md');
  assert((await state()).status.includes('Previous snapshot retained'));
  await capture('stale-navigation');
  await fill('search','café'); await click('page:Unicode.md');
  assert((await state()).snapshot_stale);
  assert((await state()).status.includes('Previous snapshot retained'));
  await capture('stale-search');
  await fill('search',''); await click('page:Home.md');
  renameSync(scratch+'/temporarily-unavailable',knowledge);
  await refresh(false); await capture('refresh-recovered');
  assert.equal((await state()).snapshot_stale,false);
  await fill('directory',scratch+'/empty');await click('pagefold.open');await settled();await capture('empty-directory');
  assert.equal((await state()).root,scratch+'/empty');assert.equal((await state()).page,'');
  const doc=await page.evaluate(()=>({url:location.href,injected:document.body.dataset.injected??null,ready:document.body.dataset.ready}));
  assert.equal(doc.injected,null);writeFileSync(evidence+'/document-state.json',JSON.stringify(doc,null,2));
  console.log('PASS: second/empty directories, loading/disabled controls, edited open page, add/search/open, rename/delete open page, attachment replacement/removal, index regeneration, failed refresh and recovery');
} finally {
  writeFileSync(evidence+'/refresh-actions.json',JSON.stringify(events,null,2));
  await browser.close();
}
