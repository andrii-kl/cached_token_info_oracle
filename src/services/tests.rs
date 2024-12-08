use crate::services::puzzle_service::{create_puzzle_task, find_nonce, sign_message_hmac, sign_message_hmac_hex, verify_access_token_hex, verify_nonce, verify_signature, verify_signature_hex};
use std::collections::HashSet;


const KEY: &[u8] = b"my_secret_key";

// Calculate test values with third party services with cross-check
const TASK_MESSAGE: &str = "dcb891a4-11ae-4a8f-a45e-f9a4b5e7f330";
const SIGNATURE: &str = "f9071a790bd757e269224b010a1b1403dd5ded374bf871f77a76f80e1a739d5e";
const DIFFICULTY: u8 = 1;
const APPROPRIATE_NONCE: u64 = 10;

const TOKEN_ACCESS: &str = "access_8a0c5cfb-4b61-4a4f-babb-c8ec8470ef0e";
const TOKEN_ACCESS_SIGNATURE: &str = "1954f946acc6b1f2da948be581b2db2f56e2d4ae2cb682944a1db737d8c4f2dc";


#[test]
fn sign_message_test(){
     match sign_message_hmac_hex(&KEY, &TASK_MESSAGE.as_bytes()) {
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
    match verify_signature_hex(&KEY, &TASK_MESSAGE.as_bytes(), &SIGNATURE) {
        Ok(is_valid) => {
            assert_eq!(is_valid, true)
        }
        Err(error) => {
            panic!("Not valid result in this test {}", error)
        }
    };
}

#[test]
fn verif_sign_token_access_test(){
    match verify_access_token_hex(&KEY, &TOKEN_ACCESS.as_bytes(), &TOKEN_ACCESS_SIGNATURE) {
        Ok(is_valid) => {
            assert_eq!(is_valid, true)
        }
        Err(error) => {
            panic!("Not valid result in this test {}", error)
        }
    };
}

#[test]
fn impossible_use_task_as_access_token_test(){
    match verify_access_token_hex(&KEY, &TASK_MESSAGE.as_bytes(), &SIGNATURE) {
        Ok(is_valid) => {
            assert_eq!(is_valid, false)
        }
        Err(error) => {
            panic!("Not valid result in this test {}", error)
        }
    };
}

#[test]
fn find_nonce_test(){
    let nonce = find_nonce(TASK_MESSAGE, DIFFICULTY);
    println!("{:?}",&nonce);
    assert_eq!(nonce, APPROPRIATE_NONCE)
}

#[test]
fn verify_nonce_true_test(){
    let is_valid = verify_nonce(TASK_MESSAGE, &APPROPRIATE_NONCE, DIFFICULTY);

    assert_eq!(is_valid, true);
}

#[test]
fn verify_nonce_false_test(){
    let is_valid = verify_nonce(TASK_MESSAGE, &1234, DIFFICULTY);

    assert_eq!(is_valid, false);
}

#[test]
fn create_task_test(){
    let mut set: HashSet<String> = HashSet::with_capacity(1000);

    for _ in 0..750 {
        let task = create_puzzle_task(KEY).unwrap().get_task().clone();

        assert_eq!(set.contains(&task), false, "Service have provide unic tasks");

        set.insert(task);
    }

    assert_eq!(set.len(), 750);
}


#[test]
fn find_verif_sig_and_nonce_full_cycle_test(){
    let task = create_puzzle_task(KEY).unwrap();
    let task_message = task.get_task();

    let nonce = find_nonce(&task_message, DIFFICULTY);

    let is_valid_sig = verify_signature_hex(KEY, task_message.as_bytes(), task.get_signature()).unwrap();
    assert_eq!(is_valid_sig, true);

    let is_valid = verify_nonce(task_message, &nonce, DIFFICULTY);

    assert_eq!(is_valid, true);
}