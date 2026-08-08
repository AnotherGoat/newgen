use newgen::New;

#[derive(New)]
struct A {
    #[new(default)]
    x: (),
}

fn main() {}
