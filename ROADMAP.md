# Roadmap

### 0.0.20
- `on_output` pipeline is called in save function **[DONE]**

### 0.0.21
- `when_many_results` modifier **[DONE]**
- Rename `Purpose` to `Intent` **[DONE]**
- `when_create` and `when_update` supports upsert and outputs **[DONE]**
- Input omissible and output omissible **[DONE]**
- Remove cache error when nesting create or connect same object **[DONE]**
- Remove cache error when nesting upsert same object **[DONE]**
- callbacks for nested created or updated objects **[DONE]**
- Fix validation pipeline is before relation manipulation bug **[DONE]**
- Do not trigger connected object's `on_save` pipeline if it's not actually updated **[DONE]**

### 0.0.22
- Ellipsis modifier (especially useful on output) **[DONE]**
- `get_object` modifier to reach the object itself **[DONE]**
- Property **[DONE]**

### 0.0.23
- SQL: Migration: insert/delete columns for most simple cases **[DONE]**
- SQL: Mutation: most simple inserts **[DONE]**
- SQL: Query: most simple queries **[DONE]**

### 0.0.24
- SQL: Mutation: most simple updates **[DONE]**
- SQL: Query: find many and find one **[DONE]**
- SQL: Query: count **[DONE]**
- SQL: Mutation: deletion **[DONE]**

### 0.0.25
- SQL: Sorting **[DONE]**
- SQL: Paging **[DONE]**
- SQL: Handle optional values **[DONE]**
- SQL: `Date` and `DateTime` **[DONE]**
- SQL: Including with batch query caches for relations without join table **[DONE]**
- SQL: Including with batch query caches for relations with join table **[DONE]**
- SQL: Update object saving order for SQL **[DONE]**

### 0.0.26
- MongoDB: Remove async mutex introduced by SQL features **[DONE]**
- Update key path APIs **[DONE]**
- Fix token decoding and encoding after modifying `identifier` **[DONE]**
- Make key path APIs better **[DONE]**
- SQL: Correct result json key order **[DONE]**

### 0.0.27
- Updated with some new style key path error messages **[DONE]**

### 0.0.28
- Fix pipeline valid check **[DONE]**
- Cached property: save into database **[DONE]**
- Cached property: mark for save on dependency update **[DONE]**
- Cached property: get cached value **[DONE]**
- Cached property: recalculate if dirty **[DONE]**
- Cached property: create database column **[DONE]**
- Add required check for present with and present without **[DONE]**
- Write and read rule **[DONE]**
- Property setter error uses path **[DONE]**

### 0.0.29
- Remove primary name from MongoDB connector's `save` method **[DONE]**
- For relationship with join table, link them after both objects are created **[DONE]**
- Read rule: check in to_json **[DONE]**
- Permissions: Read **[DONE]**
- Permissions: Create **[DONE]**
- Permissions: Update **[DONE]**
- New style key path error messages **[DONE]**
- Object environment **[DONE]**

### 0.0.30
- Delete rules **[DONE]**
- Fix HTTP API result format **[DONE]**

### 0.0.31
- Replace `serde_json` with our own `tson` **[DONE]**

### 0.0.32
- Handle manipulation uses single or many **[DONE]**
- Bug fixes **[DONE]**
- Remove position from env, it can be inferred from `intent` **[DONE]**

### 0.0.33
- MongoDB Aggregation: rewrite **[DONE]**
- MongoDB update: column keys **[DONE]**
- SQL: MySQL: Fix bool column is always altered bug **[DONE]**
- SQL query process: rewrite **[DONE]**
- SQL update: column keys **[DONE]**

### 0.0.34
- Bug fixes for MySQL CRUD without relations **[DONE]**

### 0.0.35
- Bug fixes for select and nested select **[DONE]**
- SQL: join table relationship: cannot insert into object query map **[DONE]**

### 0.0.36
- SQL: Cursor **[DONE]**
- SQL: Negative take **[DONE]**
- SQL: Nested skip and take **[DONE]**
- SQL: Nested negative take **[DONE]**
- SQL: Negative take without order by **[DONE]**
- SQL: Distinct **[DONE]**
- SQL: Relation where for without join table **[DONE]**
- SQL: Relation where for with join table **[DONE]**

### 0.0.37
- SQL: Aggregation **[DONE]**
- SQL: Group by without having **[DONE]**
- SQL: Group by with having **[DONE]**

### 0.0.38
- MongoDB: Bug fixes for one-to-many relationship **[DONE]**

### 0.0.39
- Merge json pipeline into pipeline **[DONE]**
- Fix decode credentials **[DONE]**

### 0.0.40
- PostgreSQL: CRUD and decoding **[DONE]**
- Fix including has no result keys bug **[DONE]**
- SQLite: auto create or delete database file **[DONE]**

### 0.0.41
- Rewrite app & graph **[DONE]**
- Rewrite client generation **[DONE]**

### 0.0.42
- Schema parser **[DONE]**

### 0.0.43
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

### 0.0.44
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

### 0.0.45
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

### 0.0.46
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

### 0.0.47
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

### 0.0.48
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

### 0.0.49
- Fixed SQLite connector create a strange file when `:memory:` **[DONE]**
- Fixed TypeScript client `$withToken` is not called correctly bug **[DONE]**
- Fixed TypeScript client date parsing bug when encountered null **[DONE]**
- Fixed `$ellipsis` argument passing bug **[DONE]**

### 0.0.50
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

### 0.0.51
- SQL migration: when adding required column without `default: ..` or `drop: true`, throws **[DONE]**
- SQL migration: drop table if needed when adding required column **[DONE]**
- `$queryRaw` item for SQL **[DONE]**
- SQL migration: delete tables **[DONE]**
- SQL migration: rename tables **[DONE]**
- SQL migration: actions **[DONE]**
- Server: decimal type **[DONE]**
- TypeScript client: decimal type **[DONE]**
- Fix number updater bug **[DONE]**
- PostgreSQL array type: server side **[DONE]**
- PostgreSQL array type: TypeScript client **[DONE]**
- Fix optional type decoding **[DONE]**
- PostgreSQL: fix alter column with multiple clauses **[DONE]**
- Fix query bug for date, datetime and decimal **[DONE]**

### 0.0.52
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

### 0.0.53
- Fix Node.js run loop bug **[DONE]**

### 0.0.54
- Setup integration tests: test lib **[DONE]**
- Setup integration tests: docker **[DONE]**
- Fix rust command line arguments **[DONE]**
- MySQL native enum **[DONE]**
- Fix MongoDB integer encode bug **[DONE]**
- Support MongoDB arrays **[DONE]**
- Fixed MongoDB date encoding bug **[DONE]**
- Removed enum builder **[DONE]**
- Fixed SQLite Int64 is not decoded bug **[DONE]**
- Fixed SQLite migration primary key bug **[DONE]**
- Fixed PostgreSQL migration primary key bug **[DONE]**

### 0.0.55
- Remove URL segment name **[DONE]**
- Refactor the generator code to make it production ready **[DONE]**
- Swift client **[DONE]**
- Kotlin client **[DONE]**
- Dart client **[DONE]**
- C# client: refactor **[DONE]**
- Generate decimal imports in rust entities **[DONE]**
- Schema lang: dataset **[DONE]**
- Parser: parse dataset **[DONE]**
- Fix dictionary literal parsing bug **[DONE]**
- Fix SQL non-serial id object creating bug **[DONE]**
- Fix SQL `OR` in where bug **[DONE]**
- Fix SQL empty `in` or `notIn` bug **[DONE]**
- Fix import statement **[DONE]**
- Add command `seed` **[DONE]**
- `seed --reseed` **[DONE]**
- `seed --unseed` **[DONE]**
- Add command `purge` **[DONE]**
- Use `TEO_ENV` environment variable **[DONE]**
- Data seeding **[DONE]**
- Add debug conf block **[DONE]**
- Add test conf block **[DONE]**
- CLI: Make `--schema` argument global **[DONE]**
- Setup integration tests: reset after each find request **[DONE]**
- Unit test: relations for MongoDB connector **[DONE]**
- Fix many-to-many relationship set is not reset bug **[DONE]**
- Fix many-to-many relationship cannot be nested updated bug **[DONE]**
- Fix many-to-many relationship cannot be nested updated many bug **[DONE]**
- Fix many-to-many relationship cannot be nested deleted bug **[DONE]**
- Fix many-to-many relationship cannot be nested deleted many bug **[DONE]**
- Single relation: nested set to null **[DONE]**
- Many to one required: disable set and disconnect nested action **[DONE]**
- One required to many: disable set to null and disconnect **[DONE]**
- Fix set for single relation is not set bug **[DONE]**
- One foreign to one local optional: when nested create, disconnect the previous one **[DONE]**
- Unit test: relations for MySQL connector **[IN PROGRESS]**
- Fix composite primary bug for MySQL **[DONE]**
- Fix SQL where with join error **[DONE]**
- SQL: required many-to-many relation should display an empty relation array if included **[DONE]**
- Unit test: relations for PostgreSQL connector **[DONE]**
- Fix composite primary bug for PostgreSQL **[DONE]**
- Fix string encode escape bug for PostgreSQL **[DONE]**
- Fix where name escape bug for PostgreSQL **[DONE]**
- Fix: SQL delete clause escapes identifiers **[DONE]**
- Unit test: relations for SQLite connector **[DONE]**
- Fix: SQL encode escape bug for SQLite **[DONE]**

### 0.0.56
- Find unique and find first returns null instead of `ObjectNotFound` error **[DONE]**
- Fix: on save now displays correct validate error message **[DONE]**
- Fix: compare should not be called when field is not `recordPrevious` **[DONE]**
- Refactor **[DONE]**
- Rename `source` to `initiator` **[DONE]**
- Rename `environment version` to `program` **[DONE]**
- Remove `GraphBuilder` **[DONE]**
- Remove `AppBuilder` **[DONE]**
- Remove `ModelBuilder` **[DONE]**
- Rewrite model API **[DONE]**
- Remove `ModelIndexBuilder` **[DONE]**
- Rewrite framework errors **[DONE]**
- Rewrite connector to support pooled connections **[DONE]**
- Fix SQLite memory bug: A HTTP connection should use single pooled connection **[DONE]**
- Unit test: relations for SQLite memory connector **[DONE]**
- SQL transaction **[DONE]**
- Callbacks with variant length parameters **[DONE]**
- Pass teo object to callbacks **[DONE]**
- Refactor Rust entities **[DONE]**
- Node.js bindings: refactor **[DONE]**
- Refactor Node.js entities **[DONE]**
- Fixed Node.js transform number bug **[Done]**
- Fixed rust entity generation callback method bug **[Done]**
- Fixed update Cargo.toml bug **[Done]**
- Fixed `$oneOf` bug **[Done]**
- Fixed rust enum traits bug **[Done]**
- Fixed `dotenv` is not triggered bug **[Done]**
- Schema lang: Each line can contain comments now **[Done]**
- Entity: rust entity now contains aggregate methods **[Done]**
- When running generation, ignore custom programming callbacks **[Done]**
- Fixed aggregate SQL clause bug for PostgreSQL **[Done]**
- Fixed create index on a newly defined column causes crash bug **[Done]**
- MySQL basic custom string types **[Done]**
- Fixed decimal.js import bug for generated Node.js client **[Done]**
- Fixed a bug causes virtual fields to be created **[Done]**
- When refreshed, virtual value is kept for root level object **[Done]**
- Fixed count `where` bug **[Done]**
- Add `timezone` parameter to `$today` **[Done]**
- Cached properties can be indexed **[Done]**
- Better object inspection **[Done]**

### 0.0.57
- Custom routes: action **[Done]**
- Custom routes: middleware **[Done]**
- Fixed action `program code` is recognized as any action bug **[Done]**
- Fixed where unique input is ordered bug **[Done]**
- Fixed `teon` macros are not found bug **[Done]**
- File uploading with form data **[Done]**
- Serving static files **[Done]**
- Fixed optional decimal bug in rust entities **[Done]**
- Added a user context parameter to setup callback **[Done]**
- Fixed a bug which caused generated clients contain internal classes **[Done]**
- Fix app ctx and graph memory bug for release **[Done]**
- Fix command line tool entity generation **[Done]**
- Fixed sign in meta token object **[Done]**
- Uploaded file is now placed at temporary directory **[Done]**
- Fixed where unique decode bug **[DONE]**
- Fixed previous record trigger when setting object value **[DONE]**
- When updating an object, use its previous value **[DONE]**
- Do not seed records for dropped tables and delete the seed record **[DONE]**
- Fixed "equals": null and "not": null for SQL queries **[DONE]**
- Generated models in clients and entities are in alphabetic order **[DONE]**
- Kotlin client: when generating into a project, infer its package name from the path **[DONE]**
- Node.js client: Rewrite with askama engine **[DONE]**
- Generated clients: vec relations are optional instead of required **[DONE]**
- Fixed enum triple comment block parsing bug **[DONE]**
- Remove block decorators **[DONE]**
- Added `toDate` item **[DONE]**

### 0.0.58
- Fixed duplicated import bug **[DONE]**
- Namespaces **[DONE]**
- Remove `action` from request URLs **[DONE]**
- Remove function installers **[DONE]**
- Table names are lower-cased but not plural anymore **[DONE]**
- Fixed empty dictionary literal parsing bug **[DONE]**
- Display errors and warnings **[DONE]**
- Allow comments in dictionary literal, array literal and tuple literal **[DONE]**
- Fix insert raw enum variant into SQL bug **[DONE]**
- Added `lint` command **[DONE]**
- Added `run` command and `program` definition **[DONE]**
- Type system in schema **[DONE]**
- Remove soft delete **[DONE]**
- Code diagnostics for VSCode **[DONE]**
- Auto completion **[DONE]**
- Jump to definition **[DONE]**
- Allow no connector in a project **[DONE]**
- Multiple connectors **[DONE]**
- Rewrite decorators loading and pipeline items loading **[DONE]**
- Custom decorators and pipeline items **[DONE]**
- Full set of binary operators **[DONE]**
- Enum member with arguments **[DONE]**
- Force unwrap operator **[DONE]**
- Specific database types **[DONE]**
- Server response: redirects **[DONE]**
- The copy action **[DONE]**
- Custom decorators for actions including custom routes: @ignoreNamespace: true**[DONE]**
- Server: custom request methods and url params**[DONE]**
- Availability flags **[DONE]**
- Database type mapping for field, property **[DONE]**
- All `many` actions should throw errors **[DONE]**
- Rewrite model validator and decoder **[DONE]**
- Rewrite interface validator and decoder **[DONE]**
- Input omissible and output omissible for generated clients **[DONE]**
- Client: distinct in handler args **[DONE]**

### 0.0.59
- Format source files **[DONE]**
- Dictionary uses javaScript object syntax **[DONE]**
- Alter config block with dictionary literal syntax **[DONE]**
- Declare enum in type **[DONE]**
- Declare object in type **[DONE]**
- Interface extending shapes **[DONE]**
- Type coercing **[DONE]**
- Bug fixes **[DONE]**
- Rewrite rust generators with namespaces and interfaces **[DONE]**
- Add back request logs **[DONE]**
- Add back extractors for rust entities **[DONE]**
- Add back model index decorators **[DONE]**

### 0.0.60
- Custom handler takes synthesized types **[DONE]**
- Add `to_teon` for generated rust objects **[DONE]**
- Fix rust entity `type` method escaping bug **[DONE]**
- Add borrowing from teon value for generated rust interfaces **[DONE]**
- Fix rust entity `&Value` to &interface conversion bug **[DONE]**
- Do not parse JSON body for get request and delete request **[DONE]**
- Fix use middlewares parsing bugs **[DONE]**
- Fix unsigned type bug in MySQL **[DONE]**
- Fix table name encoding bug in PostgreSQL **[DONE]**
- Fix create bug in PostgreSQL **[DONE]**
- Fix primary key is dropped bug in PostgreSQL **[DONE]**
- Fix MongoDB record decoding bugs **[DONE]**
- Fix `$hasLength` pipeline item bug **[DONE]**
- Fix argument resolving bug when type is optional **[DONE]**
- Fix server error message class error **[DONE]**
- Update MongoDB unique record error message **[DONE]**
- Update SQL databases unique record error message **[DONE]**
- Fix model `@migration` renamed bug **[DONE]**
- Server response: file **[DONE]**
- Server response: string in HTML format and other formats **[DONE]**
- Rewrite TS client generators with namespaces and interfaces **[DONE]**
- TS Client: fix running bugs **[DONE]**
- TS Client: fixed include args with boolean inputs **[DONE]**
- Fix reference shape with without bug **[DONE]**
- Remove `using` keyword from syntax **[DONE]**
- Update documentation **[DONE]**
- Update design of website **[DONE]**
- Rewrite syntax highlighting **[DONE]**

### 0.0.61
- Fix MySQL primary key migration bug **[DONE]**
- Simplified transaction API **[DONE]**
- Fix without shape bug for create and update when generating **[DONE]**
- Allow doc comment in dictionary literal **[DONE]**

### 0.0.62
- Fix `import` keyword is unrecognized bug **[DONE]**
- Improve performance when auto completing **[DONE]**
- Fix SQL string quote bugs when performing join **[DONE]**
- Fix MongoDB object update bug **[DONE]**
- Fix enum types migration bug for SQL databases **[DONE]**

### 0.0.63
- Fix app entrance display bug for Rust CLI **[DONE]**
- Allow handler declaration in namespace and top level **[DONE]**
- Allow optional handler input type **[DONE]**
- Add `nonapi` keyword to handler declaration **[DONE]**
- Handler decorator `map` **[DONE]**
- Fix launching bugs for no database server apps **[DONE]**
- Add handler with no arguments **[DONE]**
- Add `interface` parameter and generate interface for handler path arguments **[DONE]**
- Fix handler URL matching bug **[DONE]**
- Fix output type of `$when` **[DONE]**
- Add `indexmap` to generated Rust entity **[DONE]**
- Fix file uploading bugs **[DONE]**

### 0.0.64
- Fix VSCode crashing when completing relation decorators **[DONE]**
- Fix data seeding bugs **[DONE]**
- Add back relation delete rule **[DONE]**
- Add relation update rule **[DONE]**
- Remove `@recordPrevious` **[DONE]**
- Fix update input is not optional bug **[DONE]**

### 0.0.65
- Fix seeding query bug that sometimes occurs **[DONE]**
- Fix transaction bug for Node.js and Python **[DONE]**

### 0.1.0
- Rewrite Node.js server **[DONE]**
- Update design of README.md **[DONE]**
- Node.js object printing **[DONE]**
- Node.js client use dedicated `DateOnly` class instead of builtin `Date` **[DONE]**
- Cross language error handling for node.js **[DONE]**
- Reverse middleware stack order **[DONE]**
- Asynchronous schema loading **[DONE]**
- Rewrite Node.js entity generation **[DONE]**
- Fix MySQL migration bug for `DateTime` **[DONE]**
- Pipeline item target type casting **[DONE]**
- TS Client: correct return types for `count`, `aggregate` and `groupBy` **[DONE]**
- TS Client: fix `DateOnly` and `ObjectId` type to string **[DONE]**
- Extract arguments in wrapped pipeline item with **[DONE]**
- Node.js: add `count`, `aggregate` and `groupBy` **[DONE]**
- Node.js: type safe database methods **[DONE]**
- TS Client: add `decimal.js` to package.json **[DONE]**

### 0.1.1
- Fixed auto seeding bug which causes exiting **[DONE]**

### 0.1.2
- Fixed Node.js package release bug **[DONE]**

### 0.1.3
- Fixed TypeScript client array result type bug **[DONE]**

### 0.1.4
- Fixed `orderBy` input definition bug **[DONE]**

### 0.1.5
- Fixed client generation for `orderBy` **[DONE]**
- Fixed pipeline item and model decorators loading in CLI mode **[DONE]**

### 0.1.6
- Fixed `DateTime` encoding bug in TypeScript client **[DONE]**

### 0.2.0
- Python integration

### 0.3.0
- Add `onSet`, `onOutput` and `default` for interface fields
- Revamped request extension API
- Improved middleware system
- Add back model action transformers
- JWT middleware **[IN PROGRESS]**
- Identity middleware **[IN PROGRESS]**
- Refactor bcrypt pipeline items
- Soft delete

### 0.3.5
- Support first version of Teo Studio

### 0.4.0
- Add back integration tests
- Swift client
- Kotlin client
- C# client
- Dart client
- C# client: capitalized names and names with underscore
- Debug logging SQL
- Migration dry run
- MongoDB: connector is not using table name

### 0.5.0
- Model fragments
- model extends model
- enum extends enum
- Json type

### 0.6.0
- MongoDB transaction
- Custom SQL query scopes
- Custom MongoDB query scopes
- Default argument for argument list and config declaration

### 0.7.0
- PostgreSQL: Enum types
- MongoDB migration
- MongoDB `$queryRaw`
- MongoDB: root skip take and distinct bug
- MongoDB: nested skip take and distinct bug
- MongoDB: if cursor key is not orderBy key, result is wrong
- MongoDB: relation where: multiple keys should be allowed
- MongoDB: relation where: 'every' results is incorrect
- MongoDB: aggregate and group by for string and dates
- Correct count with cursor

### 0.8.0
- Full database type mapping

### 1.0.0
- Support MSSQL
- First stable major version

### 1.5.0
- Full set of unit tests