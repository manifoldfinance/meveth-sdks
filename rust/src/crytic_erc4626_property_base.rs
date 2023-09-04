pub use crytic_erc4626_property_base::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod crytic_erc4626_property_base {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AssertEqFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertEqFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssertFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssertGtFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertGtFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssertGteFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertGteFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssertLtFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertLtFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssertLteFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertLteFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssertNeqFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssertNeqFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogUint256"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LogUint256"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CRYTICERC4626PROPERTYBASE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4`\x13W`:\x90\x81`\x19\x829\xF3[`\0\x80\xFD\xFE`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xE4\xC5\x97\x85;\xEESg\xF1\x0F\xE9\xC1\xCFY\x89M\xA9\xD3\xEA\xF2\n\xA7\xD7F\x98\x8D\x8B$\xFD\r\t\xAAdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static CRYTICERC4626PROPERTYBASE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xE4\xC5\x97\x85;\xEESg\xF1\x0F\xE9\xC1\xCFY\x89M\xA9\xD3\xEA\xF2\n\xA7\xD7F\x98\x8D\x8B$\xFD\r\t\xAAdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static CRYTICERC4626PROPERTYBASE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CryticERC4626PropertyBase<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CryticERC4626PropertyBase<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CryticERC4626PropertyBase<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CryticERC4626PropertyBase<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CryticERC4626PropertyBase<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CryticERC4626PropertyBase))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CryticERC4626PropertyBase<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CRYTICERC4626PROPERTYBASE_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                CRYTICERC4626PROPERTYBASE_ABI.clone(),
                CRYTICERC4626PROPERTYBASE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Gets the contract's `AssertEqFail` event
        pub fn assert_eq_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertEqFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertFail` event
        pub fn assert_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertGtFail` event
        pub fn assert_gt_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertGtFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertGteFail` event
        pub fn assert_gte_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertGteFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertLtFail` event
        pub fn assert_lt_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertLtFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertLteFail` event
        pub fn assert_lte_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertLteFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssertNeqFail` event
        pub fn assert_neq_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssertNeqFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogAddress` event
        pub fn log_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogString` event
        pub fn log_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LogUint256` event
        pub fn log_uint_256_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogUint256Filter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CryticERC4626PropertyBaseEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CryticERC4626PropertyBase<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AssertEqFail", abi = "AssertEqFail(string)")]
    pub struct AssertEqFailFilter(pub ::std::string::String);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AssertFail", abi = "AssertFail(string)")]
    pub struct AssertFailFilter(pub ::std::string::String);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AssertGtFail", abi = "AssertGtFail(string)")]
    pub struct AssertGtFailFilter(pub ::std::string::String);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AssertGteFail", abi = "AssertGteFail(string)")]
    pub struct AssertGteFailFilter(pub ::std::string::String);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AssertLtFail", abi = "AssertLtFail(string)")]
    pub struct AssertLtFailFilter(pub ::std::string::String);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AssertLteFail", abi = "AssertLteFail(string)")]
    pub struct AssertLteFailFilter(pub ::std::string::String);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AssertNeqFail", abi = "AssertNeqFail(string)")]
    pub struct AssertNeqFailFilter(pub ::std::string::String);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "LogAddress", abi = "LogAddress(string,address)")]
    pub struct LogAddressFilter(
        pub ::std::string::String,
        pub ::ethers::core::types::Address,
    );
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "LogString", abi = "LogString(string)")]
    pub struct LogStringFilter(pub ::std::string::String);
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "LogUint256", abi = "LogUint256(string,uint256)")]
    pub struct LogUint256Filter(
        pub ::std::string::String,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CryticERC4626PropertyBaseEvents {
        AssertEqFailFilter(AssertEqFailFilter),
        AssertFailFilter(AssertFailFilter),
        AssertGtFailFilter(AssertGtFailFilter),
        AssertGteFailFilter(AssertGteFailFilter),
        AssertLtFailFilter(AssertLtFailFilter),
        AssertLteFailFilter(AssertLteFailFilter),
        AssertNeqFailFilter(AssertNeqFailFilter),
        LogAddressFilter(LogAddressFilter),
        LogStringFilter(LogStringFilter),
        LogUint256Filter(LogUint256Filter),
    }
    impl ::ethers::contract::EthLogDecode for CryticERC4626PropertyBaseEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssertEqFailFilter::decode_log(log) {
                return Ok(CryticERC4626PropertyBaseEvents::AssertEqFailFilter(decoded));
            }
            if let Ok(decoded) = AssertFailFilter::decode_log(log) {
                return Ok(CryticERC4626PropertyBaseEvents::AssertFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGtFailFilter::decode_log(log) {
                return Ok(CryticERC4626PropertyBaseEvents::AssertGtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGteFailFilter::decode_log(log) {
                return Ok(CryticERC4626PropertyBaseEvents::AssertGteFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLtFailFilter::decode_log(log) {
                return Ok(CryticERC4626PropertyBaseEvents::AssertLtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLteFailFilter::decode_log(log) {
                return Ok(CryticERC4626PropertyBaseEvents::AssertLteFailFilter(decoded));
            }
            if let Ok(decoded) = AssertNeqFailFilter::decode_log(log) {
                return Ok(CryticERC4626PropertyBaseEvents::AssertNeqFailFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(CryticERC4626PropertyBaseEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(CryticERC4626PropertyBaseEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUint256Filter::decode_log(log) {
                return Ok(CryticERC4626PropertyBaseEvents::LogUint256Filter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CryticERC4626PropertyBaseEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssertEqFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertFailFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssertGtFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertGteFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertLtFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertLteFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssertNeqFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogAddressFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogStringFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogUint256Filter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssertEqFailFilter> for CryticERC4626PropertyBaseEvents {
        fn from(value: AssertEqFailFilter) -> Self {
            Self::AssertEqFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertFailFilter> for CryticERC4626PropertyBaseEvents {
        fn from(value: AssertFailFilter) -> Self {
            Self::AssertFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGtFailFilter> for CryticERC4626PropertyBaseEvents {
        fn from(value: AssertGtFailFilter) -> Self {
            Self::AssertGtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGteFailFilter> for CryticERC4626PropertyBaseEvents {
        fn from(value: AssertGteFailFilter) -> Self {
            Self::AssertGteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLtFailFilter> for CryticERC4626PropertyBaseEvents {
        fn from(value: AssertLtFailFilter) -> Self {
            Self::AssertLtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLteFailFilter> for CryticERC4626PropertyBaseEvents {
        fn from(value: AssertLteFailFilter) -> Self {
            Self::AssertLteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertNeqFailFilter> for CryticERC4626PropertyBaseEvents {
        fn from(value: AssertNeqFailFilter) -> Self {
            Self::AssertNeqFailFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for CryticERC4626PropertyBaseEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for CryticERC4626PropertyBaseEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUint256Filter> for CryticERC4626PropertyBaseEvents {
        fn from(value: LogUint256Filter) -> Self {
            Self::LogUint256Filter(value)
        }
    }
}
