use crate::core::permission::Permission;
use crate::core::pipeline::Pipeline;


#[derive(Debug, Clone)]
pub struct PermissionBuilder {
    pub(crate) can_read: Option<Pipeline>,
    pub(crate) can_update: Option<Pipeline>,
    pub(crate) can_create: Option<Pipeline>,
    pub(crate) can_delete: Option<Pipeline>,
}

impl PermissionBuilder {

    pub(crate) fn new() -> Self {
        PermissionBuilder { can_read: None, can_update: None, can_create: None, can_delete: None }
    }

    pub fn can_read<F: Fn(&mut Pipeline)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = Pipeline::new();
        build(&mut pipeline);
        self.can_read = Some(pipeline);
        self
    }

    pub fn can_update<F: Fn(&mut Pipeline)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = Pipeline::new();
        build(&mut pipeline);
        self.can_update = Some(pipeline);
        self
    }

    pub fn can_create<F: Fn(&mut Pipeline)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = Pipeline::new();
        build(&mut pipeline);
        self.can_create = Some(pipeline);
        self
    }

    pub fn can_delete<F: Fn(&mut Pipeline)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = Pipeline::new();
        build(&mut pipeline);
        self.can_delete = Some(pipeline);
        self
    }

    pub(crate) fn build(&self) -> Permission {
        Permission {
            can_read: self.can_read.clone(),
            can_create: self.can_create.clone(),
            can_update: self.can_update.clone(),
            can_delete: self.can_delete.clone(),
        }
    }
}
