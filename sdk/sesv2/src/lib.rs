#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
//! <fullname>Amazon SES API v2</fullname>
//! <p>Welcome to the Amazon SES API v2 Reference. This guide provides information about the Amazon SES API v2,
//! including supported operations, data types, parameters, and schemas.</p>
//! <p>
//! <a href="https://aws.amazon.com/pinpoint">Amazon SES</a> is an AWS service that
//! you can use to send email messages to your customers.</p>
//! <p>If you're new to Amazon SES API v2, you might find it helpful to also review the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/">Amazon Simple Email Service Developer
//! Guide</a>. The <i>Amazon SES Developer Guide</i> provides information
//! and code samples that demonstrate how to use Amazon SES API v2 features programmatically.</p>
//! <p>The Amazon SES API v2 is available in several AWS Regions and it provides an endpoint for each
//! of these Regions. For a list of all the Regions and endpoints where the API is currently
//! available, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html#ses_region">AWS Service Endpoints</a> in the <i>Amazon Web Services General Reference</i>. To
//! learn more about AWS Regions, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html">Managing AWS Regions</a> in the
//! <i>Amazon Web Services General Reference</i>.</p>
//! <p>In each Region, AWS maintains multiple Availability Zones. These Availability Zones
//! are physically isolated from each other, but are united by private, low-latency,
//! high-throughput, and highly redundant network connections. These Availability Zones
//! enable us to provide very high levels of availability and redundancy, while also
//! minimizing latency. To learn more about the number of Availability Zones that are
//! available in each Region, see <a href="http://aws.amazon.com/about-aws/global-infrastructure/">AWS Global Infrastructure</a>.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("sesv2", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;