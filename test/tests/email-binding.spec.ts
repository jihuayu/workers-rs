import { describe, test, expect } from "vitest";
import { mf, mfUrl } from "./mf";

describe("email binding", () => {
  test("message field and methods", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}email-binding-message`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      from: "sender@example.com",
      to: "recipient@example.com",
      rawSize: 4,
      forward: {
        ok: true,
        rcptTo: "forward@example.com",
      },
      reply: {
        ok: true,
        to: "recipient@example.com",
      },
    });
  });

  test("send success path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}email-binding-send`);
    expect(resp.status).toBe(200);
    expect(await resp.json()).toMatchObject({
      ok: true,
      from: "sender@example.com",
      to: "recipient@example.com",
    });
  });

  test("send missing binding path", async () => {
    const resp = await mf.dispatchFetch(`${mfUrl}email-binding-send-missing`);
    expect(resp.status).toBe(200);
    expect(await resp.text()).toContain("MISSING_EMAIL_SENDER");
  });
});
