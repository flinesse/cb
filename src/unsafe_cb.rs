use std::mem::MaybeUninit;

pub struct CircBuffer<T, const CAP: usize> {
    // Backing data store
    // `MaybeUninit<T>` is roughly equivalent to `Option<T>` sans the discriminant
    store: [MaybeUninit<T>; CAP],
    // Read index
    r_idx: usize,
    // Write index
    w_idx: usize,
}

impl<T, const CAP: usize> CircBuffer<T, CAP> {
    pub fn new() -> Self {
        CircBuffer {
            // SAFETY: https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
            store: unsafe { MaybeUninit::uninit().assume_init() },
            r_idx: 0,
            w_idx: 0,
        }
    }

    // TODO
}

// TODO
