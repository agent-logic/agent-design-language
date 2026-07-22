import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

const sourcePath = process.argv[2];
if (!sourcePath) throw new Error("diagram path is required");

const chunks = "/opt/homebrew/opt/mermaid-cli/libexec/lib/node_modules/@mermaid-js/mermaid-cli/node_modules/mermaid/dist/chunks/mermaid.esm";
const parserFile = fs.readdirSync(chunks).find((name) => /^flowDiagram-.*\.mjs$/.test(name));
if (!parserFile) throw new Error("installed Mermaid flowchart parser was not found");

const { diagram } = await import(pathToFileURL(path.join(chunks, parserFile)));
const noop = () => undefined;

// Exercise Mermaid's generated grammar without invoking its browser renderer.
diagram.parser.parser.yy = {
  lex: { firstGraph: () => true },
  addClass: noop,
  addLink: noop,
  addSubGraph: noop,
  addVertex: noop,
  destructLink: () => ({ type: "arrow_point", stroke: "normal", length: 1 }),
  setAccDescription: noop,
  setAccTitle: noop,
  setClass: noop,
  setClickEvent: noop,
  setDirection: noop,
  setLink: noop,
  setTooltip: noop,
  updateLink: noop,
  updateLinkInterpolate: noop,
};

diagram.parser.parse(fs.readFileSync(sourcePath, "utf8"));
process.stdout.write(`${JSON.stringify({ status: "pass", parser: "mermaid-flowchart", diagram: ".csdlc/prepared/issues/5500/diagram.mmd" })}\n`);
