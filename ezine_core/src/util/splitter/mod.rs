
pub struct Splitter<'slice, 'target, T> {
    slice: &'slice [T],
    target: &'target [T],
    curr: usize
}

impl<'slice, 'target, T: PartialEq> Splitter<'slice, 'target, T> {

    pub fn new(slice: &'slice [T], target: &'target [T]) -> Self {
        Self {
            slice,
            target,
            curr: 0
        }
    }

    pub fn remainder(&self) -> &'slice [T] {
        &self.slice[self.curr..]
    }

}

impl<'slice, 'target, T: PartialEq> Iterator for Splitter<'slice, 'target, T> {
    type Item = &'slice [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.slice.len() {
            return None;
        }

        let start = self.curr;
        while self.curr + self.target.len() <= self.slice.len() {
            if &self.slice[self.curr..(self.curr + self.target.len())] == self.target {
                let result = &self.slice[start..self.curr];
                self.curr += self.target.len();
                return Some(result);
            } else {
                self.curr += 1;
            }
        }
        
        self.curr = self.slice.len();
        Some(&self.slice[start..])
    }
}

#[cfg(test)]
mod tests;
