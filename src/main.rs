pub mod account;
pub mod risk_engine;
pub mod file_loader;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate futures;
extern crate kafka;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use std::thread;
use futures::{future, Future, Stream};
use gotham::helpers::http::response::{create_response,create_empty_response};
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::handler::{HandlerFuture, IntoResponse, IntoHandlerError};
use gotham::state::{FromState, State};
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;
use hyper::{Body, Response, StatusCode};
use std::sync::{Arc, Mutex,mpsc};

//allows for interthread communication between Kafka client and Rest Client
pub struct ThreadManager {
    rest: RestServer,
    worker: Worker,
}
// Worker for interthread communication
struct Worker {
    thread: std::thread::JoinHandle<()>,
    rest: Arc<Mutex<risk_engine::RiskEngine>>,
}

impl Worker {
    fn new(rx:Arc<Mutex<mpsc::Receiver<String>>>, rest: Arc<Mutex<risk_engine::RiskEngine>>)-> Self {
        let r2 = rest.clone(); //need to clone to allow for use in worker thread
        let thread = thread::spawn(move || {
            loop {
                let msg = rx.lock().unwrap().recv().unwrap();
                if msg.len() > 0 {
                    let mut guard = r2.lock();
                    let eng = guard.as_mut().unwrap();
                    let mut settlement: risk_engine::CompletedTransaction = serde_json::from_str(&msg).unwrap();
                    println!("Settlement: {:?}", &settlement);
                    eng.process_settlement(&mut settlement);
                    // let acc = eng.get_account(100).unwrap();
                    // println!("Balance: {:?}", acc.get_balances());
                }
                
            }
        });

        Worker {
            rest,
            thread,
        }
    }
}

#[derive(Clone,StateData)]
pub struct RestServer {
    engine: Arc<Mutex<risk_engine::RiskEngine>>
}

impl RestServer {
    fn new()->Self {
        Self {
            engine: Arc::new(Mutex::new(risk_engine::RiskEngine::new()))
        }
    }
}

#[derive(Serialize)]
struct WithdrawalResult {
    result: risk_engine::WithdrawalStatus
}

impl IntoResponse for WithdrawalResult {
    fn into_response(self,state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(&self).expect("serialized withdrawal result"),
        )
    }
}

fn post_handler(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                println!("Body: {}", body_content);
                let srv = RestServer::borrow_mut_from(&mut state);
                {
                let mut guard = srv.engine.lock();
                let eng = guard.as_mut().unwrap();
                let obj: risk_engine::WithdrawTransaction = serde_json::from_str(&body_content).unwrap();
                let with_res = eng.process_withdrawal(obj);
                drop(guard);
                match with_res {
                    Ok(res) =>{
                        let result = WithdrawalResult {
                            result: res
                        };
                        let response = result.into_response(&state);
                        return future::ok((state,response))
                    },
                    Err(e) => panic!(e)
                }}
                // let res = create_empty_response(&state, StatusCode::OK);
                // future::ok((state, res))
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });
    Box::new(f)
}
fn router(server:RestServer)->Router{
    // loads data from json to populate initial state
    let mut accounts = file_loader::load_dataset().unwrap();
    {
        let mut engine_guard = server.engine.lock();
        let engine = engine_guard.as_mut().unwrap();
        for account in accounts {
        engine.put_account(account);
        }
    }

    //Middleware for rest
    let middleware = StateMiddleware::new(server);
    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain,pipelines, |route| {
        route.post("/api/v1/withdrawBalance").to(post_handler);
    })
}


fn main() {
    let (tx,rx) = mpsc::channel();
    let mut server = RestServer::new();
    let mut container = ThreadManager {
        rest: server.clone(),
        worker: Worker::new(Arc::new(Mutex::new(rx)),server.engine.clone()),
    };
    let tx1 = tx.clone();
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    // spawn thread to handle kafka client
    let handle = thread::spawn(move || {        
        let mut consumer =
        Consumer::from_hosts(vec!("localhost:9092".to_owned()))
            .with_topic("test".to_owned())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_group("my-group".to_owned())
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
            .unwrap();
        loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
            let msg = String::from_utf8(m.value.to_vec());
            // let json: risk_engine::CompletedTransaction = serde_json::from_slice(m.value).unwrap();
            // println!("JSON: {:?}",&json);
            // sends message to worker for consumption
            tx.send(msg.unwrap());
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
        }
    });
    let goth = gotham::start(addr, router(container.rest));
    handle.join().unwrap();
}
