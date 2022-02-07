use crate::aes_tools::*;

#[test]
fn test_key_gen() {
    generate_key();
}

#[test]
fn test_encrypt() {
    let message = "
    {
        \"id\": 1,
        \"name\": \"xyzab\"
    }";

    encrypt_gen_key_iv(message);

    ()
}

#[test]
fn test_message_is_same() {
    let message = "
    {
        \"id\": 1,
        \"name\": \"xyzab\"
    }"
    .to_string();

    let mut cipher: AESCipher = AESCipher::new();

    let ciphertext: Vec<u8> = cipher.encrypt(message);
    let plaintext = cipher.decrypt(ciphertext);

    println!(
        "{:?}",
        String::from_utf8(plaintext).expect("Found invalid UTF-8")
    );

    //let decrypted = decrypt(encrypted);

    assert_eq!(0, 1);
}
