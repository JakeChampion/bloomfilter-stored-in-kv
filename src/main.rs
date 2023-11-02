use fastly::http::StatusCode;
use fastly::{Error, KVStore, Request, Response};
use growable_bloom_filter::GrowableBloom;
use serde_json;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    if let Some(name) = req.get_query_parameter("name") {
        if name.is_empty() {
            Ok(Response::from_status(StatusCode::BAD_REQUEST)
                .with_body("Empty name query parameter"))
        } else {
            let store = KVStore::open("pypi")?.unwrap();
            let names = store.lookup_str("names")?.unwrap();

            let bloom: GrowableBloom = serde_json::from_str(&names).unwrap();
            Ok(Response::from_status(StatusCode::OK)
                .with_body(bloom.contains(name).to_string()))
        }
    } else {
        Ok(
            Response::from_status(StatusCode::BAD_REQUEST)
                .with_body("Missing name query parameter"),
        )
    }
}
