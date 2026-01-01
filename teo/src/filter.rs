use crate::partial::Partial;

pub trait Filter<P>: IntoIterator where P: Partial {

}

pub trait FilterIter: Iterator {

}
