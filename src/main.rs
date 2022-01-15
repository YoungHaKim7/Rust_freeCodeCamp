use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(2).unwrap();
    let second = args.nth(3).unwrap();
    println!("{:?} {} {}", first, operator, second);
}

fn nth(&mut self, n: usize) -> Option<String> {
    // assume n = 0;
    // inner = ["1", "2"]
    self.inner.next() // "1"
    // Calling next again results in second element
    self.inner.next() // "2"
}
