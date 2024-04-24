mod cb;

#[cfg(test)]
mod tests {
    use crate::cb::CircBuffer;

    #[test]
    fn new() {
        let cb: CircBuffer<i32, 8> = CircBuffer::new();
        assert_eq!(cb.capacity(), 8);
        assert_eq!(cb.len(), 0);
        assert!(cb.is_empty());
    }

    #[test]
    fn push() {
        let mut cb: CircBuffer<char, 4> = CircBuffer::new();
        assert_eq!(cb.capacity(), 4);
        cb.push('a');
        cb.push('α');
        assert_eq!(cb.len(), 2);
        cb.push('∞');
        cb.push('🦀');
        assert_eq!(cb.len(), 4);
        assert!(cb.is_full());
    }

    #[test]
    fn pull() {
        let mut cb: CircBuffer<bool, 4> = CircBuffer::new();
        assert_eq!(cb.capacity(), 4);
        cb.push(true);
        cb.push(false);
        assert_eq!(cb.len(), 2);
        assert_eq!(cb.pull(), Some(true));
        assert_eq!(cb.len(), 1);
        assert_eq!(cb.pull(), Some(false));
        assert_eq!(cb.pull(), None);
        assert_eq!(cb.pull(), None);
        assert_eq!(cb.len(), 0);
        assert!(cb.is_empty());
    }

    #[test]
    fn push_and_pull() {
        let mut cb: CircBuffer<i32, 20> = CircBuffer::new();
        assert_eq!(cb.capacity(), 20);
        cb.push(42);
        assert_eq!(cb.pull(), Some(42));
        assert_eq!(cb.pull(), None);

        for i in 1..=20 {
            cb.push(i);
        }
        assert!(cb.is_full() && cb.len() == 20);
        assert_eq!(cb.pull(), Some(1));
        assert_eq!(cb.pull(), Some(2));
        assert_eq!(cb.len(), 18);
        cb.push(99);
        cb.push(100);
        assert_eq!(cb.len(), 20);
        assert_eq!(cb.pull(), Some(3));
        assert_eq!(cb.pull(), Some(4));
        cb.push(101);
        cb.push(102);
        assert_eq!(cb.len(), 20);

        assert!(cb.is_full());
        cb.push(103);
        assert_eq!(cb.len(), 20);
        assert_eq!(cb.pull(), Some(6));
        assert_eq!(cb.len(), 19);

        cb.clear();
        assert_eq!(cb.len(), 0);
        assert_eq!(cb.capacity(), 20);
    }

    #[test]
    fn get_and_peek() {
        let mut cb: CircBuffer<char, 4> = CircBuffer::new();
        cb.push('a');
        cb.push('α');
        cb.push('∞');
        cb.push('🦀');
        assert_eq!(cb.len(), 4);
        assert!(cb.is_full());
        assert_eq!(*cb.get(0).unwrap(), 'a');
        assert_eq!(*cb.get(1).unwrap(), 'α');
        assert_eq!(*cb.get(2).unwrap(), '∞');
        assert_eq!(*cb.get(3).unwrap(), '🦀');
        assert_eq!(cb.get(4), None);
        assert!(cb.is_full());

        assert_eq!(*cb.peek().unwrap(), 'a');
        assert_eq!(cb.pull(), Some('a'));
        assert_eq!(*cb.peek().unwrap(), 'α');
        assert_eq!(cb.pull(), Some('α'));
        assert_eq!(*cb.peek().unwrap(), '∞');
        assert_eq!(cb.pull(), Some('∞'));
        assert_eq!(*cb.peek().unwrap(), '🦀');
        assert_eq!(cb.pull(), Some('🦀'));
        assert_eq!(cb.peek(), None);
        assert!(cb.is_empty());
    }

    #[test]
    fn iter() {
        let mut cb: CircBuffer<i32, 100> = CircBuffer::new();
        while !cb.is_full() {
            cb.push(10);
        }
        for val in cb {
            assert_eq!(val, 10);
        }

        let mut cb: CircBuffer<i32, 50> = CircBuffer::new();
        while !cb.is_full() {
            cb.push(10);
        }
        for val in cb.iter() {
            assert_eq!(*val, 10);
        }
        assert_eq!(cb.len(), 50);
        assert!(cb.is_full());

        let mut cb: CircBuffer<i32, 20> = CircBuffer::new();
        for i in 0..100 {
            cb.push(i);
        }
        for (i, val) in cb.drain().enumerate() {
            assert_eq!(i as i32 + 80, val);
        }
        assert_eq!(cb.len(), 0);
        assert!(cb.is_empty());
    }
}
