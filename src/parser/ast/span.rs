#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub col: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, col: usize) -> Span {
        Self { start, end, line, col }
    }

    pub fn empty() -> Span {
        Span { start: 0, end: 0, line: 0, col: 0 }
    }

    pub fn contains(&self, position: usize) -> bool {
        position >= self.start && position <= self.end
    }

    pub fn overlaps(self, other: Span) -> bool {
        self.contains(other.start) || self.contains(other.end)
    }
}
