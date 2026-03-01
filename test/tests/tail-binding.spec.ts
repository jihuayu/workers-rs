import { describe, test, expect } from "vitest";
import { mf, mfUrl } from "./mf";

describe("tail binding", () => {
  test("known variant parses to typed kind", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}tail-binding-known`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      invocationId: "inv-123",
      sequence: 42,
      kind: "log",
      raw: {
        type: "log",
      },
    });
  });

  test("unknown variant falls back safely", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}tail-binding-unknown`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      kind: "futureVariant",
      raw: {
        type: "futureVariant",
      },
    });
  });
});
