use newgen::New;

#[derive(New)]
struct A {
    #[new(into_iter)]
    x: u32
}

fn main() {}
