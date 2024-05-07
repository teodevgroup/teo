<div align="center">
  <h1>Teo</h1>
  <a href="https://crates.io/crates/teo"><img src="https://img.shields.io/crates/v/teo?style=flat-square" /></a>
  <a href="https://www.npmjs.com/package/@teocloud/teo"><img src="https://img.shields.io/npm/v/%40teocloud%2Fteo?style=flat-square" /></a>
  <a href="https://pypi.org/project/teo/"><img src="https://img.shields.io/pypi/v/teo?style=flat-square" /></a>
  <a href="https://marketplace.visualstudio.com/items?itemName=yeannylam.teo-vscode"><img src="https://img.shields.io/visual-studio-marketplace/v/yeannylam.teo-vscode?style=flat-square&label=VSCode%20marketplace&color=%2300AFD7" /></a>
  <a href="https://github.com/teocloud/teo/blob/master/LICENSE"><img src="https://img.shields.io/github/license/teocloud/teo.svg?style=flat-square" /></a>
  <a href="https://github.com/teocloud/teo"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square" /></a>
  <br />
  <br />
  <div><strong>以结构为核心的</strong>新一代网络框架，支持Node.js、Python和Rust。</div>
  <br />
  <a href="https://docs.teocloud.io/getting-started/quickstart">快速开始</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="https://teocloud.io/">官方网站</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="https://docs.teocloud.io/">文档</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="https://teocloud.io/blog">博客</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#微信群">微信群</a>
  <br />
  <hr />
</div>

## 简介

Teo是一个**以结构为核心的**新一代网络框架，支持Node.js、Python和Rust。

## 亮点 & 功能

* 极具创新的受GraphQL和Prisma启发的结构定义
* 自动的数据库迁移
* 支持Rust、Node.js和Python
* 支持MySQL，PostgreSQL，SQLite和MongoDB
* 生成的ORM类型定义
* 生成的前端请求代码
* 非常效率和高性能
* 数据净化、转换和验证
* 内建的用户session
* 内建的权限检查
* 先进后出的中间件
* 自定义的路由处理
* 自动生成的可自定义的管理端

## 现在开始入门

最快的开始入门方法就是阅读我们的[快速开始指南](https://docs.teocloud.io/getting-started/quickstart)。

### 安装

安装Node.js版。

```sh
npm install @teocloud/teo
```

安装Python版。

```sh
pip install teo
```

安装Rust版。

```sh
cargo install teo
```

### 编写一个schema-only的服务器

用Teo写一个服务器非常简单，创建一个叫`schema.teo`的文件，指定连接的数据库和监听的端口。

```teo
connector {
  provider: .sqlite,
  url: "sqlite::memory:"
}
 
server {
  bind: ("0.0.0.0", 5050)
}
 
model User {
  @id @autoIncrement @readonly
  id: Int
  @unique @onSet($if($presents, $isEmail))
  email: String
  name: String?
  @relation(fields: .id, references: .authorId)
  posts: Post[]
}
 
model Post {
  @id @autoIncrement @readonly
  id: Int
  title: String
  content: String?
  @default(false)
  published: Bool
  @foreignKey
  authorId: Int
  @relation(fields: .authorId, references: .id)
  author: User
}
```

用`teo serve`命令启动服务器，现在，你可以增删改查，聚合和分组。阅读我们的
[前端查询指南](https://docs.teocloud.io/guides/query-client-guides/crud)
来了解具体的请求方式。

### 编写编程路由

在schema中声明handler。

```teo
@map(.get, "/echo/:data", interface: "EchoPathArguments")
declare nonapi handler echo(): Any
```

使用编程代码实现handler。

#### Node.js实现

```ts
import { App, Response, RequestCtx } from '@teocloud/teo'
import { EchoPathArguments } from './entities'
 
const app = new App()
app.mainNamespace().defineHandler("echo", (ctx: RequestCtx) => {
    const pathArguments: EchoPathArguments = ctx.pathArguments()
    return Response.string(pathArguments.data, "text/plain")
})
app.run()
```

#### Python实现

```python
from asyncio import run
from teo import App, Response, RequestCtx
from entities import EchoPathArguments
 
async def main():
    app = App()
    def echo_handler(ctx: RequestCtx):
        path_arguments: EchoPathArguments = ctx.path_arguments()
        return Response.string(path_arguments["data"], "text/plain")
    app.main_namespace().define_handler("echo", echo_handler)
    await app.run()
 
run(main())
```

#### Rust实现

```rust
mod entities;
 
use tokio::main;
use teo::prelude::{App, Response, Result, path};
use crate::entities::EchoPathArguments;
 
#[main]
async fn main() -> Result<()> {
    let app = App::new()?;
    app.main_namespace_mut().define_handler("echo", |path_args: EchoPathArguments| async move {
        Ok::<Response, Error>(Response::string(path_args.data(), "text/plain"))
    });
    app.run().await
}
```

## 教程

我们准备了[新手教程系列](https://docs.teocloud.io/getting-started/beginner-tutorial/write-a-schema-only-app)，来帮助您学习和理解Teo.

## 问题

欢迎提交问题。

## 贡献

阅读我们的[贡献指南](https://gitee.com/teocloud/teo/blob/main/CONTRIBUTING.md)来搭建项目和开始贡献。

## 微信群

在Gitee为我们的项目点赞，带着截图添加群管微信caofz007，即可加入我们的微信群。

## 许可

TEO采用Apache 2.0许可。
