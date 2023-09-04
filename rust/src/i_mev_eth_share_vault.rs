pub use i_mev_eth_share_vault::*;
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
pub mod i_mev_eth_share_vault {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("fees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fees"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("logRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("logRewards"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("protocolFeesOwed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("payRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("payRewards"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recoverToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recoverToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewards"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendFees"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setNewMevEth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setNewMevEth"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newMevEth"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setProtocolFeeTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setProtocolFeeTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newFeeTo"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
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
    pub static IMEVETHSHAREVAULT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IMevEthShareVault<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IMevEthShareVault<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IMevEthShareVault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IMevEthShareVault<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IMevEthShareVault<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IMevEthShareVault))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IMevEthShareVault<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IMEVETHSHAREVAULT_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `fees` (0x9af1d35a) function
        pub fn fees(&self) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([154, 241, 211, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logRewards` (0x192e3799) function
        pub fn log_rewards(
            &self,
            protocol_fees_owed: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 46, 55, 153], protocol_fees_owed)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payRewards` (0x7288e961) function
        pub fn pay_rewards(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 136, 233, 97], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recoverToken` (0xa7229fd9) function
        pub fn recover_token(
            &self,
            token: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 34, 159, 217], (token, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewards` (0x9ec5a894) function
        pub fn rewards(&self) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([158, 197, 168, 148], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendFees` (0xdff90b5b) function
        pub fn send_fees(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 249, 11, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setNewMevEth` (0x442e493d) function
        pub fn set_new_mev_eth(
            &self,
            new_mev_eth: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 46, 73, 61], new_mev_eth)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProtocolFeeTo` (0xe0e6799f) function
        pub fn set_protocol_fee_to(
            &self,
            new_fee_to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 230, 121, 159], new_fee_to)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IMevEthShareVault<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `fees` function with signature `fees()` and selector `0x9af1d35a`
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
    #[ethcall(name = "fees", abi = "fees()")]
    pub struct FeesCall;
    ///Container type for all input parameters for the `logRewards` function with signature `logRewards(uint128)` and selector `0x192e3799`
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
    #[ethcall(name = "logRewards", abi = "logRewards(uint128)")]
    pub struct LogRewardsCall {
        pub protocol_fees_owed: u128,
    }
    ///Container type for all input parameters for the `payRewards` function with signature `payRewards()` and selector `0x7288e961`
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
    #[ethcall(name = "payRewards", abi = "payRewards()")]
    pub struct PayRewardsCall;
    ///Container type for all input parameters for the `recoverToken` function with signature `recoverToken(address,address,uint256)` and selector `0xa7229fd9`
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
    #[ethcall(name = "recoverToken", abi = "recoverToken(address,address,uint256)")]
    pub struct RecoverTokenCall {
        pub token: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `rewards` function with signature `rewards()` and selector `0x9ec5a894`
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
    #[ethcall(name = "rewards", abi = "rewards()")]
    pub struct RewardsCall;
    ///Container type for all input parameters for the `sendFees` function with signature `sendFees()` and selector `0xdff90b5b`
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
    #[ethcall(name = "sendFees", abi = "sendFees()")]
    pub struct SendFeesCall;
    ///Container type for all input parameters for the `setNewMevEth` function with signature `setNewMevEth(address)` and selector `0x442e493d`
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
    #[ethcall(name = "setNewMevEth", abi = "setNewMevEth(address)")]
    pub struct SetNewMevEthCall {
        pub new_mev_eth: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setProtocolFeeTo` function with signature `setProtocolFeeTo(address)` and selector `0xe0e6799f`
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
    #[ethcall(name = "setProtocolFeeTo", abi = "setProtocolFeeTo(address)")]
    pub struct SetProtocolFeeToCall {
        pub new_fee_to: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IMevEthShareVaultCalls {
        Fees(FeesCall),
        LogRewards(LogRewardsCall),
        PayRewards(PayRewardsCall),
        RecoverToken(RecoverTokenCall),
        Rewards(RewardsCall),
        SendFees(SendFeesCall),
        SetNewMevEth(SetNewMevEthCall),
        SetProtocolFeeTo(SetProtocolFeeToCall),
    }
    impl ::ethers::core::abi::AbiDecode for IMevEthShareVaultCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <FeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fees(decoded));
            }
            if let Ok(decoded)
                = <LogRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogRewards(decoded));
            }
            if let Ok(decoded)
                = <PayRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PayRewards(decoded));
            }
            if let Ok(decoded)
                = <RecoverTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RecoverToken(decoded));
            }
            if let Ok(decoded)
                = <RewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rewards(decoded));
            }
            if let Ok(decoded)
                = <SendFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendFees(decoded));
            }
            if let Ok(decoded)
                = <SetNewMevEthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetNewMevEth(decoded));
            }
            if let Ok(decoded)
                = <SetProtocolFeeToCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetProtocolFeeTo(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IMevEthShareVaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Fees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LogRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PayRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecoverToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rewards(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SendFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetNewMevEth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProtocolFeeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IMevEthShareVaultCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Fees(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecoverToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNewMevEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFeeTo(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FeesCall> for IMevEthShareVaultCalls {
        fn from(value: FeesCall) -> Self {
            Self::Fees(value)
        }
    }
    impl ::core::convert::From<LogRewardsCall> for IMevEthShareVaultCalls {
        fn from(value: LogRewardsCall) -> Self {
            Self::LogRewards(value)
        }
    }
    impl ::core::convert::From<PayRewardsCall> for IMevEthShareVaultCalls {
        fn from(value: PayRewardsCall) -> Self {
            Self::PayRewards(value)
        }
    }
    impl ::core::convert::From<RecoverTokenCall> for IMevEthShareVaultCalls {
        fn from(value: RecoverTokenCall) -> Self {
            Self::RecoverToken(value)
        }
    }
    impl ::core::convert::From<RewardsCall> for IMevEthShareVaultCalls {
        fn from(value: RewardsCall) -> Self {
            Self::Rewards(value)
        }
    }
    impl ::core::convert::From<SendFeesCall> for IMevEthShareVaultCalls {
        fn from(value: SendFeesCall) -> Self {
            Self::SendFees(value)
        }
    }
    impl ::core::convert::From<SetNewMevEthCall> for IMevEthShareVaultCalls {
        fn from(value: SetNewMevEthCall) -> Self {
            Self::SetNewMevEth(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeeToCall> for IMevEthShareVaultCalls {
        fn from(value: SetProtocolFeeToCall) -> Self {
            Self::SetProtocolFeeTo(value)
        }
    }
    ///Container type for all return fields from the `fees` function with signature `fees()` and selector `0x9af1d35a`
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
    pub struct FeesReturn(pub u128);
    ///Container type for all return fields from the `rewards` function with signature `rewards()` and selector `0x9ec5a894`
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
    pub struct RewardsReturn(pub u128);
}
