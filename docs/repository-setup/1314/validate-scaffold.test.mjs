// PVF: deterministic local repository-contract proof; small CPU/filesystem,
// no network or model calls. Required setup proof, not product qualification.
import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, readdirSync, mkdtempSync, cpSync, writeFileSync, rmSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const root = dirname(fileURLToPath(import.meta.url));
const scaffold = join(root, 'scaffold');
const read = path => readFileSync(path, 'utf8');
const evidence = JSON.parse(read(join(root, 'readiness.json')));
const files = dir => readdirSync(dir, { withFileTypes: true }).flatMap(entry =>
  entry.isDirectory() ? files(join(dir, entry.name)) : [join(dir, entry.name)]);
const run = (cwd, program, args) => spawnSync(program, args, { cwd, encoding: 'utf8' });

test('published identity and documented local paths agree with setup evidence', () => {
  assert.equal(evidence.full_name, 'agent-logic/atec');
  assert.equal(evidence.private, true);
  assert.equal(evidence.visibility, 'private');
  assert.equal(evidence.default_branch, 'main');
  assert.equal(evidence.ci.conclusion, 'success');
  assert.match(evidence.head, /^[0-9a-f]{40}$/);
  const readme = read(join(scaffold, 'README.md'));
  assert.ok(readme.includes(evidence.html_url + '.git'));
  assert.ok(readme.includes(evidence.local_checkout));
  assert.ok(readme.includes(evidence.worktree_parent));
  for (const file of files(scaffold)) {
    if (!file.endsWith('.md')) continue;
    for (const match of read(file).matchAll(/\]\(([^)]+)\)/g)) {
      if (/^https:\/\//.test(match[1])) continue;
      assert.doesNotThrow(() => read(join(dirname(file), match[1])), relative(scaffold, file));
    }
  }
});

test('hygiene command accepts clean files and rejects actual whitespace defects', () => {
  const temp = mkdtempSync(join(root, '.bootstrap-check-'));
  try {
    cpSync(scaffold, temp, { recursive: true });
    assert.equal(run(temp, 'git', ['init', '-q']).status, 0);
    assert.equal(run(temp, 'git', ['add', '.']).status, 0);
    let result = run(temp, 'python3', ['tools/check_repository.py']);
    assert.equal(result.status, 0, result.stderr);
    assert.match(result.stdout, /9 tracked text files checked/);
    writeFileSync(join(temp, 'README.md'), 'trailing space \r\nmissing final newline');
    result = run(temp, 'python3', ['tools/check_repository.py']);
    assert.equal(result.status, 1);
    assert.match(result.stderr, /trailing whitespace/);
    assert.match(result.stderr, /CR line ending/);
    assert.match(result.stderr, /missing final newline/);
  } finally {
    rmSync(temp, { recursive: true, force: true });
  }
});

test('CI uses a pinned checkout with read-only permission and the real hygiene command', () => {
  const workflow = read(join(scaffold, '.github/workflows/check.yml'));
  assert.match(workflow, /actions\/checkout@[0-9a-f]{40}/);
  assert.match(workflow, /contents: read/);
  assert.match(workflow, /persist-credentials: false/);
  assert.match(workflow, /run: python3 tools\/check_repository.py/);
  assert.match(workflow, /timeout-minutes: 5/);
  assert.doesNotMatch(workflow, /pull_request_target|secrets\.|write-all/);
});
