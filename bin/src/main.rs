use salvo::{prelude::ToSchema};
use serde::{Deserialize, Serialize};
fn main() {
    println!("Hello, world!");
}

#[derive(ToSchema,Deserialize,Serialize)]
struct  A{
    a:String,
}
//work #[derive(Deserialize,Serialize)]
//not work
#[derive(ToSchema,Deserialize,Serialize)]

struct B{
    #[serde(flatten)]
    a:A,
    b:String,
}