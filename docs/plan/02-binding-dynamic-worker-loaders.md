# Plan: Dynamic Worker Loaders binding

## 背景

JS SDK 支持 Dynamic Worker Loaders；`workers-rs` 缺少对应一等封装。

## 目标

新增 Rust 可直接使用的 Dynamic Worker Loaders binding API，并保证与现有 `Fetcher`/`DynamicDispatcher` 生态协同。

## 设计

1. 在 `worker-sys` 建立原始类型映射，先覆盖最常用加载与调用路径。
2. 在 `worker` 提供 `DynamicWorkerLoader`（命名可在实现阶段最终确认）包装。
3. 在 `Env` 增加 `env.dynamic_worker_loader("...")`。
4. 与 `Fetcher` 互操作：可输出可复用请求执行对象，减少重复抽象。

## 需要做的工作

1. 从 types 定义确认方法签名和返回类型，形成 MVP 范围。
2. 新增 `worker-sys/src/types/dynamic_worker_loader.rs` 并接入导出。
3. 新增 `worker/src/dynamic_worker_loader.rs` 并实现 `EnvBinding`。
4. 更新 `worker/src/env.rs`、`worker/src/lib.rs`。
5. 补充测试：
6. 成功加载并请求执行。
7. 非法 loader 名称或缺失 binding 错误。
8. 文档示例与迁移说明（如何替代 `get_binding`）。

## 风险与依赖

1. 该能力与平台 rollout 节奏绑定，测试环境可能与生产行为有差异。
2. 方法签名可能快速变化，需优先保持 `worker-sys` 与上游一致。

## 验收标准

1. 有稳定一等 API，且与 `Fetcher` 使用路径不冲突。
2. CI 中至少有端到端行为验证。
3. README 或 docs 有可运行样例。

