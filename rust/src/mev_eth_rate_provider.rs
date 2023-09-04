pub use mev_eth_rate_provider::*;
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
pub mod mev_eth_rate_provider {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_mevETH"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IMevEth"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRate"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mevETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mevETH"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IMevEth"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MEVETHRATEPROVIDER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA04a\0pW`\x1Fa\x02=8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0uW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0pWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\0pW`\x80R`@Qa\x01\xB1\x90\x81a\0\x8C\x829`\x80Q\x81\x81\x81`j\x01Ra\x01W\x01R\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x13W`\0\x80\xFD[`\0\x90\x815`\xE0\x1C\x90\x81c0q\x1A\xDF\x14a\x01+WPcg\x9A\xEF\xCE\x14a\x007W`\0\x80\xFD[4a\x01(W\x80`\x03\x196\x01\x12a\x01(W`@Qc\x03\xD1h\x9D`\xE1\x1B\x81Rg\r\xE0\xB6\xB3\xA7d\0\0`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x90\x81\x15a\x01\x1DW\x82\x91a\0\xBAW[` \x82`@Q\x90\x81R\xF3[\x90P` =\x81\x11a\x01\x16W[`\x1F\x81\x01`\x1F\x19\x16\x82\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x83\x82\x10\x17a\x01\x02W` \x91\x83\x91`@R\x81\x01\x03\x12a\0\xFEW` \x91PQ8a\0\xAFV[P\x80\xFD[cNH{q`\xE0\x1B\x84R`A`\x04R`$\x84\xFD[P=a\0\xC6V[`@Q=\x84\x82>=\x90\xFD[\x80\xFD[\x90P4a\0\xFEW\x81`\x03\x196\x01\x12a\0\xFEW` \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3\xFE\xA2dipfsX\"\x12 \xAC\xB8S\x98\xA8i\x8B\xD5g@\xAB\x03\x1Et\xA8\xD7\x9C\x1BN*\xF6|\x9F8;\xAD\x9B:q|,\x81dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MEVETHRATEPROVIDER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80\x80`@R`\x046\x10\x15a\0\x13W`\0\x80\xFD[`\0\x90\x815`\xE0\x1C\x90\x81c0q\x1A\xDF\x14a\x01+WPcg\x9A\xEF\xCE\x14a\x007W`\0\x80\xFD[4a\x01(W\x80`\x03\x196\x01\x12a\x01(W`@Qc\x03\xD1h\x9D`\xE1\x1B\x81Rg\r\xE0\xB6\xB3\xA7d\0\0`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16Z\xFA\x90\x81\x15a\x01\x1DW\x82\x91a\0\xBAW[` \x82`@Q\x90\x81R\xF3[\x90P` =\x81\x11a\x01\x16W[`\x1F\x81\x01`\x1F\x19\x16\x82\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x83\x82\x10\x17a\x01\x02W` \x91\x83\x91`@R\x81\x01\x03\x12a\0\xFEW` \x91PQ8a\0\xAFV[P\x80\xFD[cNH{q`\xE0\x1B\x84R`A`\x04R`$\x84\xFD[P=a\0\xC6V[`@Q=\x84\x82>=\x90\xFD[\x80\xFD[\x90P4a\0\xFEW\x81`\x03\x196\x01\x12a\0\xFEW` \x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3\xFE\xA2dipfsX\"\x12 \xAC\xB8S\x98\xA8i\x8B\xD5g@\xAB\x03\x1Et\xA8\xD7\x9C\x1BN*\xF6|\x9F8;\xAD\x9B:q|,\x81dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MEVETHRATEPROVIDER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MevETHRateProvider<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MevETHRateProvider<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MevETHRateProvider<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MevETHRateProvider<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MevETHRateProvider<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MevETHRateProvider))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MevETHRateProvider<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MEVETHRATEPROVIDER_ABI.clone(),
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
                MEVETHRATEPROVIDER_ABI.clone(),
                MEVETHRATEPROVIDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getRate` (0x679aefce) function
        pub fn get_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([103, 154, 239, 206], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mevETH` (0x30711adf) function
        pub fn mev_eth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([48, 113, 26, 223], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MevETHRateProvider<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getRate` function with signature `getRate()` and selector `0x679aefce`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRate", abi = "getRate()")]
    pub struct GetRateCall;
    ///Container type for all input parameters for the `mevETH` function with signature `mevETH()` and selector `0x30711adf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mevETH", abi = "mevETH()")]
    pub struct MevETHCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MevETHRateProviderCalls {
        GetRate(GetRateCall),
        MevETH(MevETHCall),
    }
    impl ::ethers::core::abi::AbiDecode for MevETHRateProviderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetRateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRate(decoded));
            }
            if let Ok(decoded)
                = <MevETHCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MevETH(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MevETHRateProviderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetRate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MevETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MevETHRateProviderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::MevETH(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetRateCall> for MevETHRateProviderCalls {
        fn from(value: GetRateCall) -> Self {
            Self::GetRate(value)
        }
    }
    impl ::core::convert::From<MevETHCall> for MevETHRateProviderCalls {
        fn from(value: MevETHCall) -> Self {
            Self::MevETH(value)
        }
    }
    ///Container type for all return fields from the `getRate` function with signature `getRate()` and selector `0x679aefce`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mevETH` function with signature `mevETH()` and selector `0x30711adf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MevETHReturn(pub ::ethers::core::types::Address);
}
