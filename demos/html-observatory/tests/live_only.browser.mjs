// PVF tooling lane; deterministic browser regression; local CPU/network only.
// Required for #720 acceptance; mocked API responses do not prove deployment.
import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import { readFile } from 'node:fs/promises';
const { chromium } = createRequire(import.meta.url)('playwright');
const root = new URL('../', import.meta.url);
const browser = await chromium.launch({ channel: 'chrome', headless: true });
try {
  for (const initiallyOffline of [false, true]) {
    const page = await browser.newPage();
    let offline = initiallyOffline;
    const requests = [];
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    await page.addInitScript(() => {
      window.WebSocket = class extends EventTarget {
        static OPEN = 1;
        constructor() { super(); setTimeout(() => this.dispatchEvent(new Event("error")), 10); }
        close() {}
      };
      window.intervals = [];
      const interval = window.setInterval;
      window.setInterval = (fn, ms, ...args) => {
        window.intervals.push({ name: fn.name, ms });
        return interval(fn, ms, ...args);
      };
    });
    const feed = {
      schema: 'adl.runtime_v3.observatory_feed.v3', runtime_instance_id: 'test-runtime',
      runtime_incarnation_id: 'test-incarnation',
      polis_identity: { polis_id: 'test', display_name: 'Live Test Polis', public_domain: 'runtime.agent-logic.ai', runtime_api_base: 'https://runtime.agent-logic.ai', observatory_public_origin: 'https://observatory.agent-logic.ai' },
      health: { observability_ready: true, snapshot: { lifecycle: 'running', event_count: 7 } },
      agents: { total_count: 7, sample: [] }, events: []
    };
    await page.route('**/*', async route => {
      const url = new URL(route.request().url());
      requests.push(url.href);
      if (url.hostname === 'observatory.agent-logic.ai' && !url.pathname.startsWith('/docs/')) {
        const name = url.pathname.split('/').pop() || 'index.html';
        if (['index.html', 'app.js', 'styles.css', 'runtime-v3.config.json'].includes(name)) {
          return route.fulfill({ body: await readFile(new URL(name, root)), contentType: name.endsWith('.js') ? 'text/javascript' : name.endsWith('.css') ? 'text/css' : name.endsWith('.json') ? 'application/json' : 'text/html' });
        }
      }
      if (url.pathname.startsWith('/v1/')) {
        if (offline) return route.fulfill({ status: 503, body: 'offline' });
        return route.fulfill({ json: url.pathname === '/v1/observatory' ? feed : { schema: 'adl.runtime_v3.readiness.v1', ready: true, degraded_reasons: [] } });
      }
      return route.fulfill({ status: 404, body: '' });
    });
    await page.goto('https://observatory.agent-logic.ai/index.html?mode=retained');
    await page.waitForFunction(() => document.querySelector('.observatory').getAttribute('data-live-connection') === (document.querySelector('#hero-agent-count').textContent === '7' ? 'live-read' : 'disconnected'));
    assert.equal(await page.locator('[data-mode="retained"], [data-mode="published"], option[value="retained"], option[value="published"]').count(), 0);
    assert.equal(await page.locator('#top-mode-select').inputValue(), 'live');
    assert.equal(await page.locator('#hero-agent-count').textContent(), initiallyOffline ? '0' : '7');
    if (!initiallyOffline) {
      offline = true;
      // A normal refresh must preserve the most recent live sample and mark it stale.
      await page.evaluate(() => document.querySelector('[data-mode="live"]').click());
      await page.waitForFunction(() => document.querySelector('.observatory').getAttribute('data-live-connection') === 'disconnected');
      assert.equal(await page.locator('#hero-agent-count').textContent(), '7');
      assert.match(await page.locator('#stale-banner-detail').textContent(), /last received snapshot/);
    }
    for (const hash of ['agents', 'runtime', 'evidence', 'communication']) {
      await page.evaluate(hash => { window.location.hash = hash; }, hash);
      assert.equal(await page.locator('#hero-agent-count').textContent(), initiallyOffline ? '0' : '7');
    }
    assert.equal(await page.locator('#stale-banner').isVisible(), true);
    assert.equal(requests.some(url => /csm_liveness_4976\/published\/api|observatory-packet.*json/.test(url)), false, requests.join('\n'));
    assert.equal(await page.evaluate(() => window.intervals.some(timer => timer.name === 'refreshRetained' || timer.ms === 3000)), false);
    assert.deepEqual(errors, []);
    await page.close();
  }
  console.log('PASS: live-only startup, offline startup, disconnection, navigation, no historical telemetry or timer');
} finally { await browser.close(); }
