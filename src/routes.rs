use crate::handlers::{
    create_order_handler,
    list_table_handler,
    create_table_handler,
    list_menu_handler,
    create_menu_handler,
    list_order_handler,
    delete_order_item_handler,
    list_order_items_for_table_handler,
    get_order_item_for_table_handler,
};

// use warp::{ Filter, Rejection, Reply };
use rusqlite::Connection;
use crate::db::get_db_conn;
use std::convert::Infallible;
use warp::{ reject::Rejection, reply::Reply, Filter };

// To handle the errors happening in the routes
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    // error route not found
    if err.is_not_found() {
        Ok(
            warp::reply::with_status(
                wrap::reply::json(&format!("Error:{:?}", err)),
                wrap::http::StatusCode::NOT_FOUND
            )
        )
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        Ok(
            warp::reply::with_status(
                warp::reply::json(&format!("Error: Failed to deserialize the request body")),
                warp::http::StatusCode::BAD_REQUEST
            )
        )
    } else {
        Ok(
            warp::reply_with_status(
                warp_reply_json(&format!("Error: {:?}", err)),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR
            )
        )
    }
}

fn with_db() -> impl Filter<Extract = (Connection,), Error = Infallible> + Clone {
    warp::any().map(|| get_db_conn())
}

pub fn list_all_orders_routes() {}

pub fn create_order_routes() {}

pub fn delete_item_from_order_route() {}

pub fn list_tables_routes() {}

pub fn create_table_routes() {}

pub fn list_order_item_for_tables_route() {}

pub fn get_item_from_order_route() {}

pub fn list_menus_routes() {}

pub fn create_menu_routes() {}

pub fn resto_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let routes = create_order_routes()
        .or(create_table_routes())
        .or(create_menu_routes())
        .or(list_tables_routes())
        .or(list_menus_routes())
        .or(list_all_orders_routes())
        .or(delete_item_from_order_route())
        .or(list_order_item_for_tables_route())
        .or(get_item_from_order_route());

    routes.recover(handle_rejection)
}
