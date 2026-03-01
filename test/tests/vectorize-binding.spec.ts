import { describe, test, expect } from "vitest";
import { mf, mfUrl } from "./mf";

describe("vectorize binding", () => {
  test("query success path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}vectorize-binding`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      ok: true,
      matches: [{ id: "v1", score: 0.99 }],
    });
  });

  test("missing binding path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}vectorize-binding-missing`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toContain("MISSING_VECTORIZE");
  });
});
