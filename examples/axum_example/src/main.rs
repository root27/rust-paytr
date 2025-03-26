use axum::{
    Json, Router,
    http::{HeaderMap, StatusCode},
    routing::post,
};

use axum_client_ip::{SecureClientIp, SecureClientIpSource};

use dotenv::dotenv;
use paytr::structs::structs::Payment;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let app = Router::new()
        .route("/payment", post(payment_handler))
        .layer(SecureClientIpSource::ConnectInfo.into_extension());

    println!("Server running on port 3000");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn payment_handler(
    SecureClientIp(ip): SecureClientIp,

    headers: HeaderMap,

    Json(payload): Json<UserRequest>,
) -> (StatusCode, Json<TokenResponse>) {
    let merchant_id = match env::var("merchantId") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            "Unknown".to_string()
        }
    };

    let merchant_key = match env::var("merchantKey") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            "Unknown".to_string()
        }
    };

    let merchant_salt = match env::var("merchantSalt") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            "Unknown".to_string()
        }
    };

    let merchant_ok_url = match env::var("okurl") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            "Unknown".to_string()
        }
    };

    let merchant_fail_url = match env::var("failurl") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            "Unknown".to_string()
        }
    };

    let user_ip = headers
        .get("x-real-ip")
        .and_then(|val| val.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| {
            headers
                .get("x-forwarded-for")
                .and_then(|val| val.to_str().ok())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| ip.to_string());

    let merchant_oid = "ORD0001".to_string();

    let mut payment = Payment {
        merchant_id: merchant_id.clone(),
        merchant_key: merchant_key.clone(),
        merchant_salt: merchant_salt.clone(),
        user_ip: user_ip.clone(),
        merchant_oid,
        email: payload.email.clone(),
        payment_amount: payload.total_payment,
        currency: "TL".to_string(),
        user_basket: "".to_string(),
        no_installment: 1,
        max_installment: 0,
        paytr_token: "".to_string(),
        user_name: payload.username.clone(),
        user_address: payload.user_address.clone(),
        user_phone: payload.user_phone.clone(),
        merchant_ok_url: merchant_ok_url.clone(),
        merchant_fail_url: merchant_fail_url.clone(),
        test_mode: "1".to_string(),
        debug_on: 0,
        timeout_limit: 30,
        lang: "tr".to_string(),
    };
    let mut basket_data: Vec<Vec<Cart>> = Default::default();

    for item in payload.basket {
        let cart = Cart {
            name: item.name,
            price: item.price,
            amount: item.amount,
        };
        basket_data.push(vec![cart]);
    }
    payment.basket_config(&basket_data);

    payment.generate_token(merchant_key, merchant_salt);

    match payment.get_iframe().await {
        Ok(response) => {
            println!("Response: {:?}", response);

            let res = TokenResponse {
                code: 200,
                message: "Token generated successfully".into(),
                iframe: response.token.unwrap(),
            };

            (StatusCode::OK, Json(res))
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TokenResponse {
                    code: 500,
                    message: "Token generation failed".into(),
                    iframe: "".into(),
                }),
            )
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    code: u16,
    message: String,
    iframe: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRequest {
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
