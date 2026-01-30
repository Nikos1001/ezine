
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SrcSpan {
    start: usize,
    end: usize
}

impl SrcSpan {

    pub fn new(start: usize, end: usize) -> Self {
        assert!(end >= start, "end cannot be before start.");
        Self {
            start,
            end,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

}
