use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::encode::{IfIMode, ToLike, ToSQLString, ToWrapped, ValueToSQLString, WrapInArray};
use crate::connectors::sql::stmts::select::r#where::{ToWrappedSQLString, WhereClause};
use crate::connectors::sql::stmts::select::r#where::WhereClause::And;
use crate::connectors::sql::stmts::SQL;
use crate::core::field::r#type::FieldType;
use crate::core::input::Input;
use crate::core::model::Model;
use crate::prelude::{Graph, Object, Value};

pub(crate) struct Query { }

impl Query {

    pub(crate) fn where_item(lhs: &str, op: &str, rhs: &str) -> String {
        format!("{lhs} {op} {rhs}")
    }

    pub(crate) fn where_from_identifier(object: &Object, dialect: SQLDialect) -> String {
        Self::where_from_value(object.model(), object.graph(), &object.identifier(), dialect)
    }

    fn where_entry_array(
        column_name: &str,
        r#type: &FieldType,
        optional: bool,
        value: &Value,
        graph: &Graph,
        op: &str
    ) -> String {
        let arr_val = value.as_vec().unwrap();
        let mut arr: Vec<String> = Vec::new();
        for val in arr_val {
            arr.push(val.to_sql_string(r#type, optional, graph));
        }
        Query::where_item(column_name, op, &arr.join(", ").to_wrapped())
    }

    fn where_entry_item(
        column_name: &str,
        r#type: &FieldType,
        optional: bool,
        value: &Value,
        graph: &Graph,
        dialect: SQLDialect,
    ) -> String {
        if let Some(map) = value.as_hashmap() {
            let mut result: Vec<String> = vec![];
            for (key, value) in map {
                match key.as_str() {
                    "equals" => {
                        result.push(Self::where_item(column_name, "=", &value.to_sql_string(r#type, optional, graph)));
                    }
                    "not" => {
                        result.push(Self::where_item(column_name, "<>", &value.to_sql_string(r#type, optional, graph)));
                    }
                    "gt" => {
                        result.push(Self::where_item(column_name, ">", &value.to_sql_string(r#type, false, graph)));
                    }
                    "gte" => {
                        result.push(Self::where_item(column_name, ">=", &value.to_sql_string(r#type, false, graph)));
                    }
                    "lt" => {
                        result.push(Self::where_item(column_name, "<", &value.to_sql_string(r#type, false, graph)));
                    }
                    "lte" => {
                        result.push(Self::where_item(column_name, "<=", &value.to_sql_string(r#type, false, graph)));
                    }
                    "in" => {
                        result.push(Self::where_entry_array(column_name, r#type, optional, value, graph, "IN"));
                    }
                    "notIn" => {
                        result.push(Self::where_entry_array(column_name, r#type, optional, value, graph, "NOT IN"));
                    }
                    "contains" => {
                        let i_mode = Input::has_i_mode(map);
                        result.push(Self::where_item(&column_name.to_i_mode(i_mode), "LIKE", &value.to_sql_string(r#type, false, graph).to_like(true, true).to_i_mode(i_mode)));
                    }
                    "startsWith" => {
                        let i_mode = Input::has_i_mode(map);
                        result.push(Self::where_item(&column_name.to_i_mode(i_mode), "LIKE", &value.to_sql_string(r#type, false, graph).to_like(false, true).to_i_mode(i_mode)));
                    }
                    "endsWith" => {
                        let i_mode = Input::has_i_mode(map);
                        result.push(Self::where_item(&column_name.to_i_mode(i_mode), "LIKE", &value.to_sql_string(r#type, false, graph).to_like(true, false).to_i_mode(i_mode)));
                    }
                    "matches" => {
                        let i_mode = Input::has_i_mode(map);
                        result.push(Self::where_item(&column_name.to_i_mode(i_mode), "REGEXP", &value.to_sql_string(r#type, false, graph).to_i_mode(i_mode)));
                    }
                    "mode" => { }
                    "has" => {
                        let element_type = r#type.element_field().unwrap();
                        result.push(Self::where_item(column_name, "@>", &value.to_sql_string(element_type.r#type(), element_type.is_optional(), graph).wrap_in_array()));
                    }
                    "hasEvery" => {
                        result.push(Self::where_item(column_name, "@>", &value.to_sql_string(r#type, false, graph)));
                    }
                    "hasSome" => {
                        result.push(Self::where_item(column_name, "&&", &value.to_sql_string(r#type, false, graph)));
                    }
                    "isEmpty" => {
                        result.push(Self::where_item(&format!("ARRAY_LENGTH({})", column_name), "=", "0"));
                    }
                    "length" => {
                        result.push(Self::where_item(&format!("ARRAY_LENGTH({})", column_name), "=", &value.to_sql_string(&FieldType::U64, false, graph)));
                    }
                    _ => panic!("Unhandled key."),
                }
            }
            And(result).to_wrapped_string(dialect)
        } else {
            Query::where_item(column_name, "=", &value.to_sql_string(r#type, optional, graph))
        }
    }

    fn where_entry(
        column_name: &str,
        field_type: &FieldType,
        optional: bool,
        value: &Value,
        graph: &Graph,
        dialect: SQLDialect,
    ) -> String {
        Self::where_entry_item(column_name, field_type, optional, value, graph, dialect)
    }

    pub(crate) fn where_from_value(model: &Model, _graph: &Graph, identifier: &Value, dialect: SQLDialect) -> String {
        let mut retval: Vec<String> = vec![];
        for (key, value) in identifier.as_hashmap().unwrap() {
            let field = model.field(key).unwrap();
            let column_name = field.column_name();
            retval.push(format!("{} = {}", column_name, value.to_string(dialect)));
        }
        And(retval).to_string(dialect)
    }

    pub(crate) fn r#where(model: &Model, graph: &Graph, r#where: &Value, dialect: SQLDialect) -> String {
        let r#where = r#where.as_hashmap().unwrap();
        let mut retval: Vec<String> = vec![];
        for (key, value) in r#where.iter() {
            if key == "AND" {
                let inner = WhereClause::And(value.as_vec().unwrap().iter().map(|w| Self::r#where(model, graph, w, dialect)).collect()).to_string(dialect);
                let val = "(".to_owned() + &inner + ")";
                retval.push(val);
            } else if key == "OR" {
                let inner = WhereClause::Or(value.as_vec().unwrap().iter().map(|w| Self::r#where(model, graph, value, dialect)).collect()).to_string(dialect);
                let val = "(".to_owned() + &inner + ")";
                retval.push(val);
            } else if key == "NOT" {
                let inner = WhereClause::Not(Self::r#where(model, graph, value, dialect)).to_string(dialect);
                let val = "(".to_owned() + &inner + ")";
                retval.push(val);
            } else {
                if let Some(field) = model.field(key) {
                    let column_name = field.column_name();
                    let optional = field.optionality.is_optional();
                    let where_entry = Query::where_entry(column_name, &field.field_type, optional, value, graph, dialect);
                    retval.push(where_entry);
                } else if let Some(relation) = model.relation(key) {
                    panic!("not handle this yet")
                }
            }
        }
        And(retval).to_string(dialect)
    }

    pub(crate) fn order_by(
        model: &Model,
        graph: &Graph,
        order_by: &Value,
        dialect: SQLDialect,
    ) -> String {
        let order_by = order_by.as_vec().unwrap();
        let mut retval: Vec<String> = vec![];
        for item in order_by.iter() {
            let (key, value) = Input::key_value(item.as_hashmap().unwrap());
            if let Some(field) = model.field(key) {
                let column_name = field.column_name();
                if let Some(str) = value.as_str() {
                    match str {
                        "asc" => retval.push(format!("{} ASC", column_name)),
                        "desc" => retval.push(format!("{} DESC", column_name)),
                        _ => panic!("Unhandled."),
                    }
                }
            }
        }
        retval.join(",")
    }

    pub(crate) fn build_for_count(
        model: &Model,
        graph: &Graph,
        value: &Value,
        dialect: SQLDialect,
        additional_where: Option<String>,
        additional_left_join: Option<String>,
        join_table_results: Option<Vec<String>>,
    ) -> String {
        format!("SELECT COUNT(*) FROM ({}) AS _", Self::build(model, graph, value, dialect, additional_where, additional_left_join, join_table_results))
    }

    pub(crate) fn build(
        model: &Model,
        graph: &Graph,
        value: &Value,
        dialect: SQLDialect,
        additional_where: Option<String>,
        additional_left_join: Option<String>,
        join_table_results: Option<Vec<String>>,
    ) -> String {
        let r#where = value.get("where");
        let order_by = value.get("orderBy");
        let page_size = value.get("pageSize");
        let page_number = value.get("pageNumber");
        let skip = value.get("skip");
        let take = value.get("take");
        let table_name = if additional_left_join.is_some() {
            model.table_name().to_string() + " AS t"
        } else {
            model.table_name().to_string()
        };
        let mut columns: Vec<String> = vec![];
        if additional_left_join.is_some() {
            columns = model.save_keys().iter().map(|k| format!("t.{} AS {}", k, k)).collect::<Vec<String>>();
        }
        if let Some(join_table_results) = join_table_results {
            for result_key in join_table_results.iter() {
                columns.push(format!("j.{} AS j_{}", result_key, result_key))
            }
        }
        let column_refs = columns.iter().map(|c| c.as_str()).collect::<Vec<&str>>();

        let mut stmt = SQL::select(if columns.is_empty() { None } else { Some(&column_refs) }, &table_name);
        if let Some(r#where) = r#where {
            if !r#where.as_hashmap().unwrap().is_empty() {
                stmt.r#where(Query::r#where(model, graph, r#where, dialect));
            }
        }
        if let Some(additional_where) = additional_where {
            if stmt.r#where.is_some() {
                stmt.r#where(stmt.r#where.as_ref().unwrap().clone() + &additional_where);
            } else {
                stmt.r#where(additional_where.to_string());
            }
        }
        if let Some(additional_left_join) = additional_left_join {
            stmt.left_join(additional_left_join);
        }
        if let Some(order_bys) = order_by {
            stmt.order_by(Query::order_by(model, graph, order_bys, dialect));
        }
        if page_size.is_some() && page_number.is_some() {
            let skip: u64 = ((page_number.unwrap().as_u64().unwrap() - 1) * page_size.unwrap().as_u64().unwrap()) as u64;
            let limit: u64 = page_size.unwrap().as_u64().unwrap() as u64;
            stmt.limit(limit, skip);
        } else if skip.is_some() || take.is_some() {
            let skip: u64 = if skip.is_some() { skip.unwrap().as_u64().unwrap() as u64 } else { 0 };
            let limit: u64 = if take.is_some() { take.unwrap().as_u64().unwrap() as u64 } else { 18446744073709551615 };
            stmt.limit(limit, skip);
        }
        stmt.to_string(dialect)
    }
}
