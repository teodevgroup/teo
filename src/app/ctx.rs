use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;
use teo_parser::ast::schema::Schema;
use teo_runtime::namespace::Namespace;
use crate::app::callbacks::callback::AsyncCallback;

#[derive(Debug)]
pub struct Ctx {
    loaded: bool,
    schema: Schema,
    main_namespace: Namespace,
    setup: Option<Arc<dyn AsyncCallback>>,
}

static CURRENT: OnceCell<Arc<Mutex<Ctx>>> = OnceCell::new();