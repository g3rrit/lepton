#![allow(dead_code)]

#[macro_use]
mod util;
mod input;
mod token;
mod lexer;


#[cfg(test)]
mod tests {
    use input::*;
    use util::*;

    #[test]
    fn test() {
        let input = Input::new("src/lib.rs");
        
        assert!(Input::exists("src/lib.rs"));
        assert!(!Input::exists("rc/lib.rs"));
        
        comp_warn!("test {} lol {} sdf", 14, "sd");

        assert_eq!(2 + 2, 4);
    }
}
