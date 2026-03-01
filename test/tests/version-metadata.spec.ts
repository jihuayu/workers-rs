import { describe, test, expect } from "vitest";
import { mf, mfUrl } from "./mf";

describe("version metadata env accessor", () => {
  test("success path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}version-metadata`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      id: "ver-001",
      tag: "stable",
      timestamp: "2026-01-01T00:00:00.000Z",
    });
  });

  test("missing binding path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}version-metadata-missing`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toContain("MISSING_VERSION_METADATA");
  });
});
