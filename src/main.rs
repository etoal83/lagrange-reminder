use std::net::SocketAddr;
use serde_derive::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

#[tokio::main]
async fn main() {
    // pretty_env_logger::init();

    // GET /
    let hello_world = warp::path::end().map(|| "Hello, World at root!");

    // GET /hi
    let hi = warp::path("hi").map(|| "Hello, World!");

    // GET /hello/from/warp
    let hello_from_warp = warp::path!("hello" / "from" / "warp").map(|| "Hello from warp!");

    // GET /sum/:u32/u32
    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));

    // GET /:u16/times/:u16
    let times = warp::path!(u16 / "times" / u16).map(|a, b| format!("{} x {} = {}", a, b, a * b));

    // GET /math
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16
    let math = warp::path("math").and(sum.or(times));
    let help = warp::path("math")
        .and(warp::path::end())
        .map(|| "This is the Math API. Try calling /math/sum/:u32/:u32 or /math/:u16/times/:u16");
    let math = help.or(math);

    let sum = sum.map(|output| format!("(This route has moved to /math/sum/:u32/:u32) {}", output));
    let times = times.map(|output| format!("(This route has moved to /math/:u16/times/:u16) {}", output));

    // GET /bye/:string
    let bye = warp::path("bye")
        .and(warp::path::param())
        .map(|name: String| format!("Good bye, {}!", name));

    // Combine all the defined routes into the single API
    let _routes = warp::get().and(
        hello_world
            .or(hi)
            .or(hello_from_warp)
            .or(bye)
            .or(math)
            .or(sum)
            .or(times),
    );

    // POST /employees/:rate  {"name": "Henrietta", "rate": 3}
    let _promote = warp::post()
        .and(warp::path("employees"))
        .and(warp::path::param::<u32>())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|rate, mut employee: Employee| {
            employee.rate = rate;
            warp::reply::json(&employee)
        });

    // A server that requires header conditions
    // - `Host` is a `SocketAddr`
    // - `Accept` is exactly `*/*`
    //
    let host = warp::header::<SocketAddr>("host");
    let accept_stars = warp::header::exact("accept", "*/*");

    let routes = host
        .and(accept_stars)
        .map(|addr| format!("accepting stars on {}", addr));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
