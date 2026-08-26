import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const testUrl = new URL(import.meta.url);

const [html, css] = await Promise.all([
  readFile(new URL("../index.html", testUrl), "utf8"),
  readFile(new URL("../styles.css", testUrl), "utf8")
]);

const requiredIds = [
  "runtime-proof",
  "panopticon",
  "communication",
  "agent-conversation-recipient",
  "agent-conversation-transcript",
  "agent-conversation-message",
  "send-agent-conversation",
  "governed-room-recipients",
  "governed-room-transcript",
  "operator-attention-inbox",
  "operator-attention-list",
  "evidence"
];

for (const id of requiredIds) {
  assert.match(html, new RegExp(`id=["']${id}["']`), `${id} must remain addressable`);
}

const focusableSelectors = [
  ".skip-link:focus",
  ".button:focus-visible",
  ".dashboard-rail a:focus-visible",
  ".surface-nav a:focus-visible",
  ".roster-row:focus-visible",
  "input:focus-visible",
  "select:focus-visible",
  "textarea:focus-visible"
];

for (const selector of focusableSelectors) {
  assert.match(css, new RegExp(selector.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `${selector} needs visible focus treatment`);
}

assert.match(html, /class=["']skip-link["'] href=["']#runtime-proof["']/, "keyboard users need a skip link");
assert.match(html, /<main[\s\S]*class=["']observatory["']/, "primary Observatory content must remain a main landmark");
assert.match(html, /<header class=["']topbar["'] aria-label=["']Observatory status["']/, "topbar must retain an accessible label");
assert.match(html, /<aside class=["']dashboard-rail["'] aria-label=["']Observatory navigation["']/, "dashboard rail must retain navigation label");
assert.match(html, /<nav class=["']surface-nav["'] aria-label=["']Observatory navigation["']/, "surface nav must retain navigation label");
assert.match(html, /aria-live=["']polite["'][\s\S]*id=["']agent-conversation-transcript["']|id=["']agent-conversation-transcript["'][\s\S]*aria-live=["']polite["']/, "agent transcript must announce updates");
assert.match(html, /aria-live=["']polite["'][\s\S]*id=["']governed-room-transcript["']|id=["']governed-room-transcript["'][\s\S]*aria-live=["']polite["']/, "room transcript must announce updates");
assert.match(html, /id=["']governed-room-recipients["'][^>]*aria-describedby=["']governed-room-help["']/, "multi-select room recipients must point to explicit recipient constraints");
assert.match(html, /aria-describedby=["']claim-boundary["']/, "status grid must expose the proof boundary");
assert.match(html, /aria-labelledby=["']hero-ready-label hero-ready-state["']/, "runtime readiness stat must expose label and state");
assert.match(html, /role=["']group["'] aria-label=["']Runtime controls["']/, "top controls must remain grouped for assistive tech");

assert.match(css, /@media\s*\(prefers-reduced-motion:\s*reduce\)/, "reduced-motion users need explicit static behavior");
assert.match(css, /scroll-behavior:\s*auto/, "reduced-motion mode must disable smooth scrolling");
assert.match(css, /@media\s*\(max-width:\s*980px\)/, "tablet responsive breakpoint must remain declared");
assert.match(css, /@media\s*\(max-width:\s*640px\)/, "mobile responsive breakpoint must remain declared");
assert.match(css, /\.observatory[\s\S]*overflow:\s*hidden/, "viewport shell must avoid horizontal body overflow");
assert.match(css, /\.panopticon-shell[\s\S]*overflow-y:\s*auto/, "detailed Observatory surfaces must remain reachable by scrolling");
assert.match(css, /\.dashboard-rail[\s\S]*overflow-x:\s*auto/, "mobile rail must remain reachable without clipping");

console.log("WP-18C.07a Observatory accessibility/responsive proof: PASS");
