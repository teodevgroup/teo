# Feature Planning

#### 0.0.20
- `on_output` pipeline is called in save function **[DONE]**

#### 0.0.21
- `when_many_results` modifier **[DONE]**
- Rename `Purpose` to `Intent` **[DONE]**
- `when_create` and `when_update` supports upsert and outputs **[DONE]**
- Input omissible and output omissible **[DONE]**
- Remove cache error when nesting create or connect same object **[DONE]**
- Remove cache error when nesting upsert same object **[DONE]**
- callbacks for nested created or updated objects **[DONE]**
- Fix validation pipeline is before relation manipulation bug **[DONE]**
- Do not trigger connected object's `on_save` pipeline if it's not actually updated **[DONE]**

#### 0.0.22
- Ellipsis modifier (especially useful on output) **[DONE]**
- `get_object` modifier to reach the object itself **[DONE]**
- Property **[DONE]**

#### 0.0.23
- SQL: Migration: insert/delete columns for most simple cases **[DONE]**
- SQL: Mutation: most simple inserts **[DONE]**
- SQL: Query: most simple queries **[DONE]**

#### 0.0.24
- SQL: Mutation: most simple updates **[DONE]**
- SQL: Query: find many and find one **[DONE]**
- SQL: Query: count **[DONE]**
- SQL: Mutation: deletion **[DONE]**

#### 0.0.25
- SQL: Sorting **[DONE]**
- SQL: Paging **[DONE]**
- SQL: Handle optional values **[DONE]**
- SQL: `Date` and `DateTime` **[DONE]**
- SQL: Including with batch query caches for relations without join table **[DONE]**
- SQL: Including with batch query caches for relations with join table **[DONE]**
- SQL: Update object saving order for SQL **[DONE]**

#### 0.0.26
- MongoDB: Remove async mutex introduced by SQL features **[DONE]**
- Update key path APIs **[DONE]**
- Fix token decoding and encoding after modifying `identifier` **[DONE]**
- Make key path APIs better **[DONE]**
- SQL: Correct result json key order **[DONE]**

#### 0.0.27
- Updated with some new style key path error messages **[DONE]**

#### 0.0.28
- Fix pipeline valid check **[DONE]**
- Cached property: save into database **[DONE]**
- Cached property: mark for save on dependency update **[DONE]**
- Cached property: get cached value **[DONE]**
- Cached property: recalculate if dirty **[DONE]**
- Cached property: create database column **[DONE]**
- Add required check for present with and present without **[DONE]**
- Write and read rule **[DONE]**
- Property setter error uses path **[DONE]**

#### 0.0.29
- Remove primary name from MongoDB connector's `save` method **[DONE]**
- For relationship with join table, link them after both objects are created **[DONE]**
- Read rule: check in to_json **[DONE]**
- Permissions: Read **[DONE]**
- Permissions: Create **[DONE]**
- Permissions: Update **[DONE]**
- New style key path error messages **[DONE]**
- Object environment **[DONE]**

#### 0.0.30
- Delete rules **[DONE]**
- Fix HTTP API result format **[DONE]**

#### 0.0.31
- Replace `serde_json` with our own `tson` **[DONE]**

#### 0.0.32
- Handle manipulation uses single or many **[DONE]**
- Bug fixes **[DONE]**
- Remove position from env, it can be inferred from `intent` **[DONE]**

#### 0.0.33
- MongoDB Aggregation: rewrite **[DONE]**
- MongoDB update: column keys **[DONE]**
- SQL: MySQL: Fix bool column is always altered bug **[DONE]**
- SQL query process: rewrite **[DONE]**
- SQL update: column keys **[DONE]**

#### 0.0.34
- Bug fixes for MySQL CRUD without relations **[DONE]**

#### 0.0.35
- Bug fixes for select and nested select **[DONE]**
- SQL: join table relationship: cannot insert into object query map **[DONE]**

#### 0.0.36
- SQL: Cursor **[DONE]**
- SQL: Negative take **[DONE]**
- SQL: Nested skip and take **[DONE]**
- SQL: Nested negative take **[DONE]**
- SQL: Negative take without order by **[DONE]**
- SQL: Distinct **[DONE]**
- SQL: Relation where for without join table **[DONE]**
- SQL: Relation where for with join table **[DONE]**

#### 0.0.37
- SQL: Aggregation **[DONE]**
- SQL: Group by without having **[DONE]**
- SQL: Group by with having **[DONE]**

#### 0.0.38
- MongoDB: Bug fixes for one-to-many relationship **[DONE]**

#### 0.0.39
- Merge json pipeline into pipeline **[DONE]**
- Fix decode credentials **[DONE]**

#### 0.0.40
- PostgreSQL: CRUD and decoding **[DONE]**
- Fix including has no result keys bug **[DONE]**
- SQLite: auto create or delete database file **[DONE]**

#### 0.0.41
- Rewrite app & graph **[DONE]**
- Rewrite client generation **[DONE]**

#### 0.0.42
- Schema parser **[DONE]**

#### 0.0.43
- Message output for starting server **[DONE]**
- Great README for VSCode teo plugin **[DONE]**
- Better syntax highlighting for VSCode **[DONE]**
- Basic Python bindings without custom callbacks **[DONE]**
- Basic Node.js bindings without custom callbacks **[DONE]**
- Basic Go bindings without custom callbacks **[DONE]**
- When starting application, show framework version **[DONE]**
- When starting application, show environment name and version **[DONE]**
- When starting application, show rust compiler version, too **[DONE]**
- When starting application, show application entrance aka CLI or APP **[DONE]**
- Node.js: remove `App.prototype.run` and rename AppBuilder into App **[DONE]**
- Python: remove `App.run` and rename AppBuilder into App **[DONE]**
- CLI --version **[DONE]**
- CLI --help **[DONE]**
- Rust CLI **[DONE]**
- Rust: move load into build **[DONE]**
- Python: move load into run **[DONE]**
- Node.js: move load into run **[DONE]**
- Go: move load into run **[DONE]**
- Python CLI **[DONE]**
- Node.js CLI **[DONE]**
- Fix Node.js cannot Ctrl+C bug **[DONE]**
- TypeScript docs and index.d.ts for Node.js **[DONE]**

#### 0.0.44
- Syntax highlighting for official website **[DONE]**
- Syntax highlighting for IntelliJ IDEA **[DONE]**
- `invalid` modifier **[DONE]**
- Remove stage from pipeline context **[DONE]**
- `if` modifier **[DONE]**
- `not` modifier **[DONE]**
- `passed` modifier **[DONE]**
- `and` and `or` modifier **[DONE]**
- `validate` supports pipeline argument **[DONE]**
- `transform` supports pipeline argument **[DONE]**
- `previous` modifier accepts both string and enum choice **[DONE]**
- Fixed range literal bug **[DONE]**
- Removed conf builder **[DONE]**
- Design client config blocks **[DONE]**
- Design entity config blocks **[DONE]**
- Rewrite client code generation with separated TS and JS **[DONE]**
- Rename `tson` to `teon` **[DONE]**
- Add callback model decorators **[DONE]**
- Fix compiler warnings **[DONE]**
- Load environment variables from .env **[DONE]**
- Generate rust entities **[DONE]**
- Fix pipeline highlighting bug for VSCode **[DONE]**
- Rename config keyword to server **[DONE]**

#### 0.0.45
- More detailed API documentation **[DONE]**
- `$print` pipeline item **[DONE]**
- Support highlighting `import` and `let` for VSCode plugin **[DONE]**
- Rename `@authIdentity` and `@authBy` **[DONE]**
- Remove permission builder and permission **[DONE]**
- Remove `FieldBuilder` and `FieldIndexBuilder` **[DONE]**
- Remove `PropertyBuilder` **[DONE]**
- Remove `RelationBuilder` **[DONE]**
- Remove `ConnectorBuilder` and `DataSourceBuilder` **[DONE]**
- Remove `ActionBuilder` **[DONE]**
- Rename `Action` into `Handler` **[DONE]**
- CLI migrate command **[DONE]**
- Migrate when starting server **[DONE]**
- Rename `intent` to action **[DONE]**
- Rename `source` to action source **[DONE]**
- Parser: bitwise `~`, `|`, `^`, `&` **[DONE]**
- Parser: `+` `-` `*` `/` `%` **[DONE]**
- Remove redundant teon number types **[DONE]**
- Add `INTERNAL_LOCATION` and `INTERNAL_AMOUNT` **[DONE]**
- Pratt parser for binary operation **[DONE]**
- Add `$identity(Pipeline)` **[DONE]**
- Fix typo: rename `$isExist` to `$exists` **[DONE]**
- Add `@canRead` and `@canMutate` to models **[DONE]**
- Add `@canMutate` to delete **[DONE]**
- Add `@canRead` and `@canMutate` to fields **[DONE]**
- Add before delete and after delete callback to delete **[DONE]**
- Rename rust binary to `cargo-teo` **[DONE]**
- MongoDB bug: @id is not unique if not mapped to `_id` **[DONE]**
- MongoDB bug: dup key should use field name instead of column name **[DONE]**
- Rust entities for optional numbers **[DONE]**
- Rust entities for optional `Date` and `DateTime` **[DONE]**
- Rust entities for new method with `teon` parameter **[DONE]**
- Rust entities for query methods **[DONE]**
- Rename `ActionError` and `ActionResult` **[DONE]**
- Improved teon value eq to support across number types **[DONE]**
- Always record previous id value and support modifying `@id` fields **[DONE]**

#### 0.0.46
- Error in pipeline **[DONE]**
- Rename pipeline modifier to pipeline item **[DONE]**
- Error in pipeline program functions **[DONE]**
- Entry level `@disable` **[DONE]**
- Nested level `@disable` **[DONE]**
- `@redirect` item to redirect action **[DONE]**
- `$set` and `$get` works for object, vec and map **[DONE]**
- Entry level `@action` **[DONE]**
- Nested level `@action` **[DONE]**
- Fix value required entry level error message bug **[DONE]**
- `$print` item label argument **[DONE]**
- `$assign` decorator **[DONE]**
- Property decorator documentation **[DONE]**
- Relation decorator documentation **[DONE]**
- Rust `validate` accepts `Option<String>` **[DONE]**
- `@disable` documentation **[DONE]**
- `@action` documentation **[DONE]**
- Support regular expression for VSCode plugin **[DONE]**
- Rust entities: `chrono` and `bson` automatically included **[DONE]**
- Rust entities: properties **[DONE]**
- Rust entities: `set`, `update` and `delete` method **[DONE]**
- New designed object relation APIs **[DONE]**
- Rust entities with relations **[DONE]**
- Entities: auto create dir **[DONE]**
- Rust server API documentation **[DONE]**
- Object API: correct nested `set` usage for request **[DONE]**
- Object API: add nested `upsert` for single relation **[DONE]**
- Object API: correct nested `set` usage for programming **[DONE]**
- Fix parsing chained calls with new line **[DONE]**
- `find` action transform when including **[DONE]**
- MongoDB bug: fix pipeline set value, int 64 and int 32 issue **[DONE]**
- Fix soft delete bugs **[DONE]**

#### 0.0.47
- Code comment parsing in schema parser **[DONE]**
- Rust entities with documentation **[DONE]**
- TypeScript client: package files **[DONE]**
- Fixed MySQL name quote bug **[DONE]**
- Client: gitCommit option: default false **[DONE]**
- TypeScript client: fetch result: DateTime should be decoded correctly **[DONE]**
- Import without extension and index **[DONE]**
- `dest` relative to file instead of cwd **[DONE]**
- Revamp server outputs **[DONE]**
- Request logging: log original handler instead of redirected and transformed **[DONE]**
- Bug: required relationship can be created without it **[DONE]**
- Primitive type constructors **[DONE]**
- Remove unused `copy` decorator **[DONE]**
- Fixed nested create key bug **[DONE]**

#### 0.0.48
- Fixed a pipeline parsing bug **[DONE]**
- Fixed a bug which causes checker value being not passed into pipeline **[DONE]**
- Added an HTTP error message when JWT token is not defined **[DONE]**
- Added `$is` item for object checking **[DONE]**
- When identity object is created and identity is null, set itself as identity **[DONE]**
- MySQL and SQLite: when object is created, use correct int type for inserted ID **[DONE]**
- Teon value: hashmap can compare equality **[DONE]**
- Find many handler: errors if any of items is denied to access **[DONE]**
- `$isA` throws correct error message if value is null **[DONE]**
- Fixed upsert and update handler has wrong user mode bug **[DONE]**

#### 0.0.49
- Fixed SQLite connector create a strange file when `:memory:` **[DONE]**
- Fixed TypeScript client `$withToken` is not called correctly bug **[DONE]**
- Fixed TypeScript client date parsing bug when encountered null **[DONE]**
- Fixed `$ellipsis` argument passing bug **[DONE]**

#### 0.0.50
- Replace buggy sqlx with quaint which powers Prisma **[DONE]**
- SQLite: unique constraint error **[DONE]**
- MySQL: auto insert `root:` to connection URL **[DONE]**
- PostgreSQL: auto insert `postgres:` to connection URL **[DONE]**
- Rewrite SQL migrations for SQLite **[DONE]**
- Rewrite SQL migrations for PostgreSQL **[DONE]**
- Migration decorator **[DONE]**
- PostgreSQL migration: list columns **[DONE]**
- SQL migration: rename fields **[DONE]**
- Model: dropped columns **[DONE]**
- Connector config: debug **[DONE]**
- SQLite: fix in memory connection **[DONE]**
- SQL migration: add column: default value **[DONE]**

#### 0.0.51
- SQL migration: when adding required column without `default: ..` or `drop: true`, throws **[DONE]**
- SQL migration: drop table if needed when adding required column **[DONE]**
- `$queryRaw` item for SQL **[DONE]**
- SQL migration: delete tables **[DONE]**
- SQL migration: rename tables **[DONE]**
- SQL migration: actions **[DONE]**
- Server: decimal type **[DONE]**
- TypeScript client: decimal type **[DONE]**
- Fix number updator bug **[DONE]**
- PostgreSQL array type: server side **[DONE]**
- PostgreSQL array type: TypeScript client **[DONE]**
- Fix optional type decoding **[DONE]**
- PostgreSQL: fix alter column with multiple clauses **[DONE]**
- Fix query bug for date, datetime and decimal **[DONE]**

#### 0.0.52
- Fix slug and cuid format **[DONE]**
- Add cuid2 modifier **[DONE]**
- SQL migration: Separate index and columns **[DONE]**
- Schema language: Enum member literal with argument **[DONE]**
- Named arguments for index decorators **[DONE]**
- Pipeline: random float modifier **[DONE]**
- Pipeline: random int modifier **[DONE]**
- Pipeline: to uppercase modifier **[DONE]**
- Pipeline: to lowercase modifier **[DONE]**
- Pipeline: to word case modifier **[DONE]**
- Pipeline: to sentence case modifier **[DONE]**
- Pipeline: to title case modifier **[DONE]**
- Export graph, model, fields, relations and properties from app for Node.js binding **[DONE]**
- Node.js object **[DONE]**
- Node.js entity generation **[DONE]**
- Node.js package **[DONE]**
- Fix PostgreSQL drop index **[DONE]**
- Generate decimal in rust entities **[DONE]**

#### 0.0.53
- Fix Node.js run loop bug **[DONE]**

#### 0.0.54
- Setup integration tests: test lib **[DONE]**
- Setup integration tests: reset database
- Setup integration tests: docker
- Fix rust command line arguments **[DONE]**
- MySQL native enum **[DONE]**
- Fix MongoDB integer encode bug **[DONE]**
- Support MongoDB arrays **[DONE]**
- Fixed MongoDB date encoding bug **[DONE]**
- Generate decimal imports in rust entities
- Cross language error handling for node.js @victorteokw
- Fix SQLite memory bug: A HTTP connection should use single pooled connection @victorteokw
- Rewrite connector to support pooled connections @victorteokw
- SQL Transaction @victorteokw
- Connector bug: unique constraint violating: error message should have key path
- Log SQL queries
- Migration dry run

#### 0.0.55
- Relation onUpdate
- Setup code style guide

#### 0.0.56
- Relation onDelete

#### 0.0.57
- Migration decorator docs
- Dropped decorator docs
- MongoDB migration
- MongoDB `$queryRaw`
- MongoDB: root skip take and distinct bug
- MongoDB: nested skip take and distinct bug
- MongoDB: if cursor key is not orderBy key, result is wrong
- MongoDB: relation where: multiple keys should be allowed
- MongoDB: relation where: 'every' results is incorrect
- MongoDB: aggregate and group by for string and dates
- All many actions should throw errors

#### 0.0.58
- Rust entities: setter documentation
- Rust entities: optional string array
- Rust entities: required string array
- Use `queryable`, `unqueryable`, `sortable`, `unsortable` to limit API
- Remove `unqueryable` and `unsortable` fields from generated clients
- Relation with read write rules
- Support code comment tags for VSCode
- Support code comment tags for IntelliJ IDEA
- Pipeline documentation
- CLI mode: When running generation, ignore custom programming callbacks
- CLI mode: When running server, panic if custom programming callback is provided
- Soft delete documentation

#### 0.1.0
- Linting, warnings and errors

#### 0.1.1
- Seed with datasets

#### 0.1.2
- The copy action

#### 0.1.5
- PostgreSQL: Enum types

#### 0.1.6
- Database type mapping for field, property and collection types' item field

#### 0.2.0
- Support MSSQL

#### 0.2.1
- Input omissible and output omissible for generated clients

#### 0.2.2
- Correct count with cursor

#### 0.3.0
- Swift package

#### 0.4.0
- Kotlin package

#### 0.5.0
- Dart client

#### 0.5.1
- `@canAccess`

#### 0.6.0
- Plugins

#### 0.7.0
- Python server

#### 0.8.0
- Java server

#### 0.9.0
- Go server

#### 1.0.0
- First stable version
