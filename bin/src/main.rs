use salvo::{endpoint, Response};

fn main() {
    println!("Hello, world!");
}
#[endpoint]
async fn hello(res: &mut Response) {
	res.render("Hello");
}
