# Plan: RPC function args/returns

## 背景

`workers-rs` README 已明确 RPC 在“函数参数和返回值”方面尚未完整支持。

## 目标

让 Rust RPC client/server 可以稳定传递常见参数与返回值，不再局限于当前实验路径。

## 设计

1. 确立序列化协议层：优先使用 `serde-wasm-bindgen` 做 JS 值桥接。
2. 将“可稳定支持类型”分层：
3. Phase 1: 基础标量、数组、对象。
4. Phase 2: 可选类型、枚举、错误类型映射。
5. 在 `worker-codegen` 生成更强类型的 client stub，减少手写 glue。

## 需要做的工作

1. 定义 RPC 参数/返回类型支持矩阵并写入文档。
2. 扩展 `worker-codegen` 的 WIT 到 JS/Rust 绑定生成能力。
3. 增加 `worker` 侧 helper（序列化、反序列化、错误上下文）。
4. 增加兼容测试：
5. 同构调用（Rust->Rust）。
6. Rust->JS 与 JS->Rust 互通。
7. 失败案例（类型不匹配）错误可读。

## 风险与依赖

1. JS 与 Rust 类型系统差异会导致边界行为复杂。
2. 过早追求“全类型覆盖”会放大复杂度，必须分阶段推进。

## 验收标准

1. README 中列出的“参数和返回值”缺口被移除或降级为明确限制。
2. 具备跨语言互通测试用例。
3. 出错时能提供结构化错误信息。

