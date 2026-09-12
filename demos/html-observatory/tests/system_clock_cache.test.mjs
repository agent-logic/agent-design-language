import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

const testUrl = new URL(import.meta.url);
const [html, app] = await Promise.all([
  readFile(new URL("../index.html", testUrl), "utf8"),
  readFile(new URL("../app.js", testUrl), "utf8")
]);

let now = "2026-09-08T03:00:00Z";
const intervals = [];
const nodes = new Map([
  ["hero-uptime", { textContent: "pending" }],
  ["rail-capture-time", { textContent: "pending" }]
]);

class ControlledDate extends Date {
  constructor(value) {
    super(value === undefined ? now : value);
  }
}

const context = vm.createContext({
  URL,
  URLSearchParams,
  Date: ControlledDate,
  setInterval(callback, delay) {
    intervals.push({ callback, delay });
    return intervals.length;
  }
});
vm.runInContext(app, context, { filename: "app.js" });
context.document = {
  getElementById(id) {
    return nodes.get(id) || null;
  }
};

context.AdlHtmlObservatory.startSystemClock();
assert.equal(intervals.length, 1, "clock startup must create one timer");
assert.equal(intervals[0].delay, 1000, "clock must refresh every second");
const first = nodes.get("rail-capture-time").textContent;

now = "2026-09-08T03:00:02Z";
intervals[0].callback();
const second = nodes.get("rail-capture-time").textContent;
assert.notEqual(second, first, "displayed System Time must advance after the timer fires");
assert.equal(nodes.get("hero-uptime").textContent, second, "both visible clocks must remain synchronized");

context.AdlHtmlObservatory.startSystemClock();
assert.equal(intervals.length, 1, "restarting the clock must not leak another timer");

const cssVersion = html.match(/styles\.css\?v=([^"']+)/)?.[1];
const appVersion = html.match(/app\.js\?v=([^"']+)/)?.[1];
assert.equal(appVersion, "v0922-live-720", "JavaScript must use the corrected cache generation");
assert.equal(cssVersion, appVersion, "JavaScript and CSS must share one cache generation");

console.log("Observatory advancing System Time and cache generation proof: PASS");
