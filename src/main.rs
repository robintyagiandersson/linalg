

#[derive(Debug)]
struct Foo {
    // placeholder
    x: i32,
    y: i32,
}

impl Foo {
    // placeholder
    fn some_sum(&self) -> i32 {
        self.x + self.y
    }
}

fn main() {
    let foo = Foo { x: 5, y: 4};
    println!("Got {:?}", foo);
    println!("And got {}", foo.some_sum());
}
