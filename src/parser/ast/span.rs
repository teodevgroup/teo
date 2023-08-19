#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub start_position: (usize, usize),
    pub end_position: (usize, usize),
}

impl Span {
    pub fn new(start: usize, end: usize, start_position: (usize, usize), end_position: (usize, usize)) -> Span {
        Self { start, end, start_position, end_position }
    }

    pub fn empty() -> Span {
        Span { start: 0, end: 0, start_position: (0, 0), end_position: (0, 0) }
    }

    pub fn contains(&self, position: usize) -> bool {
        position >= self.start && position <= self.end
    }

    pub fn overlaps(self, other: Span) -> bool {
        self.contains(other.start) || self.contains(other.end)
    }
}
