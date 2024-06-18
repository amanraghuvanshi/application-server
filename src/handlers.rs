// IMporting all the modesl from the models.rs
use crate::models::{
    Table,
    Menu,
    TableResponse,
    MenuResponse,
    OrderItem,
    OrderItemResponse,
    OrderResponseBody,
    OrderRequestBody,
};
use rusqlite::Connection;
use warp;
use rand::Rng;
use rusqlite::Params;
use serde_json::json;

pub fn list_table_handler(conn: Connection) -> Result<impl warp::Reply, warp::Rejection> {
    match Table::list(&conn) {
        Ok(tables) => {
            Ok(warp::reply::with_status(warp::reply::json(&tables), warp::http::StatusCode::OK))
        }
        Err(_err) => {
            Ok(
                warp::reply::with_status(
                    warp::reply::json::<Vec<TableResponse>>(&vec![]),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR
                )
            )
        }
    }
}
pub fn create_table_handler() {}
pub fn create_order_handler() {}

pub fn list_menu_handler() {}
pub fn create_menu_handler() {}
pub fn list_order_handler() {}
pub fn delete_order_item_handler() {}
pub fn list_order_items_for_table_handler() {}
pub fn get_order_item_for_table_handler() {}
