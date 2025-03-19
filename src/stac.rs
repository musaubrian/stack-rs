pub mod stac {
    #[derive(Debug)]
    pub struct Node<T> {
        pub value: T,
        pub prev: Option<Box<Node<T>>>, // Gotten from https://medium.com/better-programming/learning-rust-building-a-linked-list-102bcb08f93b
    }

    pub struct Stac<T> {
        pub max_size: usize,
        current_size: usize,
        head: Option<Node<T>>,
    }

    pub enum StacErr {
        MaxSizeExceeded,
        EmptyStac,
    }
    impl<T> Node<T> {
        pub fn new(value: T) -> Node<T> {
            Node { prev: None, value }
        }
    }

    impl<T> Stac<T> {
        pub fn new(size: usize) -> Stac<T> {
            Stac {
                head: None,
                current_size: 0,
                max_size: size,
            }
        }

        pub fn push(mut self, mut node: Node<T>) -> Result<(), StacErr> {
            self.current_size += 1;
            if self.current_size > self.max_size {
                return Err(StacErr::MaxSizeExceeded);
            }

            match self.head {
                Some(head) => {
                    node.prev = Some(Box::new(head));
                    self.head = Some(node);
                    Ok(())
                }
                None => {
                    self.head = Some(node);
                    Ok(())
                }
            }
        }
        // Returned the pop'd value?
        pub fn pop(mut self) -> Result<(), StacErr> {
            self.current_size -= 1;
            if self.current_size == 0 {
                return Err(StacErr::MaxSizeExceeded);
            }
            Ok(())
        }

        pub fn peek(self) -> Option<Node<T>> {
            return self.head;
        }
    }
}
