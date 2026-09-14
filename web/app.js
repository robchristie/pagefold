import init, {start, observation, address} from './pkg/pagefold_probe.js';

// Decode exactly once. Delimiters inside filenames must be percent encoded.
function parseAddress(hash) {
    if (!hash) return {workspace: '', page: ''};
    if (hash.length > 16384) throw Error('Page link exceeds 16384 characters');
    const fields = hash.slice(1).split('&');
    if (fields.length !== 2) throw Error('Expected workspace and page in the page link');
    const result = {};
    for (const field of fields) {
        const pair = field.split('=');
        if (pair.length !== 2 || !['workspace', 'page'].includes(pair[0]) || pair[0] in result)
            throw Error('Invalid or repeated page link field');
        result[pair[0]] = decodeURIComponent(pair[1]);
    }
    const {workspace, page} = result;
    if (!workspace?.startsWith('/') || workspace.length > 4096 || !page || page.length > 4096)
        throw Error('Expected an absolute workspace and relative Markdown page (at most 4096 characters each)');
    const unsafe = value => /[\\:\u0000-\u001f\u007f-\u009f]/u.test(value);
    if (unsafe(workspace) || unsafe(page) || page.startsWith('/') ||
        workspace.slice(1).split('/').some(p => !p || p === '.' || p === '..') ||
        page.split('/').some(p => !p || p === '.' || p === '..') || !/\.(md|markdown)$/i.test(page))
        throw Error('Unsafe or unsupported page link path');
    return result;
}
function fragment(workspace, page) {
    return `#workspace=${encodeURIComponent(workspace)}&page=${encodeURIComponent(page)}`;
}
await init();
await start(document.getElementById('reader'));
document.body.dataset.ready = 'true';
window.pagefoldObservation = () => JSON.parse(observation() || '{}');
let serial = 0;
let waiting = false;
let last = null;
const session = crypto.randomUUID();
let timeline = history.state?.pagefold === 1 ? history.state.timeline : crypto.randomUUID();
let position = history.state?.pagefold === 1 ? history.state.position : 0;
let mappedRoot = '';
let entries = [];
function entryState(o) { return {pagefold:1, timeline, position, session, cursor:o.history_cursor}; }
function remember(o) {
    if (mappedRoot !== o.root) entries = [];
    mappedRoot = o.root;
    entries = o.history.map((page,i) => entries[i]?.page === page ? entries[i] : {page});
    entries[o.history_cursor] = {page:o.page, position};
}
const copyButton = document.getElementById('copy-page-link');
const copyStatus = document.getElementById('copy-status');
const manualLink = document.getElementById('manual-link');
let currentLink = '';
copyButton.addEventListener('click', async () => {
    const link = currentLink;
    if (!link) return;
    try {
        await navigator.clipboard.writeText(link);
        if (currentLink === link) {
            copyStatus.textContent = 'Page link copied.';
            manualLink.hidden = true;
        }
    } catch {
        if (currentLink === link) {
            copyStatus.textContent = 'Could not copy automatically. Select the link below and copy it with Ctrl+C or ⌘C.';
            manualLink.value = link;
            manualLink.hidden = false;
            manualLink.focus();
            manualLink.select();
        }
    }
});
function loadAddress() {
    waiting = true;
    serial++;
    const state = history.state;
    if (state?.pagefold === 1 && state.timeline === timeline) {
        position = state.position;
    } else {
        // An untagged browser entry has no known offset. Start a new mapping;
        // application history remains useful, but must not guess a traversal.
        timeline = crypto.randomUUID();
        position = 0;
        entries = [];
    }
    try {
        const target = parseAddress(location.hash);
        address(target.workspace, target.page, state?.session === session ? state.cursor : undefined, serial, '');
    } catch (error) {
        address('', '', undefined, serial, `Invalid page link: ${error.message}. Choose a directory to continue.`);
    }
}
addEventListener('popstate', loadAddress);
loadAddress();
function sync() {
    const o = window.pagefoldObservation();
    let link = '';
    if (!waiting && !o.pending && o.page_available && o.root && o.page) {
        const hash = fragment(o.root, o.page);
        try { parseAddress(hash); link = location.origin + '/' + hash; } catch { /* Unsupported local names cannot form bounded links. */ }
    }
    if (link !== currentLink) {
        currentLink = link;
        copyButton.disabled = !link;
        manualLink.hidden = true;
        copyStatus.textContent = link ? 'Link opens this local workspace and page.' : 'Open a supported Markdown page to copy its link.';
    }
    if (o.address_serial === serial && !o.pending) {
        if (waiting) {
            waiting = false;
            last = o;
            if (o.root && o.page) {
                history.replaceState(entryState(o), '', fragment(o.root, o.page));
                remember(o);
            }
        } else if (o.root && o.page && (o.root !== last?.root || o.page !== last?.page)) {
            const destination = entries[o.history_cursor]?.position;
            if (o.root === last?.root && o.history.length === last?.history.length &&
                o.history.every((p,i) => p === last.history[i]) && destination !== undefined && destination !== position) {
                waiting = true;
                // Wait for popstate to deliver a new acknowledged request. The
                // old frame must not replace the URL while traversal is queued.
                serial++;
                history.go(destination - position);
            } else {
                // pushState discards the physical forward branch, including
                // entries still retained by the application's page history.
                entries = entries.map(entry => entry.position > position ? {page:entry.page} : entry);
                position++;
                history.pushState(entryState(o), '', fragment(o.root, o.page));
                remember(o);
            }
            last = o;
        } else last = o;
    }
    requestAnimationFrame(sync);
}
requestAnimationFrame(sync);
