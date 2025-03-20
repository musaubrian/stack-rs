pub mod stac {

    #[derive(Clone)]
    pub struct Node<T> {
        pub value: T,
        // Gotten from https://medium.com/better-programming/learning-rust-building-a-linked-list-102bcb08f93b
        pub prev: Option<Box<Node<T>>>,
    }

    #[derive(Clone)]
    pub struct Stac<T> {
        pub max_size: usize,
        current_size: usize,
        head: Option<Node<T>>,
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

        pub fn push(mut self, value: T) -> Result<Stac<T>, StacErr> {
            self.current_size += 1;
            if self.current_size > self.max_size {
                return Err(StacErr::MaxSizeExceeded);
            }
            let mut node = Node::<T>::new(value);

            match self.head {
                Some(head) => {
                    node.prev = Some(Box::new(head));
                    self.head = Some(node);
                    Ok(self)
                }
                None => {
                    self.head = Some(node);
                    Ok(self)
                }
            }
        }

        pub fn pop(mut self) -> Result<Stac<T>, StacErr> {
            self.current_size -= 1;
            if self.current_size == 0 {
                self.head = None;
                return Ok(self);
            }
            //	head := s.head
            //	s.head = head.prev
            //	Man pop is slightly harder
            match &self.head {
                Some(head) => {
                    let curr_head = head;
                    //self.head = *curr_head.prev;
                    Ok(self)
                }
                None => Err(StacErr::EmptyStac),
            }
        }

        pub fn peek(self) -> Option<Node<T>> {
            return self.head;
        }
    }
}
