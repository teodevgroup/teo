use crate::core::relation::Relation;
use crate::parser::ast::argument::Argument;

pub(crate) fn relation_decorator(args: &Vec<Argument>, relation: &mut Relation) {
    let fields_arg = args.iter().find(|a| {
        a.name.as_ref().unwrap().name == "fields"
    });
    let through_arg = args.iter().find(|a| {
        a.name.as_ref().unwrap().name == "through"
    });
    if fields_arg.is_some() && through_arg.is_some() {
        panic!("A relation cannot have both 'fields' and 'through'.");
    } else if fields_arg.is_some() {
        // use fields and references
        let fields = fields_arg.unwrap();
        let references = args.iter().find(|a| {
            a.name.as_ref().unwrap().name == "references"
        });
        if references.is_none() {
            panic!("A relation with 'fields' must have 'references'.");
        }
        let references = references.unwrap();
        let fields_value = fields.resolved.as_ref().unwrap().as_value().unwrap();
        let _references_value = references.resolved.as_ref().unwrap().as_value().unwrap();
        if let Some(_) = fields_value.as_vec() {
            let fields_vec: Vec<String> = fields.resolved.as_ref().unwrap().as_value().unwrap().as_vec().unwrap().iter().map(|v| {
                v.as_raw_enum_choice().unwrap().to_owned()
            }).collect();
            relation.set_fields(fields_vec);
            let references_vec: Vec<String> = references.resolved.as_ref().unwrap().as_value().unwrap().as_vec().unwrap().iter().map(|v| {
                v.as_raw_enum_choice().unwrap().to_owned()
            }).collect();
            relation.set_references(references_vec);
        } else if let Some(_) = fields_value.as_raw_enum_choice() {
            let field = fields.resolved.as_ref().unwrap().as_value().unwrap().as_raw_enum_choice().unwrap().to_owned();
            relation.set_fields(vec![field]);
            let reference = references.resolved.as_ref().unwrap().as_value().unwrap().as_raw_enum_choice().unwrap().to_owned();
            relation.set_references(vec![reference]);
        }
    } else if through_arg.is_some() {
        // use through, local and foreign
        let through_model_ref = through_arg.unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_raw_enum_choice().unwrap().to_owned();
        relation.set_through(through_model_ref);
        let local = args.iter().find(|a| {
            a.name.as_ref().unwrap().name == "local"
        }).unwrap();
        relation.set_local(local.resolved.as_ref().unwrap().as_value().unwrap().as_raw_enum_choice().unwrap().to_owned());
        let foreign = args.iter().find(|a| {
            a.name.as_ref().unwrap().name == "foreign"
        }).unwrap();
        relation.set_foreign(foreign.resolved.as_ref().unwrap().as_value().unwrap().as_raw_enum_choice().unwrap().to_owned());
    } else {
        panic!("One of 'fields' or 'through' must be provided.")
    }
    // delete rule
    // let on_delete_arg = args.iter().find(|a| {
    //     &a.name.unwrap().name == "onDelete"
    // });
    // if on_delete_arg.is_some() {
    //     let rule = on_delete_arg.unwrap().resolved.unwrap().as_value().unwrap().as_raw_enum_choice().unwrap();
    //     match rule {
    //
    //     }
    // }
    // update rule
}
