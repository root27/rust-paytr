use crate::structs::structs::{CallbackRequest, Payment, PaytrResponse};
use base64::engine::{Engine as _, general_purpose};
use hmac::{Hmac, Mac};
use itoa::Buffer;
use reqwest::Client;
use serde_json;
use sha2::Sha256;

impl Payment {
    pub fn basket_config<T: serde::Serialize>(&mut self, cart: &[Vec<T>]) {
        if let Ok(json) = serde_json::to_vec(cart) {
            self.user_basket = general_purpose::STANDARD.encode(json);
        } else {
            panic!("Failed to serialize basket");
        }
    }

    pub fn generate_token(&mut self, merchant_key: String, merchant_salt: String) {
        let mut buffer = Buffer::new();

        let hash_string = self.merchant_id.clone()
        + &self.user_ip
        + &self.merchant_oid
        + &self.email
        + buffer.format(self.payment_amount)  // Fix order: Move payment_amount before user_basket
        + &self.user_basket
        + buffer.format(self.no_installment)
        + buffer.format(self.max_installment)
        + &self.currency
        + &self.test_mode;

        let pay_token = hash_string + &merchant_salt; // Append merchant_salt at the end

        let mut hmac_token = Hmac::<Sha256>::new_from_slice(merchant_key.as_bytes())
            .expect("HMAC can take key of any size");
        hmac_token.update(pay_token.as_bytes());
        let result = hmac_token.finalize().into_bytes();

        self.paytr_token = general_purpose::STANDARD.encode(result);
        self.merchant_key = merchant_key;
        self.merchant_salt = merchant_salt;
    }

    pub async fn get_iframe(&self) -> Result<PaytrResponse, reqwest::Error> {
        let client = Client::new();
        let form_data = serde_urlencoded::to_string(self).expect("Failed to encode form data");

        let res = client
            .post("https://www.paytr.com/odeme/api/get-token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_data)
            .send()
            .await?;

        let response = res.json::<PaytrResponse>().await?;

        Ok(response)
    }
}

impl CallbackRequest {
    pub fn is_valid(&mut self, merchant_key: String, merchant_salt: String) -> bool {
        let mut buffer = itoa::Buffer::new();

        let token_str = format!(
            "{}{}{}{}",
            self.merchant_oid,
            merchant_salt,
            &self.status,
            buffer.format(self.total_amount)
        );

        let mut hmac_token = Hmac::<Sha256>::new_from_slice(merchant_key.as_bytes())
            .expect("HMAC can take key of any size");

        hmac_token.update(token_str.as_bytes());

        let result = hmac_token.finalize().into_bytes();

        let hashed = general_purpose::STANDARD.encode(result);

        self.hash == hashed
    }
}
