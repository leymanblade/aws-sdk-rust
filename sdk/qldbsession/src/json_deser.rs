// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_generic_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<smithy_types::Error, smithy_json::deserialize::Error> {
    crate::json_errors::parse_generic_error(response)
}

pub fn deser_structure_bad_request_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::bad_request_exception::Builder,
) -> Result<crate::error::bad_request_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    "Code" => {
                        builder = builder.set_code(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_capacity_exceeded_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::capacity_exceeded_exception::Builder,
) -> Result<crate::error::capacity_exceeded_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_invalid_session_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::invalid_session_exception::Builder,
) -> Result<crate::error::invalid_session_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    "Code" => {
                        builder = builder.set_code(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_limit_exceeded_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::limit_exceeded_exception::Builder,
) -> Result<crate::error::limit_exceeded_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_occ_conflict_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::occ_conflict_exception::Builder,
) -> Result<crate::error::occ_conflict_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_rate_exceeded_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::rate_exceeded_exception::Builder,
) -> Result<crate::error::rate_exceeded_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_operation_send_command(
    input: &[u8],
    mut builder: crate::output::send_command_output::Builder,
) -> Result<crate::output::send_command_output::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "StartSession" => {
                        builder = builder.set_start_session(
                            crate::json_deser::deser_structure_start_session_result(tokens)?,
                        );
                    }
                    "StartTransaction" => {
                        builder = builder.set_start_transaction(
                            crate::json_deser::deser_structure_start_transaction_result(tokens)?,
                        );
                    }
                    "EndSession" => {
                        builder = builder.set_end_session(
                            crate::json_deser::deser_structure_end_session_result(tokens)?,
                        );
                    }
                    "CommitTransaction" => {
                        builder = builder.set_commit_transaction(
                            crate::json_deser::deser_structure_commit_transaction_result(tokens)?,
                        );
                    }
                    "AbortTransaction" => {
                        builder = builder.set_abort_transaction(
                            crate::json_deser::deser_structure_abort_transaction_result(tokens)?,
                        );
                    }
                    "ExecuteStatement" => {
                        builder = builder.set_execute_statement(
                            crate::json_deser::deser_structure_execute_statement_result(tokens)?,
                        );
                    }
                    "FetchPage" => {
                        builder = builder.set_fetch_page(
                            crate::json_deser::deser_structure_fetch_page_result(tokens)?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub fn deser_structure_start_session_result<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::StartSessionResult>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::StartSessionResult::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "SessionToken" => {
                                builder = builder.set_session_token(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "TimingInformation" => {
                                builder = builder.set_timing_information(
                                    crate::json_deser::deser_structure_timing_information(tokens)?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_start_transaction_result<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::StartTransactionResult>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::StartTransactionResult::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "TransactionId" => {
                                builder = builder.set_transaction_id(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "TimingInformation" => {
                                builder = builder.set_timing_information(
                                    crate::json_deser::deser_structure_timing_information(tokens)?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_end_session_result<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::EndSessionResult>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::EndSessionResult::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "TimingInformation" => {
                                builder = builder.set_timing_information(
                                    crate::json_deser::deser_structure_timing_information(tokens)?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_commit_transaction_result<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::CommitTransactionResult>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::CommitTransactionResult::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "TransactionId" => {
                                builder = builder.set_transaction_id(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "CommitDigest" => {
                                builder = builder.set_commit_digest(
                                    smithy_json::deserialize::token::expect_blob_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "TimingInformation" => {
                                builder = builder.set_timing_information(
                                    crate::json_deser::deser_structure_timing_information(tokens)?,
                                );
                            }
                            "ConsumedIOs" => {
                                builder = builder.set_consumed_i_os(
                                    crate::json_deser::deser_structure_io_usage(tokens)?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_abort_transaction_result<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::AbortTransactionResult>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::AbortTransactionResult::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "TimingInformation" => {
                                builder = builder.set_timing_information(
                                    crate::json_deser::deser_structure_timing_information(tokens)?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_execute_statement_result<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::ExecuteStatementResult>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::ExecuteStatementResult::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "FirstPage" => {
                                builder = builder.set_first_page(
                                    crate::json_deser::deser_structure_page(tokens)?,
                                );
                            }
                            "TimingInformation" => {
                                builder = builder.set_timing_information(
                                    crate::json_deser::deser_structure_timing_information(tokens)?,
                                );
                            }
                            "ConsumedIOs" => {
                                builder = builder.set_consumed_i_os(
                                    crate::json_deser::deser_structure_io_usage(tokens)?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_fetch_page_result<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::FetchPageResult>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::FetchPageResult::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Page" => {
                                builder = builder
                                    .set_page(crate::json_deser::deser_structure_page(tokens)?);
                            }
                            "TimingInformation" => {
                                builder = builder.set_timing_information(
                                    crate::json_deser::deser_structure_timing_information(tokens)?,
                                );
                            }
                            "ConsumedIOs" => {
                                builder = builder.set_consumed_i_os(
                                    crate::json_deser::deser_structure_io_usage(tokens)?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_timing_information<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::TimingInformation>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::TimingInformation::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ProcessingTimeMilliseconds" => {
                                builder = builder.set_processing_time_milliseconds(
                                    smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i64()),
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_io_usage<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::IoUsage>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::IoUsage::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ReadIOs" => {
                                builder = builder.set_read_i_os(
                                    smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i64()),
                                );
                            }
                            "WriteIOs" => {
                                builder = builder.set_write_i_os(
                                    smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_i64()),
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

pub fn deser_structure_page<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::Page>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::Page::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Values" => {
                                builder = builder.set_values(
                                    crate::json_deser::deser_list_value_holders(tokens)?,
                                );
                            }
                            "NextPageToken" => {
                                builder = builder.set_next_page_token(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}

#[allow(clippy::type_complexity, non_snake_case)]
pub fn deser_list_value_holders<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<std::vec::Vec<crate::model::ValueHolder>>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap();
                        break;
                    }
                    _ => {
                        let value = crate::json_deser::deser_structure_value_holder(tokens)?;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start array or null",
        )),
    }
}

pub fn deser_structure_value_holder<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::model::ValueHolder>, smithy_json::deserialize::Error>
where
    I: Iterator<
        Item = Result<smithy_json::deserialize::Token<'a>, smithy_json::deserialize::Error>,
    >,
{
    match tokens.next().transpose()? {
        Some(smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::ValueHolder::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "IonBinary" => {
                                builder = builder.set_ion_binary(
                                    smithy_json::deserialize::token::expect_blob_or_null(
                                        tokens.next(),
                                    )?,
                                );
                            }
                            "IonText" => {
                                builder = builder.set_ion_text(
                                    smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            _ => smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    _ => {
                        return Err(smithy_json::deserialize::Error::custom(
                            "expected object key or end object",
                        ))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(smithy_json::deserialize::Error::custom(
            "expected start object or null",
        )),
    }
}