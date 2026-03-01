import { Miniflare, Response } from "miniflare";
import { MockAgent } from "undici";

const mockAgent = new MockAgent();

mockAgent
  .get("https://cloudflare.com")
  .intercept({ path: "/" })
  .reply(200, "cloudflare!");

mockAgent
  .get("https://miniflare.mocks")
  .intercept({ path: "/delay" })
  .reply(200, "cloudflare!")
  .delay(10000);

mockAgent
  .get("https://jsonplaceholder.typicode.com")
  .intercept({ path: "/todos/1" })
  .reply(
    200,
    {
      userId: 1,
      id: 1,
      title: "delectus aut autem",
      completed: false,
    },
    {
      headers: {
        "content-type": "application/json",
      },
    }
  );

const mf_instance = new Miniflare({
  d1Persist: false,
  kvPersist: false,
  r2Persist: false,
  cachePersist: false,
  workers: [
    {
      scriptPath: "./build/index.js",
      compatibilityDate: "2025-07-24",
      cache: true,
      d1Databases: ["DB"],
      modules: true,
      modulesRules: [
        { type: "CompiledWasm", include: ["**/*.wasm"], fallthrough: true },
      ],
      bindings: {
        EXAMPLE_SECRET: "example",
        SOME_SECRET: "secret!",
        SOME_VARIABLE: "some value",
        SOME_OBJECT_VARIABLE: {
          foo: 42,
          bar: "string"
        },
      },
      durableObjects: {
        COUNTER: "Counter",
        PUT_RAW_TEST_OBJECT: "PutRawTestObject",
        AUTO: "AutoResponseObject",
        MY_CLASS: "MyClass",
        ECHO_CONTAINER: {
          className: "EchoContainer",
          useSQLite: true,
          container: {
            imageName: "worker-dev/echocontainer:latest",
          }
        },
        SQL_COUNTER: {
          className: "SqlCounter",
          useSQLite: true,
        },
        SQL_ITERATOR: {
          className: "SqlIterator",
          useSQLite: true,
        },
      },
      kvNamespaces: ["SOME_NAMESPACE", "FILE_SIZES", "TEST"],
      serviceBindings: {
        async remote() {
          return new Response("hello world");
        },
      },
      r2Buckets: ["EMPTY_BUCKET", "PUT_BUCKET", "SEEDED_BUCKET", "DELETE_BUCKET"],
      queueConsumers: {
        my_queue: {
          maxBatchTimeout: 1,
        },
      },
      queueProducers: ["my_queue", "my_queue"],
      fetchMock: mockAgent,
      assets: {
        directory: "./public",
        binding: "ASSETS",
        routerConfig: {
          has_user_worker: true
        }
      },
      secretsStoreSecrets: {
        SECRETS: {
          store_id: "SECRET_STORE",
          secret_name: "secret-name"
        },
        MISSING_SECRET: {
          store_id: "SECRET_STORE_MISSING",
          secret_name: "missing-secret"
        }
      },
      wrappedBindings: {
        HTTP_ANALYTICS: {
          scriptName: "mini-analytics-engine" // mock out analytics engine binding to the "mini-analytics-engine" worker
        },
        BROWSER_RENDERING: {
          scriptName: "mini-browser-rendering"
        },
        DYNAMIC_WORKER_LOADER: {
          scriptName: "mini-dynamic-worker-loader"
        },
        IMAGES: {
          scriptName: "mini-images-binding"
        },
        MTLS_CERTIFICATE: {
          scriptName: "mini-mtls-certificate"
        },
        VECTORIZE: {
          scriptName: "mini-vectorize"
        },
        WORKFLOWS: {
          scriptName: "mini-workflows"
        },
        VERSION_METADATA: {
          scriptName: "mini-version-metadata"
        },
        EMAIL_MESSAGE: {
          scriptName: "mini-email-message"
        },
        EMAIL_SENDER: {
          scriptName: "mini-email-sender"
        },
        TAIL_EVENT: {
          scriptName: "mini-tail-event"
        },
        TAIL_EVENT_UNKNOWN: {
          scriptName: "mini-tail-event-unknown"
        },
        HYPERDRIVE: {
          scriptName: "mini-hyperdrive"
        }
      },
      ratelimits: {
        TEST_RATE_LIMITER: {
          simple: {
            limit: 10,
            period: 60,
          }
        }
      }
    },
    {
      name: "mini-analytics-engine",
      modules: true,
      script: `export default function (env) {
        return {
          writeDataPoint(data) {
            console.log(data)
          }
        }
      }`
    },
    {
      name: "mini-browser-rendering",
      modules: true,
      script: `export default function () {
        return {
          async render(input) {
            return { ok: true, input };
          }
        }
      }`
    },
    {
      name: "mini-dynamic-worker-loader",
      modules: true,
      script: `export default function () {
        return {
          async load(workerName) {
            return {
              ok: true,
              workerName,
            };
          }
        }
      }`
    },
    {
      name: "mini-images-binding",
      modules: true,
      script: `export default function () {
        return {
          async transform(input) {
            return {
              ok: true,
              mode: "binding",
              input,
            };
          }
        }
      }`
    },
    {
      name: "mini-mtls-certificate",
      modules: true,
      script: `export default function () {
        return {
          id: "cert-01"
        }
      }`
    },
    {
      name: "mini-vectorize",
      modules: true,
      script: `export default function () {
        return {
          async upsert() {
            return { ok: true };
          },
          async describe() {
            return {
              ok: true,
              dimensions: 3,
              count: 2,
            };
          },
          async query(input) {
            return {
              ok: true,
              matches: [{ id: "v1", score: 0.99 }],
              input,
            };
          },
          async getByIds(input) {
            return {
              ok: true,
              vectors: input.map((id) => ({ id, values: [1, 2, 3] })),
            };
          },
          async delete() {
            return { ok: true };
          },
          async deleteByIds(input) {
            return {
              ok: true,
              deleted: input,
            };
          }
        }
      }`
    },
    {
      name: "mini-workflows",
      modules: true,
      script: `export default function () {
        return {
          async trigger(input) {
            return {
              ok: true,
              id: "wf-001",
              input,
            };
          },
          async getStatus(workflowId) {
            return {
              id: workflowId,
              status: "running",
            };
          }
        }
      }`
    },
    {
      name: "mini-version-metadata",
      modules: true,
      script: `export default function () {
        return {
          id: "ver-001",
          tag: "stable",
          timestamp: "2026-01-01T00:00:00.000Z"
        }
      }`
    },
    {
      name: "mini-email-message",
      modules: true,
      script: `export default function () {
        return {
          from: "sender@example.com",
          to: "recipient@example.com",
          raw: new Uint8Array([1, 2, 3, 4]).buffer,
          headers: { subject: "test" },
          rawSize: 4,
          setReject(reason) {
            this.lastRejectReason = reason;
          },
          async forward(rcptTo, headers) {
            return {
              ok: true,
              rcptTo,
              headers: headers ?? null,
            };
          },
          async reply(message) {
            return {
              ok: true,
              to: message.to,
            };
          }
        }
      }`
    },
    {
      name: "mini-email-sender",
      modules: true,
      script: `export default function () {
        return {
          async send(message) {
            return {
              ok: true,
              from: message.from,
              to: message.to,
            };
          }
        }
      }`
    },
    {
      name: "mini-tail-event",
      modules: true,
      script: `export default function () {
        return {
          invocationId: "inv-123",
          spanContext: { traceId: "trace-abc" },
          timestamp: 1700000000000,
          sequence: 42,
          event: {
            type: "log",
            message: "hello",
          }
        }
      }`
    },
    {
      name: "mini-tail-event-unknown",
      modules: true,
      script: `export default function () {
        return {
          invocationId: "inv-unknown",
          spanContext: { traceId: "trace-unknown" },
          timestamp: 1700000001234,
          sequence: 7,
          event: {
            type: "futureVariant",
            payload: { ok: true },
          }
        }
      }`
    },
    {
      name: "mini-hyperdrive",
      modules: true,
      script: `export default function () {
        return {
          connectionString: "postgres://user:pass@db.example.com:5432/app",
          host: "db.example.com",
          port: 5432,
          user: "user",
          password: "pass",
          database: "app",
          connect() {
            return {
              close: async () => {},
              closed: Promise.resolve(),
              opened: Promise.resolve({}),
              startTls() { return this; },
              readable: new ReadableStream(),
              writable: new WritableStream(),
            };
          }
        }
      }`
    }]
});

// Seed the secret store with a value using the new API
const secretAPI = await mf_instance.getSecretsStoreSecretAPI("SECRETS");
await secretAPI().create("secret value");

export const mf = mf_instance;
export const mfUrl = await mf.ready;
