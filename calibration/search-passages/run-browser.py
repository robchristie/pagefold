"""Physical input and bounded observations for the provisional synthetic probe."""
import json
import hashlib
from pathlib import Path
import subprocess

OUT = Path('target/calibration')
HARNESS = 'calibration/search-passages/browser.mjs'
fixture = Path('calibration/search-passages/fixture.md')
before = hashlib.sha256(fixture.read_bytes()).hexdigest()


def run(*args):
    return subprocess.run(args, check=True, text=True, capture_output=True).stdout


observations = []
for width in (1100, 390):
    run('node', HARNESS, 'viewport', str(width))
    for query, visible in [('Heading Beacon', True), ('Alpha', True), ('List Beacon', True),
                           ('Linklabel', True), ('Reference', True), ('CodeBeacon', True),
                           ('TableBeacon', True), ('İSTANBUL', True), ('日本語', True),
                           ('Echo', True), ('invisible-destination', False),
                           ('ReferenceTarget', False), ('**', False), ('PathOnly', False),
                           ('DistantBeacon', True)]:
        run('node', HARNESS, 'focus')
        dispatch = json.loads(run('lantern', 'type', '--endpoint', 'http://127.0.0.1:9317',
                                 '--selector', 'input', '--text', query,
                                 '--timeout-ms', '3000', '--strict', '--json'))
        assert dispatch['interaction']['dispatch_state'] == 'acknowledged'
        state = json.loads(run('node', HARNESS, 'state', query))
        assert state['visible_source_text'] == visible, state
        assert state['query'] == query
        if query == 'PathOnly':
            assert state['match'] is None and state['marker_rect'] is None
        else:
            assert state['match'] is not None
        if state['block']:
            reader = next(n['rect'] for n in state['nodes'] if n['id'] == 'reader')
            marker = state['marker_rect']
            assert marker is not None
            assert reader[1] <= marker[1] < reader[1] + reader[3], state
        if query == 'DistantBeacon':
            assert state['scroll_y'] > 1500
            assert state['match'] == [6807, 6820]
        observations.append({'dispatch': dispatch, 'state': state})
        if (width, query) in [(1100, 'CodeBeacon'), (1100, 'TableBeacon'),
                              (1100, 'İSTANBUL'), (1100, 'invisible-destination'),
                              (1100, 'ReferenceTarget'), (1100, 'PathOnly'),
                              (1100, 'DistantBeacon'), (390, 'DistantBeacon')]:
            name = query.replace('İSTANBUL', 'unicode')
            capture = run('lantern', 'screenshot', '--endpoint', 'http://127.0.0.1:9317',
                          '--output', str(OUT / f'{width}-{name}.png'), '--overwrite', '--json')
            (OUT / f'{width}-{name}-capture.json').write_text(capture)
    layout = run('lantern', 'layout', '--endpoint', 'http://127.0.0.1:9317',
                 '--container-selector', 'body', '--json')
    (OUT / f'{width}-layout.json').write_text(layout)
(OUT / 'browser-observations.json').write_text(json.dumps(observations, indent=2) + '\n')
after = hashlib.sha256(fixture.read_bytes()).hexdigest()
assert before == after
(OUT / 'fixture-inventory.json').write_text(json.dumps({'path': str(fixture), 'before': before, 'after': after, 'unchanged': before == after}, indent=2) + '\n')
print(f'PASS: {len(observations)} physical query probes at desktop/narrow widths; distant reveal >1500 points')
