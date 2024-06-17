mod db;
mod handlers;
mod routes;
mod models;

#[tokio::main]
async fn main() {
    db::initialize_db();
    let routes = routes::resto_routes();

    println!("Running the server!!");

    warp::serve(routes.with(warp::trace::request())).run(([127, 0, 0, 1], 3030)).await;
}
