import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

test("issue 708 orientation source and Observatory surface are present", async () => {
  const [welcomePackage, observatory] = await Promise.all([
    readFile("docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md", "utf8"),
    readFile("demos/html-observatory/app.js", "utf8"),
  ]);

  assert.match(welcomePackage, /AXIOMA POLIS WELCOME PACKAGE/i);
  assert.ok(observatory.length > 0);
});
