#!/usr/bin/env python3
"""Local Pagefold transport and read-only filesystem snapshot owner."""
import argparse
import hashlib
import http.server
import json
import os
from pathlib import Path
import stat

MAX_FILE = 4 * 1024 * 1024
MAX_TOTAL = 32 * 1024 * 1024
MAX_ENTRIES = 10000


def checked_directory(path):
    """Reject symlink components; traverse by directory descriptor."""
    path = Path(os.path.abspath(path))
    fd = os.open('/', os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=fd)
            os.close(fd)
            fd = child
        return path, fd
    except BaseException:
        os.close(fd)
        raise


class Workspace:
    def __init__(self, state):
        self.state = Path(os.path.abspath(state))

    def snapshot(self, directory):
        root, root_fd = checked_directory(directory)
        try:
            state = self.state.resolve()
            if state == root or state in root.parents or root in state.parents:
                raise ValueError('State directory must be separate from, and not contain, the knowledge directory')
            # State must already exist. No source-adjacent directories are created.
            _, state_fd = checked_directory(self.state)
            try:
                result = self._scan(root, root_fd)
                index = {'version': 1, 'root': str(root), 'pages': result['index']}
                payload = json.dumps(index, ensure_ascii=False).encode()
                # Exclusive new file and replacement cannot follow index symlinks/hardlinks.
                name = '.index-' + os.urandom(16).hex()
                fd = os.open(name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=state_fd)
                try:
                    with os.fdopen(fd, 'wb') as stream:
                        stream.write(payload)
                        stream.flush()
                        os.fsync(stream.fileno())
                    os.replace(name, 'index.json', src_dir_fd=state_fd, dst_dir_fd=state_fd)
                finally:
                    try:
                        os.unlink(name, dir_fd=state_fd)
                    except FileNotFoundError:
                        pass
                result['state'] = str(self.state / 'index.json')
                return result
            finally:
                os.close(state_fd)
        finally:
            os.close(root_fd)

    def _scan(self, root, root_fd):
        result = {'root': str(root), 'pages': {}, 'images': {}, 'attachments': {}, 'index': {}, 'warnings': []}
        total = 0
        count = 0

        def walk(fd, prefix='', depth=0):
            nonlocal total, count
            if depth > 64:
                raise OverflowError('Directory nesting exceeds 64 levels')
            for name in sorted(os.listdir(fd)):
                count += 1
                if count > MAX_ENTRIES:
                    raise OverflowError('Workspace exceeds 10,000 entries; select a smaller directory')
                path = prefix + name
                try:
                    info = os.stat(name, dir_fd=fd, follow_symlinks=False)
                    if stat.S_ISLNK(info.st_mode):
                        result['attachments'][path] = 'Blocked symlink (all source symlinks are excluded)'
                        result['warnings'].append(f'Blocked symlink: {path}')
                        continue
                    if stat.S_ISDIR(info.st_mode):
                        child = os.open(name, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=fd)
                        try:
                            walk(child, path + '/', depth + 1)
                        finally:
                            os.close(child)
                        continue
                    if not stat.S_ISREG(info.st_mode):
                        result['attachments'][path] = 'Unsupported special file'
                        continue
                    if any(c in path for c in ('\\', ':')) or any(ord(c) < 32 for c in path):
                        result['warnings'].append(f'Unsupported filename: {path!r}')
                        continue
                    extension = Path(name).suffix.lower()
                    if extension not in ('.md', '.markdown', '.png', '.jpg', '.jpeg', '.gif', '.webp'):
                        result['attachments'][path] = 'Unsupported attachment; open with an appropriate external application'
                        continue
                    file_fd = os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=fd)
                    with os.fdopen(file_fd, 'rb') as stream:
                        current = os.fstat(stream.fileno())
                        if not stat.S_ISREG(current.st_mode):
                            raise ValueError('Not a regular file')
                        if current.st_size > MAX_FILE:
                            raise ValueError('File exceeds the 4 MiB preview limit')
                        data = stream.read(MAX_FILE + 1)
                    if len(data) > MAX_FILE:
                        raise ValueError('File exceeds the 4 MiB preview limit')
                    total += len(data)
                    if total > MAX_TOTAL:
                        raise OverflowError('Workspace exceeds the 32 MiB snapshot limit')
                    if extension in ('.md', '.markdown'):
                        text = data.decode('utf-8')
                        result['pages'][path] = text
                        result['index'][path] = text.lower()
                    else:
                        result['images'][path] = list(data)
                        result['attachments'][path] = 'Local image'
                except (OSError, ValueError) as error:
                    message = f'Unreadable or unsupported: {error}'
                    result['attachments'][path] = message
                    result['warnings'].append(f'{path}: {message}')
        walk(root_fd)
        result['generation'] = hashlib.sha256(json.dumps(result, sort_keys=True).encode()).hexdigest()
        return result


class Handler(http.server.BaseHTTPRequestHandler):
    workspace = None
    web = Path(__file__).resolve().parents[1] / 'web'

    def allowed(self):
        expected = f'127.0.0.1:{self.server.server_port}'
        origin = self.headers.get('Origin')
        return self.headers.get('Host') == expected and (origin is None or origin == 'http://' + expected)

    def respond(self, status, data, mime='application/json'):
        self.send_response(status)
        self.send_header('Content-Type', mime)
        self.send_header('Content-Length', str(len(data)))
        self.send_header('Cache-Control', 'no-store')
        self.send_header('X-Content-Type-Options', 'nosniff')
        self.send_header('Content-Security-Policy', "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; connect-src 'self'; img-src 'self' data:; frame-ancestors 'none'")
        self.end_headers()
        self.wfile.write(data)

    def do_POST(self):
        if not self.allowed() or self.path != '/api/snapshot' or self.headers.get('Content-Type') != 'application/json':
            self.respond(403, b'{"error":"Request rejected"}')
            return
        try:
            length = int(self.headers.get('Content-Length', '0'))
            if not 0 < length <= 16384:
                raise ValueError('Invalid request length')
            request = json.loads(self.rfile.read(length))
            directory = request['directory']
            if not isinstance(directory, str) or not directory:
                raise ValueError('Enter a knowledge directory')
            snapshot = self.workspace.snapshot(directory)
            self.respond(200, json.dumps(snapshot, ensure_ascii=False).encode())
        except (OSError, ValueError, OverflowError, KeyError, TypeError) as error:
            self.respond(400, json.dumps({'error': str(error)}).encode())

    def do_GET(self):
        if not self.allowed():
            self.respond(403, b'{}')
            return
        files = {'/': ('index.html', 'text/html'), '/app.js': ('app.js', 'text/javascript'),
                 '/pkg/pagefold_probe.js': ('pkg/pagefold_probe.js', 'text/javascript'),
                 '/pkg/pagefold_probe_bg.wasm': ('pkg/pagefold_probe_bg.wasm', 'application/wasm')}
        if self.path not in files:
            self.respond(404, b'{}')
            return
        path, mime = files[self.path]
        try:
            self.respond(200, (self.web / path).read_bytes(), mime)
        except OSError:
            self.respond(404, b'{"error":"Build the browser application first"}')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--state', required=True, help='Existing separate derived-state directory')
    parser.add_argument('--port', type=int, default=3817)
    args = parser.parse_args()
    Handler.workspace = Workspace(args.state)
    server = http.server.HTTPServer(('127.0.0.1', args.port), Handler)
    print(f'Pagefold: http://127.0.0.1:{args.port}/ (state: {args.state})', flush=True)
    server.serve_forever()


if __name__ == '__main__':
    main()
