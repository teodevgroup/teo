use lazy_static::lazy_static;

pub(crate) fn rt() -> &'static tokio::runtime::Runtime {
    lazy_static! {
        static ref RT: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Should create a tokio runtime");
    }
    &RT
}
