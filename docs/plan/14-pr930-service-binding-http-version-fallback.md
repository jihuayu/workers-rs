# Plan: PR930 - service binding HTTP version fallback

## 背景

`http` 特性下，请求版本转换依赖 `cf.httpProtocol` 字符串。当前未知值走 `unreachable!`，会导致运行时 panic。

## 目标

把未知协议版本降级为安全默认值（HTTP/1.1），避免 panic，同时保留已知版本精确映射。

## 设计

1. 将 `version_from_string` 的未知分支从 `unreachable!` 改为 `HTTP_11`。
2. 可选记录一次低级别日志（debug/warn）用于诊断未知协议输入。
3. 行为策略：
4. 已知值 `HTTP/0.9, 1.0, 1.1, 2, 3` 保持原映射。
5. 其他字符串一律 fallback 到 `HTTP_11`。

## 需要做的工作

1. 修改 `worker/src/http/request.rs` 的版本映射函数。
2. 新增单元测试：
3. 已知协议映射正确。
4. 未知协议不会 panic，且返回 `HTTP_11`。
5. 如果可复现实例，补一个 service binding 端到端回归测试。

## 风险与依赖

1. fallback 会掩盖上游异常输入，需要配合最小可观测日志。
2. 若上游引入新协议常量，后续应补充显式映射而非长期依赖 fallback。

## 验收标准

1. 未知协议输入不触发 panic。
2. HTTP 版本转换逻辑具备测试覆盖并在 CI 通过。

