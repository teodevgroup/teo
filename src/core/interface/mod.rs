#[derive(Debug)]
pub struct InterfaceRef {
    name: String,
    args: Vec<String>,
}

#[derive(Debug)]
pub struct InterfaceDefinition {
    name: String,
    args: Vec<String>,
    extends: Vec<InterfaceRef>,
}

#[derive(Debug)]
pub struct InterfaceField {
    name: String,
    kind: InterfaceRef,
}

#[derive(Debug)]
pub struct CustomActionDefinition {
    group: String,
    name: String,
    input: InterfaceRef,
    output: InterfaceRef,
}