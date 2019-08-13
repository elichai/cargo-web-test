use std::os::raw::*;

extern "C" {
    pub fn do_something(input: *const c_uchar) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let b = 7;
        let a = unsafe { do_something(&b) };
        println!("{:?}", b);
        assert_eq!(a, 7);

        assert_eq!(2 + 2, 4);
    }
}
