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
        if idx >= self.len() {
            None
        } else {
            Some(self.read_at(idx))
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if !self.is_empty() {
            Some(self.read())
        } else {
            None
        }
    }

    pub fn iter(&self) -> CircBufferIterator<T, CAP> {
        CircBufferIterator::new(self)
    }

    pub fn drain(&mut self) -> CircBufferDrain<T, CAP> {
        CircBufferDrain::new(self)
    }

    pub fn clear(&mut self) {
        self.store.clear();
        self.r_idx = 0;
        self.w_idx = 0;
    }
}

impl<T: Clone, const CAP: usize> Iterator for CircBuffer<T, CAP> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pull()
    }
}

pub struct CircBufferIterator<'cb, T: Clone, const CAP: usize> {
    obj: &'cb CircBuffer<T, CAP>,
    idx: usize,
    len: usize,
}

impl<'cb, T: Clone, const CAP: usize> CircBufferIterator<'cb, T, CAP> {
    pub fn new(obj: &'cb CircBuffer<T, CAP>) -> Self {
        CircBufferIterator {
            obj,
            idx: 0,
            len: obj.len(),
        }
    }
}

impl<'cb, T: Clone, const CAP: usize> Iterator for CircBufferIterator<'cb, T, CAP> {
    type Item = &'cb T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            self.idx += 1;
            Some(self.obj.read_at(self.idx))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.obj.len(), Some(self.obj.len()))
    }
}

pub struct CircBufferDrain<'cb, T: Clone, const CAP: usize> {
    obj: &'cb mut CircBuffer<T, CAP>,
}

impl<'cb, T: Clone, const CAP: usize> CircBufferDrain<'cb, T, CAP> {
    pub fn new(obj: &'cb mut CircBuffer<T, CAP>) -> Self {
        CircBufferDrain { obj }
    }
}

impl<'cb, T: Clone, const CAP: usize> Iterator for CircBufferDrain<'cb, T, CAP> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.obj.pull()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.obj.len(), Some(self.obj.len()))
    }
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

    #[inline]
    fn read_at(&self, offset: usize) -> &T {
        &self.store[(self.r_idx + offset) % CAP]
    }
}
