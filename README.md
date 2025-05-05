- 网络协议
    - API交互：HTTP1.0 HTTP2.0
    - 客户端和服务器的通知机制
        - websocket
        - sse
- API
    - auth
        - POST api/signin 登录
        - POST api/signup 注册
    - chat
        - POST api/chat 创建一个chat
        - GET api/chat 获取聊天信息
        - PATCH api/chat/:id 更新chat的属性，更新用户，name，类型等。
        - POST api/chat/:id 发送内容，content，image等。
        - GET api/chat/:id/messages?last_id=xxx&size=100 获取chat的消息列表
- 数据结构
    - FromRow -> User
    - User(server) -> User(client)
- trait

依赖：

```shell
# tokio 依赖
cargo add tokio --features rt --features rt-multi-thread --features macros
# axum
cargo add axum --features http2 --features query --features tracing --features multipart
# anyhow
cargo add anyhow
# thiserror
cargo add thiserror
# sqlx postgres
cargo add sqlx --features postgres --features runtime-tokio-rustls
# serde
cargo add serde --features derive
cargo add serde-yaml
# tracing
cargo add tracing
cargo add tracing-subscriber --features env-filter
```