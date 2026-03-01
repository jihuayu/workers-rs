import { describe, test, expect } from "vitest";
import { mf, mfUrl } from "./mf";

describe("workflows binding", () => {
  test("trigger success path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}workflows-binding`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      ok: true,
      id: "wf-001",
      input: { name: "demo" },
    });
  });

  test("missing binding path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}workflows-binding-missing`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toContain("MISSING_WORKFLOWS");
  });
});
