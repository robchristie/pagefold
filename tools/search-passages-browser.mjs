// Disposable application journey. Setup changes viewport/focus; inputs use real browser events.
const { chromium } = await import(process.env.PAGEFOLD_PLAYWRIGHT_MODULE || 'playwright');
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync, readdirSync, renameSync, mkdirSync, copyFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';
const out = 'target/search-passages';
const root = process.cwd()+'/'+out+'/knowledge';
const inventory = () => Object.fromEntries(readdirSync(root).sort().map(p => [p,createHash('sha256').update(readFileSync(root+'/'+p)).digest('hex')]));
mkdirSync(root,{recursive:true});
const original = readFileSync('calibration/search-passages/fixture.md','utf8')+'\n\n![AltNeedle](gradient.png)\n\n    **IndentNeedle**\n    second line\n';
writeFileSync(root+'/Fixture.md',original);
writeFileSync(root+'/PathOnly.md','# Other page\n\nOrdinary content.');
writeFileSync(root+'/ThirdOnly.md','# Third page\n\nDifferent content.');
copyFileSync('tests/fixtures/knowledge/attachments/gradient.png',root+'/gradient.png');
for(let i=0;i<24;i++) writeFileSync(root+'/Row'+String(i).padStart(2,'0')+'.md','# Result '+i+'\n\nRowNeedle with a bounded preview for scrolling rows.');
const before = inventory();
const browser = await chromium.connectOverCDP('http://127.0.0.1:9318');
const page = browser.contexts()[0].pages().find(p=>p.url().startsWith('http://127.0.0.1:3828/'));
const observations = [];
const state = async()=>{await page.waitForTimeout(550);return page.evaluate(()=>window.pagefoldObservation());};
const lantern = (command,args=[]) => JSON.parse(execFileSync('lantern',[command,'--endpoint','http://127.0.0.1:9318',...args,'--json'],{encoding:'utf8'}));
async function click(id) {
 const s=await state(); const n=s.nodes.find(n=>n.id===id); assert(n?.enabled!==false && n, id);
 await page.mouse.click(n.rect[0]+Math.min(30,n.rect[2]/2),n.rect[1]+n.rect[3]/2, {delay:100});
 const result=await state(); observations.push({input:'physical Playwright pointer',id,state:result}); return result;
}
async function fill(id,text) {
 await page.bringToFront();
 const n=(await state()).nodes.find(n=>n.id===id);
 // Eframe's 1px browser input has padding and can cover the old caret.
 // Choose a measured point on the canvas, avoiding that DOM input overlay.
 const point = await page.evaluate(rect => {
  const y=rect[1]+rect[3]/2;
  for(const fraction of [0.8,0.5,0.2]) {
   const x=rect[0]+rect[2]*fraction;
   if(document.elementFromPoint(x,y)?.tagName==='CANVAS') return [x,y];
  }
  throw new Error('No uncovered canvas input point');
 },n.rect);
 await page.mouse.click(point[0],point[1],{delay:100});
 await page.waitForFunction(id => window.pagefoldObservation().nodes.find(n=>n.id===id)?.focused, id, {timeout:3000});
 await page.keyboard.press('Control+a'); await page.keyboard.press('Backspace');
 await page.waitForTimeout(100);
 const dispatch=lantern('type',['--selector','input','--text',text,'--timeout-ms','3000','--strict']);
 const s=await state(); observations.push({input:'Lantern type after measured focus/clear',id,dispatch,state:s}); return s;
}
function capture(name) {writeFileSync(out+'/'+name+'-capture.json',JSON.stringify(lantern('screenshot',['--output',out+'/'+name+'.png','--overwrite']),null,2));}
async function refresh() {await click('pagefold.refresh'); await page.waitForFunction(()=>!window.pagefoldObservation().pending);const s=await state(); observations.push({observation:'refresh settled',state:s});return s;}
try {
 await page.waitForFunction(()=>document.body.dataset.ready==='true');
 await page.setViewportSize({width:1100,height:900});
 await fill('directory',root); await click('pagefold.open');
 await page.waitForFunction(()=>!window.pagefoldObservation().pending && window.pagefoldObservation().root!=='');
 for (const width of [1100,390]) {
  await page.setViewportSize({width,height:900});
  for (const query of ['Heading Beacon','Alpha','List Beacon','Linklabel','Reference','CodeBeacon','TableBeacon','İSTANBUL','日本語','eCHO','\u0307','AltNeedle','IndentNeedle','invisible-destination','ReferenceTarget','**','PathOnly','DistantBeacon']) {
   let s=await fill('search',query); assert.equal(s.query,query); assert.equal(s.results.length,1);
   const path=query==='PathOnly'?'PathOnly.md':'Fixture.md';
   s=await click('page:'+path);
   if(query==='PathOnly') {assert.equal(s.target,null);assert.match(s.passage_notice,/Path-only/);}
   else {
    assert(s.target); assert(s.results[0][1].matched.length>0);
    assert.equal(s.target.visible,!['invisible-destination','ReferenceTarget','**','AltNeedle'].includes(query));
    if(s.target.block) {const r=s.nodes.find(n=>n.id==='reader').rect;assert(s.marker_rect);assert(s.marker_rect[1]>=r[1]-1 && s.marker_rect[1]<r[1]+r[3],JSON.stringify(s));}
   }
   if(query==='DistantBeacon') {assert(s.nodes.find(n=>n.id==='reader').scroll_y>1500);assert.deepEqual(s.target.matched,{start:6807,end:6820});}
   if(['CodeBeacon','TableBeacon','İSTANBUL','invisible-destination','ReferenceTarget','PathOnly','DistantBeacon'].includes(query)) capture(width+'-'+query.replace('İSTANBUL','unicode'));
  }
  let rows=await fill('search','RowNeedle'); assert.equal(rows.result_count,24);
  const initial=rows.nodes.filter(n=>n.id.startsWith('page:Row')).map(n=>n.id);
  await page.mouse.move(200,230); await page.mouse.wheel(0,255);
  rows=await state(); observations.push({input:'physical pointer wheel',state:rows});
  const scrolled=rows.nodes.filter(n=>n.id.startsWith('page:Row')).map(n=>n.id);
  assert.notDeepEqual(scrolled,initial);
  capture(width+'-multiple-results');
  await fill('search','DistantBeacon'); await click('page:Fixture.md');
  writeFileSync(out+'/'+width+'-layout.json',JSON.stringify(lantern('layout',['--container-selector','body']),null,2));
 }
 let s=await state(); const history=s.history;
 s=await click('page:Fixture.md'); assert.deepEqual(s.history,history);
 s=await click('pagefold.back'); assert.equal(s.query,'DistantBeacon');
 s=await click('pagefold.forward'); assert.equal(s.query,'DistantBeacon'); assert.deepEqual(s.history,history);
 s=await click('pagefold.back');
 await fill('search','ThirdOnly'); s=await click('page:ThirdOnly.md'); assert.equal(s.can_forward,false);
 await fill('search','DistantBeacon'); s=await click('page:Fixture.md');
 assert.deepEqual(inventory(),before);
 writeFileSync(out+'/inventory-before-edits.json',JSON.stringify({before,after:inventory(),unchanged:true},null,2));
 // Only this harness edits disposable fixture copies.
 writeFileSync(root+'/Fixture.md',original.replace('DistantBeacon','Edited prefix DistantBeacon'));
 s=await refresh(); assert.equal(s.target.matched.start,6821);assert(s.results[0][1].before.includes('Edited prefix'));
 writeFileSync(root+'/Fixture.md',original.replaceAll('DistantBeacon','RemovedNeedle'));
 s=await refresh(); assert.equal(s.target,null);assert.match(s.passage_notice,/no longer present/);capture('removed-match');
 renameSync(root+'/Fixture.md',out+'/removed-fixture.md');
 s=await refresh(); assert.equal(s.target,null);assert.equal(s.page,'Fixture.md');capture('removed-page');
 renameSync(out+'/removed-fixture.md',root+'/Fixture.md');
 writeFileSync(root+'/Fixture.md',original);
 s=await refresh();
 renameSync(root,root+'-offline');
 s=await refresh();assert.equal(s.snapshot_stale,true);
 await fill('search','PathOnly');s=await click('page:PathOnly.md');assert(s.snapshot_stale);
 s=await click('pagefold.back');assert(s.snapshot_stale);
 s=await click('pagefold.forward');assert(s.snapshot_stale);capture('stale-forward');
 renameSync(root+'-offline',root);
 s=await refresh();assert.equal(s.snapshot_stale,false);
 assert.deepEqual(inventory(),before);
 writeFileSync(out+'/inventory-final.json',JSON.stringify({before,after:inventory(),unchanged:true},null,2));
 console.log('PASS actual-app structures, excerpts, reveal, history, refresh/removal/stale and source inventories');
} finally {
 writeFileSync(out+'/browser-observations.json',JSON.stringify(observations,null,2));
 await browser.close();
}
