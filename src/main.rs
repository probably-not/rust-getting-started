use std::fmt;

fn main() {
    println!("Hello {}", "there");
    println!("Hello {1} {0}", "there", "wow");
    println!("Hello {name}", name = "there");

    struct Nope(i32);
    impl fmt::Display for Nope {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug)]
    struct Yup(i32);
    println!("Nope: {}", Nope(1));
    println!("Yup: {:?}", Yup(1));
}
