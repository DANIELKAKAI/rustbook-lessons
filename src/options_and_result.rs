pub fn test_option(number: i32) -> Option<i32> {
    if number > 10 {
        Some(number)
    } else {
        return None;
    }
}

pub fn test_error(number: i32) -> Result<i32, String> {
    if number > 10 {
        Ok(number)
    } else {
        Err(String::from("less than 10 number"))
    }
}
