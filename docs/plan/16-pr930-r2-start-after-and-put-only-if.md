# Plan: PR930 - R2 `startAfter` + `put(...).onlyIf(...)`

## 背景

R2 侧存在两个对齐缺口：

1. `list` 缺少 `startAfter`（增量分页常用）。
2. `put(...).onlyIf(...)` 语义未完整对齐，返回值与条件失败行为需要统一。

## 目标

补齐 R2 分页与条件写入能力，使 `workers-rs` 与 JS 侧语义一致并减少用户手动拼接 `JsValue`。

## 设计

1. `ListOptionsBuilder` 新增 `start_after(...)`，序列化到 JS 的 `startAfter`。
2. `PutOptionsBuilder` 新增 `only_if(...)` 条件字段，调用时传给底层 `onlyIf`。
3. 返回值语义调整：
4. 条件满足时返回 `Some(Object)`。
5. 条件不满足时返回 `None`，避免用错误流表达预期分支。
6. 兼容策略：
7. 以新方法和返回类型为主。
8. 如有破坏性签名变更，需要在 changelog 标注并给迁移示例。

## 需要做的工作

1. 修改 `worker/src/r2/builder.rs`：
2. `ListOptionsBuilder` 增加 `start_after` 字段与 builder 方法。
3. `execute` 的 JS 对象增加 `startAfter` 映射。
4. `PutOptionsBuilder` 增加 `only_if` 和 `only_if(...)`。
5. `PutOptionsBuilder::execute` 调整返回类型为 `Result<Option<Object>>`（按目标语义）。
6. 检查 `worker-sys` 是否需要补充对应方法/类型映射。
7. 更新 `worker/src/r2/mod.rs` 和受影响调用方。
8. 更新测试：
9. list + start_after 分页正确。
10. put + only_if 成功/失败两条分支。
11. 更新文档示例和迁移说明。

## 风险与依赖

1. 返回类型变化会影响已有用户代码，需要清晰迁移路径。
2. 条件失败与真实错误需要严格区分，避免错误吞没。

## 验收标准

1. R2 list 支持 `start_after` 且行为正确。
2. R2 put 支持 `only_if`，并在条件失败时返回 `None`。
3. 相关测试与示例全部通过。

