-- 用户表
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(64) NOT NULL DEFAULT '' COMMENT '用户名',
    nickname VARCHAR(64) NOT NULL DEFAULT '' COMMENT '昵称',
    email VARCHAR(128) NOT NULL DEFAULT '' COMMENT '邮箱',
    -- hashed argon2 password
    password VARCHAR(64) NOT NULL DEFAULT '' COMMENT '密码',
    create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间'
);

-- 聊天类型
CREATE TYPE IF NOT EXISTS chat_type AS ENUM ('single', 'group', 'private_channel', 'public_channel');

-- 聊天表
CREATE TABLE IF NOT EXISTS chats (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL UNIQUE DEFAULT '' COMMENT '聊天名称',
    type chat_type NOT NULL DEFAULT 0 COMMENT '聊天类型',
    -- 用户id列表
    members BIGINT[] NOT NULL DEFAULT 0 COMMENT '用户id列表',
    message TEXT NOT NULL COMMENT '聊天内容',
    create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间'
);

-- 消息表
CREATE TABLE IF NOT EXISTS messages (
    id BIGSERIAL PRIMARY KEY,
    chat_id BIGINT NOT NULL DEFAULT 0 COMMENT '关联聊天id',
    sender_id BIGINT NOT NULL DEFAULT 0 COMMENT '发送者id',
    content TEXT NOT NULL COMMENT '消息内容',
    create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间'
);

CREATE INDEX IF NOT EXISTS chat_id_create_at_sender_id_idx ON messages(chat_id,create_at DESC,sender_id);
