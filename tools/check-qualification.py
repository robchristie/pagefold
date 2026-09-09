"""Validate retained qualification postconditions and unchanged accepted source."""
import hashlib
import json
import os
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = Path(os.environ.get('PAGEFOLD_EVIDENCE', ROOT / 'evidence/qualification'))

def state(name):
    return json.loads((EVIDENCE / f'{name}-state.json').read_text())

manifest = EVIDENCE / 'source-artefacts.json'
if not manifest.exists():
    manifest = EVIDENCE / 'accepted-inputs.json'
for artefact in json.loads(manifest.read_text()):
    if artefact['path'] in ('README.md', 'docs/active-plan.md'):
        continue  # This package owns final use/plan documentation.
    data = (ROOT / artefact['path']).read_bytes()
    assert len(data) == artefact['bytes'], artefact['path']
    assert hashlib.sha256(data).hexdigest() == artefact['sha256'], artefact['path']

baseline = (ROOT / 'evidence/calibration/fixtures-before.json').read_bytes()
assert (EVIDENCE / 'fixtures-before.json').read_bytes() == baseline
assert (EVIDENCE / 'fixtures-after.json').read_bytes() == baseline
assert state('home')['root'] == '/nvme/development/pagefold-supervision-20260909-a/knowledge'
assert state('keyboard-focus')['nodes'][1]['focused']
assert state('home')['page'] == 'Home.md'
assert state('unicode')['page'] == 'Unicode.md'
assert state('back-home')['page'] == 'Home.md'
assert state('reading')['page'] == 'guides/Reading.md'
scroll = lambda name: next(n['scroll_y'] for n in state(name)['nodes'] if n['id'] == 'reader')
assert scroll('reading-scrolled') > 2000
assert scroll('reading-return') == 0
for name, expected in [('missing-page', 'Missing.md'), ('missing-image-link', 'attachments/missing.png'),
                       ('unsupported', 'Unsupported attachment'), ('blocked', 'Blocked path outside'),
                       ('external', 'Blocked absolute, external or unsafe')]:
    assert expected in state(name)['status'], name
assert state('narrow-search')['query'] == 'café'
assert [n['id'] for n in state('narrow-search')['nodes'] if n['id'].startswith('page:')] == ['page:Unicode.md']
assert state('narrow-search')['page'] == 'Unicode.md'
assert state('loading')['pending']
assert all(not n['enabled'] for n in state('loading')['nodes'] if n['id'].startswith('pagefold.'))
assert state('edited-open')['page'] == 'Change.md'
assert not any(n['id'].startswith('page:') for n in state('edited-open')['nodes'])
assert state('added-result')['page'] == 'Added.md'
assert state('renamed-open')['page'] == 'Added.md'
assert any(n['id'] == 'page:Renamed.md' for n in state('renamed-open')['nodes'])
assert state('deleted-open')['page'] == 'Renamed.md'
assert not any(n['id'].startswith('page:') for n in state('deleted-open')['nodes'])
assert state('failed-refresh')['generation'] == state('attachment-removed')['generation']
assert 'Previous snapshot retained' in state('failed-refresh')['status']
assert 'refreshed' in state('refresh-recovered')['status']
if (EVIDENCE / 'stale-navigation-state.json').exists():
    for name in ('failed-refresh', 'stale-navigation', 'stale-search'):
        assert state(name)['snapshot_stale'], name
        assert 'Previous snapshot retained' in state(name)['status'], name
    assert state('stale-navigation')['page'] == 'guides/Reading.md'
    assert state('stale-search')['page'] == 'Unicode.md'
    assert not state('refresh-recovered')['snapshot_stale']
assert state('empty-directory')['page'] == ''
assert json.loads((EVIDENCE/'document-state.json').read_text())['injected'] is None
assert (EVIDENCE/'refresh.log').read_text().startswith('PASS:')
print('PASS: manifest source unchanged, fixture history equal, retained physical UI postconditions consistent; pixel judgements remain in report.md')
