pub use redemption_proxy::*;
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
pub mod redemption_proxy {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_vault"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IERC4626"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("redeemOnBehalf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeemOnBehalf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokensWithdrawn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawOnBehalf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawOnBehalf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sharesRedeemed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
    pub static REDEMPTIONPROXY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x804a\0tW`\x1Fa\x02z8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0yW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0tWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0tW`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa\x01\xEA\x90\x81a\0\x90\x829\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80`@\x81\x81R`\x046\x10\x15a\0\x14W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x90\x81c$\xFBV\x9F\x14a\0\xF4WPc\xCA0yZ\x14a\08W`\0\x80\xFD[4a\0\xF0Wa\0\x8F\x90` a\0L6a\x01BV[\x86T\x85Qc]\x04;)`\xE1\x1B\x81R`\x04\x81\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`$\x85\x01R\x90\x82\x16`D\x84\x01R\x91\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90`d\x82\x01\x90V[\x03\x92Z\xF1\x91\x82\x15a\0\xE6W\x83\x92a\0\xABW[` \x83\x83Q\x90\x81R\xF3[\x90\x91P` \x81=\x82\x11a\0\xDEW[\x81a\0\xC6` \x93\x83a\x01|V[\x81\x01\x03\x12a\0\xDAW` \x92PQ\x908a\0\xA1V[\x82\x80\xFD[=\x91Pa\0\xB9V[\x81Q=\x85\x82>=\x90\xFD[P\x80\xFD[\x91\x90P4a\0\xDAW` \x82a\0\x8F\x81\x86\x81a\x01\x0E6a\x01BV[\x84Tc-\x18+\xE5`\xE2\x1B\x85R`\x04\x85\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`$\x85\x01R\x81\x16`D\x84\x01R\x16\x93`d\x82\x01\x90V[``\x90`\x03\x19\x01\x12a\x01wW`\x045\x90`\x01`\x01`\xA0\x1B\x03\x90`$5\x82\x81\x16\x81\x03a\x01wW\x91`D5\x90\x81\x16\x81\x03a\x01wW\x90V[`\0\x80\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01\x9EW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xC8\xDB\xA4W\xB0-\xA7pA\xDB\xA4\xB9\xC9\xB2\xC8\x03\xD0\xEF\xC4\xFF\x1B\xE5m\xE6j4\x1F\xDF\x80R=hdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static REDEMPTIONPROXY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x046\x10\x15a\0\x14W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x90\x81c$\xFBV\x9F\x14a\0\xF4WPc\xCA0yZ\x14a\08W`\0\x80\xFD[4a\0\xF0Wa\0\x8F\x90` a\0L6a\x01BV[\x86T\x85Qc]\x04;)`\xE1\x1B\x81R`\x04\x81\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`$\x85\x01R\x90\x82\x16`D\x84\x01R\x91\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90`d\x82\x01\x90V[\x03\x92Z\xF1\x91\x82\x15a\0\xE6W\x83\x92a\0\xABW[` \x83\x83Q\x90\x81R\xF3[\x90\x91P` \x81=\x82\x11a\0\xDEW[\x81a\0\xC6` \x93\x83a\x01|V[\x81\x01\x03\x12a\0\xDAW` \x92PQ\x908a\0\xA1V[\x82\x80\xFD[=\x91Pa\0\xB9V[\x81Q=\x85\x82>=\x90\xFD[P\x80\xFD[\x91\x90P4a\0\xDAW` \x82a\0\x8F\x81\x86\x81a\x01\x0E6a\x01BV[\x84Tc-\x18+\xE5`\xE2\x1B\x85R`\x04\x85\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`$\x85\x01R\x81\x16`D\x84\x01R\x16\x93`d\x82\x01\x90V[``\x90`\x03\x19\x01\x12a\x01wW`\x045\x90`\x01`\x01`\xA0\x1B\x03\x90`$5\x82\x81\x16\x81\x03a\x01wW\x91`D5\x90\x81\x16\x81\x03a\x01wW\x90V[`\0\x80\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01\x9EW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xC8\xDB\xA4W\xB0-\xA7pA\xDB\xA4\xB9\xC9\xB2\xC8\x03\xD0\xEF\xC4\xFF\x1B\xE5m\xE6j4\x1F\xDF\x80R=hdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static REDEMPTIONPROXY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RedemptionProxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RedemptionProxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RedemptionProxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RedemptionProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RedemptionProxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RedemptionProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RedemptionProxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    REDEMPTIONPROXY_ABI.clone(),
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
                REDEMPTIONPROXY_ABI.clone(),
                REDEMPTIONPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `redeemOnBehalf` (0xca30795a) function
        pub fn redeem_on_behalf(
            &self,
            shares: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 48, 121, 90], (shares, receiver, owner))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawOnBehalf` (0x24fb569f) function
        pub fn withdraw_on_behalf(
            &self,
            tokens: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([36, 251, 86, 159], (tokens, receiver, owner))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RedemptionProxy<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `redeemOnBehalf` function with signature `redeemOnBehalf(uint256,address,address)` and selector `0xca30795a`
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
    #[ethcall(name = "redeemOnBehalf", abi = "redeemOnBehalf(uint256,address,address)")]
    pub struct RedeemOnBehalfCall {
        pub shares: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawOnBehalf` function with signature `withdrawOnBehalf(uint256,address,address)` and selector `0x24fb569f`
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
    #[ethcall(
        name = "withdrawOnBehalf",
        abi = "withdrawOnBehalf(uint256,address,address)"
    )]
    pub struct WithdrawOnBehalfCall {
        pub tokens: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum RedemptionProxyCalls {
        RedeemOnBehalf(RedeemOnBehalfCall),
        WithdrawOnBehalf(WithdrawOnBehalfCall),
    }
    impl ::ethers::core::abi::AbiDecode for RedemptionProxyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <RedeemOnBehalfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RedeemOnBehalf(decoded));
            }
            if let Ok(decoded)
                = <WithdrawOnBehalfCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WithdrawOnBehalf(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RedemptionProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::RedeemOnBehalf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawOnBehalf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RedemptionProxyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RedeemOnBehalf(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawOnBehalf(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RedeemOnBehalfCall> for RedemptionProxyCalls {
        fn from(value: RedeemOnBehalfCall) -> Self {
            Self::RedeemOnBehalf(value)
        }
    }
    impl ::core::convert::From<WithdrawOnBehalfCall> for RedemptionProxyCalls {
        fn from(value: WithdrawOnBehalfCall) -> Self {
            Self::WithdrawOnBehalf(value)
        }
    }
    ///Container type for all return fields from the `redeemOnBehalf` function with signature `redeemOnBehalf(uint256,address,address)` and selector `0xca30795a`
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
    pub struct RedeemOnBehalfReturn {
        pub tokens_withdrawn: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `withdrawOnBehalf` function with signature `withdrawOnBehalf(uint256,address,address)` and selector `0x24fb569f`
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
    pub struct WithdrawOnBehalfReturn {
        pub shares_redeemed: ::ethers::core::types::U256,
    }
}
