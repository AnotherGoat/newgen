use newgen::New;

#[derive(New)]
struct A {
    #[new(optional, into)]
    x: u32
}

fn main() {}
