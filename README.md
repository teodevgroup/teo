<div align="center">
  <h1>Teo</h1>
  <a href="https://github.com/teocloud/teo/blob/master/LICENSE"><img src="https://img.shields.io/github/license/teocloud/teo.svg?style=flat-square" /></a>
  <a href="https://github.com/teocloud/teo"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square" /></a>
  <br />
  <br />
  <div><strong>Schema-centered</strong> next-generation web framework for Rust, Node.js and Python.</div>
  <br />
  <a href="https://docs.teocloud.io/getting-started/quickstart">Quickstart</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="https://teocloud.io/">Official website</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="https://docs.teocloud.io/">Docs</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="https://teocloud.io/blog">Blog</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="https://teocloud.io/discord">Discord</a>
  <br />
  <hr />
</div>

## Introduction

Teo is a **schema-centered** next-generation web framework for Rust, Node.js and Python.

## Highlights & Features

* Innovative schema definition inspired by GraphQL and Prisma
* Auto database migration
* Supports Rust, Node.js and Python
* Supports MySQL, PostgreSQL, SQLite and MongoDB
* Generated ORM types and interfaces
* Generated query clients for frontend
* Very efficient and performant
* Data sanitization, transformation and validation
* Builtin user sessions
* Builtin permission check
* First in last out middlewares
* Custom routes and handlers

## Getting started

The fastest way to get started with Teo is by following the [Quickstart guide](https://docs.teocloud.io/getting-started/quickstart).

### Installation

Install Node.js edition.

```sh
npm install @teocloud/teo
```

Install Python edition.

```sh
pip install teo
```

Install Rust edition.

```sh
cargo install teo
```

### Write a schema-only server

To write a server is quite simple with Teo. Create a file named `schema.teo`.
Specify which database to connect and which port to listen.

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

Start the server with `teo serve` command. Now you can create, update, delete,
read, aggregate and group by. Read our
[Query client guide](https://docs.teocloud.io/guides/query-client-guides/crud)
for detailed usage.

## Tutorials

We prepared a [Beginner tutorial series](https://docs.teocloud.io/getting-started/beginner-tutorial/write-a-schema-only-app)
to help you learn and understand Teo.

## Issues

Welcome to submit issues in this repo.

## License

TEO is under Apache 2.0 license.
