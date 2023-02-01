# Feature List

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
- Node.js: remove App.prototype.run and rename AppBuilder into App **[DONE]**
- Python: remove App.run and rename AppBuilder into App **[DONE]**
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
- Fix pipeline highlighting bug for VSCode
- Design client config blocks
- Rewrite client code generation with TS and JS
- `invalid` modifier **[DONE]**
- Remove stage from pipeline context **[DONE]**
- `if` modifier **[DONE]**
- `not` modifier **[DONE]**
- `passed` modifier **[DONE]**
- `and` and `or` modifier **[DONE]**
- `validate` supports pipeline argument **[DONE]**
- `transform` supports pipeline argument **[DONE]**
- Fixed range literal bug **[DONE]**

#### 0.0.45
- CLI mode: When running generation, ignore custom programming callbacks
- CLI mode: When running server, panic if custom programming callback is provided
- Cross language error handling

#### 0.0.46
- Support code comment tags for VSCode
- Support code comment parsing in schema parser
- Parser: parsing generations
- Rewrite generation process
- Rewrite TypeScript client package
- Rust entity generation

#### 0.0.47
- Swift package
- Kotlin package

#### 0.0.50
- Pipeline allows action error
- More pipeline modifiers
- MongoDB: root skip take and distinct bug
- MongoDB: nested skip take and distinct bug
- MongoDB: if cursor key is not orderBy key, result is wrong
- MongoDB: relation where: multiple keys should be allowed
- MongoDB: relation where: 'every' results is incorrect
- MongoDB: aggregate and group by for string and dates
- All many actions should throw errors
- SQL: Enum types
- Delete callbacks: `when_delete`, `before_delete` and `after_delete`

#### 0.0.51
- PostgreSQL: Migration I
- SQLite: Migration I

#### 0.0.52
- Rewrite TypeScript client

#### 0.0.53
- The copy action

#### 0.0.54 - 0.0.70
- Setup unit tests
- Replace buggy sqlx
- Full support of decimal type
- Support unsigned types for MySQL
- Custom SQL database types
- MySQL: Enum types

#### 0.0.71 - 0.0.81
- Fix assign identity
- Fix broken `connectIdentity` modifier
- MongoDB transaction
- SQL transaction

#### 0.0.82
- Migration II with MongoDB
- Migration II with SQL databases

#### 0.0.83
- Input omissible and output omissible for generated clients

#### 0.0.84
- Correct count with cursor

#### 0.0.85
- `auth_by_companion`

#### 0.0.86
- Relation with read write rules

#### 0.0.87
- random float modifier
- random int modifier

#### 0.0.88
- to uppercase modifier
- to lowercase modifier
- to word case modifier
- to sentence case modifier
- to title case modifier

#### 0.1.0
- Swift client
- Kotlin client
- Dart client
- Full unit test covering

#### 0.3.0
- Plugins

#### 0.6.0
- Java support

#### 1.0.0
- First stable version
