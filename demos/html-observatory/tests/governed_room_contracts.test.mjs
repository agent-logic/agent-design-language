// PVF runtime lane: local deterministic contract assertions required for #1145.
// Named node:test case makes legacy assertion scripts actual runner evidence.
import test from 'node:test';
test('governed-room protocol, accessibility and conversation contracts', async () => {
  await import('../../../adl/tools/validate_v092_governed_room_observatory.mjs');
  await import('./accessibility_responsive.test.mjs');
  await import('./conversation_sessions.test.mjs');
});
