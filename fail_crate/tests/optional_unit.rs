use newgen::New;

#[derive(New)]
struct A {
    #[new(optional)]
    x: (),
}

fn main() {}
