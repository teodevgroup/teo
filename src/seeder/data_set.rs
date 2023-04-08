use to_mut_proc_macro::ToMut;
use to_mut::ToMut;
use crate::core::relation::Relation;
use crate::prelude::{Graph, Value};
use crate::teon;

#[derive(Debug, Clone)]
pub(crate) struct DataSet {
    pub(crate) notrack: bool,
    pub(crate) autoseed: bool,
    pub(crate) name: String,
    pub(crate) groups: Vec<Group>
}

#[derive(Debug, Clone, ToMut)]
pub(crate) struct Group {
    pub(crate) name: String,
    pub(crate) records: Vec<Record>,
}

#[derive(Debug, Clone)]
pub(crate) struct Record {
    pub(crate) name: String,
    pub(crate) value: Value,
}

pub(crate) fn normalize_dataset_relations<'a>(dataset: &'a DataSet, graph: &Graph) -> &'a DataSet {
    for group in &dataset.groups {
        let model = graph.model(group.name.as_str()).unwrap();
        for record in &group.records {
            for (k, v) in record.value.as_hashmap().unwrap() {
                if let Some(relation) = model.relation(k) {
                    let (opposite_model, opposite_rel) = graph.opposite_relation(relation);
                    // If there isn't a relation defined on the opposite side, just leave it here
                    if opposite_rel.is_none() {
                        continue
                    }
                    let opposite_rel = opposite_rel.unwrap();
                    if relation.is_vec() {
                        for v in v.as_vec().unwrap() {
                            assign_relation_other_side(dataset, record, v, relation, opposite_rel);
                        }
                    } else {
                        assign_relation_other_side(dataset, record, v, relation, opposite_rel);
                    }
                }
            }
        }
    }
    dataset
}

fn assign_relation_other_side(dataset: &DataSet, record: &Record, v: &Value, relation: &Relation, opposite_rel: &Relation) {
    let that_group = dataset.groups.iter().find(|g| &g.name == relation.model()).unwrap();
    let that_record = that_group.to_mut().records.iter_mut().find(|r| r.name == v.as_raw_enum_choice().unwrap()).unwrap();
    if opposite_rel.is_vec() {
        if that_record.value.as_hashmap_mut().unwrap().contains_key(opposite_rel.name()) {
            let array = that_record.value.as_hashmap_mut().unwrap().get_mut(opposite_rel.name()).unwrap().as_vec_mut().unwrap();
            let to_insert = Value::RawEnumChoice(record.name.clone(), None);
            if !array.contains(&to_insert) {
                array.push(to_insert);
            }
        } else {
            that_record.value.as_hashmap_mut().unwrap().insert(opposite_rel.name().to_owned(), teon!([Value::RawEnumChoice(record.name.clone(), None)]));
        }
    } else {
        that_record.value.as_hashmap_mut().unwrap().insert(opposite_rel.name().to_owned(), Value::RawEnumChoice(record.name.clone(), None));
    }
}