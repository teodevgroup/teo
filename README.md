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
- SQL: join table relationship: cannot insert into object query map
- SQL: Enum types
- MongoDB update handle column key
- Remove primary name from MongoDB connector's `save` method
- Add `PresentWith` and `PresentWithout` to `Optionality`
- Fix broken `connectIdentity` modifier
- For relationship with join table, link them after both objects are created

#### 0.0.28
- Custom SQL database types

#### 0.0.29 - 0.0.50
- Migration with SQL databases

#### 0.0.51
- MongoDB transaction session, make sure if errors, rollback writes

#### 0.0.52
- Input omissible and output omissible for generated clients

#### 0.0.53
- Delete rules

#### 0.0.54
- Delete callbacks: `when_delete`, `before_delete` and `after_delete`

#### 0.0.55
- Frontend lib for aggregate, count and groupBy

#### 0.0.56
- Correct count with cursor

#### 0.0.57
- Frontend lib for aggregate, count and groupBy

#### 0.0.58
- The copy action

#### 0.0.59
- Test column key remap with aggregate and groupBy

#### 0.0.60
- Auto migration

#### 0.0.61
- `auth_by_companion`

#### 0.0.62
- Fix decimal problems
- Relation with read write rules

#### 0.0.63
- Fix date problems

#### 0.0.64
- Fix datetime problems

#### 0.0.65
- random float modifier
- random int modifier

#### 0.0.66
- to uppercase modifier
- to lowercase modifier
- to word case modifier
- to sentence case modifier
- to title case modifier

#### 0.0.67
- `present_with` on field
- `present_without` on field

#### 0.0.68 - 0.0.110
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
