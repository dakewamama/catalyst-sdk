import { expect, test } from "bun:test";
import { decodeEvent } from "../src/decoder.ts";
import { EVENT_IX_TAG } from "../src/constants.ts";

test("decodes SubscriptionCreated from known wire bytes", () => {
  const buf = new Uint8Array(9 + 32 + 32 + 32 + 8);
  buf.set(EVENT_IX_TAG, 0);
  buf[8] = 0; // SubscriptionCreated discriminator
  buf.fill(1, 9, 41);   // plan
  buf.fill(2, 41, 73);  // subscriber
  buf.fill(3, 73, 105); // mint
  new DataView(buf.buffer).setBigInt64(105, 42n, true); // created_ts

  const event = decodeEvent(buf);
  expect(event?.type).toBe("SubscriptionCreated");
});