use newgen::New;

#[derive(New)]
struct A {
    #[new()]
    x: u32,
}

fn main() {}
