use serde;
use rusqlite::params;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    #[serde(skip)]
    pub id: i64,
    pub code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TableResponse {
    pub id:i64,
    pub code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Menu {
    #[serde(skip)]
    pub id:i64,
    pub name:String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuResponse {
    pub id:i64,
    pub name:String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRequestBody {
    pub table_id : i64,
    pub menu_id : Vec<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponseBody {
    pub id: i64,
    pub table_id: i64,
    pub table_name: String,
    pub total_cooking_time: i32,
    pub menus: Vec<OrderItemResponse>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    #[serde(skip)]
    pub id: i64,
    pub order_id: i64,
    pub menu_id: i64,
    pub cooking_time: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemResponse {
    pub id: i64,
    pub order_id: i64, 
    pub menu_id: i64, 
    pub menu_name: String,
    pub cooking_time: i64, 
    pub quantity: i64,
}

impl Table{

}

impl Menu{

}

impl OrderResponseBody{

}

impl OrderItem {
    
}