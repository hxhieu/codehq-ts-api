use std::collections::HashMap;

use alcoholic_jwt::ValidJWT;

pub struct TokenClaims {
    claims: HashMap<String, String>,
}

impl TokenClaims {
    pub fn new(result: ValidJWT) -> TokenClaims {
        let mut claims = HashMap::new();
        for (key, value) in result.claims.as_object().unwrap() {
            claims.insert(
                key.to_string(),
                match value.as_str() {
                    Some(v) => v.to_string(),
                    None => "".to_string(),
                },
            );
        }
        TokenClaims { claims }
    }

    pub fn get(&self, key: &str) -> String {
        let value = "".to_string();
        self.claims.get(key).unwrap_or(&value).to_string()
    }
}
