mod db;
mod handlers;
mod routes;
mod models;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Initializing the Database
    db::initialize_db();

    // Combining all the routes in the program
    let routes = routes::resto_routes();

    println!("Running the server!!");

    warp::serve(routes.with(warp::trace::request())).run(([127, 0, 0, 1], 3030)).await;
}
