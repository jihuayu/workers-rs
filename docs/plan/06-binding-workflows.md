# Plan: Workflows binding

## 背景

JS SDK 支持 Workflows 绑定；`workers-rs` 目前无一等封装。

## 目标

支持 Rust Worker 直接触发和查询 Workflow 执行，形成可维护的异步编排接入路径。

## 设计

1. `worker-sys` 映射 Workflows 绑定对象及常用方法。
2. `worker` 增加 `WorkflowBinding` 包装和 typed request/response 结构。
3. `Env` 增加 `env.workflow("...")` 或 `env.workflows("...")`（命名在实现评审时确定）。
4. 默认返回强类型结构，保留原始 `JsValue` 扩展入口。

## 需要做的工作

1. 确认 MVP 方法：触发执行、查询状态、取消/重试（按上游能力分期）。
2. 新增 `worker-sys` 类型文件并导出。
3. 新增 `worker` 包装模块与 serde 模型。
4. 更新 `env.rs` 与 `lib.rs`。
5. 新增测试：
6. 成功触发并读取状态。
7. 不存在的 workflow 名称错误。
8. 更新文档（分期能力与已支持方法清单）。

## 风险与依赖

1. Workflows 可能存在异步最终一致性，测试需要轮询和超时策略。
2. 过度抽象会影响后续 API 演进，MVP 应先做薄封装。

## 验收标准

1. 具备可用的一等 API 和基础示例。
2. 测试可稳定验证主流程。
3. 文档明确已支持/未支持方法，避免误用。

