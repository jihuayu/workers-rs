import { describe, test, expect } from "vitest";
import { mf, mfUrl } from "./mf";

describe("images binding", () => {
  test("success path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}images-binding`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      ok: true,
      mode: "binding",
      input: { source: "raw-image" },
    });
  });

  test("missing binding path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}images-binding-missing`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toContain("MISSING_IMAGES");
  });
});
