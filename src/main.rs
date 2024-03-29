use std::convert::Infallible;
use std::time::Duration;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    tokio::time::sleep(Duration::from_secs(5)).await;
    Ok(Response::new(Body::from("Hello World!")))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    // This will never shut down while a connection is open
    // let server = Server::bind(&addr).serve(make_svc);

    // This will finish responding to a client then shut down the connection
    let server = Server::bind(&addr).http2_only(true).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    println!("Listening on http://{}", addr);

    graceful.await?;

    Ok(())
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

