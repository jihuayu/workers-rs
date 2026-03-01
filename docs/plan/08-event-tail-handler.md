# Plan: tail event handler

## 背景

`workers-rs` 目前没有 `tail` 事件入口，而 JS 侧支持 Tail Worker 处理链路日志/事件。

## 目标

新增 `#[event(tail)]`，支持 Rust Worker 作为 Tail Consumer。

## 设计

1. `worker-macros` 新增 `Tail` handler 类型。
2. `worker-sys` 增加 tail 事件 FFI 类型。
3. `worker` 增加对应包装结构（如 `TailEvent`、日志记录结构）。
4. `worker-build` 把 `tail` 纳入 handler 生成分支，并传入 `env/ctx`。

## 需要做的工作

1. 明确 handler 签名与返回类型（通常为 `Result<()>`，以平台规范为准）。
2. 改造 `worker-macros/src/event.rs`：
3. 属性解析支持 `tail`。
4. glue 代码生成支持事件参数转换。
5. 改造 `worker-build/src/main.rs`：
6. `generate_handlers` 识别 `tail` 并添加包装。
7. 增加 `worker-sys` 与 `worker` 类型实现。
8. 新增测试：宏编译测试 + 运行时行为测试（至少验证导出与参数解析）。
9. 文档示例补充。

## 风险与依赖

1. tail 数据结构可能层级复杂，serde 模型需分阶段实现。
2. 模拟环境支持可能不足，需准备 CI 的替代验证策略。

## 验收标准

1. `#[event(tail)]` 能稳定编译并导出。
2. 事件字段访问具备基本可用性。
3. 文档示例与测试覆盖主流程。

