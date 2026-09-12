// PVF tooling: deterministic, local CPU, #720 release acceptance regression.
// The companion live_only.browser.mjs proves page-level transitions in Chrome.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
const [html, app] = await Promise.all(['index.html', 'app.js'].map(name => readFile(new URL('../' + name, import.meta.url), 'utf8')));
await import('../app.js');
test('Live UI has no historical telemetry routes, mode switches or polling', () => {
  assert.doesNotMatch(html, /(?:data-mode|value)="(?:published|retained)"/);
  assert.match(html, /data-mode="live"/);
  assert.doesNotMatch(app, /refreshRetained|retainedPollTimer|fetchRetainedRuntimeSnapshot|loadJson\(packetRef\)|setRuntimeTestStatus/);
  assert.match(app, /const packet = FALLBACK_PACKET/);
  assert.match(app, /modeSelect.value = "live"/);
});
test('initial live model is empty and cannot inherit historical citizens', () => {
  const { FALLBACK_PACKET, buildPanopticonViewModel } = globalThis.AdlHtmlObservatory;
  const model = buildPanopticonViewModel({ mode: 'live' }, FALLBACK_PACKET);
  assert.equal(model.mode, 'live');
  assert.equal(model.agentTotal, 0);
  assert.deepEqual(model.events, []);
  assert.equal(model.readyState, 'unknown');
});
