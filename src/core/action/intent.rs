pub(crate) enum ActionIntent {
    Create,
    Update,
    Upsert,
    Delete,
    Find,
    Connect,
    ConnectOrCreate,
    Disconnect,
    Set,
    JoinCreate,
    JoinDelete,
}
