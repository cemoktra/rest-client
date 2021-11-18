use async_trait::async_trait;
use rest_client::Endpoint;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CatResult {
    id: u32,
    url: String,
    webpurl: String,
    x: f64,
    y: f64
}

struct CatAPI;

#[async_trait]
impl Endpoint for CatAPI {
    type RequestBody = ();
    type ResponseBody = CatResult;
    type ErrorBody = ();

    fn endpoint(&self) -> &str {
        "/catapi/rest"
    }
}

#[tokio::main]
async fn main() {
    let endpoint = CatAPI;
    let base_uri = "https://thatcopy.pw/".parse().unwrap();
    let response = endpoint.call(&base_uri).await;

    match response {
        Ok(response) => {
            println!("{:#?}", response);
        },
        Err(err) => {
            println!("{:?}", err);
        },
    };
}