use std::borrow::Cow;

pub(in crate::gen) struct DelegateAction<'a> {
    pub(in crate::gen) name: &'static str,
    pub(in crate::gen) response: Cow<'a, str>,
    pub(in crate::gen) docs: Option<Cow<'a, str>>,
}

pub(in crate::gen) struct Delegate<'a> {
    pub(in crate::gen) model_name: Cow<'a, str>,
    pub(in crate::gen) actions: Vec<DelegateAction<'a>>,
}

impl<'a> Delegate<'a> {
    pub(in crate::gen) fn value_for_data_transformer_dart(&self, action_name: &str) -> Cow<'a, str> {
        match action_name {
            "findUnique" | "findFirst" | "create" | "update" | "upsert" | "delete" | "signIn" | "identity" => Cow::Owned(format!("(p0) => {}.fromJson(p0)", self.model_name.as_ref())),
            "findMany" | "createMany" | "updateMany" | "deleteMany" => Cow::Owned(format!("(p0) => p0.map((e) => {}.fromJson(e)).toList()", self.model_name.as_ref())),
            _ => Cow::Borrowed("(p0) => p0"),
        }
    }
    pub(in crate::gen) fn value_for_meta_transformer_dart(&self, action_name: &str) -> &'static str {
        match action_name {
            "findMany" | "createMany" | "updateMany" | "deleteMany" => "(p0) => PagingInfo.fromJson(p0)",
            "signIn" => "(p0) => TokenInfo.fromJson(p0)",
            _ => "null",
        }
    }
}
