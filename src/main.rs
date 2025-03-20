pub mod stac;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use crate::stac::stac::Stac;

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
        let res = stac.push(4);
        assert!(res.is_ok());
    }
    #[test]
    fn pop_empty_stack() {
        let mut stac = Stac::<u8>::new(2);
        let pop = stac.pop();
        assert!(pop.is_err())
    }

    #[test]
    fn pop_non_empty_stack() {
        let ss = 2;
        let stac = Stac::<u32>::new(ss);
        let pushed_st = stac.push(4);
        assert!(pushed_st.is_ok());
        let popd = pushed_st.pop();

        // TODO: Same as here
        //assert!(stac.pop().is_ok());
    }

    #[test]
    fn peek_empty_stack() {
        let ss = 2;
        let stac = Stac::<String>::new(ss);
        let head = stac.peek();
        assert!(head.is_none());
    }
    #[test]
    fn peek_non_empty_stack() {
        // TODO: Figure out how to get it to not
        // move, tried to use reference but it somehow broke
        //
        //
        //
        //let ss = 2;
        //let mut stac = Stac::<String>::new(ss);
        //let push = stac.push("Yellow".to_string());
        //assert!(push.is_ok());
        //let head = stac.peek();
        //assert!(head.is_some())
        assert!(false)
    }

    #[test]
    fn max_size_exceeded() {
        let ss = 0;
        let stac = Stac::<String>::new(ss);
        let head = stac.push("Hello".to_string());
        assert!(head.is_err());
    }
}
