#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Self { start, end }
    }

    pub fn empty() -> Span {
        Span { start: 0, end: 0 }
    }

    pub fn contains(&self, position: usize) -> bool {
        position >= self.start && position <= self.end
    }

    pub fn overlaps(self, other: Span) -> bool {
        self.contains(other.start) || self.contains(other.end)
    }
}

impl From<pest::Span<'_>> for Span {
    fn from(s: pest::Span<'_>) -> Self {
        Span {
            start: s.start(),
            end: s.end(),
        }
    }
}
