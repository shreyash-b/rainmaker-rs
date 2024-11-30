#![cfg(target_os = "linux")]

static LOGGER_TARGET: &str = "http_server";

use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use std::{net::SocketAddr, sync::mpsc};

use log::error;

use crate::error::Error;
use crate::http::base::*;
use crate::utils::{wrap_in_arc_mutex, WrappedInArcMutex};

impl From<&tiny_http::Method> for HttpMethod {
    fn from(inp: &tiny_http::Method) -> Self {
        match inp {
            tiny_http::Method::Get => HttpMethod::GET,
            tiny_http::Method::Post => HttpMethod::POST,
            _ => unreachable!(),
        }
    }
}

impl From<&mut tiny_http::Request> for HttpRequest {
    fn from(req: &mut tiny_http::Request) -> Self {
        let buf_len = req.body_length().unwrap_or(0);
        let mut buf = vec![0; buf_len];
        req.as_reader().read_exact(&mut buf).unwrap();

        Self {
            method: req.method().into(),
            url: req.url().to_string(),
            data: buf,
        }
    }
}

pub trait HttpEndpointCallback<'a> = Fn(HttpRequest) -> HttpResponse + Send + Sync + 'a;

// http server from esp-idf-svc starts listening as soon as it is initialized and supports registering callback handlers later on
// however tiny_http is a blocking server
// we linux http server with idf by creating a hashmap mutex and spawning tiny_http in a separate thread
type HttpCallbackMethodMapping<'a> = HashMap<HttpMethod, Box<dyn HttpEndpointCallback<'static>>>;
pub struct HttpServerLinux {
    // inner hashmap to store mapping of endpoint method with callback
    // outer hashmap to store mapping to endpoint url with inner hashmap
    callbacks: WrappedInArcMutex<HashMap<String, HttpCallbackMethodMapping<'static>>>,
    execution_thread_handle: Option<JoinHandle<()>>,
    executor_channel: mpsc::Sender<()>,
}

impl HttpServer<HttpServerLinux> {
    pub fn new(config: HttpConfiguration) -> Result<Self, Error> {
        let callbacks: HashMap<String, HttpCallbackMethodMapping<'static>> = HashMap::new();
        let callbacks_mutex = wrap_in_arc_mutex(callbacks);
        let callbacks_mutex_clone = callbacks_mutex.clone();

        let server = tiny_http::Server::http(SocketAddr::new(config.addr, config.port)).unwrap();

        // use a channel to send a dummy data to stop http server
        let (sender, recver) = mpsc::channel::<()>();

        // execute a server in a separate thread
        let executor_joinhandle = thread::spawn(move || {
            let callbacks = callbacks_mutex_clone.to_owned();
            while recver.try_recv().is_err() {
                // untill there is no data in the buffer
                let mut req = match server.recv() {
                    Ok(r) => r,
                    Err(e) => {
                        error!(target: LOGGER_TARGET, "unable to receive http request: {e}");
                        continue;
                    }
                };

                let req_ep = req.url();
                let req_method = req.method();

                let callbacks_lock = callbacks.lock().unwrap();

                let res = match callbacks_lock.get(req_ep) {
                    Some(h) => match h.get(&req_method.into()) {
                        Some(cb) => {
                            // callback exists, execute callback
                            cb(HttpRequest::from(&mut req))
                        }
                        None => HttpResponse::from_bytes("invalid method"),
                    },
                    None => HttpResponse::from_bytes("invalid url"),
                };

                req.respond(tiny_http::Response::from_data(res.get_bytes_vectored()))
                    .unwrap();
            }
        });

        // let executor_joinhandle = thread::spawn(|| {});

        Ok(Self(HttpServerLinux {
            callbacks: callbacks_mutex,
            execution_thread_handle: Some(executor_joinhandle),
            executor_channel: sender,
        }))
    }

    pub fn add_listener<T>(&mut self, path: String, method: HttpMethod, callback: T)
    where
        T: HttpEndpointCallback<'static>,
    {
        // if inner hashmap does not exist for a path, create it
        let mut paths_hmap = self.0.callbacks.lock().unwrap();
        let _ = paths_hmap.try_insert(path.clone(), HashMap::new()); // we can safely ignore the err

        // insert the callback and check for error
        let callbacks_hashmap = paths_hmap.get_mut(&path).unwrap();
        match callbacks_hashmap.try_insert(method, Box::new(callback)) {
            Ok(_) => {
                log::debug!(target: LOGGER_TARGET, "Registered handler for {path}")
            }
            Err(_) => {
                error!(target: LOGGER_TARGET, "handler for {path} for already exists");
            }
        }
    }
}

impl Drop for HttpServerLinux {
    fn drop(&mut self) {
        // send a message to stop the server from listening
        self.executor_channel.send(()).unwrap();

        let join_handle = self.execution_thread_handle.take();
        // wait for thread to gracefully exit
        join_handle.map(|s| s.join());
    }
}
