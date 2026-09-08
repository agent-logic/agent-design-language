import { readFileSync } from "node:fs";
import { randomUUID } from "node:crypto";

const tokenPath = process.argv[2];
if (!tokenPath) throw new Error("token path is required");
const token = readFileSync(tokenPath, "utf8").trim();
const nonce = randomUUID().replaceAll("-", "");
const conversationId = `conversation-707-${nonce}`;
const turnId = `turn-707-${nonce}`;
const correlationId = nonce;
const socket = new WebSocket(
  "wss://127.0.0.1:20997/v1/observatory/ws?schema=v2",
);
let intentSent = false;
const deadline = setTimeout(() => {
  console.error("live A2A proof timed out");
  socket.close();
  process.exitCode = 2;
}, 15 * 60 * 1000);

socket.addEventListener("open", () => {
  socket.send(JSON.stringify({
    schema: "adl.runtime_v3.observatory_ws_auth.v1",
    bearer_token: token,
  }));
});

socket.addEventListener("message", ({ data }) => {
  const message = JSON.parse(String(data));
  if (message.schema === "adl.runtime_v3.observatory_ws_control_result.v1"
      && message.status === "authenticated" && !intentSent) {
    intentSent = true;
    socket.send(JSON.stringify({
      schema: "adl.runtime_v3.observatory_conversation_intent.v1",
      conversation_id: conversationId,
      turn_id: turnId,
      recipient_id: "beacon",
      correlation_id: correlationId,
      message: "Use the initiate_agent action to send Ember Axioma (recipient_id gemma-e4b) a short welcome to Axioma Polis. Tell Ember that you are Beacon Axioma, the Polis Shepherd, and offer help. Return a brief confirmation to the operator after initiating the governed message.",
    }));
    console.log(JSON.stringify({event: "intent_sent", conversation_id: conversationId, turn_id: turnId, correlation_id: correlationId}));
    return;
  }
  if (message.schema === "adl.runtime_v3.observatory_conversation_result.v1"
      && message.correlation_id === correlationId) {
    console.log(JSON.stringify(message));
    if (["delivered", "refused", "failed", "cancelled"].includes(message.status)) {
      clearTimeout(deadline);
      socket.close();
      if (message.status !== "delivered" || !message.initiated_work_id
          || message.initiated_recipient_id !== "gemma-e4b") {
        process.exitCode = 1;
      }
    }
  }
});

socket.addEventListener("error", (error) => {
  clearTimeout(deadline);
  console.error(`websocket error: ${error.message || "unknown"}`);
  process.exitCode = 3;
});
