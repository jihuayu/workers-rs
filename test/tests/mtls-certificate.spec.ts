import { describe, test, expect } from "vitest";
import { mf, mfUrl } from "./mf";

describe("mtls certificate binding", () => {
  test("success path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}mtls-certificate`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toBe("cert-01");
  });

  test("missing binding path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}mtls-certificate-missing`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toContain("MISSING_MTLS_CERTIFICATE");
  });

  test("request init path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}mtls-certificate-request-init`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toBe("true");
  });
});
