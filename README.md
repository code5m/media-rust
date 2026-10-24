# Media Studio — 全媒体内容创作与分发工作台

> 两种后端、同一套产品设计、同一组接口契约、同一套验收标准。

本仓库是 Media Studio 的独立实现。Java 与 Rust 仓库通过 `docs/CONTRACT.md`、`docs/ACCEPTANCE.md`、`prototype/index.html` 对齐，禁止通过两套互不一致的产品需求各自开发。

**重要边界**：平台被列入能力矩阵，不代表已经获得其内容发布权限。任何未经平台实际确认的发送不得标注成功；不使用盗用 Cookie、规避验证码或未授权自动化。工作台原型 ≠ 实际平台发布成功。

## 仓库导航

- `prototype/index.html`：可交互的中文产品原型与工作台界面
- `docs/PRD.md`：完整需求、模块和体验规范
- `docs/PLATFORM-RESEARCH.md`：平台官方 API 证据与权限分级
- `docs/CONTRACT.md`：两种语言一致的 REST 合约
- `docs/ACCEPTANCE.md`：功能对照与验收用例
- `docs/ROADMAP.md`：已实现功能与待交付项，防止把原型当上线能力

> 首次创建项目，后续提交追加服务端实现与测试。
