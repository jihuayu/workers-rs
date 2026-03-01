# Plan: mTLS certificates binding

## 背景

JS SDK 支持 mTLS certificates 绑定；`workers-rs` 尚无对应一等封装。

## 目标

提供可直接注入到子请求/连接流程中的 mTLS binding 类型，支持常见证书选择与错误处理场景。

## 设计

1. 在 `worker-sys` 暴露 mTLS 绑定类型与必要方法。
2. 在 `worker` 增加 `MtlsCertificate` 包装（命名待最终实现确认）。
3. `Env` 增加 `env.mtls_certificate("...")`。
4. 与 `RequestInit`/`fetch` 路径打通，保证使用链路完整。

## 需要做的工作

1. 基线调研：确认 JS 侧 mTLS 绑定对象的方法和数据结构。
2. 新增 `worker-sys` 类型与导出。
3. 新增 `worker` 包装模块，含文档注释和错误映射。
4. 在 `Env` 与 `lib` 层暴露。
5. 增加测试：
6. 缺失 binding 的错误。
7. 正常请求链路（可在支持环境执行）。
8. 补充使用文档和最小示例。

## 风险与依赖

1. 安全相关能力需要更严格测试，不适合仅靠 mock。
2. 平台权限与账户配置会影响测试稳定性。

## 验收标准

1. 提供一等 API，示例能清晰展示接入方式。
2. 错误路径可诊断（缺 binding、类型不匹配、调用失败）。
3. 测试策略覆盖至少一个真实链路场景。

