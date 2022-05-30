use inflector::Inflector;
use crate::action::action::ActionType;
use crate::client::shared::code::Code;
use crate::client::typescript::r#type::ToTypeScriptType;
use crate::core::field::Availability;
use crate::core::graph::Graph;


pub async fn generate_index_ts(graph: &'static Graph) -> String {
    Code::new(0, 4, |c| {
        // enum definitions
        graph.enums().iter().for_each(|e| {
            let name = e.0;
            let choices = e.1.iter().map(|i| {String::from("\"") + i + "\""}).collect::<Vec<String>>().join(" | ");
            c.line(format!("export type {name} = {choices}"));
            c.empty_line();
        });
        // model definitions
        graph.models().iter().for_each(|m| {
            if m.actions().len() > 0 {
                let model_name = m.name();
                c.block(format!("export type {model_name} = {{"), |b| {
                    m.output_keys().iter().for_each(|k| {
                        let field = m.field(k);
                        let field_name = field.name;
                        let field_type = field.r#type.to_typescript_type(field.availability == Availability::Optional);
                        b.line(format!("{field_name}: {field_type}"));
                    });
                }, "}");
                c.empty_line();
            }
        });
        // delegates
        graph.models().iter().for_each(|m| {
            if m.actions().len() > 0 {
                let model_name = m.name();
                let model_var_name = model_name.to_camel_case();
                c.block(format!("const {model_var_name}Delegate = {{"), |b| {
                    if m.actions().contains(&ActionType::FindUnique) {
                        b.empty_line();
                        b.block(format!("findUnique(args: {model_name}FindUniqueArgs): Promise<{model_name}> {{"), |b| {
                            b.empty_line();
                        }, "},");
                    }
                    if m.actions().contains(&ActionType::FindMany) {
                        b.empty_line();
                        b.block(format!("findMany(args: {model_name}FindManyArgs): Promise<{model_name}[]> {{"), |b| {
                            b.empty_line();
                        }, "},");
                    }
                    if m.actions().contains(&ActionType::FindFirst) {
                        b.empty_line();
                        b.block(format!("findFirst(args: {model_name}FindFirstArgs): Promise<{model_name}> {{"), |b| {
                            b.empty_line();
                        }, "},");
                    }
                    if m.actions().contains(&ActionType::Create) {
                        b.empty_line();
                        b.block(format!("create(args: {model_name}CreateArgs): Promise<{model_name}> {{"), |b| {
                            b.empty_line();
                        }, "},");
                    }
                    if m.actions().contains(&ActionType::Update) {
                        b.empty_line();
                        b.block(format!("update(args: {model_name}UpdateArgs): Promise<{model_name}> {{"), |b| {
                            b.empty_line();
                        }, "},");
                    }
                    if m.actions().contains(&ActionType::Upsert) {
                        b.empty_line();
                        b.block(format!("upsert(args: {model_name}UpsertArgs): Promise<{model_name}> {{"), |b| {
                            b.empty_line();
                        }, "},");
                    }
                    if m.actions().contains(&ActionType::Delete) {
                        b.empty_line();
                        b.block(format!("delete(args: {model_name}DeleteArgs): Promise<{model_name}> {{"), |b| {
                            b.empty_line();
                        }, "},");
                    }
                    if m.actions().contains(&ActionType::CreateMany) {
                        b.empty_line();
                        b.block(format!("createMany(args: {model_name}CreateManyArgs): Promise<{model_name}[]> {{"), |b| {
                            b.empty_line();
                        }, "},");
                    }
                    if m.actions().contains(&ActionType::UpdateMany) {
                        b.empty_line();
                        b.block(format!("updateMany(args: {model_name}UpdateManyArgs): Promise<{model_name}[]> {{"), |b| {
                            b.empty_line();
                        }, "},");
                    }
                    if m.actions().contains(&ActionType::DeleteMany) {
                        b.empty_line();
                        b.block(format!("deleteMany(args: {model_name}DeleteManyArgs): Promise<{model_name}[]> {{"), |b| {
                            b.empty_line();
                        }, "},");
                    }
                    if m.actions().contains(&ActionType::SignIn) {
                        b.empty_line();
                        b.block(format!("signIn(args: {model_name}SignInArgs): Promise<{model_name}> {{"), |b| {
                            b.empty_line();
                        }, "},");
                    }
                }, "}");
                c.empty_line();
            }
        });
        // main interface
        c.block("const teo = {", |b| {
            graph.models().iter().for_each(|m| {
                if m.actions().len() > 0 {
                    let model_name = m.name();
                    let model_var_name = model_name.to_camel_case();
                    b.line(format!("{model_var_name}: {model_var_name}Delegate,"));
                }
            });
        }, "}");
        c.empty_line();
        c.line("export default teo");
    }).to_string()
}
