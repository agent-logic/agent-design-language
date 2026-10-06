// PVF runtime lane: deterministic UI contract proof; local CPU only, small resources.
// Required for #1145 acceptance; does not establish installed/provider/release proof.
import assert from 'node:assert/strict';
import test from 'node:test';
await import('../app.js');
const { createGovernedRoomSelection, completeGovernedRoomRoster, buildGovernedRoomTurnIntent } = globalThis.AdlHtmlObservatory;
const agent = (id, extra = {}) => ({ id, label: id, state: 'configured', communication_eligible: true, ...extra });
const population = (sample, extra = {}) => ({ sample, total_count: sample.length, revision: 1, event_cursor: 'cursor', scope: 'local', population_complete: false, ...extra });
const ids = state => state.recipients.map(p => p.participant_id);
test('Everyone selects policy-eligible configured and busy agents, never a wildcard', () => {
  const selection = createGovernedRoomSelection();
  selection.refresh(population([agent('a'), agent('b', { state: 'busy' }), agent('c', { communication_eligible: false }), agent('*'), agent('all')]));
  assert.deepEqual(ids(selection.everyone()), ['a', 'b']);
  assert.equal(selection.view().canSend, true);
  const intent = buildGovernedRoomTurnIntent({ roomId: 'room-a-b', turnId: 'turn-1', senderId: 'operator', correlationId: 'corr-1', recipients: ids(selection.view()), message: 'hello' });
  assert.deepEqual(intent.addressed_recipients, ['a', 'b']);
  assert.deepEqual(ids(selection.select(['b'])), ['b']);
  assert.equal(selection.clear().canSend, false);
  selection.refresh(population([]));
  assert.deepEqual(ids(selection.everyone()), []);
});
test('roster newcomers never join; removed or ineligible selections block until explicit reselection', () => {
  const selection = createGovernedRoomSelection();
  selection.refresh(population([agent('a'), agent('b')]), 'polis/runtime/incarnation1');
  selection.everyone();
  selection.refresh(population([agent('a'), agent('b'), agent('c')]), 'polis/runtime/incarnation1');
  assert.deepEqual(ids(selection.view()), ['a', 'b']);
  selection.refresh(population([agent('a'), agent('b', { communication_eligible: false })]));
  assert.deepEqual(ids(selection.view()), ['a', 'b']);
  assert.equal(selection.view().canSend, false);
  selection.refresh(population([agent('a'), agent('b')]));
  assert.equal(selection.view().canSend, false, 'a returning ID cannot silently regain consent');
  assert.equal(selection.everyone().canSend, true);
  selection.refresh(population([agent('a'), agent('b')]), 'polis/runtime/incarnation2');
  assert.deepEqual(ids(selection.view()), [], 'new incarnation clears prepared recipients');
});
test('nine eligible agents remain visible and block sends until reduced to eight', () => {
  const selection = createGovernedRoomSelection();
  selection.refresh(population(Array.from({length: 9}, (_, i) => agent(`agent${i}`))));
  const all = selection.everyone();
  assert.equal(all.recipients.length, 9);
  assert.equal(all.overLimit, true);
  assert.equal(all.canSend, false);
  assert.equal(selection.select(ids(all).slice(1)).canSend, true);
});
test('Everyone collects raw Runtime pages at the same revision and rejects incomplete or drifting pages', async () => {
  const first = population([agent('a')], {total_count: 2, has_more: true, next_page_token: 'page2'});
  const page = { agents: [agent('b')], visible_count: 2, revision: 1, event_cursor: 'cursor', scope: 'local', population_complete: false, has_more: false, next_page_token: null };
  const full = await completeGovernedRoomRoster(first, async token => { assert.equal(token, 'page2'); return page; });
  assert.deepEqual(full.sample.map(a => a.id), ['a', 'b']);
  for (const delta of [{revision: 2}, {event_cursor: 'other'}, {visible_count: 3}, {scope: 'other'}, {agents: [agent('a')]}, {agents: []}]) {
    await assert.rejects(completeGovernedRoomRoster(first, async () => ({...page, ...delta})));
  }
  assert.deepEqual((await completeGovernedRoomRoster(population([]), async () => page)).sample, []);
  await assert.rejects(completeGovernedRoomRoster({...first, has_more: false, next_page_token: null}, async () => page));
});
