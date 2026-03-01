# Plan: Images binding

## 背景

`workers-rs` 已支持 `RequestInit.cf.image`（请求级图片变换参数），但缺少 Images 产品 binding 的一等 API。

## 目标

补齐 Images binding 类型封装，避免用户把“图片变换参数”和“Images binding 对象”混为一类能力。

## 设计

1. 保持现有 `ResizeConfig` 不变，继续用于 `cf.image`。
2. 新增独立 `ImagesBinding`（最终命名可在实现阶段确认）包装类型。
3. 在 `Env` 增加 `env.images("...")`。
4. 文档明确区分两条路径：
5. `cf.image`: 子请求变换。
6. `images` binding: 产品级对象操作。

## 需要做的工作

1. 梳理 JS/TS API 面，列出 MVP 方法。
2. 在 `worker-sys` 增加 Images binding 的 extern 定义。
3. 在 `worker` 增加包装与错误转换。
4. 更新 `Env` 和 `lib` 导出。
5. 新增测试：
6. 正常对象操作流程。
7. 参数校验与错误处理。
8. 更新 README，增加“cf.image vs images binding”对照段落。

## 风险与依赖

1. 命名冲突风险：需要避免与现有 `Resize*` 类型造成 API 误导。
2. 若 Miniflare 不完整支持，需要补充真实环境测试策略。

## 验收标准

1. Rust 用户可直接通过 `Env` 获取 Images binding。
2. 文档中两类能力边界清晰。
3. 至少一条端到端测试验证核心方法。

