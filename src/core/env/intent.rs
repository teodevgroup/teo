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
    NestedConnectMany,
    NestedConnectManyOrCreateActuallyCreate,
    NestedConnectManyOrCreateActuallyConnect,
    NestedDisconnect,
    NestedDisconnectMany,
    NestedSet,
    NestedCreate,
    NestedUpdate,
    NestedUpsertActuallyCreate,
    NestedUpsertActuallyUpdate,
    NestedUpsertManyActuallyCreate,
    NestedUpsertManyActuallyUpdate,
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

    pub(crate) fn is_root_single(&self) -> bool {
        match self {
            Create | Update | UpsertActuallyCreate | UpsertActuallyUpdate | Delete |
            FindFirst | FindUnique => true,
            _ => false,
        }
    }

    pub(crate) fn is_nested_single(&self) -> bool {
        match self {
            NestedConnect |
            NestedConnectOrCreateActuallyCreate |
            NestedConnectOrCreateActuallyConnect |
            NestedDisconnect |
            NestedSet |
            NestedCreate |
            NestedUpdate |
            NestedUpsertActuallyCreate |
            NestedUpsertActuallyUpdate |
            NestedDelete => true,
            _ => false,
        }
    }

    pub(crate) fn is_single(&self) -> bool {
        self.is_root_single() || self.is_nested_single()
    }

    pub(crate) fn is_root_many(&self) -> bool {
        match self {
            CreateMany | UpdateMany | DeleteMany | FindMany => true,
            _ => false,
        }
    }

    pub(crate) fn is_nested_many(&self) -> bool {
        match self {
            NestedConnectMany |
            NestedConnectManyOrCreateActuallyCreate |
            NestedConnectManyOrCreateActuallyConnect |
            NestedDisconnectMany |
            NestedUpsertManyActuallyCreate |
            NestedUpsertManyActuallyUpdate |
            NestedCreateMany |
            NestedUpdateMany |
            NestedDeleteMany => true,
            _ => false,
        }
    }

    pub(crate) fn is_many(&self) -> bool {
        self.is_root_single() || self.is_nested_single()
    }

    pub(crate) fn is_root(&self) -> bool {
        self.is_root_single() || self.is_root_many()
    }

    pub(crate) fn is_nested(&self) -> bool {
        self.is_nested_single() || self.is_nested_many()
    }
}
