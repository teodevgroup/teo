TEO
===
The new generation server software.

## Roadmap

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
- Handle manipulation uses single or many
- Bug fixes
- Remove position from env, it can be inferred from `intent`
- Fix assign identity

#### 0.0.33
- SQL: join table relationship: cannot insert into object query map
- SQL: Enum types
- MongoDB update handle column key
- Fix broken `connectIdentity` modifier

#### 0.0.34
- Custom SQL database types

#### 0.0.35 - 0.0.50
- Migration with SQL databases

#### 0.0.51
- MongoDB transaction session, make sure if errors, rollback writes

#### 0.0.52
- Input omissible and output omissible for generated clients

#### 0.0.53
- Delete callbacks: `when_delete`, `before_delete` and `after_delete`

#### 0.0.54
- Frontend lib for aggregate, count and groupBy

#### 0.0.55
- Correct count with cursor

#### 0.0.56
- Frontend lib for aggregate, count and groupBy

#### 0.0.57
- The copy action

#### 0.0.58
- Test column key remap with aggregate and groupBy

#### 0.0.59
- Auto migration

#### 0.0.60
- `auth_by_companion`

#### 0.0.61
- Fix decimal problems
- Relation with read write rules

#### 0.0.62
- Fix date problems

#### 0.0.63
- Fix datetime problems

#### 0.0.64
- random float modifier
- random int modifier

#### 0.0.65
- to uppercase modifier
- to lowercase modifier
- to word case modifier
- to sentence case modifier
- to title case modifier

#### 0.0.66 - 0.0.110
- parsers and schema
- language servers

#### 0.1.0
- Unit test all sql features

#### 0.1.0
- Unit test all mongodb features

#### 0.3.0
- Unit test all modifier features

#### 0.4.0
- Unit test more

#### 0.5.0
- Unit test parsers and schema if needed

#### 0.6.0
- Make language things robust

#### 0.7.0
- Plugins

#### 1.0.0
- First release

#### 2.0.0
- Node.js support

#### 3.0.0
- Try to support java
