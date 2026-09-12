"""Adversarial verification uses only disposable synthetic directories."""
import hashlib
import http.client
import importlib.util
import json
import os
from pathlib import Path
import shutil
import tempfile
import threading
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location('pagefold_server', Path(__file__).resolve().parents[1] / 'tools/server.py')
server = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(server)
SUPPLIED = Path(__file__).resolve().parent / 'fixtures/knowledge'


def inventory(root):
    return {str(p.relative_to(root)): (p.stat().st_size, hashlib.sha256(p.read_bytes()).hexdigest())
            for p in root.rglob('*') if p.is_file() and not p.is_symlink()}


class WorkspaceTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='pagefold-test-')
        self.base = Path(self.temp.name)
        self.root = self.base / 'knowledge'
        self.state = self.base / 'state'
        self.root.mkdir()
        self.state.mkdir()
        self.workspace = server.Workspace(self.state)
        (self.root / 'Home.md').write_text('# Home\nFirst searchable sentence.\n', encoding='utf-8')

    def tearDown(self):
        self.temp.cleanup()

    def test_external_add_edit_rename_delete_attachment_and_regeneration(self):
        initial = self.workspace.snapshot(self.root)
        self.assertIn('first searchable', initial['index']['Home.md'])
        (self.root / 'Home.md').write_text('# Home\nChanged kangaroo\n', encoding='utf-8')
        (self.root / 'New.md').write_text('Added wombat', encoding='utf-8')
        (self.root / 'image.png').write_bytes(b'synthetic image version 1')
        changed = self.workspace.snapshot(self.root)
        self.assertNotEqual(initial['generation'], changed['generation'])
        self.assertNotIn('first searchable', changed['index']['Home.md'])
        self.assertIn('kangaroo', changed['index']['Home.md'])
        self.assertIn('New.md', changed['pages'])
        (self.root / 'New.md').rename(self.root / 'Renamed.md')
        (self.root / 'Home.md').unlink()
        (self.root / 'image.png').write_bytes(b'synthetic image version 2')
        before = inventory(self.root)
        renamed = self.workspace.snapshot(self.root)
        self.assertNotIn('New.md', renamed['pages'])
        self.assertNotIn('Home.md', renamed['index'])
        self.assertIn('Renamed.md', renamed['index'])
        self.assertNotEqual(changed['images'], renamed['images'])
        (self.state / 'index.json').unlink()
        regenerated = self.workspace.snapshot(self.root)
        self.assertEqual(renamed, regenerated)
        self.assertEqual(before, inventory(self.root))
        persisted = json.loads((self.state / 'index.json').read_text())
        self.assertEqual(persisted['pages'], regenerated['index'])
        (self.state / 'index.json').write_bytes(b'{corrupt generated index')
        self.assertEqual(regenerated, self.workspace.snapshot(self.root))
        self.assertEqual(json.loads((self.state / 'index.json').read_text())['pages'], regenerated['index'])
        self.assertEqual(before, inventory(self.root))
        (self.root / 'image.png').unlink()
        self.assertNotIn('image.png', self.workspace.snapshot(self.root)['images'])

    def test_state_overlap_rejected_before_writes(self):
        nested = self.root / 'derived'
        nested.mkdir()
        before = inventory(self.base)
        for path in (self.root, nested, nested / 'not-created', self.base):
            with self.assertRaises(ValueError):
                server.Workspace(path).snapshot(self.root)
        self.assertEqual(before, inventory(self.base))
        self.assertFalse((nested / 'not-created').exists())

    def test_symlinks_special_files_and_index_hardlinks(self):
        outside = self.base / 'outside'
        outside.mkdir()
        secret = outside / 'Outside.md'
        secret.write_text('Must not appear')
        (self.root / 'escape.md').symlink_to(secret)
        (self.root / 'escape-dir').symlink_to(outside, target_is_directory=True)
        (self.root / 'internal.md').symlink_to(self.root / 'Home.md')
        os.mkfifo(self.root / 'pipe.md')
        before = inventory(self.base)
        snap = self.workspace.snapshot(self.root)
        self.assertEqual(list(snap['pages']), ['Home.md'])
        self.assertNotIn('Must not appear', json.dumps(snap))
        self.assertIn('Blocked symlink', snap['attachments']['escape.md'])
        self.assertEqual(snap['attachments']['pipe.md'], 'Unsupported special file')
        alias = self.base / 'source-alias'
        alias.symlink_to(self.root, target_is_directory=True)
        with self.assertRaises(OSError):
            self.workspace.snapshot(alias)
        alias_state = self.base / 'state-alias'
        alias_state.symlink_to(self.state, target_is_directory=True)
        with self.assertRaises(OSError):
            server.Workspace(alias_state).snapshot(self.root)
        (self.state / 'index.json').unlink()
        os.link(secret, self.state / 'index.json')
        self.workspace.snapshot(self.root)
        self.assertEqual(secret.read_text(), 'Must not appear')
        (self.state / 'index.json').unlink()
        (self.state / 'index.json').symlink_to(secret)
        self.workspace.snapshot(self.root)
        self.assertEqual(secret.read_text(), 'Must not appear')
        for path, identity in before.items():
            self.assertEqual(inventory(self.base)[path], identity)

    def test_unreadable_unsupported_and_limits_are_explicit(self):
        (self.root / 'Bad.md').write_bytes(b'\xff\xfe')
        (self.root / 'movie.mp4').write_bytes(b'unsupported')
        (self.root / 'Huge.md').write_bytes(b'x' * (server.MAX_FILE + 1))
        snap = self.workspace.snapshot(self.root)
        self.assertNotIn('Bad.md', snap['pages'])
        self.assertIn('Unreadable', snap['attachments']['Bad.md'])
        self.assertIn('Unsupported', snap['attachments']['movie.mp4'])
        self.assertIn('4 MiB', snap['attachments']['Huge.md'])
        previous = (self.state / 'index.json').read_bytes()
        with patch.object(server, 'MAX_TOTAL', 1):
            with self.assertRaises(OverflowError):
                self.workspace.snapshot(self.root)
        self.assertEqual(previous, (self.state / 'index.json').read_bytes())

    def test_supplied_fixtures_read_only_and_isolated_copy_changes(self):
        before = inventory(SUPPLIED)
        snap = self.workspace.snapshot(SUPPLIED)
        self.assertIn('guides/Reading.md', snap['pages'])
        self.assertIn('attachments/gradient.png', snap['images'])
        copy = self.base / 'copy'
        shutil.copytree(SUPPLIED, copy)
        (copy / 'Home.md').write_text('External replacement')
        copied = self.workspace.snapshot(copy)
        self.assertEqual(copied['pages']['Home.md'], 'External replacement')
        self.assertEqual(before, inventory(SUPPLIED))
        self.assertNotEqual(copied['pages']['Home.md'], snap['pages']['Home.md'])

    def test_transport_uses_same_snapshot_and_rejects_cross_origin(self):
        handler = type('TestHandler', (server.Handler,), {'workspace': self.workspace, 'log_message': lambda *args: None})
        httpd = server.http.server.HTTPServer(('127.0.0.1', 0), handler)
        thread = threading.Thread(target=httpd.serve_forever)
        thread.start()
        try:
            port = httpd.server_port
            conn = http.client.HTTPConnection('127.0.0.1', port)
            body = json.dumps({'directory': str(self.root)})
            for headers in (
                {'Content-Type': 'application/json', 'Origin': 'https://evil.invalid'},
                {'Content-Type': 'application/json', 'Host': 'evil.invalid'},
                {'Content-Type': 'text/plain'},
            ):
                conn.request('POST', '/api/snapshot', body, headers)
                response = conn.getresponse()
                self.assertEqual(response.status, 403)
                response.read()
            conn.request('POST', '/api/snapshot', body, {'Content-Type': 'application/json'})
            response = conn.getresponse()
            self.assertEqual(response.status, 200)
            self.assertEqual(json.loads(response.read()), self.workspace.snapshot(self.root))
            conn.request('GET', '/../../etc/passwd')
            response = conn.getresponse()
            self.assertEqual(response.status, 404)
            response.read()
            conn.close()
        finally:
            httpd.shutdown()
            thread.join()
            httpd.server_close()


if __name__ == '__main__':
    unittest.main()
