// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    client: aws_smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// Client for AWS AppConfig Data
///
/// Client for invoking operations on AWS AppConfig Data. Each operation on AWS AppConfig Data is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_appconfigdata::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::RetryConfig;
/// # async fn docs() {
///     let shared_config = aws_config::load_from_env().await;
///     let config = aws_sdk_appconfigdata::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_appconfigdata::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client<
    C = aws_smithy_client::erase::DynConnector,
    M = crate::middleware::DefaultMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl<C, M, R> From<aws_smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: aws_smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    /// Creates a client with the given service configuration.
    pub fn with_config(client: aws_smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Constructs a fluent builder for the `GetLatestConfiguration` operation.
    ///
    /// See [`GetLatestConfiguration`](crate::client::fluent_builders::GetLatestConfiguration) for more information about the
    /// operation and its arguments.
    pub fn get_latest_configuration(&self) -> fluent_builders::GetLatestConfiguration<C, M, R> {
        fluent_builders::GetLatestConfiguration::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `StartConfigurationSession` operation.
    ///
    /// See [`StartConfigurationSession`](crate::client::fluent_builders::StartConfigurationSession) for more information about the
    /// operation and its arguments.
    pub fn start_configuration_session(
        &self,
    ) -> fluent_builders::StartConfigurationSession<C, M, R> {
        fluent_builders::StartConfigurationSession::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `GetLatestConfiguration`.
    ///
    /// <p>Retrieves the latest deployed configuration. This API may return empty Configuration data if the client already has the latest version. See StartConfigurationSession to obtain an InitialConfigurationToken to call this API.</p>
    /// <important>
    /// <p>Each call to GetLatestConfiguration returns a new ConfigurationToken (NextPollConfigurationToken
    /// in the response). This new token MUST be provided to the next call to GetLatestConfiguration when
    /// polling for configuration updates.</p>
    /// <p>To avoid excess charges, we recommend that you include the
    /// <code>ClientConfigurationVersion</code> value with every call to
    /// <code>GetConfiguration</code>. This value must be saved on your client. Subsequent
    /// calls to <code>GetConfiguration</code> must pass this value by using the
    /// <code>ClientConfigurationVersion</code> parameter. </p>
    /// </important>
    #[derive(std::fmt::Debug)]
    pub struct GetLatestConfiguration<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_latest_configuration_input::Builder,
    }
    impl<C, M, R> GetLatestConfiguration<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetLatestConfiguration`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetLatestConfigurationOutput,
            aws_smithy_http::result::SdkError<crate::error::GetLatestConfigurationError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetLatestConfigurationInputOperationOutputAlias,
                crate::output::GetLatestConfigurationOutput,
                crate::error::GetLatestConfigurationError,
                crate::input::GetLatestConfigurationInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>Token describing the current state of the configuration session. To obtain a token, first call the StartConfigurationSession API. Note that every call to GetLatestConfiguration will return a new ConfigurationToken (NextPollConfigurationToken in the response) and MUST be provided to subsequent GetLatestConfiguration API calls.</p>
        pub fn configuration_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.configuration_token(inp);
            self
        }
        /// <p>Token describing the current state of the configuration session. To obtain a token, first call the StartConfigurationSession API. Note that every call to GetLatestConfiguration will return a new ConfigurationToken (NextPollConfigurationToken in the response) and MUST be provided to subsequent GetLatestConfiguration API calls.</p>
        pub fn set_configuration_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_token(input);
            self
        }
    }
    /// Fluent builder constructing a request to `StartConfigurationSession`.
    ///
    /// <p>Starts a configuration session used to retrieve a deployed configuration. See the GetLatestConfiguration API for more details.</p>
    #[derive(std::fmt::Debug)]
    pub struct StartConfigurationSession<
        C = aws_smithy_client::erase::DynConnector,
        M = crate::middleware::DefaultMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::start_configuration_session_input::Builder,
    }
    impl<C, M, R> StartConfigurationSession<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `StartConfigurationSession`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Sends the request and returns the response.
        ///
        /// If an error occurs, an `SdkError` will be returned with additional details that
        /// can be matched against.
        ///
        /// By default, any retryable failures will be retried twice. Retry behavior
        /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
        /// set when configuring the client.
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::StartConfigurationSessionOutput,
            aws_smithy_http::result::SdkError<crate::error::StartConfigurationSessionError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::StartConfigurationSessionInputOperationOutputAlias,
                crate::output::StartConfigurationSessionOutput,
                crate::error::StartConfigurationSessionError,
                crate::input::StartConfigurationSessionInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The application ID or the application name.</p>
        pub fn application_identifier(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.application_identifier(inp);
            self
        }
        /// <p>The application ID or the application name.</p>
        pub fn set_application_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_application_identifier(input);
            self
        }
        /// <p>The environment ID or the environment name.</p>
        pub fn environment_identifier(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.environment_identifier(inp);
            self
        }
        /// <p>The environment ID or the environment name.</p>
        pub fn set_environment_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_environment_identifier(input);
            self
        }
        /// <p>The configuration profile ID or the configuration profile name.</p>
        pub fn configuration_profile_identifier(
            mut self,
            inp: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.configuration_profile_identifier(inp);
            self
        }
        /// <p>The configuration profile ID or the configuration profile name.</p>
        pub fn set_configuration_profile_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_configuration_profile_identifier(input);
            self
        }
        /// <p>The interval at which your client will poll for configuration. If provided, the service
        /// will throw a BadRequestException if the client polls before the specified poll interval. By default,
        /// client poll intervals are not enforced.</p>
        pub fn required_minimum_poll_interval_in_seconds(mut self, inp: i32) -> Self {
            self.inner = self.inner.required_minimum_poll_interval_in_seconds(inp);
            self
        }
        /// <p>The interval at which your client will poll for configuration. If provided, the service
        /// will throw a BadRequestException if the client polls before the specified poll interval. By default,
        /// client poll intervals are not enforced.</p>
        pub fn set_required_minimum_poll_interval_in_seconds(
            mut self,
            input: std::option::Option<i32>,
        ) -> Self {
            self.inner = self
                .inner
                .set_required_minimum_poll_interval_in_seconds(input);
            self
        }
    }
}
impl<C> Client<C, crate::middleware::DefaultMiddleware, aws_smithy_client::retry::Standard> {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::new()
            .connector(conn)
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        aws_smithy_client::erase::DynConnector,
        crate::middleware::DefaultMiddleware,
        aws_smithy_client::retry::Standard,
    >
{
    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut builder = aws_smithy_client::Builder::dyn_https()
            .middleware(crate::middleware::DefaultMiddleware::new());
        builder.set_retry_config(retry_config.into());
        builder.set_timeout_config(timeout_config);
        // the builder maintains a try-state. To avoid suppressing the warning when sleep is unset,
        // only set it if we actually have a sleep impl.
        if let Some(sleep_impl) = sleep_impl {
            builder.set_sleep_impl(Some(sleep_impl));
        }
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}