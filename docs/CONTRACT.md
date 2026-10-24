# Java/Rust 统一 API 契约 v0.1

根路径：`/api/v1`；UTF-8 JSON；错误统一 `{code,message,traceId,details}`。请求带 `X-Request-Id`；发布批次必须支持 `Idempotency-Key`。真实接口文档须在两边生成 OpenAPI。

| 方法 | 路径 | 作用 |
|---|---|---|
| GET | /health | 运行状态 |
| GET | /platforms | 平台清单、渠道能力、验证等级 |
| GET/POST | /contents | 内容列表与创建 |
| GET/PATCH/DELETE | /contents/{id} | 内容详情与更新 |
| GET/POST | /contents/{id}/revisions | 内容版本 |
| GET/POST | /templates | 模板清单与创建 |
| GET/POST | /assets | 素材检索与上传 |
| GET/POST | /accounts | 已授权账号及新增绑定 |
| POST | /contents/{id}/variants | 为目标平台生成/保存变体 |
| POST | /contents/{id}/preflight | 校验字数、媒体、链接、scope |
| POST | /publish-batches | 建立异步多渠道任务，按每个账号独立结果 |
| GET | /publish-batches/{id} | 批次及任务状态 |
| POST | /publish-jobs/{id}/retry | 可重试的失败任务 |
| GET | /calendar | 排期 |
| GET | /analytics | 已获授权的统计 |
| GET | /audit-events | 发布与权限审计 |

## 创建发布批次示例
```json
{
  "contentId": "c_001",
  "targets": [{"platform": "wechat", "accountId":"a_wechat"}, {"platform": "xiaohongshu", "accountId":"a_xhs"}],
  "scheduleAt": null,
  "dryRun": true
}
```
返回 `202 Accepted`，每个任务均返回 platform/accountId/mode/status/reason/receiptUrl；DRY_RUN 永远不得返回 PUBLISHED。

任务状态：QUEUED -> VALIDATING -> READY -> SUBMITTING -> SUBMITTED -> PUBLISHED，或 REQUIRES_HANDOFF / BLOCKED_AUTH / BLOCKED_CAPABILITY / FAILED / CANCELED。SUBMITTED 不等于 PUBLISHED。

两端必须共享 JSON fixture + 黑盒测试：错误码、排序/分页、预检结果、发布状态迁移、幂等、响应结构和审计一致。
