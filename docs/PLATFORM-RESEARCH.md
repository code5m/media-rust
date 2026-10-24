# 官方平台能力与发布边界（研究基线）

**注意：以下属于接入设计分级，不是宣称已获权限。平台政策、接口准入和账号资质会变；上线前逐一核验。**

| 平台 | 主要内容 | 拟定接入方式 | 上线前必须确认 |
|---|---|---|---|
| 微信公众号 | 图文、图片 | 官方草稿 + 发布能力（需符合账号资格及接口权限） | access_token、素材、草稿、发布任务轮询、规则、限额 |
| 小红书 | 笔记、图组、视频 | 合规辅助交付优先；仅在获得可用官方/合作发布能力后才 DIRECT | 真实官方授权范围、账号准入和发布审核 |
| 视频号 | 短视频、直播相关 | 需官方适用开放能力验证；默认辅助 | 视频号发布权限及授权主体 |
| 抖音 | 短视频、图文 | 官方开放平台内容发布相关能力，按应用审核权限 | OAuth scope、审核、配额、返回的视频状态 |
| 快手 | 短视频 | 官方开放平台能力，需具体审核授权 | 上传及发布接口、scope |
| Bilibili | 视频、专栏/动态 | 视频投稿开放能力按资格判断，其他先辅助 | 开放平台审核、媒资分片上传、稿件审核 |
| 微博 | 短帖、图片 | 官方开放能力按可用 scope；不开放时辅助 | statuses/share 等接口当前可用性及审核 |
| 知乎 | 回答、文章、想法 | 默认辅助 | 是否提供适用于当前开发者的发布接口 |
| 今日头条 | 文章、视频 | 默认辅助，官方合作能力可升级 | 账号/服务商发布准入 |
| 百家号 | 文章、图片 | 默认辅助，官方合作能力可升级 | API 权限与资质 |
| 搜狐号、网易号、企鹅号、一点号 | 文章、图文 | 默认辅助 | 接口是否真实对外开放 |
| 掘金、CSDN、博客园、简书 | 技术文章 | 官方接口合法性逐项验证，否则辅助 | 平台条款、Markdown/HTML 兼容性 |
| WordPress 自建站 | 文章、图片 | 官方 REST API 可设计为 DIRECT | WordPress 应用密码/其它合法认证、媒体权限 |
| Ghost 自建站 | 文章 | 官方 Admin API 可设计为 DIRECT | 集成密钥安全、版本兼容性 |
| YouTube | 视频 | 官方 Data API，审核 OAuth 配额 | upload scope、上传审核与可见性 |
| TikTok | 视频、图文 | Content Posting API，权限审核 | Direct Post 审核与用户同意 |
| LinkedIn | 短帖、图文 | 官方 Posts API，需相应权限 | 产品准入和 OAuth scope |
| X | 帖文、图片 | 官方 API，按收费档位/授权范围 | API v2 准入、媒体上传配额 |
| Bluesky | 短帖、图片 | AT Protocol | 会话、记录、图片限制 |
| Mastodon | 短帖、图片 | 实例 REST API | 实例规则与发布 scope |

## 研究与技术参考（上线前需重新核验）
- 微信公众号开发者：https://developers.weixin.qq.com/doc/offiaccount/
- 抖音开放平台：https://open.douyin.com/
- 快手开放平台：https://open.kuaishou.com/
- B站开放平台：https://open.bilibili.com/
- 小红书开放平台：https://open.xiaohongshu.com/
- 微信视频号开放平台：https://channels.weixin.qq.com/
- WordPress REST API：https://developer.wordpress.org/rest-api/reference/posts/
- YouTube Data API：https://developers.google.com/youtube/v3/docs/videos/insert
- TikTok Content Posting API：https://developers.tiktok.com/products/content-posting-api/
- LinkedIn Posts API：https://learn.microsoft.com/en-us/linkedin/marketing/community-management/shares/posts-api
- X API：https://docs.x.com/x-api/posts/create-post
- Bluesky：https://docs.bsky.app/
- Mastodon：https://docs.joinmastodon.org/methods/statuses/

## 能力状态等级
UNRESEARCHED -> DOCUMENTED -> AUTHORIZED -> SANDBOX_VERIFIED -> LIVE_VERIFIED。UI 不得将 DOCUMENTED 画成 VERIFIED。
