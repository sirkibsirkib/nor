pub struct VecMutQ<'a, T> {
    vec: &'a mut Vec<T>,
    processed_before: usize,
}

impl<'a, T> VecMutQ<'a, T> {
    pub fn in_place_endo_map(vec: &'a mut Vec<T>, mut func: impl FnMut(T) -> T) {
        let mut me = Self::new(vec);
        while let Some(x) = me.take_unprocessed() {
            me.add_processed(func(x));
        }
    }
    pub fn new(vec: &'a mut Vec<T>) -> Self {
        Self { vec, processed_before: 0 }
    }
    pub fn take_unprocessed(&mut self) -> Option<T> {
        if self.processed_before < self.vec.len() {
            Some(self.vec.swap_remove(self.processed_before))
        } else {
            None
        }
    }
    pub fn add_unprocessed(&mut self, t: T) {
        self.vec.push(t)
    }
    pub fn extend_processed(&mut self, ts: impl IntoIterator<Item = T>) {
        for t in ts {
            self.add_processed(t);
        }
    }
    pub fn add_processed(&mut self, t: T) {
        self.vec.push(t);
        self.processed_before += 1;
        let len = self.vec.len();
        self.vec.swap(len - 1, self.processed_before - 1);
    }
}
