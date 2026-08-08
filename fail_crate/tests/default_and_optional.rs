use newgen::New;

#[derive(New)]
struct A {
    #[new(default, optional)]
    x: u32,
}

fn main() {}
