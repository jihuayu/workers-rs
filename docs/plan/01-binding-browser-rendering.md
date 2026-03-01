# Plan: Browser Rendering binding

## 背景

JS Workers 侧有 Browser Rendering 绑定；`workers-rs` 当前没有对应的一等 `Env` 包装方法与类型。

## 目标

为 Rust 用户提供稳定可用的 Browser Rendering 绑定封装，避免依赖 `Env::get_binding` 手写 `JsCast`。

## 设计

1. 在 `worker-sys` 增加 Browser Rendering 的原始 FFI 类型与方法映射。
2. 在 `worker` 增加 `BrowserRendering` 包装类型，实现 `EnvBinding` 和高频方法。
3. 在 `Env` 增加 `env.browser_rendering("BINDING")` 一等入口。
4. 保留 escape hatch：低频或实验方法允许通过 `as_ref()` 访问底层 `JsValue`。

## 需要做的工作

1. API 基线梳理：从 `@cloudflare/workers-types` 抽取 Browser Rendering 类型签名，确定 MVP 方法集合。
2. `worker-sys` 新增类型文件并导出到 `worker-sys/src/types.rs`。
3. `worker` 新增 `worker/src/browser_rendering.rs`，实现错误转换、`Send/Sync` 语义与文档注释。
4. 更新 `worker/src/env.rs` 与 `worker/src/lib.rs` 导出。
5. 在 `test/wrangler.toml` 添加对应 binding，并新增 `test/src` 与 `test/tests` 的覆盖用例。
6. README 增加示例（最小 fetch + 调用）。

## 风险与依赖

1. Miniflare 对该绑定的模拟能力可能不足，可能需要真实环境集成测试。
2. API 仍在演进时，`worker-sys` 需要避免过度封装，优先保持可扩展。

## 验收标准

1. `Env` 提供明确方法可直接获取绑定，不再要求用户手写 `get_binding`。
2. 至少一个集成测试覆盖成功路径，一个覆盖错误路径。
3. 文档示例可编译并在 CI 中通过。

