extern crate hyper;
extern crate hyper_native_tls;
extern crate iron;
extern crate router;
extern crate serde_json;

use std::collections::HashMap;
use std::io::Read;
use std::convert::From;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::UserAgent;

use iron::prelude::*;
use iron::{Handler};
use iron::status;

use router::Router;

use serde_json::{Value, Error};

fn main() {
    let mut router = Router::new();

    router.get("/", handler, "index");

    fn handler(req: &mut Request) -> IronResult<Response> {
        let url = "https://api.github.com";
        let mut data = String::new();
        
        
        
        let tls = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(tls);
        let client = Client::with_connector(connector);
        
        let mut res = client.get(url)
                            .header(UserAgent("string".to_string()))
                            .send()
                            .unwrap()
                            .read_to_string(&mut data)
                            .unwrap();        
        println!("{}", data);
        
        let json: Value = serde_json::from_str(&data).unwrap();
        println!("{}", json);
        let elem = json.get("authorizations_url").unwrap().to_string();
        println!("{}", elem);
    
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, elem)))
    }
    
    let _server = Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}