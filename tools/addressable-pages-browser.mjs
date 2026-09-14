// Disposable browser qualification. PAGEFOLD_PLAYWRIGHT names an installed module;
// PAGEFOLD_CDP names an already owned browser. This harness owns its service only.
import assert from 'node:assert/strict';
import {mkdir, writeFile, readFile, readdir, rename, symlink, readlink} from 'node:fs/promises';
import {spawn, execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {resolve} from 'node:path';
const {chromium} = await import(process.env.PAGEFOLD_PLAYWRIGHT || 'playwright');
const scratch = resolve(process.argv[2] || 'target/addressable-browser');
const origin = 'http://127.0.0.1:3830';
const cdp = process.env.PAGEFOLD_CDP || 'http://127.0.0.1:9328';
const root = scratch+'/root space 日本 %?#';
const other = scratch+'/other';
const special = 'nested/space 日本 %?#.md';
const hash = text => createHash('sha256').update(text).digest('hex');
const observations = [], errors = [];
let server, browser, context, p;
const url = (r,page) => origin+'/#workspace='+encodeURIComponent(r)+'&page='+encodeURIComponent(page);
async function startServer() {
    server = spawn('python3', ['tools/server.py','--state',scratch+'/state','--port','3830'], {stdio:['ignore','pipe','pipe']});
    await new Promise((res,rej) => { server.stdout.once('data',res);server.once('exit',code=>rej(Error('Service exited '+code)));server.once('error',rej); });
}
async function stopServer() { if(server && server.exitCode === null) { const done = new Promise(res=>server.once('exit',res));server.kill('SIGTERM');await done; } }
async function inventory(directory) {
    const files={};
    async function walk(dir) {for(const entry of await readdir(dir,{withFileTypes:true})) {const p=dir+'/'+entry.name;if(entry.isDirectory()) await walk(p);else if(entry.isFile()) files[p.slice(directory.length+1)]=hash(await readFile(p));else if(entry.isSymbolicLink()) files[p.slice(directory.length+1)]='symlink:'+await readlink(p);}}
    await walk(directory);return files;
}
function lantern(command,args,name) {
    const result=JSON.parse(execFileSync('lantern',[command,'--endpoint',cdp,...args,'--json'],{encoding:'utf8',timeout:35000}));
    assert.equal(result.ok,true);return writeFile(scratch+'/'+name+'.json',JSON.stringify(result,null,2));
}
try {
    await mkdir(root+'/nested',{recursive:true});await mkdir(other,{recursive:true});await mkdir(scratch+'/state',{recursive:true});
    await writeFile(root+'/A.md','# Alpha\n\n[Special](nested/space%20日本%20%25%3F%23.md)\n\nneedle alpha\n');
    await writeFile(root+'/'+special,'# Special page\n\nneedle special\n\n[Alpha](../A.md)\n');
    await writeFile(other+'/B.md','# Other workspace\n');
    await symlink(other+'/B.md',root+'/escape.md');
    const original=await inventory(root), otherOriginal=await inventory(other);
    await startServer();browser=await chromium.connectOverCDP(cdp);context=await browser.newContext({viewport:{width:1100,height:900}});
    await context.grantPermissions(['clipboard-read','clipboard-write'],{origin});
    p=await context.newPage();
    p.on('pageerror',e=>errors.push(e.message));
    async function wait(page,r=root) {
        await p.waitForFunction(({page,r})=>window.pagefoldObservation?.().page===page&&pagefoldObservation().root===r&&!pagefoldObservation().pending,{page,r});
        await p.waitForTimeout(120);
    }
    async function state(name) {
        console.log(name);
        const o=await p.evaluate(()=>({url:location.href,length:history.length,...pagefoldObservation()}));
        observations.push({name,url:o.url,root:o.root,page:o.page,history:o.history,cursor:o.history_cursor,query:o.query,status:o.status,stale:o.snapshot_stale,target:o.target,marker:o.marker_rect,length:o.length});return o;
    }
    async function click(id) {
        const n=await p.evaluate(id=>pagefoldObservation().nodes.find(n=>n.id===id),id);assert(n?.enabled!==false,id+' disabled');
        const canvas=await p.locator('canvas').boundingBox();
        await p.mouse.click(canvas.x+n.rect[0]+n.rect[2]/2,canvas.y+n.rect[1]+n.rect[3]/2);
    }
    await p.goto(url(root,special));await wait(special);await state('independently constructed special-character URL');
    await p.locator('#copy-page-link').click();await p.getByText('Page link copied.',{exact:true}).waitFor();
    const copied=await p.evaluate(()=>navigator.clipboard.readText());assert.equal(copied,url(root,special));
    const fresh=await browser.newContext({viewport:{width:1100,height:900}});const tab=await fresh.newPage();
    await tab.goto(copied);await tab.waitForFunction(s=>window.pagefoldObservation?.().page===s,special);assert.equal(await tab.evaluate(()=>pagefoldObservation().root),root);await fresh.close();
    await p.reload();await wait(special);await state('reload copied link');
    await stopServer();await startServer();await p.reload();await wait(special);await state('service restart copied link');
    await click('page:A.md');await wait('A.md');assert.equal(p.url(),url(root,'A.md'));await state('list changes address');
    // Canvas link position derives from the opened reference image: first link
    // below the page heading. The resulting page, not dispatch, establishes pass.
    const reader=await p.evaluate(()=>pagefoldObservation().nodes.find(n=>n.id==='reader'));
    const canvas=await p.locator('canvas').boundingBox();
    await p.mouse.click(canvas.x+reader.rect[0]+30,canvas.y+reader.rect[1]+90);
    await wait(special);await state('internal Markdown link changes address');
    await click('search');await p.keyboard.insertText('needle');
    await p.waitForFunction(()=>pagefoldObservation().query==='needle');
    await click('page:A.md');await wait('A.md');let o=await state('search selects passage');assert(o.target);assert(o.marker_rect);
    const length=o.length;await click('page:A.md');await wait('A.md');o=await state('repeat result reveal');assert.equal(o.length,length);assert(o.marker_rect);
    await p.goBack();await wait(special);o=await state('browser Back retains query');assert.equal(o.query,'needle');
    await p.goForward();await wait('A.md');await click('pagefold.back');await wait(special);assert.equal(p.url(),url(root,special));
    await click('pagefold.forward');await wait('A.md');o=await state('app Forward retains query');assert.equal(o.query,'needle');assert.equal(o.length,length);
    assert.deepEqual(await inventory(root),original);assert.deepEqual(await inventory(other),otherOriginal);await state('source inventories unchanged by copy and navigation');
    await p.goto(url(root,special));await wait(special);
    const generation=await p.evaluate(()=>pagefoldObservation().generation);
    await writeFile(root+'/'+special,'# Fresh external edit\n\nneedle changed body\n');
    const edited=await inventory(root);await click('pagefold.refresh');await p.waitForFunction(g=>pagefoldObservation().generation!==g&&!pagefoldObservation().pending,generation);await state('external edit refreshed');
    await rename(root+'/'+special,root+'/nested/Renamed.md');await click('pagefold.refresh');await p.waitForFunction(()=>!pagefoldObservation().pending&&!pagefoldObservation().page_available);o=await state('renamed page remains intended unavailable target');assert.equal(o.page,special);
    await rename(root+'/nested/Renamed.md',root+'/'+special);await click('pagefold.refresh');await p.waitForFunction(()=>!pagefoldObservation().pending&&pagefoldObservation().page_available);
    await rename(root,root+'-offline');await click('pagefold.refresh');await p.waitForFunction(()=>pagefoldObservation().snapshot_stale);
    await click('page:A.md');await wait('A.md');o=await state('failed refresh warning survives list URL update');assert(o.snapshot_stale);assert(o.status.includes('Previous snapshot'));
    await p.goBack();await wait(special);o=await state('failed refresh warning survives browser Back');assert(o.snapshot_stale);
    await rename(root+'-offline',root);await click('pagefold.refresh');await p.waitForFunction(()=>!pagefoldObservation().snapshot_stale&&!pagefoldObservation().pending);assert.deepEqual(await inventory(root),edited);
    await p.goto(url(root+'/absent','Intended.md'));await p.waitForFunction(()=>pagefoldObservation().status.includes('Unable to read workspace'));o=await state('unavailable workspace names target');assert(o.status.includes(root+'/absent'));assert.equal(o.root,'');
    await click('directory');await p.keyboard.press('Control+A');await p.keyboard.insertText(other);await click('pagefold.open');await wait('B.md',other);await state('choose another directory recovery and isolation');
    await p.goto(url(other,special));await wait(special,other);o=await state('same page name cannot leak from other workspace');assert.equal(await p.evaluate(()=>pagefoldObservation().page_available),false);
    await p.goto(url(root,'escape.md'));await wait('escape.md');assert.equal(await p.evaluate(()=>pagefoldObservation().page_available),false);await state('symlink page excluded');
    const bad=['#workspace='+encodeURIComponent(root)+'&page=..%2FA.md','#workspace=%ZZ&page=A.md','#workspace='+encodeURIComponent(root)+'&page=%2Fetc%2Fpasswd.md','#workspace='+encodeURIComponent(root)+'&page=A.md&extra=x','#workspace='+encodeURIComponent(root)+'&page='+('x'.repeat(16400))+'.md'];
    for(const target of bad) {
        const serial=await p.evaluate(()=>pagefoldObservation().address_serial);
        await p.goto(origin+'/'+target);
        await p.waitForFunction(serial=>pagefoldObservation().address_serial>serial&&pagefoldObservation().status.includes('Invalid page link'),serial);
        assert.equal(await p.evaluate(()=>pagefoldObservation().root),'');
        if(target.length>16384) assert((await p.evaluate(()=>pagefoldObservation().status)).includes('exceeds 16384'));
    }
    await state('malformed oversized and traversal targets rejected');
    await p.goto(url(root,special));await wait(special);
    // Collect Lantern against the exact candidate with collection before reload.
    const targetId=(await context.newCDPSession(p));const info=await targetId.send('Target.getTargetInfo');const target=['--target-id',info.targetInfo.targetId];
    await p.goto('about:blank');
    await lantern('flow',[...target,'--open',url(root,special),'--timeout-ms','15000','--quiet-ms','1000'],'flow');
    await wait(special);await p.locator('#copy-page-link').click();await p.getByText('Page link copied.',{exact:true}).waitFor();
    await lantern('layout',[...target,'--container-selector','body'],'desktop-layout');
    await lantern('screenshot',[...target,'--output',scratch+'/desktop.png','--overwrite'],'desktop-capture');
    await targetId.send('Browser.setPermission',{permission:{name:'clipboard-write'},setting:'denied',origin});
    await targetId.send('Browser.setPermission',{permission:{name:'clipboard-write',allowWithoutSanitization:true},setting:'denied',origin});
    // Chromium may permit sanitised writes on user activation despite a permission
    // override. A real document policy establishes clipboard unavailability;
    // application code and navigator.clipboard are not replaced by a mock.
    await p.route(origin+'/',async route=>{const response=await route.fetch();await route.fulfill({response,headers:{...response.headers(),'permissions-policy':'clipboard-write=()'}});});
    await p.reload();await wait(special);
    await p.locator('#copy-page-link').click();await p.locator('#manual-link').waitFor({state:'visible'});assert.equal(await p.locator('#manual-link').inputValue(),url(root,special));
    assert.equal(await p.locator('#manual-link').evaluate(el=>el.selectionEnd-el.selectionStart),url(root,special).length);
    await p.setViewportSize({width:390,height:844});await p.waitForTimeout(250);
    await lantern('layout',[...target,'--container-selector','body'],'narrow-layout');
    await lantern('screenshot',[...target,'--output',scratch+'/narrow.png','--overwrite'],'narrow-capture');await state('clipboard denial manual fallback at narrow width');
    await p.goto(url(root,'Removed.md'));await wait('Removed.md');
    await lantern('screenshot',[...target,'--output',scratch+'/unavailable.png','--overwrite'],'unavailable-capture');
    assert.deepEqual(await inventory(root),edited);assert.deepEqual(await inventory(other),otherOriginal);assert.deepEqual(errors,[]);
    await writeFile(scratch+'/results.json',JSON.stringify({pass:true,observations,errors,inventories:{original,edited,otherOriginal},intentionalEdits:['special body replaced','special renamed and restored','root renamed offline and restored'],source:{head:execFileSync('git',['rev-parse','HEAD'],{encoding:'utf8'}).trim(),files:Object.fromEntries(await Promise.all(['src/lib.rs','web/app.js','web/index.html','web/pkg/pagefold_probe_bg.wasm','tools/addressable-pages-browser.mjs'].map(async f=>[f,hash(await readFile(f))])))}},null,2));
    console.log(JSON.stringify({pass:true,cases:observations.length,evidence:scratch}));
} catch(error) {
    if(p) {
        await writeFile(scratch+'/failure.json',JSON.stringify({error:String(error),errors,observations,state:await p.evaluate(()=>({url:location.href,...window.pagefoldObservation?.()}))},null,2));
        await p.screenshot({path:scratch+'/failure.png'});
    }
    throw error;
} finally {await context?.close();await browser?.close();await stopServer();}
