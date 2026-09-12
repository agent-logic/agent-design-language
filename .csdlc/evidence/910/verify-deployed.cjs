// PVF: live deployment proof; nondeterministic network/cloud/browser lane.
// Read-only, no mocks/interception, no credentials or privileged commands.
const { chromium } = require('playwright');
const fs = require('node:fs');
const assert = require('node:assert/strict');
(async () => {
  const browser = await chromium.launch({ executablePath: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome', headless: true });
  try {
    const context = await browser.newContext();
    await context.grantPermissions(["local-network-access"], {origin:"https://observatory.csm.agent-logic.ai"});
    const page = await context.newPage();
    const runtimeResponses = []; const websocketSchemas = new Set(); const acceptedAgentCounts = new Set();
    const acceptedSchema = "adl.runtime_v3.observatory_feed.v3";
    let frames = 0, pageErrors = 0, websocketCount = 0, writeRequests = 0;
    page.on('pageerror', () => { pageErrors++; });
    page.on('request', r => { if (!['GET', 'HEAD', 'OPTIONS'].includes(r.method())) writeRequests++; });
    page.on('response', r => {
      const u = new URL(r.url());
      if (u.hostname === 'wuji.dev.csm.agent-logic.ai') runtimeResponses.push({ path: u.pathname, status: r.status() });
    });
    page.on('websocket', ws => {
      if (!ws.url().startsWith('wss://wuji.dev.csm.agent-logic.ai:20997/v1/observatory/ws')) return;
      websocketCount++;
      ws.on('framereceived', ({payload}) => {
        frames++;
        try { const frame = JSON.parse(payload); const s = frame.schema; if (s === acceptedSchema && Number.isInteger(frame.agents?.total_count)) acceptedAgentCounts.add(frame.agents.total_count); if (typeof s === 'string' && /^adl\.[a-zA-Z0-9_.]+$/.test(s)) websocketSchemas.add(s); } catch {}
      });
    });
    const response = await page.goto('https://observatory.csm.agent-logic.ai/?runtime=v3&live=1', {waitUntil:'domcontentloaded', timeout:60000});
    assert.equal(response.status(),200);
    await page.waitForSelector('.observatory[data-live-connection="connected"]', {timeout:60000});
    await page.waitForFunction(() => document.getElementById('statusbar-source')?.textContent !== 'awaiting live Runtime', {timeout:15000});
    const ui = await page.evaluate(() => ({
      liveConnection: document.querySelector('.observatory').dataset.liveConnection,
      staleBannerHidden: document.getElementById('stale-banner').hidden,
      sendMessageDisabled: document.getElementById('send-agent-conversation').disabled,
      sendRoomTurnDisabled: document.getElementById('send-governed-room-turn').disabled,
      sendSignedCommandDisabled: document.getElementById('send-signed-command').disabled,
      loginAvailable: Boolean(document.getElementById('operator-login')),
      localStorageEntryCount: localStorage.length,
      displayedAgentCount: Number(document.getElementById("hero-agent-count").textContent.replaceAll(",", "")),
      websocketStatus: document.getElementById("statusbar-websocket").textContent
    }));
    assert.equal(ui.liveConnection,'connected'); assert.equal(ui.staleBannerHidden,true);
    assert.ok(ui.sendMessageDisabled && ui.sendRoomTurnDisabled && ui.sendSignedCommandDisabled);
    assert.ok(websocketCount>0 && frames>0);
    assert.ok(websocketSchemas.has(acceptedSchema));
    assert.ok(acceptedAgentCounts.has(ui.displayedAgentCount), "Rendered agent total must match an actual accepted live v3 frame");
    assert.equal(ui.websocketStatus,"connected"); assert.equal(pageErrors,0); assert.equal(writeRequests,0);
    const result = {captured_at:new Date().toISOString(),url:page.url(),http_status:response.status(),network_interception:false,temporary_site_permission:"local-network-access",permission_origin:"https://observatory.csm.agent-logic.ai",persistent_browser_setting_changed:false,public_read_verified:true,authenticated_write_claim:false,ui,websocket_count:websocketCount,received_frames:frames,frame_schemas:[...websocketSchemas].sort(),accepted_schema:acceptedSchema,accepted_live_agent_counts:[...acceptedAgentCounts],runtime_responses:runtimeResponses,page_errors:pageErrors,write_requests:writeRequests};
    fs.writeFileSync(process.argv[2],JSON.stringify(result,null,2)+'\n');
    console.log('Actual HTTPS/WSS browser public-read and unauthenticated write-gating proof PASS');
  } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode=1; });
