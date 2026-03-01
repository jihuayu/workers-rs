import { describe, test, expect } from "vitest";
import { mf, mfUrl } from "./mf";

describe("dynamic dispatch binding", () => {
  test("get_with_args passes args payload", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}dynamic-dispatch-args`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      name: "svc-1",
      args: {
        tenant: "acme",
      },
    });
  });

  test("get_with_options passes args and options", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}dynamic-dispatch-options`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      name: "svc-2",
      args: ["alpha", "beta"],
      options: {
        trace: true,
        region: "eu",
      },
    });
  });
});
