use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct HttpSuccess {
    pub code: i16,
    pub message: String,
    pub iframe: String,
}

#[derive(Debug, Serialize)]
pub struct HttpError {
    pub code: i16,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestRequest {
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub basket: Vec<Cart>,
    pub username: String,
    pub email: String,
    pub user_address: String,
    pub user_phone: String,
    pub total_payment: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cart {
    pub name: String,
    pub price: i32,
    pub amount: i32,
}
