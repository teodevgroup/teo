// tson_set
// tson_get
// tson_set_default








// pub(crate) fn tson_set(args: &Vec<Argument>) -> Arc<dyn Modifier> {
//     let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
//     Arc::new(TsonSetItem::new(value))
// }
//
// pub(crate) fn tson_get(args: &Vec<Argument>) -> Arc<dyn Modifier> {
//     let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
//     Arc::new(TsonSetDefaultItem::new(value))
// }
//
// pub(crate) fn tson_get(args: &Vec<Argument>) -> Arc<dyn Modifier> {
//     let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
//     Arc::new(TsonGetItem::new(value))
// }
