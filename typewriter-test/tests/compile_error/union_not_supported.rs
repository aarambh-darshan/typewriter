use typebridge::TypeWriter;

#[derive(TypeWriter)]
#[sync_to(typescript)]
pub union MyUnion {
    a: u32,
    b: f64,
}

fn main() {}
