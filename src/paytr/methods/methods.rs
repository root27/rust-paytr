use crate::structs::structs::{CallbackRequest, Payment, PaytrResponse};
use base64::engine::{general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use itoa;
use reqwest::blocking::Client;
use serde_json;
use serde_urlencoded;
use sha2::Sha256;

impl Payment {
    pub fn basket_config<T: serde::Serialize>(&mut self, cart: &[Vec<T>]) {
        if let Ok(json) = serde_json::to_string(cart) {
            self.basket = general_purpose::STANDARD.encode(json);
        } else {
            panic!("Failed to serialize basket");
        }
    }

    pub fn generate_token(&mut self, merchant_key: &str, merchant_salt: &str) -> String {
        let mut buffer = itoa::Buffer::new();

        let hash_string = self.merchant_id.clone()
            + &self.user_ip
            + &self.merchant_oid
            + &self.email
            + &self.basket
            + &self.currency
            + &self.test_mode
            + buffer.format(self.total_amount)
            + buffer.format(self.no_installment)
            + buffer.format(self.max_installment);
        let pay_token = hash_string + &self.merchant_salt;

        let mut hmac_token = Hmac::<Sha256>::new_from_slice(merchant_key.as_bytes())
            .expect("HMAC can take key of any size");
        hmac_token.update(pay_token.as_bytes());
        let result = hmac_token.finalize().into_bytes();
        self.paytr_token = general_purpose::STANDARD.encode(result);

        self.merchant_key = merchant_key.to_string();
        self.merchant_salt = merchant_salt.to_string();

        self.paytr_token.clone()
    }

    pub fn get_iframe(&self) -> Result<PaytrResponse, reqwest::Error> {
        let client = Client::new();
        let form_data = serde_urlencoded::to_string(self).expect("Failed to encode form data");

        let res = client
            .post("https://www.paytr.com/odeme/api/get-token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_data)
            .send()?;

        let response = res.json::<PaytrResponse>()?;

        Ok(response)
    }
}

impl CallbackRequest {
    pub fn is_valid(&mut self, merchant_key: &str, merchant_salt: &str) -> bool {
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
