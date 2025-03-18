use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Payment {
    pub merchant_id: String,
    pub merchant_key: String,
    pub merchant_salt: String,
    pub user_ip: String,
    pub merchant_oid: String,
    pub email: String,
    pub payment_amount: i64,
    pub currency: String,
    pub user_basket: String,
    pub no_installment: i64,
    pub max_installment: i64,
    pub paytr_token: String,
    pub user_name: String,
    pub user_address: String,
    pub user_phone: String,
    pub merchant_ok_url: String,
    pub merchant_fail_url: String,
    pub test_mode: String,
    pub debug_on: i8,
    pub timeout_limit: i64,
    pub lang: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaytrResponse {
    pub status: Option<String>,
    pub token: Option<String>,
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackRequest {
    pub installment_count: i16,
    pub merchant_id: String,
    pub merchant_oid: String,
    pub status: String,
    pub total_amount: i64,
    pub hash: String,
    pub fail_reason_code: i16,
    pub fail_reason_message: String,
    pub test_mode: String,
    pub payment_type: String,
    pub currency: String,
    pub payment_amount: i64,
}
