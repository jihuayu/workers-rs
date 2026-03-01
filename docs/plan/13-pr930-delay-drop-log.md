# Plan: PR930 - Delay drop debug log

## 背景

`Delay` 在未完成时被 drop，会打印 debug 日志。该日志在高并发或大量短延时场景会造成噪音。

## 目标

降低运行时日志噪音，保持 `Delay` 在提前 drop 时的安全行为不变。

## 设计

1. 保留 `clear_timeout_with_handle` 清理逻辑，不改变内存安全语义。
2. 移除 `drop` 路径中的 `console_debug!`。
3. 如果需要可观测性，改为可选调试开关（后续增量，不在本次 MVP）。

## 需要做的工作

1. 修改 `worker/src/delay.rs`，删除 `drop` 中的调试输出。
2. 补充/更新测试，验证提前 drop 时仍会清理 timeout。
3. 更新变更说明，标注行为不变、仅日志变化。

## 风险与依赖

1. 失去默认日志后，排查个别超时逻辑问题可见性下降。
2. 若后续要恢复可观测性，应通过 feature flag 或环境变量控制。

## 验收标准

1. `Delay` 提前 drop 不再输出 debug 日志。
2. 现有行为测试通过，未引入超时泄漏或回调触发异常。

