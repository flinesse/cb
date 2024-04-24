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
        assert!(cb.is_empty());
    }

    #[test]
    fn push_and_pull() {
        // TODO: multiple push and pulls
        // TODO: push after full
    }

    #[test]
    fn get_and_peek() {}

    #[test]
    fn iter() {}
}
