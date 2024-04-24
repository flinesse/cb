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
    /// Constructs a new, empty `CircBuffer<T>` with capacity `CAP`.
    pub fn new() -> Self {
        CircBuffer {
            store: Buffer::new(),
            r_idx: 0,
            w_idx: 0,
        }
    }

    /// Returns the capacity of the buffer.
    pub fn capacity(&self) -> usize {
        CAP
    }

    /// Returns the current number of elements queued and ready to be read.
    pub fn len(&self) -> usize {
        self.w_idx - self.r_idx
    }

    /// Returns whether the buffer has items available to be dequeued. (`len() == 0`)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns whether the buffer has reached its capacity. (`len() == capacity()`)
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// Pushes `val` onto the buffer. If the capacity is reached, this wraps around and overwrites the oldest data.
    // FIX: In the unlikely event that `push()` or `pull()` is invoked
    //      `usize::MAX` number of times and the internal write or read
    //      indices wrap, `CircBuffer` may become invalid
    pub fn push(&mut self, val: T) {
        if self.is_full() {
            self.r_idx += 1;
        }

        self.write(val);
        self.w_idx += 1;
    }

    /// Pulls the next item to be read off the head of the queue. Returns `None` if the queue is empty.
    // FIX: In the unlikely event that `push()` or `pull()` is invoked
    //      `usize::MAX` number of times and the internal write or read
    //      indices wrap, `CircBuffer` may become invalid
    pub fn pull(&mut self) -> Option<T> {
        if !self.is_empty() {
            let val = self.read().clone();
            self.r_idx += 1;
            Some(val)
        } else {
            None
        }
    }

    /// Returns a reference to the item at `idx`, where `idx` is the offset to the next item to be read. (0 <= `idx` < len())
    pub fn get(&self, idx: usize) -> Option<&T> {
        if !self.is_empty() && idx < self.len() {
            Some(self.read_at(idx))
        } else {
            None
        }
    }

    /// Returns a reference to the next item to be read, `None` if the queue is empty. (equivalent to `get(0)`)
    pub fn peek(&self) -> Option<&T> {
        if !self.is_empty() {
            Some(self.read())
        } else {
            None
        }
    }

    /// Returns an iterator over the references of elements on the queue.
    pub fn iter(&self) -> CircBufferIterator<T, CAP> {
        CircBufferIterator::new(self)
    }

    /// Returns an iterator over elements on the queue, dequeuing them as the iterator yields.
    pub fn drain(&mut self) -> CircBufferDrain<T, CAP> {
        CircBufferDrain::new(self)
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.store.clear();
        self.r_idx = 0;
        self.w_idx = 0;
    }

    /* Below are private methods to make working with the underlying data store (`ArrayVec`) a bit nicer */

    #[inline]
    fn read(&self) -> &T {
        &self.store[self.r_idx % CAP]
    }

    #[inline]
    fn read_at(&self, offset: usize) -> &T {
        &self.store[(self.r_idx + offset) % CAP]
    }

    #[inline]
    fn write(&mut self, val: T) {
        if self.store.is_full() {
            self.store[self.w_idx % CAP] = val;
        } else {
            self.store.push(val);
        }
    }
}

// Consuming iterator over `CircBuffer`
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

// Non-consuming and non-draining iterator over `CircBuffer`
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

// Non-consuming and draining iterator over `CircBuffer`
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
