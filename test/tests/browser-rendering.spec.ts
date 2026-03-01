import { describe, test, expect } from "vitest";
import { mf, mfUrl } from "./mf";

describe("browser rendering binding", () => {
  test("success path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}browser-rendering`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      ok: true,
      input: { url: "https://example.com" },
    });
  });

  test("missing binding path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}browser-rendering-missing`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toContain("MISSING_BROWSER_RENDERING");
  });
});
