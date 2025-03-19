pub mod stac;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use stac::stac;

    use crate::stac::stac::{Node, Stac};

    use super::*;

    #[test]
    fn new_stack() {
        let ss = 2;
        let stac = Stac::<u32>::new(ss);
        assert_eq!(stac.max_size, ss);
    }

    #[test]
    fn push_stack() {
        let ss = 2;
        let stac = Stac::<u32>::new(ss);
        let node = Node::<u32>::new(4);
        let res = stac.push(node);
        assert!(res.is_ok());
    }

    #[test]
    fn pop_stack() {
        let ss = 2;
        let stac = Stac::<u32>::new(ss);
        let node = Node::<u32>::new(4);
        assert!(stac.push(node).is_ok());
    }

    #[test]
    fn peek_stack() {
        let ss = 2;
        let stac = Stac::<String>::new(ss);
        let head = stac.peek();
        assert!(head.is_none());

        stac.push(Node::<String>::new("Yellow".to_string()));
        assert!(head.is_some(), "Expected head to not be None");
    }

    #[test]
    fn max_size_exceeded() {}
}
