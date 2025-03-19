pub mod stac {

    pub struct Node<T> {
        pub value: T,
        // Gotten from https://medium.com/better-programming/learning-rust-building-a-linked-list-102bcb08f93b
        pub prev: Option<Box<Node<T>>>,
    }

    pub struct Stac<T> {
        pub max_size: usize,
        current_size: usize,
        head: Option<Box<Node<T>>>,
    }

    pub enum StacErr {
        MaxSizeExceeded,
        EmptyStac,
        Welp,
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

        pub fn push(mut self, value: T) -> Result<(), StacErr> {
            self.current_size += 1;
            if self.current_size > self.max_size {
                return Err(StacErr::MaxSizeExceeded);
            }
            let mut node = Node::<T>::new(value);

            match self.head {
                Some(head) => {
                    node.prev = Some(head);
                    self.head = Some(Box::new(node));
                    Ok(())
                }
                None => {
                    self.head = Some(Box::new(node));
                    Ok(())
                }
            }
        }

        pub fn pop(&mut self) -> Result<(), StacErr> {
            self.current_size -= 1;
            if self.current_size == 0 {
                self.head = None;
                return Err(StacErr::MaxSizeExceeded);
            }

            Err(StacErr::Welp)
        }

        pub fn peek(self) -> Option<Box<Node<T>>> {
            return self.head;
        }
    }
}
