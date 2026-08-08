use newgen::New;

#[derive(New)]
union A {
    x: u32,
    y: f32,
}

fn main() {}
