use crate::core::pipeline::builder::PipelineBuilder;
use crate::core::permission::Permission;
use crate::core::pipeline::Pipeline;

#[derive(Debug, Clone)]
pub struct PermissionBuilder {
    can_read: Option<Pipeline>,
    can_update: Option<Pipeline>,
    can_create: Option<Pipeline>,
    can_delete: Option<Pipeline>,
}

impl PermissionBuilder {

    pub(crate) fn new() -> Self {
        PermissionBuilder { can_read: None, can_update: None, can_create: None, can_delete: None }
    }

    pub fn can_read<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = PipelineBuilder::new();
        build(&mut builder);
        let pipeline = builder.build();
        if let Some(can_read) = &mut self.can_read {
            can_read.modifiers.extend(pipeline.modifiers)
        } else {
            self.can_read = Some(pipeline)
        }
        self
    }

    pub fn can_update<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = PipelineBuilder::new();
        build(&mut builder);
        let pipeline = builder.build();
        if let Some(can_update) = &mut self.can_update {
            can_update.modifiers.extend(pipeline.modifiers)
        } else {
            self.can_update = Some(pipeline)
        }
        self
    }

    pub fn can_create<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = PipelineBuilder::new();
        build(&mut builder);
        let pipeline = builder.build();
        if let Some(can_create) = &mut self.can_create {
            can_create.modifiers.extend(pipeline.modifiers)
        } else {
            self.can_create = Some(pipeline)
        }
        self
    }

    pub fn can_delete<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = PipelineBuilder::new();
        build(&mut builder);
        let pipeline = builder.build();
        if let Some(can_delete) = &mut self.can_delete {
            can_delete.modifiers.extend(pipeline.modifiers)
        } else {
            self.can_delete = Some(pipeline)
        }
        self
    }

    pub fn can_write<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = PipelineBuilder::new();
        build(&mut builder);
        let pipeline = builder.build();
        if let Some(can_create) = &mut self.can_create {
            can_create.modifiers.extend(pipeline.modifiers.clone())
        } else {
            self.can_create = Some(pipeline.clone())
        }
        if let Some(can_update) = &mut self.can_update {
            can_update.modifiers.extend(pipeline.modifiers.clone())
        } else {
            self.can_update = Some(pipeline.clone())
        }
        self
    }

    pub fn can_mutate<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = PipelineBuilder::new();
        build(&mut builder);
        let pipeline = builder.build();
        if let Some(can_create) = &mut self.can_create {
            can_create.modifiers.extend(pipeline.modifiers.clone())
        } else {
            self.can_create = Some(pipeline.clone())
        }
        if let Some(can_update) = &mut self.can_update {
            can_update.modifiers.extend(pipeline.modifiers.clone())
        } else {
            self.can_update = Some(pipeline.clone())
        }
        if let Some(can_delete) = &mut self.can_delete {
            can_delete.modifiers.extend(pipeline.modifiers.clone())
        } else {
            self.can_delete = Some(pipeline.clone())
        }
        self
    }

    pub(crate) fn build(&self) -> Permission {
        Permission::new(self.can_read.clone(), self.can_update.clone(), self.can_create.clone(), self.can_delete.clone())
    }
}
