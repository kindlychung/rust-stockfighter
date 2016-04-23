use request::Request;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct CheckApiResponse {
    pub ok: bool,
    pub error: String,
}

pub fn check_api() -> Result<(), String> {

    let request = Request {
        requires_auth: false,
        request_url: "https://api.stockfighter.io/ob/api/heartbeat".to_string(),
    };

    let response = request.send_request();

    let deserialized: CheckApiResponse = json::decode(&response).unwrap();

    if deserialized.ok {
        return Ok(());
    }

    return Err(deserialized.error)
}
