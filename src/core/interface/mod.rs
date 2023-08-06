#[derive(Debug)]
pub struct InterfaceRef {
    pub name: String,
    pub args: Vec<InterfaceRef>,
}

#[derive(Debug)]
pub struct InterfaceDefinition {
    pub name: String,
    pub args: Vec<String>,
    pub extends: Vec<InterfaceRef>,
}

#[derive(Debug)]
pub struct InterfaceField {
    pub name: String,
    pub kind: InterfaceRef,
}

#[derive(Debug)]
pub struct CustomActionDefinition {
    pub group: String,
    pub name: String,
    pub input: InterfaceRef,
    pub output: InterfaceRef,
}