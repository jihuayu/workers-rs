import { describe, test, expect } from "vitest";
import { mf, mfUrl } from "./mf";

describe("hyperdrive connect", () => {
  test("success path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}hyperdrive-connect`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toBe("connected");
  });

  test("missing binding path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}hyperdrive-connect-missing`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toContain("MISSING_HYPERDRIVE");
  });
});
