# Plan: email event handler

## 背景

当前 `#[event(...)]` 仅支持 `fetch/scheduled/queue/start`，缺少 JS 侧常见的 `email` 入口。

## 目标

新增 `#[event(email)]`，使 Rust Worker 可直接处理 Email Worker 事件。

## 设计

1. `worker-macros` 在 `HandlerType` 中新增 `Email` 分支。
2. `worker-sys` 增加 Email 事件原始类型。
3. `worker` 增加 Email 事件包装类型（如 `EmailMessage`）与常见字段访问器。
4. `worker-build` 的 handler 生成逻辑识别 `email` 导出并接入 `env/ctx`。

## 需要做的工作

1. 事件签名确定：`async fn main(message: EmailMessage, env: Env, ctx: Context) -> Result<()>`（最终以平台签名为准）。
2. 更新 `worker-macros/src/event.rs` 的属性解析与 glue 代码生成。
3. 更新 `worker-build/src/main.rs` 的 `generate_handlers`，将 `email` 纳入调用包装。
4. 新增 `worker-sys` 与 `worker` 对应类型。
5. 增加测试：
6. 宏展开单测（编译期）。
7. 运行时集成测试（若本地模拟不足，至少保证 build 产物中导出正确）。
8. README 增加 `#[event(email)]` 示例。

## 风险与依赖

1. 本地模拟器对 email 事件支持可能不完整，需准备替代测试方案。
2. 入口签名若与 `fetch` 差异大，`worker-build` 包装逻辑需分支处理。

## 验收标准

1. 新属性 `#[event(email)]` 可编译并生成正确导出。
2. 最少一条端到端处理链路可验证。
3. 文档示例能通过编译测试。

