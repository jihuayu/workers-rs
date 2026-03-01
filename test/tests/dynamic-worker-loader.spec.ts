import { describe, test, expect } from "vitest";
import { mf, mfUrl } from "./mf";

describe("dynamic worker loader binding", () => {
  test("success path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}dynamic-worker-loader`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      ok: true,
      workerName: "sample-worker",
    });
  });

  test("missing binding path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}dynamic-worker-loader-missing`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toContain("MISSING_DYNAMIC_WORKER_LOADER");
  });
});
