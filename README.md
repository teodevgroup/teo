<div align="center">
  <h1>Teo</h1>
  <a href="https://crates.io/crates/teo"><img src="https://img.shields.io/crates/v/teo?style=flat-square" /></a>
  <a href="https://www.npmjs.com/package/@teodevgroup/teo"><img src="https://img.shields.io/npm/v/%40teodevgroup%2Fteo?style=flat-square" /></a>
  <a href="https://pypi.org/project/teo/"><img src="https://img.shields.io/pypi/v/teo?style=flat-square" /></a>
  <a href="https://marketplace.visualstudio.com/items?itemName=yeannylam.teo-vscode"><img src="https://img.shields.io/visual-studio-marketplace/v/yeannylam.teo-vscode?style=flat-square&label=VSCode%20marketplace&color=%2300AFD7" /></a>
  <a href="https://github.com/teodevgroup/teo/blob/master/LICENSE"><img src="https://img.shields.io/github/license/teodevgroup/teo.svg?style=flat-square" /></a>
  <a href="https://github.com/teodevgroup/teo"><img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square" /></a>
  <br />
  <br />
  <div><strong>Schema-driven</strong> web server framework.</div>
  <br />
  <a href="https://docs.teodev.io/getting-started/quickstart">Quickstart</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="https://teodev.io/">Official website</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="https://docs.teodev.io/">Docs</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="https://blog.teodev.io">Blog</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="https://teodev.io/discord">Discord</a>
  <br />
  <hr />
</div>

## Notice

Teo 0.4 will be a total new version. Teo is relocated as an ORM. CLI, Code generation, Node.js and Python will be removed.

## Introduction

Teo is **schema-driven** web server framework. The server side API is native to Rust, Node.js and Python.

## Highlights & Features

* Native to **Rust**, **Node.js** and **Python**
* Innovative schema definition inspired by **GraphQL** and **Prisma**
* Auto database migration
* Supports **MySQL**, **PostgreSQL**, **SQLite** and **MongoDB**
* Generated **ORM** types and interfaces
* Generated **query clients** for frontend
* Very efficient and performant
* Data sanitization, transformation and validation
* Builtin user **sessions**
* Builtin **permission** check
* First in last out **middlewares**
* Custom **route handlers**
* Generated customizable **admin dashboard**
* Plays great with AI tools

## Getting started

The fastest way to get started with Teo is by following the [Quickstart guide](https://docs.teodev.io/getting-started/quickstart).

### Installation

Install Node.js edition.

```sh
npm install @teodevgroup/teo
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
[Query client guide](https://docs.teodev.io/guides/query-client-guides/crud)
for detailed usage.

### Write custom handlers

Declare the handler in the schema.

```teo
@map(.get, "/echo/:data", interface: "EchoCaptures")
declare nonapi handler echo(): Any
```

Implement the handler with program code.

#### Node.js implementation

```ts
import { App, Response, Request } from '@teodevgroup/teo'
import { EchoCaptures } from './entities'
 
const app = new App()
app.mainNamespace().defineHandler("echo", (request: Request) => {
    const captures: EchoCaptures = request.captures()
    return Response.string(captures.data, "text/plain")
})
app.run()
```

#### Python implementation

```python
from asyncio import run
from teo import App, Response, Request
from entities import EchoCaptures
 
async def main():
    app = App()
    def echo_handler(request: Request):
        captures: EchoCaptures = request.captures()
        return Response.string(captures["data"], "text/plain")
    app.main_namespace.define_handler("echo", echo_handler)
    await app.run()
 
run(main())
```

#### Rust implementation

```rust
mod entities;

use tokio::main;
use teo::prelude::{App, Response, Result, path};
use crate::entities::EchoCaptures;

#[main]
async fn main() -> Result<()> {
    let app = App::new()?;
    app.main_namespace().define_handler("echo", |captures: EchoCaptures| async move {
        Ok::<Response, Error>(Response::string(captures.data(), "text/plain"))
    });
    app.run().await
}
```

## Tutorials

We prepared a [Beginner tutorial series](https://docs.teodev.io/getting-started/beginner-tutorial/write-a-schema-only-app)
to help you learn and understand Teo.

## Issues

Welcome to submit issues in this repo.

## Contributing

Read our [Contributing guide](https://github.com/teodevgroup/teo/blob/main/CONTRIBUTING.md)
to set projects up and start contributing.

## License

TEO is under Apache 2.0 license.
