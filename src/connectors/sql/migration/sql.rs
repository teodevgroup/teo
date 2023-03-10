pub(crate) fn sqlite_list_indices_query(table_name: &str) -> String {
    format!("SELECT
  m.tbl_name AS table_name,
  il.name AS key_name,
  \"unique\" AS \"unique\",
  ii.name AS column_name
FROM
  sqlite_master AS m,
  pragma_index_list(m.name) AS il,
  pragma_index_info(il.name) AS ii
WHERE
  m.type = 'table' AND
  il.origin = 'u' AND
  table_name = '{}'
ORDER BY table_name, key_name, ii.seqno", table_name)
}

pub(crate) fn sqlite_auto_increment_query(table_name: &str) -> String {
    format!("SELECT \"is-autoincrement\" FROM sqlite_master WHERE tbl_name=\"{}\" AND sql LIKE \"%AUTOINCREMENT%\"", table_name)
}

pub(crate) fn psql_is_auto_increment(table_name: &str, column_name: &str) -> String {
    format!("select relname from pg_class where relname = '{}_{}_seq'", table_name, column_name)
}
