# rust-wasm-demo

# 前置条件
安装 Rust 环境


# 构建
进入项目文件夹，使用 cargo 命令构建，构建的 wasm 程序在 `target/wasm32-wasi/release/wasm-plugin.wasm`

```bash
cargo build --target wasm32-wasi --release
```

# pg demo
在本地按照 pg 数据库，数据库、账号、密码都是 postgres
在数据库中创建 user 表，插入测试数据

``` sql
CREATE TABLE "public"."users" ( "name" varchar(255) COLLATE "pg_catalog"."default","authorization" varchar(255) COLLATE "pg_catalog"."default");
INSERT INTO "public"."users" ("name", "authorization") VALUES ('test', 'Basic dGVzdDoxMjM0NTY=');
```