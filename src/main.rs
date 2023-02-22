use std::fmt;

enum List {
    Next(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, elem: u32) -> List {
        List::Next(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            List::Next(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            List::Next(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            List::Nil => format!("Nil"),
        }
    }
}

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

    struct ListFmt(Vec<i32>);
    impl fmt::Display for ListFmt {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;

            for (idx, v) in vec.iter().enumerate() {
                write!(f, "{}: {}, ", idx, v)?;
            }

            write!(f, "")
        }
    }
    println!("ListFmt: {}", ListFmt(vec![1]));

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("list: {}", list.stringify());
    println!("list len: {}", list.len())
}
