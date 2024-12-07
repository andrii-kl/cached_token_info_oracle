use crate::services::puzzle_service::{sign_message_hmac_hex, verify_signature_hex};


const KEY: &[u8] = b"my_secret_key";
const MESSAGE: &[u8] = b"This is a secret message";
const SIGNATURE: &str = "6294fcdd51787b064b1678478a89aeecdf4eb1c808feb2fc2f8626dd539e10eb";

#[test]
fn sign_message_test(){
     match sign_message_hmac_hex(&KEY, &MESSAGE) {
         Ok(signature) => {
             assert_eq!(signature, SIGNATURE)
         }
         Err(error) => {
             panic!("Not valid result in this test {}", error)
         }
     } ;
}

#[test]
fn verif_sign_message_test(){
    match verify_signature_hex(&KEY, &MESSAGE, &SIGNATURE) {
        Ok(isValid) => {
            assert_eq!(isValid, true)
        }
        Err(error) => {
            panic!("Not valid result in this test {}", error)
        }
    } ;
}
