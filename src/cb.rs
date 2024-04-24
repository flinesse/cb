use arrayvec::ArrayVec as Buffer;

pub struct CircBuffer<T: Clone, const CAP: usize> {
    // Backing data store
    store: Buffer<T, CAP>,
    // Read index
    r_idx: usize,
    // Write index
    w_idx: usize,
}

impl<T: Clone, const CAP: usize> CircBuffer<T, CAP> {
    pub fn new() -> Self {
        CircBuffer {
            store: Buffer::new(),
            r_idx: 0,
            w_idx: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        CAP
    }

    pub fn len(&self) -> usize {
        let (delta, _) = self.w_idx.overflowing_sub(self.r_idx);
        delta
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    pub fn push(&mut self, val: T) {
        if self.is_full() {
            self.r_idx += 1;
        }

        self.write(val);
        self.w_idx += 1;
    }

    pub fn pull(&mut self) -> Option<T> {
        if !self.is_empty() {
            let val = self.read().clone();
            self.r_idx += 1;
            Some(val)
        } else {
            None
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        todo!();
    }

    pub fn peek(&self) -> Option<&T> {
        todo!();
    }

    // TODO: Iterator
}

// Private methods to make working with underlying data store a bit nicer
impl<T: Clone, const CAP: usize> CircBuffer<T, CAP> {
    #[inline]
    fn write(&mut self, val: T) {
        if self.store.is_full() {
            self.store[self.w_idx % CAP] = val;
        } else {
            self.store.push(val);
        }
    }

    #[inline]
    fn read(&self) -> &T {
        &self.store[self.r_idx % CAP]
    }
}
