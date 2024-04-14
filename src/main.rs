mod request_helpers;
use request_helpers::request_helpers::make_request;

#[tokio::main]
async fn main() {
    let result = make_request();
}
