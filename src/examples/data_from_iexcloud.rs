use lazy_static::lazy_static;

lazy_static! {
    static ref TOKEN: String = var("IEX_TOKEN").expect("IEX_TOKEN env variable is required");
}

#[tokio::main]
async fn main() {}
