use regex::Regex;

pub(crate) fn is_valid_username(username: &str) -> bool {
    //用户名正则，4到16位（字母，数字，下划线，减号）
    let re = Regex::new(r"^[a-zA-Z0-9_-]{4,16}$").unwrap();
    re.is_match(username)
}

pub(crate) fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9_-]+@[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)+$").unwrap();
    re.is_match(email)
}

pub(crate) fn is_valid_password(password: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9_-]{6,18}$").unwrap();
    re.is_match(password)
}