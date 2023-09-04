pub use mev_eth_share_vault::*;
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
pub mod mev_eth_share_vault {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("authority"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_mevEth"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_protocolFeeTo"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
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
                    ::std::borrow::ToOwned::to_owned("addOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addOperator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOperator"),
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
                    ::std::borrow::ToOwned::to_owned("admins"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("admins"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deleteAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deleteAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oldAdmin"),
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
                    ::std::borrow::ToOwned::to_owned("deleteOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deleteOperator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oldOperator"),
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
                    ::std::borrow::ToOwned::to_owned("mevEth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mevEth"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("operators"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("payValidatorWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "payValidatorWithdraw",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("protocolBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("protocolBalance"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rewards"),
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
                    ::std::borrow::ToOwned::to_owned("protocolFeeTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("protocolFeeTo"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                                    name: ::std::borrow::ToOwned::to_owned("newProtocolFeeTo"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AdminAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AdminDeleted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminDeleted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeesSent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FeesSent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feesSent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MevEthUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MevEthUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("meveth"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OperatorAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOperator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorDeleted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OperatorDeleted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldOperator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProtocolFeeToUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ProtocolFeeToUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newProtocolFeeTo"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RewardPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RewardPayment"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("coinbase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RewardsCollected"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RewardsCollected"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("protocolFeesOwed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rewardsOwed"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RewardsPaid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RewardsPaid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rewards"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenRecovered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokenRecovered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ValidatorWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ValidatorWithdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AlreadySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AlreadySet"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeesTooHigh"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FeesTooHigh"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NoAdmin"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotEnoughEth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotEnoughEth"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SendError"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SendError"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unauthorized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Unauthorized"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroAddress"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MEVETHSHAREVAULT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x804a\x01\x02W`\x1Fa\x0C\xD48\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x92\x91\x90`\x01`\x01`@\x1B\x03\x84\x11\x83\x85\x10\x17a\x01\x07W\x81``\x92\x84\x92`@\x96\x87R\x839\x81\x01\x03\x12a\x01\x02Wa\0K\x81a\x01\x1DV[\x90a\0c\x83a\0\\` \x84\x01a\x01\x1DV[\x92\x01a\x01\x1DV[\x91`\x01\x80`\xA0\x1B\x03\x80\x80\x92\x16\x93\x84`\0R`\x02` R\x85`\0 `\xFF\x19\x90`\x01\x82\x82T\x16\x17\x90U`\0T\x81`\xFF`\x01\x81\x84\x16\x01\x16\x91\x16\x17`\0U`\x01` R`\x01\x87`\0 \x91\x82T\x16\x17\x90U\x16\x92\x83\x15\x90\x81\x15a\0\xF9W[Pa\0\xE8W`\x01\x80`\xA0\x1B\x03\x19\x91\x16\x81`\x04T\x16\x17`\x04U`\x05T\x16\x17`\x05UQa\x0B\xA2\x90\x81a\x012\x829\xF3[\x83Qc\xD9.#=`\xE0\x1B\x81R`\x04\x90\xFD[\x90P\x158a\0\xBBV[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\x02WV\xFE`\x80`@\x81\x81R`\x04\x91\x826\x10\x15a\0JW[PPP6\x15a\0 W`\0\x80\xFD[4AC\x7F\xBC\x0B|\x01g7\x13\xEB\xEF\x84\xF2\xB0(\x9C`\x1F3\x826Q\xDF9\xC4\xBB~Z\x91\xA2\x95\xF9\xA3X`\0\x80\xA4\0[`\0\x92\x835`\xE0\x1C\x91\x82c\x13\xE7\xC9\xD8\x14a\n\xB4WP\x81c\x14\xB7\x1A\x83\x14a\t\xBBW\x81c\x19.7\x99\x14a\x08\x90W\x81c'\xE1\xF7\xDF\x14a\x07\xB5W\x81cB\x9Bb\xE5\x14a\x07yW\x81cD.I=\x14a\x06\xF5W\x81ca\xE9\x8D\xB8\x14a\x06\xC6W\x81cpH\x02u\x14a\x05\xEEW\x81cr\x88\xE9a\x14a\x051W\x81c\x98p\xD7\xFE\x14a\x04\xA7W\x81c\x9A\xF1\xD3Z\x14a\x04\x7FW\x81c\x9E\xC5\xA8\x94\x14a\x04]W\x81c\xA7\"\x9F\xD9\x14a\x03]W\x81c\xB4\t\x92\xA1\x14a\x02\xBAW\x81c\xBA\xDF&c\x14a\x02\x90W\x81c\xDF\xF9\x0B[\x14a\x01\xE3W\x81c\xE0\xE6y\x9F\x14a\x01JWPc\xEF6\x92R\x14a\x01 W\x80a\0\x12V[4a\x01FW\x81`\x03\x196\x01\x12a\x01FW` \x90`\x01`\x01`\xA0\x1B\x03`\x05T\x16\x90Q\x90\x81R\xF3[P\x80\xFD[\x90P4a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFWa\x01ea\n\xEEV[3\x84R`\x02` R`\xFF\x83\x85 T\x16\x15a\x01\xD1W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15a\x01\xC4WPP\x80`\x01`\x01`\xA0\x1B\x03\x19`\x05T\x16\x17`\x05U\x7FS\x84\xCD\xF418\x10\xFA\xBC\x89B\x9F\xAF \xF0\x12\xEF\xAAq\xE0\xA3\xBD\xEB\xC9\xC0\xA6\xEC\xB9\xFE\x1F\x98\xE8\x82\x80\xA2\x80\xF3[Qc\xD9.#=`\xE0\x1B\x81R\xFD[P\x90Qb\x82\xB4)`\xE8\x1B\x81R\xFD[\x82\x80\xFD[\x90P4a\x01\xDFW\x82`\x03\x196\x01\x12a\x01\xDFW3\x83R`\x02` R`\xFF\x82\x84 T\x16\x15a\x02\x83W`\x03T\x91`\x01`\x01`\x80\x1B\x03\x83\x16\x92`\x01`\x01`\x80\x1B\x03\x19\x16`\x03U\x83\x80\x80\x80\x86`\x01`\x01`\xA0\x1B\x03`\x05T\x16\x82\x82\x15a\x02zW[\xF1\x15a\x02mWPP\x7F\xCC,|f\xF7/e\0\xCD\x1C\xE7\xD3\xDF\xD9>\x02\xF00k\x97\xEC\xDA\xC4\xED?@\xFBE?\x91\xD8{\x82\x80\xA2\x80\xF3[Qc\x8AH\xAA\x1F`\xE0\x1B\x81R\xFD[Pa\x08\xFCa\x02>V[\x90Qb\x82\xB4)`\xE8\x1B\x81R\xFD[\x82\x844a\x02\xB7W\x80`\x03\x196\x01\x12a\x02\xB7WP`\x01`\x01`\xA0\x1B\x03` \x92T\x16\x90Q\x90\x81R\xF3[\x80\xFD[\x91\x90P4a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFWa\x02\xD6a\n\xEEV[3\x84R`\x02` R`\xFF\x82\x85 T\x16\x15a\x03PW`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x84R`\x01` R`\xFF\x82\x85 T\x16\x15a\x03BWP\x81\x83R`\x01` R\x82 \x80T`\xFF\x19\x16\x90U\x7Fi\xDF,^\xC2\xEAM\x1F\xBE\x1EP5$\xF5\x93\xB3V\x16,\xA7\x10g\x12c\x82\x7F.\x19\x92\xB9Z\xE1\x82\x80\xA2\x80\xF3[\x90Qc\xA7A\xA0E`\xE0\x1B\x81R\xFD[PQb\x82\xB4)`\xE8\x1B\x81R\xFD[\x91\x90P4a\x01\xDFW``6`\x03\x19\x01\x12a\x01\xDFWa\x03ya\n\xEEV[\x91`$5\x90`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x80\x93\x03a\x04XW`D5\x943\x87R`\x02` R`\xFF\x85\x88 T\x16\x15a\x04IW\x16\x92` \x86`D\x83Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x86\x86\x82\x01R\x88`$\x82\x01R\x82\x88Z\xF1=\x15`\x1F=\x11`\x01\x89Q\x14\x16\x17\x16\x15a\x04\x06WPP\x7F\x87\x9F\x92\xDD\xED\x0F&\xB8<>\0\xB1.\x03\x95\xDCr\xCF\xC3\x07sC\xD1\x85N\xD6\x98\x8E\xDD\x1F\x90\x96\x84\x80\xA4\x80\xF3[\x90` `d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x84Qb\x82\xB4)`\xE8\x1B\x81R\x83\x90\xFD[`\0\x80\xFD[PP4a\x01FW\x81`\x03\x196\x01\x12a\x01FW` \x90`\x03T`\x80\x1C\x90Q\x90\x81R\xF3[PP4a\x01FW\x81`\x03\x196\x01\x12a\x01FW` \x90`\x01`\x01`\x80\x1B\x03`\x03T\x16\x90Q\x90\x81R\xF3[\x91\x90P4a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFWa\x04\xC3a\n\xEEV[3\x84R`\x02` R`\xFF\x82\x85 T\x16\x15a\x03PW`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x84R`\x01` R`\xFF\x82\x85 T\x16a\x03BWP\x81\x83R`\x01` R\x82 `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\xACo\xA8X\xE95\nF\xCE\xC1e9\x92n\x0F\xDE%\xB7b\x9F\x84\xB5\xA7+\xFF\xAA\xE4\xDF\x88\x8A\xE8m\x82\x80\xA2\x80\xF3[\x90P4a\x01\xDFW\x82`\x03\x196\x01\x12a\x01\xDFW3\x83R`\x01` R`\xFF\x82\x84 T\x16\x15a\x02\x83W\x82\x90`\x03T\x92`\x01`\x01`\x80\x1B\x03\x84`\x80\x1C\x94\x16`\x03U`\x01`\x01`\xA0\x1B\x03\x82T\x16\x91\x82;\x15a\x05\xEAW\x81QcU\x8C\xB7\xF7`\xE0\x1B\x81R\x92\x84\x91\x84\x91\x82\x90\x88\x90Z\xF1\x90\x81\x15a\x05\xE1WPa\x05\xCDW[P\x80\x7F\xB0\xC6Z[20\"\xD9&\xC4V\xE1VM\x86\xF0\xBD\xA4\x01`\xA7\x81\xC8 \xF5q\xD7c[4\x88\x01\x91\xA2\x80\xF3[a\x05\xD6\x90a\x0B\x04V[a\x01FW\x818a\x05\xA5V[Q=\x84\x82>=\x90\xFD[\x83\x80\xFD[\x83\x834a\x01FW` 6`\x03\x19\x01\x12a\x01FWa\x06\ta\n\xEEV[3\x83R`\x02` R`\xFF\x82\x84 T\x16\x15a\x06\xB7W`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x83R`\x02` R`\xFF\x81\x84 T\x16a\x06\xA8W\x82T`\xFF\x81\x16`\xFF\x81\x14a\x06\x95W`\xFF\x19\x91\x82\x16`\x01\x91\x82\x01`\xFF\x16\x17\x85U\x83\x85R`\x02` R\x91\x84 \x80T\x90\x91\x16\x90\x91\x17\x90U\x7FD\xD6\xD2Yc\xF0\x97\xAD\x14\xF2\x9F\x06\x85J\x01\xF5ud\x8A\x1E\xF8/0\xE5b\xCC\xD3\x88\x97\x17\xE39\x82\x80\xA2\x80\xF3[cNH{q`\xE0\x1B\x85R`\x11\x86R`$\x85\xFD[Qc\xA7A\xA0E`\xE0\x1B\x81R\x83\x90\xFD[\x81Qb\x82\xB4)`\xE8\x1B\x81R\x84\x90\xFD[\x82\x844a\x02\xB7W\x80`\x03\x196\x01\x12a\x02\xB7WP`\x03T\x81Q\x90`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x80\x1C` \x82\x01R\xF3[\x90P4a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFWa\x07\x10a\n\xEEV[3\x84R`\x02` R`\xFF\x83\x85 T\x16\x15a\x01\xD1W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15a\x01\xC4WP\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82\x17\x90U\x7F\x05\xBF\\\xA0b)=\xE5g\xF8C\xF2\xE7\xE8\xB0$\xD0\xB1Ukg\x98T\xE7\x9FS\xC1\x94\x02\xA8;\x07\x82\x80\xA2\x80\xF3[PP4a\x01FW` 6`\x03\x19\x01\x12a\x01FW`\xFF\x81` \x93`\x01`\x01`\xA0\x1B\x03a\x07\xA2a\n\xEEV[\x16\x81R`\x02\x85R T\x16\x90Q\x90\x15\x15\x81R\xF3[\x90P4a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFWa\x07\xD0a\n\xEEV[3\x84R`\x02` R`\xFF\x83\x85 T\x16\x15a\x01\xD1W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x84R`\x02` R`\xFF\x81\x85 T\x16\x15a\x08\x83W\x83T\x91`\xFF\x83\x16\x80\x15a\x08pW`\xFF\x90`\0\x19\x01\x16\x85`\xFF\x19\x94\x82\x86\x82\x16\x17\x82U\x16\x17\x15a\x08bWP\x82\x84R`\x02` R\x83 \x90\x81T\x16\x90U\x7F\x98\x9D\xDF\xCE\x05}\xAD!\x9E\n\xE1oi\x1B\x12\x1B\xB0\xE3H\xF0\xD8\xAE\n\xD4\0\xB4\xD5\xAC\x8Dal\x8B\x82\x80\xA2\x80\xF3[\x90Qc\x1F\x8C\x1D\xBD`\xE1\x1B\x81R\xFD[cNH{q`\xE0\x1B\x86R`\x11\x82R`$\x86\xFD[Qc\xA7A\xA0E`\xE0\x1B\x81R\xFD[\x90P4a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFW`\x01`\x01`\x80\x1B\x03\x815\x81\x81\x16\x93\x91\x84\x82\x03a\t\xB7W3\x86R`\x01` R`\xFF\x83\x87 T\x16\x15a\t\xA9W\x82Q\x91\x83\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\x94W\x84R`\x03T\x93` \x83\x86\x16\x94\x85\x81R\x86`\x80\x1C\x91\x82\x91\x01Ra\t\x14G\x85a\t\r\x84\x89a\x0B.V[\x16\x90a\x0B_V[\x96\x84\x88\x16\x92\x83\x8A\x11a\t\x87WPP\x92a\tR\x83\x8A\x93a\tH\x8B\x99\x97\x85a\t_\x9C\x9A\x98`\x01`\x01`\x80\x1B\x03\x19\x94\x03\x16\x90a\x0B.V[`\x80\x1B\x16\x94a\x0B.V[\x16\x92\x16\x17\x17`\x03Ua\x0B_V[\x90\x7F\x86\x11dC\xBDc\xD6G\xE9\xAA.\t?\x84~\xF3\x0E\xE2\xA3Y\xFF\xCB\xA4\xBD\xF4\xBB~\xB0\x92\x9C\xCE\x8B\x83\x80\xA3\x80\xF3[Qc\x19 i\xC3`\xE3\x1B\x81R\xFD[`A\x86cNH{q`\xE0\x1B`\0RR`$`\0\xFD[PPQb\x82\xB4)`\xE8\x1B\x81R\xFD[\x85\x80\xFD[\x83\x834a\x01FW\x81`\x03\x196\x01\x12a\x01FW3\x82R`\x01` R`\xFF\x81\x83 T\x16\x15a\n\xA6Wh\x01\xBC\x16\xD6t\xEC\x80\0\0\x92G\x84\x11a\n\x98W`\x01`\x01`\xA0\x1B\x03\x81T\x16\x90\x81;\x15a\x05\xEAW\x82Qc\xFE\x182\x11`\xE0\x1B\x81R\x91\x84\x91\x83\x91\x82\x90\x88\x90Z\xF1\x80\x15a\n\x8CWa\n[W[P\x7F\x12\xB9d\xA3\x99=\x15\x98\xDD\x8A;bz;\x90\xB4\xBCkz\x8FO\x8B\xB6\xAF\xDE\x02\xA3\r\x17\x8E(\xEF\x91\x92\x81Q\x903\x82R` \x82\x01R\xA1\x80\xF3[\x91a\n\x86\x7F\x12\xB9d\xA3\x99=\x15\x98\xDD\x8A;bz;\x90\xB4\xBCkz\x8FO\x8B\xB6\xAF\xDE\x02\xA3\r\x17\x8E(\xEF\x93a\x0B\x04V[\x91a\n(V[PPQ\x90=\x90\x82>=\x90\xFD[\x90Qc\xF1JB\xB7`\xE0\x1B\x81R\xFD[Qb\x82\xB4)`\xE8\x1B\x81R\x90P\xFD[\x84\x90\x844a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFW`\xFF\x90` \x93`\x01`\x01`\xA0\x1B\x03a\n\xDEa\n\xEEV[\x16\x81R`\x01\x85R T\x16\x15\x15\x81R\xF3[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x04XWV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0B\x18W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x91\x90\x91`\x01`\x01`\x80\x1B\x03\x80\x80\x94\x16\x91\x16\x01\x91\x82\x11a\x0BIWV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x0BIWV\xFE\xA2dipfsX\"\x12 .}\xC3\xBFs\x9E\x17\xBB\xBE\xCC/;\x9BA\xDC[\xEDG\x1C\xDA\xC0\x11\x19[\xFD\x1F\xF5\xE0\x82\xE9\x01\xF7dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MEVETHSHAREVAULT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x91\x826\x10\x15a\0JW[PPP6\x15a\0 W`\0\x80\xFD[4AC\x7F\xBC\x0B|\x01g7\x13\xEB\xEF\x84\xF2\xB0(\x9C`\x1F3\x826Q\xDF9\xC4\xBB~Z\x91\xA2\x95\xF9\xA3X`\0\x80\xA4\0[`\0\x92\x835`\xE0\x1C\x91\x82c\x13\xE7\xC9\xD8\x14a\n\xB4WP\x81c\x14\xB7\x1A\x83\x14a\t\xBBW\x81c\x19.7\x99\x14a\x08\x90W\x81c'\xE1\xF7\xDF\x14a\x07\xB5W\x81cB\x9Bb\xE5\x14a\x07yW\x81cD.I=\x14a\x06\xF5W\x81ca\xE9\x8D\xB8\x14a\x06\xC6W\x81cpH\x02u\x14a\x05\xEEW\x81cr\x88\xE9a\x14a\x051W\x81c\x98p\xD7\xFE\x14a\x04\xA7W\x81c\x9A\xF1\xD3Z\x14a\x04\x7FW\x81c\x9E\xC5\xA8\x94\x14a\x04]W\x81c\xA7\"\x9F\xD9\x14a\x03]W\x81c\xB4\t\x92\xA1\x14a\x02\xBAW\x81c\xBA\xDF&c\x14a\x02\x90W\x81c\xDF\xF9\x0B[\x14a\x01\xE3W\x81c\xE0\xE6y\x9F\x14a\x01JWPc\xEF6\x92R\x14a\x01 W\x80a\0\x12V[4a\x01FW\x81`\x03\x196\x01\x12a\x01FW` \x90`\x01`\x01`\xA0\x1B\x03`\x05T\x16\x90Q\x90\x81R\xF3[P\x80\xFD[\x90P4a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFWa\x01ea\n\xEEV[3\x84R`\x02` R`\xFF\x83\x85 T\x16\x15a\x01\xD1W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15a\x01\xC4WPP\x80`\x01`\x01`\xA0\x1B\x03\x19`\x05T\x16\x17`\x05U\x7FS\x84\xCD\xF418\x10\xFA\xBC\x89B\x9F\xAF \xF0\x12\xEF\xAAq\xE0\xA3\xBD\xEB\xC9\xC0\xA6\xEC\xB9\xFE\x1F\x98\xE8\x82\x80\xA2\x80\xF3[Qc\xD9.#=`\xE0\x1B\x81R\xFD[P\x90Qb\x82\xB4)`\xE8\x1B\x81R\xFD[\x82\x80\xFD[\x90P4a\x01\xDFW\x82`\x03\x196\x01\x12a\x01\xDFW3\x83R`\x02` R`\xFF\x82\x84 T\x16\x15a\x02\x83W`\x03T\x91`\x01`\x01`\x80\x1B\x03\x83\x16\x92`\x01`\x01`\x80\x1B\x03\x19\x16`\x03U\x83\x80\x80\x80\x86`\x01`\x01`\xA0\x1B\x03`\x05T\x16\x82\x82\x15a\x02zW[\xF1\x15a\x02mWPP\x7F\xCC,|f\xF7/e\0\xCD\x1C\xE7\xD3\xDF\xD9>\x02\xF00k\x97\xEC\xDA\xC4\xED?@\xFBE?\x91\xD8{\x82\x80\xA2\x80\xF3[Qc\x8AH\xAA\x1F`\xE0\x1B\x81R\xFD[Pa\x08\xFCa\x02>V[\x90Qb\x82\xB4)`\xE8\x1B\x81R\xFD[\x82\x844a\x02\xB7W\x80`\x03\x196\x01\x12a\x02\xB7WP`\x01`\x01`\xA0\x1B\x03` \x92T\x16\x90Q\x90\x81R\xF3[\x80\xFD[\x91\x90P4a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFWa\x02\xD6a\n\xEEV[3\x84R`\x02` R`\xFF\x82\x85 T\x16\x15a\x03PW`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x84R`\x01` R`\xFF\x82\x85 T\x16\x15a\x03BWP\x81\x83R`\x01` R\x82 \x80T`\xFF\x19\x16\x90U\x7Fi\xDF,^\xC2\xEAM\x1F\xBE\x1EP5$\xF5\x93\xB3V\x16,\xA7\x10g\x12c\x82\x7F.\x19\x92\xB9Z\xE1\x82\x80\xA2\x80\xF3[\x90Qc\xA7A\xA0E`\xE0\x1B\x81R\xFD[PQb\x82\xB4)`\xE8\x1B\x81R\xFD[\x91\x90P4a\x01\xDFW``6`\x03\x19\x01\x12a\x01\xDFWa\x03ya\n\xEEV[\x91`$5\x90`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x80\x93\x03a\x04XW`D5\x943\x87R`\x02` R`\xFF\x85\x88 T\x16\x15a\x04IW\x16\x92` \x86`D\x83Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x86\x86\x82\x01R\x88`$\x82\x01R\x82\x88Z\xF1=\x15`\x1F=\x11`\x01\x89Q\x14\x16\x17\x16\x15a\x04\x06WPP\x7F\x87\x9F\x92\xDD\xED\x0F&\xB8<>\0\xB1.\x03\x95\xDCr\xCF\xC3\x07sC\xD1\x85N\xD6\x98\x8E\xDD\x1F\x90\x96\x84\x80\xA4\x80\xF3[\x90` `d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x84Qb\x82\xB4)`\xE8\x1B\x81R\x83\x90\xFD[`\0\x80\xFD[PP4a\x01FW\x81`\x03\x196\x01\x12a\x01FW` \x90`\x03T`\x80\x1C\x90Q\x90\x81R\xF3[PP4a\x01FW\x81`\x03\x196\x01\x12a\x01FW` \x90`\x01`\x01`\x80\x1B\x03`\x03T\x16\x90Q\x90\x81R\xF3[\x91\x90P4a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFWa\x04\xC3a\n\xEEV[3\x84R`\x02` R`\xFF\x82\x85 T\x16\x15a\x03PW`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x84R`\x01` R`\xFF\x82\x85 T\x16a\x03BWP\x81\x83R`\x01` R\x82 `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\xACo\xA8X\xE95\nF\xCE\xC1e9\x92n\x0F\xDE%\xB7b\x9F\x84\xB5\xA7+\xFF\xAA\xE4\xDF\x88\x8A\xE8m\x82\x80\xA2\x80\xF3[\x90P4a\x01\xDFW\x82`\x03\x196\x01\x12a\x01\xDFW3\x83R`\x01` R`\xFF\x82\x84 T\x16\x15a\x02\x83W\x82\x90`\x03T\x92`\x01`\x01`\x80\x1B\x03\x84`\x80\x1C\x94\x16`\x03U`\x01`\x01`\xA0\x1B\x03\x82T\x16\x91\x82;\x15a\x05\xEAW\x81QcU\x8C\xB7\xF7`\xE0\x1B\x81R\x92\x84\x91\x84\x91\x82\x90\x88\x90Z\xF1\x90\x81\x15a\x05\xE1WPa\x05\xCDW[P\x80\x7F\xB0\xC6Z[20\"\xD9&\xC4V\xE1VM\x86\xF0\xBD\xA4\x01`\xA7\x81\xC8 \xF5q\xD7c[4\x88\x01\x91\xA2\x80\xF3[a\x05\xD6\x90a\x0B\x04V[a\x01FW\x818a\x05\xA5V[Q=\x84\x82>=\x90\xFD[\x83\x80\xFD[\x83\x834a\x01FW` 6`\x03\x19\x01\x12a\x01FWa\x06\ta\n\xEEV[3\x83R`\x02` R`\xFF\x82\x84 T\x16\x15a\x06\xB7W`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x83R`\x02` R`\xFF\x81\x84 T\x16a\x06\xA8W\x82T`\xFF\x81\x16`\xFF\x81\x14a\x06\x95W`\xFF\x19\x91\x82\x16`\x01\x91\x82\x01`\xFF\x16\x17\x85U\x83\x85R`\x02` R\x91\x84 \x80T\x90\x91\x16\x90\x91\x17\x90U\x7FD\xD6\xD2Yc\xF0\x97\xAD\x14\xF2\x9F\x06\x85J\x01\xF5ud\x8A\x1E\xF8/0\xE5b\xCC\xD3\x88\x97\x17\xE39\x82\x80\xA2\x80\xF3[cNH{q`\xE0\x1B\x85R`\x11\x86R`$\x85\xFD[Qc\xA7A\xA0E`\xE0\x1B\x81R\x83\x90\xFD[\x81Qb\x82\xB4)`\xE8\x1B\x81R\x84\x90\xFD[\x82\x844a\x02\xB7W\x80`\x03\x196\x01\x12a\x02\xB7WP`\x03T\x81Q\x90`\x01`\x01`\x80\x1B\x03\x81\x16\x82R`\x80\x1C` \x82\x01R\xF3[\x90P4a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFWa\x07\x10a\n\xEEV[3\x84R`\x02` R`\xFF\x83\x85 T\x16\x15a\x01\xD1W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15a\x01\xC4WP\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82\x17\x90U\x7F\x05\xBF\\\xA0b)=\xE5g\xF8C\xF2\xE7\xE8\xB0$\xD0\xB1Ukg\x98T\xE7\x9FS\xC1\x94\x02\xA8;\x07\x82\x80\xA2\x80\xF3[PP4a\x01FW` 6`\x03\x19\x01\x12a\x01FW`\xFF\x81` \x93`\x01`\x01`\xA0\x1B\x03a\x07\xA2a\n\xEEV[\x16\x81R`\x02\x85R T\x16\x90Q\x90\x15\x15\x81R\xF3[\x90P4a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFWa\x07\xD0a\n\xEEV[3\x84R`\x02` R`\xFF\x83\x85 T\x16\x15a\x01\xD1W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x84R`\x02` R`\xFF\x81\x85 T\x16\x15a\x08\x83W\x83T\x91`\xFF\x83\x16\x80\x15a\x08pW`\xFF\x90`\0\x19\x01\x16\x85`\xFF\x19\x94\x82\x86\x82\x16\x17\x82U\x16\x17\x15a\x08bWP\x82\x84R`\x02` R\x83 \x90\x81T\x16\x90U\x7F\x98\x9D\xDF\xCE\x05}\xAD!\x9E\n\xE1oi\x1B\x12\x1B\xB0\xE3H\xF0\xD8\xAE\n\xD4\0\xB4\xD5\xAC\x8Dal\x8B\x82\x80\xA2\x80\xF3[\x90Qc\x1F\x8C\x1D\xBD`\xE1\x1B\x81R\xFD[cNH{q`\xE0\x1B\x86R`\x11\x82R`$\x86\xFD[Qc\xA7A\xA0E`\xE0\x1B\x81R\xFD[\x90P4a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFW`\x01`\x01`\x80\x1B\x03\x815\x81\x81\x16\x93\x91\x84\x82\x03a\t\xB7W3\x86R`\x01` R`\xFF\x83\x87 T\x16\x15a\t\xA9W\x82Q\x91\x83\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\x94W\x84R`\x03T\x93` \x83\x86\x16\x94\x85\x81R\x86`\x80\x1C\x91\x82\x91\x01Ra\t\x14G\x85a\t\r\x84\x89a\x0B.V[\x16\x90a\x0B_V[\x96\x84\x88\x16\x92\x83\x8A\x11a\t\x87WPP\x92a\tR\x83\x8A\x93a\tH\x8B\x99\x97\x85a\t_\x9C\x9A\x98`\x01`\x01`\x80\x1B\x03\x19\x94\x03\x16\x90a\x0B.V[`\x80\x1B\x16\x94a\x0B.V[\x16\x92\x16\x17\x17`\x03Ua\x0B_V[\x90\x7F\x86\x11dC\xBDc\xD6G\xE9\xAA.\t?\x84~\xF3\x0E\xE2\xA3Y\xFF\xCB\xA4\xBD\xF4\xBB~\xB0\x92\x9C\xCE\x8B\x83\x80\xA3\x80\xF3[Qc\x19 i\xC3`\xE3\x1B\x81R\xFD[`A\x86cNH{q`\xE0\x1B`\0RR`$`\0\xFD[PPQb\x82\xB4)`\xE8\x1B\x81R\xFD[\x85\x80\xFD[\x83\x834a\x01FW\x81`\x03\x196\x01\x12a\x01FW3\x82R`\x01` R`\xFF\x81\x83 T\x16\x15a\n\xA6Wh\x01\xBC\x16\xD6t\xEC\x80\0\0\x92G\x84\x11a\n\x98W`\x01`\x01`\xA0\x1B\x03\x81T\x16\x90\x81;\x15a\x05\xEAW\x82Qc\xFE\x182\x11`\xE0\x1B\x81R\x91\x84\x91\x83\x91\x82\x90\x88\x90Z\xF1\x80\x15a\n\x8CWa\n[W[P\x7F\x12\xB9d\xA3\x99=\x15\x98\xDD\x8A;bz;\x90\xB4\xBCkz\x8FO\x8B\xB6\xAF\xDE\x02\xA3\r\x17\x8E(\xEF\x91\x92\x81Q\x903\x82R` \x82\x01R\xA1\x80\xF3[\x91a\n\x86\x7F\x12\xB9d\xA3\x99=\x15\x98\xDD\x8A;bz;\x90\xB4\xBCkz\x8FO\x8B\xB6\xAF\xDE\x02\xA3\r\x17\x8E(\xEF\x93a\x0B\x04V[\x91a\n(V[PPQ\x90=\x90\x82>=\x90\xFD[\x90Qc\xF1JB\xB7`\xE0\x1B\x81R\xFD[Qb\x82\xB4)`\xE8\x1B\x81R\x90P\xFD[\x84\x90\x844a\x01\xDFW` 6`\x03\x19\x01\x12a\x01\xDFW`\xFF\x90` \x93`\x01`\x01`\xA0\x1B\x03a\n\xDEa\n\xEEV[\x16\x81R`\x01\x85R T\x16\x15\x15\x81R\xF3[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x04XWV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0B\x18W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x91\x90\x91`\x01`\x01`\x80\x1B\x03\x80\x80\x94\x16\x91\x16\x01\x91\x82\x11a\x0BIWV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x0BIWV\xFE\xA2dipfsX\"\x12 .}\xC3\xBFs\x9E\x17\xBB\xBE\xCC/;\x9BA\xDC[\xEDG\x1C\xDA\xC0\x11\x19[\xFD\x1F\xF5\xE0\x82\xE9\x01\xF7dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MEVETHSHAREVAULT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MevEthShareVault<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MevEthShareVault<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MevEthShareVault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MevEthShareVault<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MevEthShareVault<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MevEthShareVault))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MevEthShareVault<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MEVETHSHAREVAULT_ABI.clone(),
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
                MEVETHSHAREVAULT_ABI.clone(),
                MEVETHSHAREVAULT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addAdmin` (0x70480275) function
        pub fn add_admin(
            &self,
            new_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 72, 2, 117], new_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addOperator` (0x9870d7fe) function
        pub fn add_operator(
            &self,
            new_operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([152, 112, 215, 254], new_operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `admins` (0x429b62e5) function
        pub fn admins(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([66, 155, 98, 229], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deleteAdmin` (0x27e1f7df) function
        pub fn delete_admin(
            &self,
            old_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 225, 247, 223], old_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deleteOperator` (0xb40992a1) function
        pub fn delete_operator(
            &self,
            old_operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 9, 146, 161], old_operator)
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `mevEth` (0xbadf2663) function
        pub fn mev_eth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([186, 223, 38, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operators` (0x13e7c9d8) function
        pub fn operators(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([19, 231, 201, 216], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payRewards` (0x7288e961) function
        pub fn pay_rewards(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 136, 233, 97], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payValidatorWithdraw` (0x14b71a83) function
        pub fn pay_validator_withdraw(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 183, 26, 131], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `protocolBalance` (0x61e98db8) function
        pub fn protocol_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([97, 233, 141, 184], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `protocolFeeTo` (0xef369252) function
        pub fn protocol_fee_to(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([239, 54, 146, 82], ())
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
            new_protocol_fee_to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 230, 121, 159], new_protocol_fee_to)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AdminAdded` event
        pub fn admin_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AdminDeleted` event
        pub fn admin_deleted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminDeletedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FeesSent` event
        pub fn fees_sent_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FeesSentFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MevEthUpdated` event
        pub fn mev_eth_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MevEthUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorAdded` event
        pub fn operator_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorDeleted` event
        pub fn operator_deleted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorDeletedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProtocolFeeToUpdated` event
        pub fn protocol_fee_to_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProtocolFeeToUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RewardPayment` event
        pub fn reward_payment_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RewardPaymentFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RewardsCollected` event
        pub fn rewards_collected_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RewardsCollectedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RewardsPaid` event
        pub fn rewards_paid_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RewardsPaidFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokenRecovered` event
        pub fn token_recovered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenRecoveredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ValidatorWithdraw` event
        pub fn validator_withdraw_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ValidatorWithdrawFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MevEthShareVaultEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MevEthShareVault<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AlreadySet` with signature `AlreadySet()` and selector `0xa741a045`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AlreadySet", abi = "AlreadySet()")]
    pub struct AlreadySet;
    ///Custom Error type `FeesTooHigh` with signature `FeesTooHigh()` and selector `0xc9034e18`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FeesTooHigh", abi = "FeesTooHigh()")]
    pub struct FeesTooHigh;
    ///Custom Error type `NoAdmin` with signature `NoAdmin()` and selector `0x3f183b7a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NoAdmin", abi = "NoAdmin()")]
    pub struct NoAdmin;
    ///Custom Error type `NotEnoughEth` with signature `NotEnoughEth()` and selector `0xf14a42b7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotEnoughEth", abi = "NotEnoughEth()")]
    pub struct NotEnoughEth;
    ///Custom Error type `SendError` with signature `SendError()` and selector `0x8a48aa1f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SendError", abi = "SendError()")]
    pub struct SendError;
    ///Custom Error type `Unauthorized` with signature `Unauthorized()` and selector `0x82b42900`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Unauthorized", abi = "Unauthorized()")]
    pub struct Unauthorized;
    ///Custom Error type `ZeroAddress` with signature `ZeroAddress()` and selector `0xd92e233d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ZeroAddress", abi = "ZeroAddress()")]
    pub struct ZeroAddress;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MevEthShareVaultErrors {
        AlreadySet(AlreadySet),
        FeesTooHigh(FeesTooHigh),
        NoAdmin(NoAdmin),
        NotEnoughEth(NotEnoughEth),
        SendError(SendError),
        Unauthorized(Unauthorized),
        ZeroAddress(ZeroAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MevEthShareVaultErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <AlreadySet as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AlreadySet(decoded));
            }
            if let Ok(decoded)
                = <FeesTooHigh as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeesTooHigh(decoded));
            }
            if let Ok(decoded)
                = <NoAdmin as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoAdmin(decoded));
            }
            if let Ok(decoded)
                = <NotEnoughEth as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotEnoughEth(decoded));
            }
            if let Ok(decoded)
                = <SendError as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendError(decoded));
            }
            if let Ok(decoded)
                = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unauthorized(decoded));
            }
            if let Ok(decoded)
                = <ZeroAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MevEthShareVaultErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AlreadySet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeesTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotEnoughEth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MevEthShareVaultErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AlreadySet as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <FeesTooHigh as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NoAdmin as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotEnoughEth as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <SendError as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ZeroAddress as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MevEthShareVaultErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AlreadySet(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeesTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotEnoughEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendError(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MevEthShareVaultErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AlreadySet> for MevEthShareVaultErrors {
        fn from(value: AlreadySet) -> Self {
            Self::AlreadySet(value)
        }
    }
    impl ::core::convert::From<FeesTooHigh> for MevEthShareVaultErrors {
        fn from(value: FeesTooHigh) -> Self {
            Self::FeesTooHigh(value)
        }
    }
    impl ::core::convert::From<NoAdmin> for MevEthShareVaultErrors {
        fn from(value: NoAdmin) -> Self {
            Self::NoAdmin(value)
        }
    }
    impl ::core::convert::From<NotEnoughEth> for MevEthShareVaultErrors {
        fn from(value: NotEnoughEth) -> Self {
            Self::NotEnoughEth(value)
        }
    }
    impl ::core::convert::From<SendError> for MevEthShareVaultErrors {
        fn from(value: SendError) -> Self {
            Self::SendError(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for MevEthShareVaultErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
        }
    }
    impl ::core::convert::From<ZeroAddress> for MevEthShareVaultErrors {
        fn from(value: ZeroAddress) -> Self {
            Self::ZeroAddress(value)
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
    #[ethevent(name = "AdminAdded", abi = "AdminAdded(address)")]
    pub struct AdminAddedFilter {
        #[ethevent(indexed)]
        pub new_admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "AdminDeleted", abi = "AdminDeleted(address)")]
    pub struct AdminDeletedFilter {
        #[ethevent(indexed)]
        pub old_admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "FeesSent", abi = "FeesSent(uint256)")]
    pub struct FeesSentFilter {
        #[ethevent(indexed)]
        pub fees_sent: ::ethers::core::types::U256,
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
    #[ethevent(name = "MevEthUpdated", abi = "MevEthUpdated(address)")]
    pub struct MevEthUpdatedFilter {
        #[ethevent(indexed)]
        pub meveth: ::ethers::core::types::Address,
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
    #[ethevent(name = "OperatorAdded", abi = "OperatorAdded(address)")]
    pub struct OperatorAddedFilter {
        #[ethevent(indexed)]
        pub new_operator: ::ethers::core::types::Address,
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
    #[ethevent(name = "OperatorDeleted", abi = "OperatorDeleted(address)")]
    pub struct OperatorDeletedFilter {
        #[ethevent(indexed)]
        pub old_operator: ::ethers::core::types::Address,
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
    #[ethevent(name = "ProtocolFeeToUpdated", abi = "ProtocolFeeToUpdated(address)")]
    pub struct ProtocolFeeToUpdatedFilter {
        #[ethevent(indexed)]
        pub new_protocol_fee_to: ::ethers::core::types::Address,
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
    #[ethevent(name = "RewardPayment", abi = "RewardPayment(uint256,address,uint256)")]
    pub struct RewardPaymentFilter {
        #[ethevent(indexed)]
        pub block_number: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub coinbase: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "RewardsCollected", abi = "RewardsCollected(uint256,uint256)")]
    pub struct RewardsCollectedFilter {
        #[ethevent(indexed)]
        pub protocol_fees_owed: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub rewards_owed: ::ethers::core::types::U256,
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
    #[ethevent(name = "RewardsPaid", abi = "RewardsPaid(uint256)")]
    pub struct RewardsPaidFilter {
        #[ethevent(indexed)]
        pub rewards: ::ethers::core::types::U256,
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
    #[ethevent(name = "TokenRecovered", abi = "TokenRecovered(address,address,uint256)")]
    pub struct TokenRecoveredFilter {
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "ValidatorWithdraw", abi = "ValidatorWithdraw(address,uint256)")]
    pub struct ValidatorWithdrawFilter {
        pub sender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MevEthShareVaultEvents {
        AdminAddedFilter(AdminAddedFilter),
        AdminDeletedFilter(AdminDeletedFilter),
        FeesSentFilter(FeesSentFilter),
        MevEthUpdatedFilter(MevEthUpdatedFilter),
        OperatorAddedFilter(OperatorAddedFilter),
        OperatorDeletedFilter(OperatorDeletedFilter),
        ProtocolFeeToUpdatedFilter(ProtocolFeeToUpdatedFilter),
        RewardPaymentFilter(RewardPaymentFilter),
        RewardsCollectedFilter(RewardsCollectedFilter),
        RewardsPaidFilter(RewardsPaidFilter),
        TokenRecoveredFilter(TokenRecoveredFilter),
        ValidatorWithdrawFilter(ValidatorWithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for MevEthShareVaultEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminAddedFilter::decode_log(log) {
                return Ok(MevEthShareVaultEvents::AdminAddedFilter(decoded));
            }
            if let Ok(decoded) = AdminDeletedFilter::decode_log(log) {
                return Ok(MevEthShareVaultEvents::AdminDeletedFilter(decoded));
            }
            if let Ok(decoded) = FeesSentFilter::decode_log(log) {
                return Ok(MevEthShareVaultEvents::FeesSentFilter(decoded));
            }
            if let Ok(decoded) = MevEthUpdatedFilter::decode_log(log) {
                return Ok(MevEthShareVaultEvents::MevEthUpdatedFilter(decoded));
            }
            if let Ok(decoded) = OperatorAddedFilter::decode_log(log) {
                return Ok(MevEthShareVaultEvents::OperatorAddedFilter(decoded));
            }
            if let Ok(decoded) = OperatorDeletedFilter::decode_log(log) {
                return Ok(MevEthShareVaultEvents::OperatorDeletedFilter(decoded));
            }
            if let Ok(decoded) = ProtocolFeeToUpdatedFilter::decode_log(log) {
                return Ok(MevEthShareVaultEvents::ProtocolFeeToUpdatedFilter(decoded));
            }
            if let Ok(decoded) = RewardPaymentFilter::decode_log(log) {
                return Ok(MevEthShareVaultEvents::RewardPaymentFilter(decoded));
            }
            if let Ok(decoded) = RewardsCollectedFilter::decode_log(log) {
                return Ok(MevEthShareVaultEvents::RewardsCollectedFilter(decoded));
            }
            if let Ok(decoded) = RewardsPaidFilter::decode_log(log) {
                return Ok(MevEthShareVaultEvents::RewardsPaidFilter(decoded));
            }
            if let Ok(decoded) = TokenRecoveredFilter::decode_log(log) {
                return Ok(MevEthShareVaultEvents::TokenRecoveredFilter(decoded));
            }
            if let Ok(decoded) = ValidatorWithdrawFilter::decode_log(log) {
                return Ok(MevEthShareVaultEvents::ValidatorWithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MevEthShareVaultEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminDeletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeesSentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MevEthUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorDeletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProtocolFeeToUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardPaymentFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardsCollectedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardsPaidFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenRecoveredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorWithdrawFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AdminAddedFilter> for MevEthShareVaultEvents {
        fn from(value: AdminAddedFilter) -> Self {
            Self::AdminAddedFilter(value)
        }
    }
    impl ::core::convert::From<AdminDeletedFilter> for MevEthShareVaultEvents {
        fn from(value: AdminDeletedFilter) -> Self {
            Self::AdminDeletedFilter(value)
        }
    }
    impl ::core::convert::From<FeesSentFilter> for MevEthShareVaultEvents {
        fn from(value: FeesSentFilter) -> Self {
            Self::FeesSentFilter(value)
        }
    }
    impl ::core::convert::From<MevEthUpdatedFilter> for MevEthShareVaultEvents {
        fn from(value: MevEthUpdatedFilter) -> Self {
            Self::MevEthUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorAddedFilter> for MevEthShareVaultEvents {
        fn from(value: OperatorAddedFilter) -> Self {
            Self::OperatorAddedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorDeletedFilter> for MevEthShareVaultEvents {
        fn from(value: OperatorDeletedFilter) -> Self {
            Self::OperatorDeletedFilter(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeToUpdatedFilter> for MevEthShareVaultEvents {
        fn from(value: ProtocolFeeToUpdatedFilter) -> Self {
            Self::ProtocolFeeToUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<RewardPaymentFilter> for MevEthShareVaultEvents {
        fn from(value: RewardPaymentFilter) -> Self {
            Self::RewardPaymentFilter(value)
        }
    }
    impl ::core::convert::From<RewardsCollectedFilter> for MevEthShareVaultEvents {
        fn from(value: RewardsCollectedFilter) -> Self {
            Self::RewardsCollectedFilter(value)
        }
    }
    impl ::core::convert::From<RewardsPaidFilter> for MevEthShareVaultEvents {
        fn from(value: RewardsPaidFilter) -> Self {
            Self::RewardsPaidFilter(value)
        }
    }
    impl ::core::convert::From<TokenRecoveredFilter> for MevEthShareVaultEvents {
        fn from(value: TokenRecoveredFilter) -> Self {
            Self::TokenRecoveredFilter(value)
        }
    }
    impl ::core::convert::From<ValidatorWithdrawFilter> for MevEthShareVaultEvents {
        fn from(value: ValidatorWithdrawFilter) -> Self {
            Self::ValidatorWithdrawFilter(value)
        }
    }
    ///Container type for all input parameters for the `addAdmin` function with signature `addAdmin(address)` and selector `0x70480275`
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
    #[ethcall(name = "addAdmin", abi = "addAdmin(address)")]
    pub struct AddAdminCall {
        pub new_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addOperator` function with signature `addOperator(address)` and selector `0x9870d7fe`
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
    #[ethcall(name = "addOperator", abi = "addOperator(address)")]
    pub struct AddOperatorCall {
        pub new_operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `admins` function with signature `admins(address)` and selector `0x429b62e5`
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
    #[ethcall(name = "admins", abi = "admins(address)")]
    pub struct AdminsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `deleteAdmin` function with signature `deleteAdmin(address)` and selector `0x27e1f7df`
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
    #[ethcall(name = "deleteAdmin", abi = "deleteAdmin(address)")]
    pub struct DeleteAdminCall {
        pub old_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deleteOperator` function with signature `deleteOperator(address)` and selector `0xb40992a1`
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
    #[ethcall(name = "deleteOperator", abi = "deleteOperator(address)")]
    pub struct DeleteOperatorCall {
        pub old_operator: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `mevEth` function with signature `mevEth()` and selector `0xbadf2663`
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
    #[ethcall(name = "mevEth", abi = "mevEth()")]
    pub struct MevEthCall;
    ///Container type for all input parameters for the `operators` function with signature `operators(address)` and selector `0x13e7c9d8`
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
    #[ethcall(name = "operators", abi = "operators(address)")]
    pub struct OperatorsCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `payValidatorWithdraw` function with signature `payValidatorWithdraw()` and selector `0x14b71a83`
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
    #[ethcall(name = "payValidatorWithdraw", abi = "payValidatorWithdraw()")]
    pub struct PayValidatorWithdrawCall;
    ///Container type for all input parameters for the `protocolBalance` function with signature `protocolBalance()` and selector `0x61e98db8`
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
    #[ethcall(name = "protocolBalance", abi = "protocolBalance()")]
    pub struct ProtocolBalanceCall;
    ///Container type for all input parameters for the `protocolFeeTo` function with signature `protocolFeeTo()` and selector `0xef369252`
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
    #[ethcall(name = "protocolFeeTo", abi = "protocolFeeTo()")]
    pub struct ProtocolFeeToCall;
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
        pub new_protocol_fee_to: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MevEthShareVaultCalls {
        AddAdmin(AddAdminCall),
        AddOperator(AddOperatorCall),
        Admins(AdminsCall),
        DeleteAdmin(DeleteAdminCall),
        DeleteOperator(DeleteOperatorCall),
        Fees(FeesCall),
        LogRewards(LogRewardsCall),
        MevEth(MevEthCall),
        Operators(OperatorsCall),
        PayRewards(PayRewardsCall),
        PayValidatorWithdraw(PayValidatorWithdrawCall),
        ProtocolBalance(ProtocolBalanceCall),
        ProtocolFeeTo(ProtocolFeeToCall),
        RecoverToken(RecoverTokenCall),
        Rewards(RewardsCall),
        SendFees(SendFeesCall),
        SetNewMevEth(SetNewMevEthCall),
        SetProtocolFeeTo(SetProtocolFeeToCall),
    }
    impl ::ethers::core::abi::AbiDecode for MevEthShareVaultCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddAdmin(decoded));
            }
            if let Ok(decoded)
                = <AddOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddOperator(decoded));
            }
            if let Ok(decoded)
                = <AdminsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Admins(decoded));
            }
            if let Ok(decoded)
                = <DeleteAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeleteAdmin(decoded));
            }
            if let Ok(decoded)
                = <DeleteOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeleteOperator(decoded));
            }
            if let Ok(decoded)
                = <FeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fees(decoded));
            }
            if let Ok(decoded)
                = <LogRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogRewards(decoded));
            }
            if let Ok(decoded)
                = <MevEthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MevEth(decoded));
            }
            if let Ok(decoded)
                = <OperatorsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Operators(decoded));
            }
            if let Ok(decoded)
                = <PayRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PayRewards(decoded));
            }
            if let Ok(decoded)
                = <PayValidatorWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PayValidatorWithdraw(decoded));
            }
            if let Ok(decoded)
                = <ProtocolBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProtocolBalance(decoded));
            }
            if let Ok(decoded)
                = <ProtocolFeeToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProtocolFeeTo(decoded));
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
    impl ::ethers::core::abi::AbiEncode for MevEthShareVaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admins(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeleteAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeleteOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Fees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LogRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MevEth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Operators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PayRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PayValidatorWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProtocolBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProtocolFeeTo(element) => {
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
    impl ::core::fmt::Display for MevEthShareVaultCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admins(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fees(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::MevEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::Operators(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayValidatorWithdraw(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProtocolBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFeeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecoverToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNewMevEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFeeTo(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddAdminCall> for MevEthShareVaultCalls {
        fn from(value: AddAdminCall) -> Self {
            Self::AddAdmin(value)
        }
    }
    impl ::core::convert::From<AddOperatorCall> for MevEthShareVaultCalls {
        fn from(value: AddOperatorCall) -> Self {
            Self::AddOperator(value)
        }
    }
    impl ::core::convert::From<AdminsCall> for MevEthShareVaultCalls {
        fn from(value: AdminsCall) -> Self {
            Self::Admins(value)
        }
    }
    impl ::core::convert::From<DeleteAdminCall> for MevEthShareVaultCalls {
        fn from(value: DeleteAdminCall) -> Self {
            Self::DeleteAdmin(value)
        }
    }
    impl ::core::convert::From<DeleteOperatorCall> for MevEthShareVaultCalls {
        fn from(value: DeleteOperatorCall) -> Self {
            Self::DeleteOperator(value)
        }
    }
    impl ::core::convert::From<FeesCall> for MevEthShareVaultCalls {
        fn from(value: FeesCall) -> Self {
            Self::Fees(value)
        }
    }
    impl ::core::convert::From<LogRewardsCall> for MevEthShareVaultCalls {
        fn from(value: LogRewardsCall) -> Self {
            Self::LogRewards(value)
        }
    }
    impl ::core::convert::From<MevEthCall> for MevEthShareVaultCalls {
        fn from(value: MevEthCall) -> Self {
            Self::MevEth(value)
        }
    }
    impl ::core::convert::From<OperatorsCall> for MevEthShareVaultCalls {
        fn from(value: OperatorsCall) -> Self {
            Self::Operators(value)
        }
    }
    impl ::core::convert::From<PayRewardsCall> for MevEthShareVaultCalls {
        fn from(value: PayRewardsCall) -> Self {
            Self::PayRewards(value)
        }
    }
    impl ::core::convert::From<PayValidatorWithdrawCall> for MevEthShareVaultCalls {
        fn from(value: PayValidatorWithdrawCall) -> Self {
            Self::PayValidatorWithdraw(value)
        }
    }
    impl ::core::convert::From<ProtocolBalanceCall> for MevEthShareVaultCalls {
        fn from(value: ProtocolBalanceCall) -> Self {
            Self::ProtocolBalance(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeToCall> for MevEthShareVaultCalls {
        fn from(value: ProtocolFeeToCall) -> Self {
            Self::ProtocolFeeTo(value)
        }
    }
    impl ::core::convert::From<RecoverTokenCall> for MevEthShareVaultCalls {
        fn from(value: RecoverTokenCall) -> Self {
            Self::RecoverToken(value)
        }
    }
    impl ::core::convert::From<RewardsCall> for MevEthShareVaultCalls {
        fn from(value: RewardsCall) -> Self {
            Self::Rewards(value)
        }
    }
    impl ::core::convert::From<SendFeesCall> for MevEthShareVaultCalls {
        fn from(value: SendFeesCall) -> Self {
            Self::SendFees(value)
        }
    }
    impl ::core::convert::From<SetNewMevEthCall> for MevEthShareVaultCalls {
        fn from(value: SetNewMevEthCall) -> Self {
            Self::SetNewMevEth(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeeToCall> for MevEthShareVaultCalls {
        fn from(value: SetProtocolFeeToCall) -> Self {
            Self::SetProtocolFeeTo(value)
        }
    }
    ///Container type for all return fields from the `admins` function with signature `admins(address)` and selector `0x429b62e5`
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
    pub struct AdminsReturn(pub bool);
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
    ///Container type for all return fields from the `mevEth` function with signature `mevEth()` and selector `0xbadf2663`
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
    pub struct MevEthReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `operators` function with signature `operators(address)` and selector `0x13e7c9d8`
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
    pub struct OperatorsReturn(pub bool);
    ///Container type for all return fields from the `protocolBalance` function with signature `protocolBalance()` and selector `0x61e98db8`
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
    pub struct ProtocolBalanceReturn {
        pub fees: u128,
        pub rewards: u128,
    }
    ///Container type for all return fields from the `protocolFeeTo` function with signature `protocolFeeTo()` and selector `0xef369252`
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
    pub struct ProtocolFeeToReturn(pub ::ethers::core::types::Address);
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
