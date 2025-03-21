use hyper::{HeaderMap, header::HeaderValue};
use std::env;
use thunder_rs::http::{error::HttpError, routes::Req};

use paytr::structs::structs::{CallbackRequest, Payment};

use crate::structs;

use structs::{Cart, Request};

pub async fn handle_payment(
    mut req: Req,
    mut headers: HeaderMap,
) -> (Result<serde_json::Value, HttpError>, HeaderMap) {
    headers.insert("Accept", HeaderValue::from_static("application/json"));

    headers.append(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("POST"),
    );

    let merchant_id = match env::var("MERCHANT_ID") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            "Unknown".to_string()
        }
    };

    let merchant_key = match env::var("MERCHANT_KEY") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            "Unknown".to_string()
        }
    };

    let merchant_salt = match env::var("MERCHANT_SALT") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            "Unknown".to_string()
        }
    };

    let result = match req.get_data::<serde_json::Value>().await {
        Ok(Some(data)) => match serde_json::from_value::<Request>(data) {
            Ok(jsonreq) => {
                println!("Request: {:?}", jsonreq);

                let mut basket_data: Vec<Vec<Cart>> = Default::default();

                for item in jsonreq.basket {
                    let cart = Cart {
                        name: item.name,
                        price: item.price,
                        amount: item.amount,
                    };
                    basket_data.push(vec![cart]);
                }

                let merchant_oid = "test1".to_string();

                let mut p = Payment {
                    merchant_id: merchant_id.to_string(),
                    merchant_key: merchant_key.to_string(),
                    merchant_salt: merchant_salt.to_string(),
                    merchant_oid: merchant_oid.clone(),
                    user_ip: "".to_string(),
                    email: jsonreq.email,
                    payment_amount: jsonreq.total_payment,
                    currency: "TL".to_string(),
                    user_basket: "".to_string(),
                    no_installment: 1,
                    max_installment: 0,
                    user_name: jsonreq.username,
                    user_address: jsonreq.user_address,
                    user_phone: jsonreq.user_phone,
                    merchant_ok_url: "https://oguzhandogan.dev".to_string(),
                    merchant_fail_url: "https://example.com".to_string(),
                    test_mode: "1".to_string(),
                    lang: "tr".to_string(),
                    debug_on: 0,
                    timeout_limit: 30,
                    paytr_token: "".to_string(),
                };

                let user_ip = req.get_headers();
                if let Some(user_ip) = user_ip.get("x-real-ip") {
                    p.user_ip = user_ip.to_str().unwrap().to_string();
                } else if let Some(user_ip) = user_ip.get("x-forwarded-for") {
                    p.user_ip = user_ip.to_str().unwrap().to_string();
                } else {
                    println!("User IP: {:?}", "Unknown");
                }
                p.basket_config(&basket_data);

                p.generate_token(p.merchant_key.clone(), p.merchant_salt.clone());

                match p.get_iframe().await {
                    Ok(response) => {
                        let token_response = structs::HttpSuccess {
                            code: 200,
                            message: "Success".to_string(),
                            iframe: response.token.unwrap_or("".to_string()),
                        };

                        println!("Reason: {:?}", response.reason);

                        Ok(serde_json::to_value(token_response).unwrap())
                    }

                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        let error_response = structs::HttpError {
                            code: 400,
                            message: "Failed to get iframe".to_string(),
                        };

                        Ok(serde_json::to_value(error_response).unwrap())
                    }
                }
            }
            Err(e) => {
                eprintln!("Deserialization error: {:?}", e);
                let error_response = structs::HttpError {
                    code: 400,
                    message: "Invalid JSON payload".to_string(),
                };

                Ok(serde_json::to_value(error_response).unwrap())
            }
        },

        Ok(None) => {
            let error_response = structs::HttpError {
                code: 400,
                message: "Bad Request: Empty body".to_string(),
            };

            Ok(serde_json::to_value(error_response).unwrap())
        }

        Err(_e) => {
            let error_response = structs::HttpError {
                code: 400,
                message: "Bad Request: Failed to read request".to_string(),
            };

            Ok(serde_json::to_value(error_response).unwrap())
        }
    };

    (result, headers)
}

pub async fn handle_callback(
    mut req: Req,
    headers: HeaderMap,
) -> (Result<String, HttpError>, HeaderMap) {
    let mut headers = headers;
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.append(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("POST"),
    );
    let result = match req.get_data::<serde_json::Value>().await {
        Ok(Some(data)) => match serde_json::from_value::<CallbackRequest>(data) {
            Ok(mut paytr_request) => {
                let valid =
                    paytr_request.is_valid("merhant_key".to_string(), "merchant_salt".to_string());

                if !valid {
                    //TODO:handle error
                    //
                    println!("Invalid request");
                }

                if paytr_request.status != "success" {
                    println!("Failed payment");
                    return (Ok("OK".to_string()), headers);
                }

                Ok("OK".to_string())
            }

            Err(e) => {
                println!("Deserialization error: {:?}", e);
                Ok("OK".to_string())
            }
        },

        Ok(None) => {
            println!("Empty body");
            Ok("OK".to_string())
        }

        Err(e) => {
            println!("Failed to read request: {:?}", e);
            Ok("OK".to_string())
        }
    };

    (result, headers)
}
