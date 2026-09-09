import init, {start, observation} from './pkg/pagefold_probe.js';
await init();
await start(document.getElementById('reader'));
document.body.dataset.ready = 'true';
window.pagefoldObservation = () => JSON.parse(observation());
