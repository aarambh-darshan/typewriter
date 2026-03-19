use typebridge::TypeWriter;

#[derive(TypeWriter)]
#[sync_to(ruby)]
pub struct MyStruct {
    name: String,
}

fn main() {}
