pub async fn get_user_by_id() -> String {
    String::from("get_user_by_id")
}

pub async fn get_user_by_email() -> String {
    String::from("get_user_by_email")
}

pub async fn post_create_user() -> String {
    String::from("create_user")
}

pub async fn put_update_user() -> String {
    String::from("update_user")
}

pub async fn delete_user() -> String {
    String::from("delete_user")
}

pub async fn get_all_users() -> String {
    String::from("get_all_users")
}

pub async fn post_login_user() -> String {
    String::from("login_user")
}

pub async fn post_logout_user() -> String {
    String::from("logout_user")
}

pub async fn post_refresh_token() -> String {
    String::from("refresh_token")
}

pub async fn post_verify_password() -> String {
    String::from("verify_password")
}

pub async fn post_verify_email() -> String {
    String::from("verify_email")
}

pub async fn post_reset_password() -> String {
    String::from("reset_password")
}
