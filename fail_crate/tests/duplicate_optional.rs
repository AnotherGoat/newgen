use newgen::New;

#[derive(New)]
struct A {
    #[new(optional, optional)]
    x: u32,
}

fn main() {}
