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
cargo add sqlx --features postgres --features runtime-tokio-rustls --features chrono
# sqlx 工具
cargo install sqlx-cli
# serde
cargo add serde --features derive
cargo add serde-yaml
# tracing
cargo add tracing
cargo add tracing-subscriber --features env-filter

# sse
cargo add axum-extra -p notify_server --features typed-header
cargo add futures -p notify_server
cargo add tokio_stream -p notify_server

## 时间处理
cargo add chrono --features serde -p chat_server

## 哈希处理,std实现了 error trait 可以使用thiserror直接 from转化
cargo add argon2 --features std -p chat_server
```

创建migration

```shell
# 生成migrate的sql
sqlx migrate add initial
# 执行migrate，记得在.env添加 DATABASE_URL
sqlx migrate run
```