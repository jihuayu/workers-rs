# Plan: Vectorize binding

## 背景

JS 侧已有 Vectorize binding；`workers-rs` 缺少对应类型封装与 `Env` 入口。

## 目标

让 Rust Worker 可直接调用向量索引能力，覆盖最常见的 upsert/query/delete 管理流程。

## 设计

1. `worker-sys` 层完整映射 Vectorize 基础 API。
2. `worker` 层提供面向 Rust 的 `VectorizeIndex` 包装。
3. `Env` 层新增 `env.vectorize("...")`。
4. 类型设计优先使用 `serde` 结构，减少用户直接拼 `JsValue`。

## 需要做的工作

1. 定义 MVP：向量写入、查询、删除、命名空间或过滤能力。
2. `worker-sys` 新增类型、方法、导出。
3. `worker` 新增封装和错误类型补充。
4. `Env`/`lib` 暴露与文档说明。
5. 测试：
6. 写入后查询命中。
7. 错误输入（维度不匹配等）转为可读错误。
8. 示例工程或 README 片段。

## 风险与依赖

1. 向量维度和 payload 序列化错误较常见，需优先优化错误信息。
2. 若 API 演进快，需将高级参数设计为可扩展结构体。

## 验收标准

1. Rust 用户无需 `get_binding` 即可使用 Vectorize。
2. 核心 CRUD/查询路径有自动化验证。
3. 文档给出最小可运行例子。

