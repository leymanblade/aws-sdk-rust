// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ResourceType {
    /// Resource type value for the Application resource.
    Application,
    /// Resource type value for the Configuration resource.
    Configuration,
    /// Resource type value for the ConfigurationProfile resource.
    ConfigurationProfile,
    /// Resource type value for the Deployment resource.
    Deployment,
    /// Resource type value for the Environment resource.
    Environment,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ResourceType {
    fn from(s: &str) -> Self {
        match s {
            "Application" => ResourceType::Application,
            "Configuration" => ResourceType::Configuration,
            "ConfigurationProfile" => ResourceType::ConfigurationProfile,
            "Deployment" => ResourceType::Deployment,
            "Environment" => ResourceType::Environment,
            other => ResourceType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ResourceType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ResourceType::from(s))
    }
}
impl ResourceType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ResourceType::Application => "Application",
            ResourceType::Configuration => "Configuration",
            ResourceType::ConfigurationProfile => "ConfigurationProfile",
            ResourceType::Deployment => "Deployment",
            ResourceType::Environment => "Environment",
            ResourceType::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &[
            "Application",
            "Configuration",
            "ConfigurationProfile",
            "Deployment",
            "Environment",
        ]
    }
}
impl AsRef<str> for ResourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Details describing why the request was invalid</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub enum BadRequestDetails {
    /// <p>Present if the Reason for the bad request was 'InvalidParameters'</p>
    InvalidParameters(
        std::collections::HashMap<std::string::String, crate::model::InvalidParameterDetail>,
    ),
    /// The `Unknown` variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.
    /// An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has not been updated.
    /// To investigate this, consider turning on debug logging to print the raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl BadRequestDetails {
    #[allow(irrefutable_let_patterns)]
    /// Tries to convert the enum instance into [`InvalidParameters`](crate::model::BadRequestDetails::InvalidParameters), extracting the inner [`HashMap`](std::collections::HashMap).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_invalid_parameters(
        &self,
    ) -> std::result::Result<
        &std::collections::HashMap<std::string::String, crate::model::InvalidParameterDetail>,
        &Self,
    > {
        if let BadRequestDetails::InvalidParameters(val) = &self {
            Ok(val)
        } else {
            Err(self)
        }
    }
    /// Returns true if this is a [`InvalidParameters`](crate::model::BadRequestDetails::InvalidParameters).
    pub fn is_invalid_parameters(&self) -> bool {
        self.as_invalid_parameters().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// <p>Contains details about an invalid parameter.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidParameterDetail {
    /// <p>Detail describing why an individual parameter did not satisfy the constraints specified by the service</p>
    pub problem: std::option::Option<crate::model::InvalidParameterProblem>,
}
impl InvalidParameterDetail {
    /// <p>Detail describing why an individual parameter did not satisfy the constraints specified by the service</p>
    pub fn problem(&self) -> std::option::Option<&crate::model::InvalidParameterProblem> {
        self.problem.as_ref()
    }
}
impl std::fmt::Debug for InvalidParameterDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidParameterDetail");
        formatter.field("problem", &self.problem);
        formatter.finish()
    }
}
/// See [`InvalidParameterDetail`](crate::model::InvalidParameterDetail)
pub mod invalid_parameter_detail {
    /// A builder for [`InvalidParameterDetail`](crate::model::InvalidParameterDetail)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) problem: std::option::Option<crate::model::InvalidParameterProblem>,
    }
    impl Builder {
        /// <p>Detail describing why an individual parameter did not satisfy the constraints specified by the service</p>
        pub fn problem(mut self, input: crate::model::InvalidParameterProblem) -> Self {
            self.problem = Some(input);
            self
        }
        /// <p>Detail describing why an individual parameter did not satisfy the constraints specified by the service</p>
        pub fn set_problem(
            mut self,
            input: std::option::Option<crate::model::InvalidParameterProblem>,
        ) -> Self {
            self.problem = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidParameterDetail`](crate::model::InvalidParameterDetail)
        pub fn build(self) -> crate::model::InvalidParameterDetail {
            crate::model::InvalidParameterDetail {
                problem: self.problem,
            }
        }
    }
}
impl InvalidParameterDetail {
    /// Creates a new builder-style object to manufacture [`InvalidParameterDetail`](crate::model::InvalidParameterDetail)
    pub fn builder() -> crate::model::invalid_parameter_detail::Builder {
        crate::model::invalid_parameter_detail::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum InvalidParameterProblem {
    /// The parameter was corrupted and could not be understood by the service.
    Corrupted,
    /// The parameter was expired and can no longer be used.
    Expired,
    /// The client called the service before the time specified in the poll interval.
    PollIntervalNotSatisfied,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for InvalidParameterProblem {
    fn from(s: &str) -> Self {
        match s {
            "Corrupted" => InvalidParameterProblem::Corrupted,
            "Expired" => InvalidParameterProblem::Expired,
            "PollIntervalNotSatisfied" => InvalidParameterProblem::PollIntervalNotSatisfied,
            other => InvalidParameterProblem::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for InvalidParameterProblem {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(InvalidParameterProblem::from(s))
    }
}
impl InvalidParameterProblem {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            InvalidParameterProblem::Corrupted => "Corrupted",
            InvalidParameterProblem::Expired => "Expired",
            InvalidParameterProblem::PollIntervalNotSatisfied => "PollIntervalNotSatisfied",
            InvalidParameterProblem::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["Corrupted", "Expired", "PollIntervalNotSatisfied"]
    }
}
impl AsRef<str> for InvalidParameterProblem {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum BadRequestReason {
    /// Indicates there was a problem with one or more of the parameters.
    /// See InvalidParameters in the BadRequestDetails for more information.
    InvalidParameters,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for BadRequestReason {
    fn from(s: &str) -> Self {
        match s {
            "InvalidParameters" => BadRequestReason::InvalidParameters,
            other => BadRequestReason::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for BadRequestReason {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(BadRequestReason::from(s))
    }
}
impl BadRequestReason {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            BadRequestReason::InvalidParameters => "InvalidParameters",
            BadRequestReason::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["InvalidParameters"]
    }
}
impl AsRef<str> for BadRequestReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}