use super::span::Span;
use super::get_span::GetSpan;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl GetSpan for Identifier {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T: pest::RuleType> From<pest::iterators::Pair<'_, T>> for Identifier {
    fn from(pair: pest::iterators::Pair<'_, T>) -> Self {
        Identifier {
            name: pair.as_str().to_owned(),
            span: pair.as_span().into(),
        }
    }
}
