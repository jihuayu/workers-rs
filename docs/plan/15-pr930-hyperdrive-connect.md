# Plan: PR930 - Hyperdrive connect API

## 背景

当前 `Hyperdrive` 仅暴露连接信息 getter（如 `connection_string`），缺少直接发起连接的便捷 API。

## 目标

在 `Hyperdrive` 上提供 `connect` 能力，统一与现有 `Socket` 连接模型，减少用户手工解析连接信息的样板代码。

## 设计

1. 在 `worker-sys` 增加 Hyperdrive `connect` 对应的 FFI 方法。
2. 在 `worker` 增加 `Hyperdrive::connect(...) -> Result<Socket>`（具体签名以上游 API 为准）。
3. 连接选项尽量复用 `socket` 模块已有 `SecureTransport`/`SocketOptions` 语义。
4. 保持兼容：
5. 现有 getter 保留不变。
6. 新增 API 不破坏现有调用方。

## 需要做的工作

1. 扩展 `worker-sys/src/types/hyperdrive.rs` 方法定义。
2. 扩展 `worker/src/hyperdrive.rs` 封装实现。
3. 增加示例或测试：
4. 成功建立连接并读写最小数据流。
5. 连接失败路径（错误地址/权限不足）可读报错。
6. 更新 README 或 API 文档，给出推荐调用方式。

## 风险与依赖

1. `connect` 签名若与通用 socket 差异较大，会引入重复配置模型。
2. 网络能力测试可能依赖环境，CI 需区分 mock 与集成用例。

## 验收标准

1. `Hyperdrive` 提供可用 `connect` 一等 API。
2. 至少有一条成功链路测试与一条失败链路测试。
3. 文档示例可编译并能指导迁移。

