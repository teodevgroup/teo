#[derive(Debug, Clone)]
pub enum Top {
    Enum(Enum),
    Model(Model),
    Connector(Connector),
    Client(Client),
}
