pub mod account;
pub mod risk_engine;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};
use hyper::Method;

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct QueryStringExtractor {
    userid: usize,
    currency: account::OrderType,
    amount: f64,
}
#[derive(Serialize)]
struct WithdrawalResult {
    result: risk_engine::WithdrawalStatus
}

fn post_withdrawal_handler(&mut state:State,&mut risk_engine:risk_engine::RiskEngine)->(State,(mime::Mime,Vec<u8>){
 let res = {
     let query_param = QueryStringExtractor::take_from(&mut state);
 }
}
fn router()->Router{
    build_simple_router(|route| {
        route.post("/api/v1/withdrawBalance").to(post_withdrawal_handler);
    })
}


fn main() {
    println!("Hello, world!");
}
