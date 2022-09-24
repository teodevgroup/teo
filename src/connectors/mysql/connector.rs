impl MySQLConnector {
    pub async fn new(pool: Pool, mut conn: Conn, database_name: String, models: &Vec<Model>) -> MySQLConnector {

        MySQLConnector { pool, conn, database_name }
    }

    pub async fn table_migrate(conn: &mut Conn, model: &Model) {
        let table_name = model.table_name();
        let desc = SQL::describe(table_name).to_string(SQLDialect::MySQL);
        let mut reviewed_columns: Vec<String> = Vec::new();
        let columns: Vec<Row> = desc.fetch(&mut *conn).await.unwrap();
        for column in &columns {
            let db_column: MySQLColumn = column.into();
            let schema_field = model.field(&db_column.field);
            if schema_field.is_none() {
                // remove this column
                let stmt = SQL::alter_table(table_name).drop_column(db_column.field.clone()).to_string(SQLDialect::MySQL);
                let _ = stmt.ignore(&mut *conn).await;
            }
            let sql_column_def: SQLColumnDef = schema_field.unwrap().into();
            let schema_column: MySQLColumn = (&sql_column_def).into();
            if schema_column != db_column {
                // this column is different, alter it
                let alter = SQL::alter_table(table_name).modify(sql_column_def).to_string(SQLDialect::MySQL);
                let _ = alter.ignore(&mut *conn).await;
            }
            reviewed_columns.push(db_column.field.clone());
        }
        for field in &model.fields_vec {
            if !reviewed_columns.contains(&field.column_name()) {
                let sql_column_def: SQLColumnDef = field.into();
                let add = SQL::alter_table(table_name).add(sql_column_def).to_string(SQLDialect::MySQL);
                let _ = add.ignore(&mut *conn).await;
            }
        }
        // then indices / unique / primary
        let show_index = SQL::show().index_from(table_name).to_string(SQLDialect::MySQL);
        let rows: Vec<Row> = show_index.fetch(&mut *conn).await.unwrap();
        let indices = mysql_indices_from_rows(&rows);
        let mut reviewed_indices: Vec<String> = Vec::new();
        for index in &indices {
            if &index.key_name == "PRIMARY" {
                continue;
            }
            let model_index = model.indices.iter().find(|i| i.name == index.key_name);
            if model_index.is_none() {
                // model doesn't have this index, while database has. Delete it.
                let drop = SQL::drop().index(&index.key_name).on(table_name).to_string(SQLDialect::MySQL);
                let _ = drop.ignore(&mut *conn).await;
            } else {
                let model_index = model_index.unwrap();
                let sql_model_index: MySQLIndex = model_index.into();
                if index != &sql_model_index {
                    // alter this index, drop and create
                    let drop = SQL::drop().index(&index.key_name).on(table_name).to_string(SQLDialect::MySQL);
                    let _ = drop.ignore(&mut *conn).await;
                    let create = SQL::create().index(&index.key_name).on(table_name)
                        .columns(model_index.items.iter().map(|item| {
                            let mut index = SQLIndexColumn::new(&item.field_name);
                            if item.sort == Sort::Desc {
                                index.desc();
                            }
                            index
                        }).collect()).to_string(SQLDialect::MySQL);
                    let _ = create.ignore(&mut *conn).await;
                }
            }
            reviewed_indices.push(index.key_name.clone());
        }
        for model_index in &model.indices {
            if !reviewed_indices.contains(&model_index.name) {
                // create this index
                let create = SQL::create().index(&model_index.name).on(table_name)
                    .columns(model_index.items.iter().map(|item| {
                        let mut index = SQLIndexColumn::new(&item.field_name);
                        if item.sort == Sort::Desc {
                            index.desc();
                        }
                        index
                    }).collect()).to_string(SQLDialect::MySQL);
                let _ = create.ignore(&mut *conn).await;
            }
        }
    }
}
