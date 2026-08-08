use newgen::New;

#[derive(New)]
struct A {
    #[new(default, into)]
    x: u32
}

fn main() {}
