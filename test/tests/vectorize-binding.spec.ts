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

  test("describe success path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}vectorize-binding-describe`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      ok: true,
      dimensions: 3,
      count: 2,
    });
  });

  test("getByIds success path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}vectorize-binding-get-by-ids`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      ok: true,
      vectors: [
        { id: "v1", values: [1, 2, 3] },
        { id: "v2", values: [1, 2, 3] },
      ],
    });
  });

  test("deleteByIds success path", async () => {
    const resp = await mf.dispatchFetch(
      `${mfUrl}vectorize-binding-delete-by-ids`
    );
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      ok: true,
      deleted: ["v1"],
    });
  });

  test("missing binding path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}vectorize-binding-missing`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toContain("MISSING_VECTORIZE");
  });
});
