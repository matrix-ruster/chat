-- Add migration script here
-- 用户表
CREATE TABLE IF NOT EXISTS users
(
    id         BIGSERIAL PRIMARY KEY,
    username   VARCHAR(128) NOT NULL DEFAULT '',
    nickname   VARCHAR(128) NOT NULL DEFAULT '',
    email      VARCHAR(128) NOT NULL DEFAULT '',
    -- hashed argon2 password
    password   VARCHAR(128) NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ           DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON COLUMN users.username IS '用户名';
COMMENT ON COLUMN users.nickname IS '昵称';
COMMENT ON COLUMN users.email IS '邮箱';
COMMENT ON COLUMN users.password IS '密码';
COMMENT ON COLUMN users.created_at IS '创建时间';

CREATE UNIQUE INDEX IF NOT EXISTS email_idx ON users (email);


-- 聊天类型
CREATE TYPE chat_type AS ENUM ('single', 'group', 'private_channel', 'public_channel');

-- 聊天表
CREATE TABLE IF NOT EXISTS chats
(
    id         BIGSERIAL PRIMARY KEY,
    name       VARCHAR(128) NOT NULL UNIQUE DEFAULT '',
    type       chat_type    NOT NULL        DEFAULT 'single',
    -- 用户id列表
    members    BIGINT[]     NOT NULL        DEFAULT '{}',
    message    TEXT         NOT NULL,
    created_at TIMESTAMPTZ  NOT NULL        DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON COLUMN chats.name IS '聊天名称';
COMMENT ON COLUMN chats.type IS '聊天类型';
COMMENT ON COLUMN chats.members IS '用户id列表';
COMMENT ON COLUMN chats.message IS '聊天内容';
COMMENT ON COLUMN chats.created_at IS '创建时间';

-- 消息表
CREATE TABLE IF NOT EXISTS messages
(
    id         BIGSERIAL PRIMARY KEY,
    chat_id    BIGINT NOT NULL DEFAULT 0,
    sender_id  BIGINT NOT NULL DEFAULT 0,
    content    TEXT   NOT NULL,
    created_at TIMESTAMPTZ     DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON COLUMN messages.chat_id IS '关联聊天id';
COMMENT ON COLUMN messages.sender_id IS '发送者id';
COMMENT ON COLUMN messages.content IS '消息内容';
COMMENT ON COLUMN messages.created_at IS '创建时间';

CREATE INDEX IF NOT EXISTS chat_id_created_at_sender_id_idx ON messages (chat_id, created_at DESC, sender_id);
