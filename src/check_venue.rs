use request::Request;
use rustc_serialize::json;
use rustc_serialize::{Encoder, Encodable, Decoder, Decodable};

pub struct CheckVenueResponse{
    pub ok: bool,
    pub venue: String,
    pub error: String
}

impl Decodable for CheckVenueResponse {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 0, |d| {

            let okresponse = try!(d.read_struct_field("ok", 0, |d| Decodable::decode(d)));

            let mut venueresponse:String = "".to_string();
            let mut errorreponse:String= "".to_string();
            if okresponse
            {
                venueresponse = try!(d.read_struct_field("venue", 0, |d| Decodable::decode(d)));
            }
            else
            {
                errorreponse = try!(d.read_struct_field("error", 0, |d| Decodable::decode(d)));
            }

            Ok(CheckVenueResponse{
                ok: okresponse,
                venue: venueresponse,
                error: errorreponse

            })
        })
    }
}

pub fn check_venue(venue: String) -> CheckVenueResponse {

    let request = Request {
            requires_auth: false,
            request_url: format!("https://api.stockfighter.io/ob/api/venues/{}/heartbeat", venue)
        };

    let deserialized: CheckVenueResponse = json::decode(&request.send_request()).unwrap();

    return deserialized;
}
