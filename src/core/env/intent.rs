use crate::core::env::intent::Intent::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum Intent {
    Create,
    Update,
    UpsertActuallyCreate,
    UpsertActuallyUpdate,
    Delete,
    CreateMany,
    UpdateMany,
    DeleteMany,
    FindFirst,
    FindUnique,
    FindMany,
    NestedIncluded,
    NestedConnect,
    NestedConnectOrCreateActuallyCreate,
    NestedConnectOrCreateActuallyConnect,
    NestedDisconnect,
    NestedSet,
    NestedCreate,
    NestedUpdate,
    NestedUpsertActuallyCreate,
    NestedUpsertActuallyUpdate,
    NestedDelete,
    NestedCreateMany,
    NestedUpdateMany,
    NestedDeleteMany,
    NestedJoinTableRecordCreate,
    NestedJoinTableRecordDelete,
}

impl Intent {

    pub(crate) fn is_create(&self) -> bool {
        match self {
            Create | UpsertActuallyCreate | CreateMany | NestedCreate | NestedConnectOrCreateActuallyCreate | NestedUpsertActuallyCreate => true,
            _ => false,
        }
    }

    pub(crate) fn is_update(&self) -> bool {
        match self {
            Update | UpsertActuallyUpdate | UpdateMany | NestedUpdate | NestedUpsertActuallyUpdate => true,
            _ => false,
        }
    }

    pub(crate) fn is_delete(&self) -> bool {
        match self {
            Delete | DeleteMany | NestedDelete | NestedDeleteMany => true,
            _ => false,
        }
    }

    pub(crate) fn is_upsert(&self) -> bool {
        match self {
            UpsertActuallyCreate | UpsertActuallyUpdate => true,
            _ => false,
        }
    }
}
