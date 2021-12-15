// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_header_create_app_monitor_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_1 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_1.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_1.len()),
        ))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}

pub fn deser_header_create_app_monitor_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_2 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_2.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_2.len()),
        ))
    } else {
        let mut var_2 = var_2;
        Ok(var_2.pop())
    }
}

pub fn deser_header_delete_app_monitor_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_3 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_3.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_3.len()),
        ))
    } else {
        let mut var_3 = var_3;
        Ok(var_3.pop())
    }
}

pub fn deser_header_delete_app_monitor_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_4 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_4.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_4.len()),
        ))
    } else {
        let mut var_4 = var_4;
        Ok(var_4.pop())
    }
}

pub fn deser_header_get_app_monitor_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_5 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_5.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_5.len()),
        ))
    } else {
        let mut var_5 = var_5;
        Ok(var_5.pop())
    }
}

pub fn deser_header_get_app_monitor_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_6 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_6.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_6.len()),
        ))
    } else {
        let mut var_6 = var_6;
        Ok(var_6.pop())
    }
}

pub fn deser_header_get_app_monitor_data_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_7 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_7.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_7.len()),
        ))
    } else {
        let mut var_7 = var_7;
        Ok(var_7.pop())
    }
}

pub fn deser_header_get_app_monitor_data_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_8 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_8.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_8.len()),
        ))
    } else {
        let mut var_8 = var_8;
        Ok(var_8.pop())
    }
}

pub fn deser_header_list_app_monitors_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_9 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_9.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_9.len()),
        ))
    } else {
        let mut var_9 = var_9;
        Ok(var_9.pop())
    }
}

pub fn deser_header_list_app_monitors_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_10 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_10.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_10.len()),
        ))
    } else {
        let mut var_10 = var_10;
        Ok(var_10.pop())
    }
}

pub fn deser_header_list_tags_for_resource_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_11 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_11.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_11.len()),
        ))
    } else {
        let mut var_11 = var_11;
        Ok(var_11.pop())
    }
}

pub fn deser_header_put_rum_events_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_12 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_12.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_12.len()),
        ))
    } else {
        let mut var_12 = var_12;
        Ok(var_12.pop())
    }
}

pub fn deser_header_put_rum_events_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_13 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_13.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_13.len()),
        ))
    } else {
        let mut var_13 = var_13;
        Ok(var_13.pop())
    }
}

pub fn deser_header_tag_resource_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_14 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_14.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_14.len()),
        ))
    } else {
        let mut var_14 = var_14;
        Ok(var_14.pop())
    }
}

pub fn deser_header_untag_resource_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_15 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_15.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_15.len()),
        ))
    } else {
        let mut var_15 = var_15;
        Ok(var_15.pop())
    }
}

pub fn deser_header_update_app_monitor_internal_server_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_16 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_16.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_16.len()),
        ))
    } else {
        let mut var_16 = var_16;
        Ok(var_16.pop())
    }
}

pub fn deser_header_update_app_monitor_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_17 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_17.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new_with_message(
            format!("expected one item but found {}", var_17.len()),
        ))
    } else {
        let mut var_17 = var_17;
        Ok(var_17.pop())
    }
}