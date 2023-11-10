use controller::SomeQueryStruct;
use warp::Filter;

mod controller;

#[tokio::main]
async fn main() {
    // GET /greetings/hello
    let greetings_base_route = warp::path("greetings");
    let hello_world_route = greetings_base_route
        .and(warp::get())
        .and(warp::path!("hello"))
        .map(|| "Hello World");

    let greeting_router = warp::any().and(hello_world_route);

    // POST /foo/bar
    let foobar_base_route = warp::path("foo");
    let foobar_route = foobar_base_route
        .and(warp::post())
        .and(warp::path!("bar" / String / usize))
        .and(warp::query::<SomeQueryStruct>())
        .and(warp::body::json())
        .and_then(controller::foo_controller_handler);

    let foo_router = warp::any().and(foobar_route);

    let routers = warp::any().and(greeting_router).or(foo_router);

    warp::serve(routers).run(([0, 0, 0, 0], 3030)).await;
}
