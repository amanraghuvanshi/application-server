use crate::handlers::{
    create_menu_handler, create_order_handler, create_table_handler, delete_order_item_handler,
    get_order_item_for_table_handler, list_menu_handler, list_order_handler,
    list_order_items_for_table_handler, list_table_handler,
};

// use warp::{ Filter, Rejection, Reply };
use crate::db::get_db_conn;
use rusqlite::Connection;
use std::{clone, convert::Infallible};
use warp::{reject::Rejection, reply::Reply, Filter};

// To handle the errors happening in the routes
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    // error route not found
    if err.is_not_found() {
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error:{:?}", err)),
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error: Failed to deserialize the request body")),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else {
        Ok(warp::reply_with_status(
            warp_reply_json(&format!("Error: {:?}", err)),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

fn with_db() -> impl Filter<Extract = (Connection,), Error = Infallible> + Clone {
    warp::any().map(|| get_db_conn())
}

pub fn list_all_orders_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path("orders")
        .and(warp::get())
        .and(with_db())
        .and_then(|conn| list_order_handler(conn));
}

pub fn create_order_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path("orders" / "create")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|conn, req_body| create_order_handler(conn, req_body));
}

pub fn delete_item_from_order_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    return warp::path("orders" / i64 / "items" / i64)
        .and(warp::delete())
        .and(with_db())
        .and_then(|table_id, menu_id, conn| delete_order_item_handler(conn, table_id, menu_id));
}

pub fn list_tables_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path("tables")
        .and(warp::get())
        .and(with_db())
        .and_then(|conn| list_table_handler(conn));
}

pub fn create_table_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path("tables" / "create")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req_body, conn| create_table_handler(conn, req_body));
}

pub fn list_order_item_for_tables_route(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path("tables" / i64 / "items")
        .and(warp::get())
        .and(with_db())
        .and_then(|table_id, conn| list_order_items_for_table_handler(conn, table_id));
}

pub fn get_item_from_order_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path("tables" / i64 / "items" / i64)
        .and(warp::get())
        .and(with_db())
        .and_then(|table_id, menu_id, conn| get_item_from_order_route(conn, table_id, menu_id));
}

pub fn list_menus_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path("menus")
        .and(warp::get())
        .and(with_db())
        .and_then(|conn| list_menu_handler());
}

pub fn create_menu_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    return warp::path("menus" / "create")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req_body, conn| create_menu_handler(conn, req_body));
}

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
