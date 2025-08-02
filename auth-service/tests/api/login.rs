use crate::helpers::{get_random_email, TestApp};
use auth_service::ErrorResponse;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": random_email
        }),
        serde_json::json!({
            "password": "password"
        }),
        serde_json::json!({
            "email": true,
            "password": "password"
        }),
        serde_json::json!({
            "email": random_email,
            "password": true,
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "foobar.com",
            "password": "abcd1234",
        }),
        serde_json::json!({
            "email": "",
            "password": "abcd1234",
        }),
        serde_json::json!({
            "email": "a@b.com",
            "password": "abcd123",
        }),
        serde_json::json!({
            "email": "a@b.com",
            "password": "12345678901234567890123456789012345678901234567890123456789012345",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Should fail with HTTP400 for input: {}",
            test_case
        );
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialise response body to ErrorResponse")
                .error,
            "Invalid input".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_401_if_credentials_incorrect() {
    let app = TestApp::new().await;
    let email = get_random_email();
    let password = String::from("abcd1234");

    let signup_data = serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": true
    });

    let response = app.post_signup(&signup_data).await;
    assert_eq!(
        response.status().as_u16(),
        201,
        "Should return 201 for new account created with data: {}",
        signup_data
    );

    let login_data = [
        serde_json::json!({
            "email": "foo@bar.com",
            "password": password
        }),
        serde_json::json!({
            "email": email,
            "password": "incorrect"
        }),
    ];

    for invalid_login in login_data {
        let response = app.post_login(&invalid_login).await;
        assert_eq!(
            response.status().as_u16(),
            401,
            "Should fail with HTTP401 (Incorrect credentials)"
        );
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialise response body to ErrorResponse")
                .error,
            "Incorrect credentials".to_owned()
        );
    }
}
