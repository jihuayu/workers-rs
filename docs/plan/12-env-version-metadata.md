# Plan: Env version metadata ergonomics

## 背景

已有 `WorkerVersionMetadata` 类型，但 `Env` 没有类似 `env.kv(...)` 的便捷方法，用户需手动 `get_binding`。

## 目标

新增一等方法，降低版本元数据读取门槛并提升 API 一致性。

## 设计

1. 在 `Env` 增加 `env.version_metadata("...")`（或 `env.version("...")`，命名待评审）。
2. 复用现有 `WorkerVersionMetadata` 类型，不引入额外复杂抽象。
3. 文档中给出与 `get_binding` 的对应关系，保持向后兼容。

## 需要做的工作

1. 更新 `worker/src/env.rs` 增加方法。
2. 检查并更新 `worker/src/lib.rs` 导出注释（若需要）。
3. 增加测试：
4. 获取成功并读取 `id/tag/timestamp`。
5. 绑定缺失时错误信息符合现有风格。
6. README 增加最小示例。

## 风险与依赖

1. 命名要避免与已有概念混淆（如 R2 object version）。
2. 低风险改动，但需保持错误文案一致性。

## 验收标准

1. 用户可直接通过一等方法获取版本元数据。
2. 回归测试通过，且不影响现有 `get_binding` 使用者。
3. 文档示例可编译。

