use std::sync::Arc;
use educe::Educe;
use crate::app::callbacks::callback::AsyncCallback;

#[derive(Educe)]
#[educe(Debug)]
#[derive(Clone)]
pub struct Program {
    #[educe(Debug(ignore))]
    pub(crate) func: Arc<dyn AsyncCallback>,
    pub(crate) desc: Option<String>,
}

impl Program {
    pub(crate) fn new(desc: Option<String>, func: Arc<dyn AsyncCallback>) -> Self {
        Self { desc, func }
    }
}