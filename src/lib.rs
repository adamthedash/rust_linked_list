mod single;


#[cfg(test)]
mod tests {
    use crate::single::LinkedList;

    #[test]
    fn zero_length() {
        let mut ll = LinkedList::new();
        assert_eq!(ll.len(), 0);
        assert!(ll.is_empty());

        // Inserts
        ll.append(1);
        assert_eq!(ll.len(), 1);

        let mut ll = LinkedList::new();
        ll.insert(1, 0).unwrap();
        assert_eq!(ll.len(), 1);

        // Pops
        let mut ll = LinkedList::<i32>::new();
        assert!(ll.pop(0).is_err());
        assert!(ll.pop(1).is_err());

        // Get
        assert!(ll.get_value(0).is_err());
        assert!(ll.get_value(1).is_err());
    }

    #[test]
    fn one_length() {
        fn new() -> LinkedList<i32> {
            let mut ll = LinkedList::new();
            ll.append(1);
            ll
        }

        let mut ll = new();
        assert_eq!(ll.len(), 1);
        assert!(!ll.is_empty());

        // Inserts
        assert!(ll.insert(1, 0).is_ok());
        assert_eq!(ll.len(), 2);

        let mut ll = new();
        assert!(ll.insert(1, ll.len()).is_ok());
        assert_eq!(ll.len(), 2);

        assert!(ll.insert(1, 20).is_err());

        // Pops
        let mut ll = LinkedList::<i32>::new();
        ll.append(1);
        assert!(ll.pop(1).is_err());
        assert_eq!(ll.pop(0), Ok(1));
        assert_eq!(ll.len(), 0);

        // Get value
        let mut ll = LinkedList::<i32>::new();
        ll.append(1);
        assert_eq!(ll.get_value(0), Ok(&1));
        assert!(ll.get_value(1).is_err());
    }

    #[test]
    fn many_length() {
        let mut ll = LinkedList::new();
        for i in 0..5 {
            ll.append(i);
        }

        // Swap
        ll.swap(0, 1).unwrap();
        assert_eq!(ll.get_value(0), Ok(&1));
        assert_eq!(ll.get_value(1), Ok(&0));
    }
}
