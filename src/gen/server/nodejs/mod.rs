pub(crate) mod utils;
pub(crate) mod runtime_d_ts;

use array_tool::vec::Join;
use crate::core::app::conf::EntityGeneratorConf;
use crate::gen::lib::generator::Generator;
use crate::gen::server::EntityGenerator;
use crate::prelude::Graph;
use async_trait::async_trait;
use inflector::Inflector;
use crate::gen::client::typescript::pkg::src::index_d_ts::generate_index_d_ts;
use crate::gen::lib::code::Code;
use crate::gen::server::nodejs::utils::{field_to_nodejs_api_type, relation_to_nodejs_api_type};

pub(crate) struct NodeJSEntityGenerator { }

impl NodeJSEntityGenerator {
    pub fn new() -> Self {
        Self { }
    }

    async fn generate_index_js(&self, graph: &Graph, generator: &Generator) -> std::io::Result<()> {
        let names: Vec<&str> = graph.models().iter().map(|m| m.name()).collect();
        let prefixed_names: Vec<String> = names.iter().map(|n| "  ".to_owned() + n).collect();
        let import = "const { getModelClass } = require(\"@teocloud/teo\")";
        let body = names.iter().map(|n| format!("const {} = getModelClass('{}')", *n, *n)).collect::<Vec<String>>().join("\n");
        let export = format!("module.exports = {{\n{},\n}}", prefixed_names.join(",\n"));
        generator.generate_file("index.js", format!("{import}\n\n{body}\n\n{export}\n")).await
    }

    async fn generate_index_d_ts(&self, graph: &Graph, generator: &Generator, shared: String) -> std::io::Result<()> {
        let shared_ref = &shared;
        let content = Code::new(0, 4, |b| {
            b.line(shared_ref);
            b.empty_line();
            for model in graph.models() {
                let name = model.name();
                b.block(format!("export class {} {{", name), |b| {
                    b.line("private constructor(): never");
                    // create doc
                    b.line(format!("static create(input?: {}CreateInput): Promise<{}>", name, name));
                    // find many doc
                    b.line(format!("static findMany(input?: {}FindManyArgs): Promise<{}[]>", name, name));
                    // find first doc
                    b.line(format!("static findFirst(input?: {}FindManyArgs): Promise<{} | null>", name, name));
                    // find unique doc
                    b.line(format!("static findUnique(input?: {}FindUniqueArgs): Promise<{} | null>", name, name));
                    // get isNew doc
                    b.line("get isNew(): boolean");
                    // get isModified doc
                    b.line("get isModified(): boolean");
                    // set doc
                    b.line(format!("set(input?: {}UpdateInput): Promise<void>", name));
                    // update doc
                    b.line(format!("update(input?: {}ScalarUpdateInput): Promise<void>", name));
                    // save doc
                    b.line("save(): Promise<void>");
                    // delete doc
                    b.line("delete(): Promise<void>");
                    for field in model.fields() {
                        let field_name = field.name();
                        let field_type = field_to_nodejs_api_type(field.as_ref());
                        // set doc
                        b.line(format!("set {field_name}(newValue: {field_type}): void"));
                        // get doc
                        b.line(format!("get {field_name}(): {field_type}"))
                    }
                    for relation in model.relations() {
                        let relation_name = relation.name();
                        let relation_type = relation_to_nodejs_api_type(relation);
                        let pascal_name = relation_name.to_pascal_case();
                        if relation.is_vec() {
                            // get doc
                            b.line(format!("get {relation_name}(): {relation_type}"));
                            // set doc
                            b.line(format!("set{pascal_name}(newValue: {relation_type}): Promise<void>"));
                            // add to doc
                            b.line(format!("addTo{pascal_name}(newValue: {relation_type}): Promise<void>"));
                            // remove from doc
                            b.line(format!("removeFrom{pascal_name}(newValue: {relation_type}): Promise<void>"));
                        } else {
                            // get doc
                            b.line(format!("get {relation_name}(): {relation_type}"));
                            // set doc
                            b.line(format!("set{pascal_name}(newValue: {relation_type}): Promise<void>"));
                        }
                    }
                    for property in model.properties() {
                        let property_name = property.name();
                        let field_type = field_to_nodejs_api_type(property.as_ref());
                        if property.has_getter() {
                            // get doc
                            b.line(format!("get {property_name}(): Promise<{field_type}>"))
                        }
                        if property.has_setter() {
                            // set doc
                            let pascal_name = property_name.to_pascal_case();
                            b.line(format!("set{pascal_name}(newValue: {field_type}): Promise<void>"))
                        }
                    }
                }, "}\n");
            }
        }).to_string();
        generator.generate_file("index.d.ts", content).await?;
        Ok(())
    }
}

#[async_trait]
impl EntityGenerator for NodeJSEntityGenerator {
    async fn generate_entity_files(&self, graph: &Graph, _conf: &EntityGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        self.generate_index_js(graph, generator).await?;
        let content = generate_index_d_ts(graph,None, true);
        self.generate_index_d_ts(graph, generator, content).await?;
        Ok(())
    }
}
