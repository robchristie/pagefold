"""Read-only fixture inventory; compare before and after calibration."""
import hashlib
import json
import pathlib
import sys

root = pathlib.Path('/nvme/development/pagefold-supervision-20260909-a/knowledge')
inventory = [
    {'path': str(p.relative_to(root)), 'bytes': p.stat().st_size,
     'sha256': hashlib.sha256(p.read_bytes()).hexdigest()}
    for p in sorted(root.rglob('*')) if p.is_file()
]
destination = pathlib.Path(sys.argv[1])
destination.write_text(json.dumps(inventory, indent=2) + '\n')
if len(sys.argv) > 2:
    assert inventory == json.loads(pathlib.Path(sys.argv[2]).read_text())
    print('PASS: fixture inventory and all content hashes unchanged')
else:
    print('Recorded fixture inventory:', len(inventory), 'files')
