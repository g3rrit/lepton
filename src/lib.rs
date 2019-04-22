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
        let mut input = Input::new("test_files/test.txt");
        
        assert!(Input::exists("src/lib.rs"));
        assert!(!Input::exists("rc/lib.rs"));
        
        comp_warn!("test {} lol {} sdf", 14, "sd");
        
        println!("read: {}", input.next().unwrap());
        println!("read: {}", input.next().unwrap());
        println!("read: {}", input.next().unwrap());
        
        input.rewind(2);
        
        println!("read: {}", input.next().unwrap());
        input.commit();
        
        input.rewind(1);
        
        println!("read: {}", input.next().unwrap());

        assert_eq!(2 + 2, 4);
    }
}
