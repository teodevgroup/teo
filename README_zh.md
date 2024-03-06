<div align="center">
  <h1>Teo</h1>
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

## 教程

我们准备了[新手教程系列](https://docs.teocloud.io/getting-started/beginner-tutorial/write-a-schema-only-app)，来帮助您学习和理解Teo.

## 问题

欢迎提交问题。

## 微信群

在Gitee为我们的项目点赞，带着截图添加群管微信caofz007，即可加入我们的微信群。

## 许可

TEO采用Apache 2.0许可。
