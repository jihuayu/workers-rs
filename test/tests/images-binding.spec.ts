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

  test("info path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}images-binding-info`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      width: 640,
      height: 480,
      format: "png",
      input: { source: "raw-image" },
    });
  });

  test("pipeline path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}images-binding-pipeline`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      contentType: "image/png",
      image: {
        source: { source: "raw-image" },
        steps: [
          {
            type: "transform",
          },
          {
            type: "draw",
            image: { overlay: "logo" },
          },
        ],
      },
    });
  });

  test("missing binding path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}images-binding-missing`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toContain("MISSING_IMAGES");
  });
});
