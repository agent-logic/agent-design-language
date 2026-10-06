// PVF runtime lane: deterministic mocked browser proof, local CPU only.
// Required #1145 desktop/mobile acceptance; no live Runtime or provider execution.
import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import { readFile } from 'node:fs/promises';
const { chromium } = createRequire(import.meta.url)('playwright');
const root = new URL('../', import.meta.url);
const browser = await chromium.launch({channel: 'chrome', headless: true});
const agent = id => ({id, label: `Agent ${id}`, state: 'configured', communication_eligible: true});
const feed = sample => ({
  schema: 'adl.runtime_v3.observatory_feed.v3', runtime_instance_id: 'fixture-runtime', runtime_incarnation_id: 'incarnation1',
  polis_identity: {polis_id: 'fixture', display_name: 'Fixture Polis', public_domain: 'runtime.agent-logic.ai', runtime_api_base: 'https://runtime.agent-logic.ai', observatory_public_origin: 'https://observatory.agent-logic.ai'},
  health: {observability_ready: true, snapshot: {lifecycle: 'running'}}, events: [],
  agents: {revision: 1, event_cursor: 'cursor1', scope: 'local_runtime', population_complete: true, total_count: sample.length, sample}
});
try {
  for (const viewport of [{width: 1280, height: 900}, {width: 390, height: 844}]) {
    const page = await browser.newPage({viewport});
    page.setDefaultTimeout(10000);
    const errors = []; page.on('pageerror', e => errors.push(e.message));
    let current = feed([agent('a'), agent('b')]);
    let releasePage;
    let pageStarted;
    let nextPageStarted = new Promise(resolve => { pageStarted = resolve; });
    await page.addInitScript(() => {
      window.fixtureSent = [];
      window.WebSocket = class extends EventTarget {
        static OPEN = 1;
        constructor() { super(); this.readyState = 1; window.fixtureSocket = this; setTimeout(() => this.dispatchEvent(new Event('open')), 0); }
        send(bytes) { window.fixtureSent.push(JSON.parse(bytes)); }
        close() { this.readyState = 3; }
      };
      window.fixtureEmit = frame => window.fixtureSocket.dispatchEvent(new MessageEvent('message', {data: JSON.stringify(frame)}));
    });
    await page.route('**/*', async route => {
      const url = new URL(route.request().url());
      const name = url.pathname.split('/').pop() || 'index.html';
      if (url.hostname === 'observatory.agent-logic.ai' && ['index.html','app.js','styles.css','runtime-v3.config.json'].includes(name)) {
        if (name === 'runtime-v3.config.json') return route.fulfill({json: {...JSON.parse(await readFile(new URL(name, root))), api_base:'https://runtime.agent-logic.ai', trusted_hosts:['runtime.agent-logic.ai']}});
        return route.fulfill({body: await readFile(new URL(name, root)), contentType: name.endsWith('.js') ? 'text/javascript' : name.endsWith('.css') ? 'text/css' : 'text/html'});
      }
      if (url.pathname === '/v1/agents') {
        assert.equal(url.searchParams.has('event_cursor'), false, 'same-revision page must omit successor cursor');
        assert.equal(url.searchParams.get('page_size'), '1');
        await new Promise(resolve => {releasePage = resolve; pageStarted();});
        return route.fulfill({json:{schema:'adl.runtime_v3.agent_roster_page.v1', revision:current.agents.revision, event_cursor:current.agents.event_cursor, scope:'local_runtime', visible_count:2, population_complete:true, agents:[agent('b')], has_more:false, next_page_token:null}});
      }
      if (url.pathname.startsWith('/v1/')) return route.fulfill({json: url.pathname === '/v1/observatory' ? current : {ready:true, degraded_reasons:[]}});
      return route.fulfill({status:404, body:''});
    });
    await page.goto('https://observatory.agent-logic.ai/index.html?live=1#communication');
    await page.waitForFunction(() => !!window.fixtureSocket);
    await page.evaluate(value => window.fixtureEmit(value), current);
    await page.locator('[data-dashboard-link="communication"]').first().click();
    await page.locator('.chat-advanced-summary').filter({hasText:'Multi-agent room'}).click();
    const everyone = page.locator('#governed-room-everyone'), send = page.locator('#send-governed-room-turn'), summary = page.locator('#governed-room-selection-summary');
    await everyone.waitFor({state:'visible'});
    await everyone.focus(); await page.keyboard.press('Enter');
    await page.waitForFunction(() => document.querySelector('#governed-room-recipients').selectedOptions.length === 2);
    await page.locator('#governed-room-message').fill('fixture only');
    assert.equal(await send.isDisabled(), true, 'selection cannot grant write authorization');
    assert.equal((await page.evaluate(() => window.fixtureSent)).length, 0, 'Everyone cannot implicitly send');
    await page.evaluate(() => window.fixtureEmit({schema:'adl.runtime_v3.observatory_ws_control_result.v1', status:'authenticated'}));
    assert.equal(await send.isEnabled(), true);
    await page.getByRole('button', {name:'Deselect Agent b (b)', exact:true}).click();
    assert.match(await summary.textContent(), /1 selected \/ 2 eligible/);
    await send.click();
    const sent = await page.evaluate(() => window.fixtureSent);
    assert.deepEqual(sent.at(-1).intent.addressed_recipients, ['a']);
    await page.locator('#governed-room-message').fill('next fixture');
    // Add a newcomer without expanding the prepared selection; skip one revision
    // to avoid the separate successor-cursor authentication route in this fixture.
    current = feed([agent('a'),agent('b'),agent('c')]); current.agents.revision = 3; current.agents.event_cursor = 'cursor3';
    await page.evaluate(value => window.fixtureEmit(value), current);
    await page.waitForFunction(() => document.querySelector('#governed-room-selection-summary').textContent.includes('1 selected / 3 eligible'));
    current.agents.sample = [agent('b'),agent('c')]; current.agents.total_count = 2; current.agents.revision = 5; current.agents.event_cursor = 'cursor5';
    await page.evaluate(value => window.fixtureEmit(value), current);
    await page.waitForFunction(() => document.querySelector('#governed-room-selection-summary').textContent.includes('availability'));
    assert.equal(await send.isDisabled(), true);
    // Nine agents are selected visibly; remove one using an ordinary touch button.
    current = feed(Array.from({length:9}, (_, i) => agent(`n${i}`))); current.agents.revision=7; current.agents.event_cursor='cursor7';
    await page.evaluate(value => window.fixtureEmit(value), current); await everyone.click();
    assert.match(await summary.textContent(), /9 selected.*Limit is 8/);
    assert.equal(await send.isDisabled(), true);
    await page.getByRole('button', {name:'Deselect Agent n0 (n0)', exact:true}).click();
    assert.equal(await send.isEnabled(), true);
    const touch = await page.locator('.room-recipient-remove').first().boundingBox(); assert(touch.height >= 44);
    for (const selector of ['#governed-room-everyone', '#governed-room-clear', '#governed-room-recipients', '#governed-room-selection-summary']) {
      const bounds = await page.locator(selector).boundingBox();
      assert(bounds.x >= 0 && bounds.x + bounds.width <= viewport.width, `${selector} must fit the viewport`);
    }
    // Page loading disables old-selection sends; Clear cancels the eventual result.
    current = feed([agent('a')]); Object.assign(current.agents, {revision:9,event_cursor:'cursor9',total_count:2,rendered_sample_count:1,has_more:true,next_page_token:'page2'});
    await page.evaluate(value => window.fixtureEmit(value), current);
    await page.locator('#governed-room-recipients').selectOption(['a']);
    assert.equal(await send.isEnabled(), true, 'valid old selection exists before loading');
    await everyone.click();
    await nextPageStarted;
    await page.waitForFunction(() => document.querySelector('#governed-room-selection-summary').textContent.includes('Loading'));
    assert.equal(await send.isDisabled(), true);
    await page.locator('#governed-room-clear').click();
    const cancelledResponse = page.waitForResponse(response => new URL(response.url()).pathname === '/v1/agents');
    releasePage(); await cancelledResponse;
    assert.match(await summary.textContent(), /0 selected/);
    nextPageStarted = new Promise(resolve => { pageStarted = resolve; });
    await everyone.click();
    await nextPageStarted; releasePage();
    await page.waitForFunction(() => document.querySelector('#governed-room-recipients').selectedOptions.length === 2);
    // Replacement runtime at the same endpoint cannot retain consent by agent ID.
    current.runtime_incarnation_id='incarnation2'; current.agents={...feed([agent('a'),agent('b')]).agents};
    await page.evaluate(value => window.fixtureEmit(value), current);
    await page.waitForFunction(() => document.querySelector('#governed-room-recipients').selectedOptions.length === 0);
    await page.locator('#governed-room-title').scrollIntoViewIfNeeded();
    await page.screenshot({path:`/tmp/1145-room-${viewport.width}.png`});
    assert.deepEqual(errors, []);
    await page.close();
  }
  console.log('PASS: desktop/mobile Everyone, explicit sends, authorization, limits, roster drift, paging cancellation, incarnation reset');
} finally { await browser.close(); }
