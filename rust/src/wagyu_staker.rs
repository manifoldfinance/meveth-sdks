pub use wagyu_staker::*;
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
pub mod wagyu_staker {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_authority"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_depositContract"),
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
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BEACON_CHAIN_DEPOSIT_CONTRACT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BEACON_CHAIN_DEPOSIT_CONTRACT",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IBeaconDepositContract",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VALIDATOR_DEPOSIT_SIZE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "VALIDATOR_DEPOSIT_SIZE",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("batchMigrate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("batchMigrate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("batchData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakingModule.ValidatorData[]",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakingModule.ValidatorData",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("latestDepositRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rewards"),
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
                    ::std::borrow::ToOwned::to_owned("record"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("record"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalDeposited"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalWithdrawn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalRewardsPaid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "totalValidatorExitsPaid",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("registerExit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerExit"),
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
                    ::std::borrow::ToOwned::to_owned("validators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validators"),
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
                    ::std::borrow::ToOwned::to_owned("NewValidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewValidator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawalCredentials",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deposit_data_root"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("RewardsPaid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RewardsPaid"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("DepositWasFrontrun"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DepositWasFrontrun"),
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
                    ::std::borrow::ToOwned::to_owned("UnAuthorizedCaller"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnAuthorizedCaller"),
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
                    ::std::borrow::ToOwned::to_owned("WrongDepositAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("WrongDepositAmount"),
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
    pub static WAGYUSTAKER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA04a\0\xE1W`\x1Fa\x10\xC98\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xE6W\x80\x84\x92``\x94`@R\x839\x81\x01\x03\x12a\0\xE1Wa\0G\x81a\0\xFCV[\x90a\0``@a\0Y` \x84\x01a\0\xFCV[\x92\x01a\0\xFCV[\x91`\x01\x80`\xA0\x1B\x03\x92\x83\x80\x92\x16`\0R`\x02` R`@`\0 `\xFF\x19\x90`\x01\x82\x82T\x16\x17\x90U`\0T\x81`\xFF`\x01\x81\x84\x16\x01\x16\x91\x16\x17`\0U`\x01` R`\x01`@`\0 \x91\x82T\x16\x17\x90U\x16`\x01\x80`\xA0\x1B\x03\x19`\x06T\x16\x17`\x06U\x16`\x80R`@Qa\x0F\xB8\x90\x81a\x01\x11\x829`\x80Q\x81\x81\x81a\x01\x16\x01Ra\x01\xE8\x01R\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0\xE1WV\xFE`\x80`@\x81\x81R`\x04\x91\x826\x10\x15a\0\"W[PPP6\x15a\0 W`\0\x80\xFD[\0[`\0\x92\x835`\xE0\x1C\x91\x82c\x13\xE7\xC9\xD8\x14a\r\xFBWP\x81c\x14\xB7\x1A\x83\x14a\r\x01W\x81c&l\xF1\t\x14a\x0C\xBDW\x81c'\xE1\xF7\xDF\x14a\x0B\xE2W\x81cB\x9Bb\xE5\x14a\x0B\xA6W\x81cD.I=\x14a\x0B\x11W\x81cJ\xD8\xD3K\x14a\n\x85W\x81cUR\xAAe\x14a\naW\x81c`\xE3\xA0\xAC\x14a\t_W\x81cpH\x02u\x14a\x08\x87W\x81c\x83+|k\x14a\x06pW\x81c\x98p\xD7\xFE\x14a\x05\xE6W\x81c\xA7\"\x9F\xD9\x14a\x04\xE6W\x81c\xB4\t\x92\xA1\x14a\x04CW\x81c\xB7x\xA3\xA7\x14a\x01\x83WP\x80c\xBA\xDF&c\x14a\x01\\W\x80c\xCA\x1Ex\x19\x14a\x01>Wc\xCC\xEA\xB7P\x14a\0\xF8W\x80a\0\x12V[4a\x01:W\x81`\x03\x196\x01\x12a\x01:W` \x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P\x80\xFD[P4a\x01:W\x81`\x03\x196\x01\x12a\x01:W` \x90`\x05T\x90Q\x90\x81R\xF3[P4a\x01:W\x81`\x03\x196\x01\x12a\x01:W` \x90`\x01`\x01`\xA0\x1B\x03`\x06T\x16\x90Q\x90\x81R\xF3[\x83\x83`\x03\x19\x92\x81\x846\x01\x12a\x04?W\x805\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x11a\x04;W`\xA0\x86\x84\x01\x92\x876\x03\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03\x93\x84`\x06T\x163\x03a\x04,Wh\x01\xBC\x16\xD6t\xEC\x80\0\0\x91\x824\x03a\x04\x1CW\x81Qc\xC5\xF2\x89/`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x16\x92` \x92\x90\x91\x83\x81\x89\x81\x88Z\xFA\x90\x81\x15a\x04\x12W\x8A\x91a\x03\xE1W[P`$5\x03a\x03\xD1W`\x03T`\x01`\x01`\x80\x1B\x03\x86\x81\x83\x16\x01\x16\x90`\x01`\x01`\x80\x1B\x03\x19\x16\x17`\x03U`\x01`\x05T\x01`\x05U`$\x8A\x01\x93a\x02e\x85\x88a\x0E\x97V[\x90\x96`D\x8D\x015\x99\x85Q\x90\x8B\x88\x83\x01R\x87\x82R\x86\x82\x01\x95\x82\x87\x10\x90\x87\x11\x17a\x03\xBCWP\x92\x8C\x9D\x92`\x84\x92\x86\x95\x8C\x9D\x9E\x9F\x97\x89Ra\x02\xA6`d\x87\x01\x80\x9Ea\x0E\x97V[\x95\x90\x96\x015\x9B\x84;\x15a\x03\x9AWc\x04Q*#`\xE3\x1B\x88R`\x80`D\x83\x01R\x88\x96\x88\x96\x87\x95a\x03\x03\x93a\x02\xDC\x91`\xC4\x87\x01\x91a\x0E\xCAV[\x91a\x02\xF4`C\x19\x93\x84\x87\x82\x03\x01`d\x88\x01R\x86a\x0E\xEBV[\x92\x85\x84\x03\x01`\x84\x86\x01Ra\x0E\xCAV[`\xA4\x82\x01\x8D\x90R\x03`?\x19\x01\x92Z\xF1\x80\x15a\x03\xB2Wa\x03\x9EW[PP\x855\x97\x88\x16\x80\x98\x03a\x03\x9AW\x7F\xFC\x94\"\xD7\x96S6\x11\xE7A\xD6\xD7tB6go!\x14\xB7\xE0\x1C\xB7\xF2\xCCA+\x12\xCE\x1D\xA5V\x96a\x03ea\x03]a\x03\x8E\x95\x89a\x0E\x97V[\x97\x90\x98a\x0E\x97V[\x92\x90\x91a\x03~\x82Q\x99\x8A\x99`\x80\x8BR`\x80\x8B\x01\x91a\x0E\xCAV[\x94\x88\x01R\x86\x84\x03\x90\x87\x01Ra\x0E\xCAV[\x90``\x83\x01R\x03\x90\xA2\x80\xF3[\x88\x80\xFD[a\x03\xA7\x90a\x0EKV[a\x03\x9AW\x88\x8Aa\x03\x1DV[\x83Q=\x84\x82>=\x90\xFD[`A\x90cNH{q`\xE0\x1B`\0RR`$`\0\xFD[\x81Qc\xC6\x1C\xF4\xC5`\xE0\x1B\x81R\x87\x90\xFD[\x90P\x83\x81\x81=\x83\x11a\x04\x0BW[a\x03\xF8\x81\x83a\x0EuV[\x81\x01\x03\x12a\x04\x07WQ\x8Ba\x02$V[\x89\x80\xFD[P=a\x03\xEEV[\x83Q=\x8C\x82>=\x90\xFD[\x81Qco\xF0\xAC\xF9`\xE1\x1B\x81R\x85\x90\xFD[Qc\xE3'+\xBB`\xE0\x1B\x81R\x83\x90\xFD[\x84\x80\xFD[\x82\x80\xFD[\x91\x90P4a\x04?W` 6`\x03\x19\x01\x12a\x04?Wa\x04_a\x0E5V[3\x84R`\x02` R`\xFF\x82\x85 T\x16\x15a\x04\xD9W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x84R`\x01` R`\xFF\x82\x85 T\x16\x15a\x04\xCBWP\x81\x83R`\x01` R\x82 \x80T`\xFF\x19\x16\x90U\x7Fi\xDF,^\xC2\xEAM\x1F\xBE\x1EP5$\xF5\x93\xB3V\x16,\xA7\x10g\x12c\x82\x7F.\x19\x92\xB9Z\xE1\x82\x80\xA2\x80\xF3[\x90Qc\xA7A\xA0E`\xE0\x1B\x81R\xFD[PQb\x82\xB4)`\xE8\x1B\x81R\xFD[\x91\x90P4a\x04?W``6`\x03\x19\x01\x12a\x04?Wa\x05\x02a\x0E5V[\x91`$5\x90`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x80\x93\x03a\x05\xE1W`D5\x943\x87R`\x02` R`\xFF\x85\x88 T\x16\x15a\x05\xD2W\x16\x92` \x86`D\x83Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x86\x86\x82\x01R\x88`$\x82\x01R\x82\x88Z\xF1=\x15`\x1F=\x11`\x01\x89Q\x14\x16\x17\x16\x15a\x05\x8FWPP\x7F\x87\x9F\x92\xDD\xED\x0F&\xB8<>\0\xB1.\x03\x95\xDCr\xCF\xC3\x07sC\xD1\x85N\xD6\x98\x8E\xDD\x1F\x90\x96\x84\x80\xA4\x80\xF3[\x90` `d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x84Qb\x82\xB4)`\xE8\x1B\x81R\x83\x90\xFD[`\0\x80\xFD[\x91\x90P4a\x04?W` 6`\x03\x19\x01\x12a\x04?Wa\x06\x02a\x0E5V[3\x84R`\x02` R`\xFF\x82\x85 T\x16\x15a\x04\xD9W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x84R`\x01` R`\xFF\x82\x85 T\x16a\x04\xCBWP\x81\x83R`\x01` R\x82 `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\xACo\xA8X\xE95\nF\xCE\xC1e9\x92n\x0F\xDE%\xB7b\x9F\x84\xB5\xA7+\xFF\xAA\xE4\xDF\x88\x8A\xE8m\x82\x80\xA2\x80\xF3[\x91\x90P4a\x04?W` \x91\x82`\x03\x196\x01\x12a\x08\x83W\x805\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11a\x08\x7FW6`#\x85\x01\x12\x15a\x08\x7FW\x83\x83\x015\x94\x81\x86\x11a\x08{W`$\x95`\x05\x956\x88\x83\x89\x1B\x83\x01\x01\x11a\x03\x9AW3\x89R`\x02\x83R`\xFF\x85\x8A T\x16\x15a\x08lW`\x03\x96\x91\x96T`\x01`\x01`\x80\x1B\x03\x80h\x01\xBC\x16\xD6t\xEC\x80\0\0\x8A\x02\x16\x81\x83\x16\x01\x16\x90`\x01`\x01`\x80\x1B\x03\x19\x16\x17`\x03U\x86\x82T\x01\x82U\x88\x91`\xC2\x19\x826\x03\x01\x92[\x88\x81\x10a\x07\"W\x8A\x80\xF3[\x89\x81\x83\x1B\x84\x01\x015\x84\x81\x12\x15a\x08hW\x83\x01`\xA0\x90\x81`#\x19\x826\x03\x01\x12a\x08dW\x88Q\x91\x82\x01\x82\x81\x10\x89\x82\x11\x17a\x08PW\x89R`\x01`\x01`\xA0\x1B\x03\x90\x80\x8D\x015\x82\x81\x16\x81\x03a\x05\xE1W\x83R`D\x81\x015\x89\x81\x11a\x08LWa\x07\x89\x90\x8E6\x91\x84\x01\x01a\x0F+V[\x92\x88\x81\x01\x93\x84R\x8A\x81\x01\x91`d\x81\x015\x83R`\x84\x81\x015\x93\x8B\x85\x11a\x08GW\x8F\x8B\x8Ea\x07\xDE\x7F\xFC\x94\"\xD7\x96S6\x11\xE7A\xD6\xD7tB6go!\x14\xB7\xE0\x1C\xB7\xF2\xCCA+\x12\xCE\x1D\xA5V\x98a\x08\x1D\x946\x91\x88\x01\x01a\x0F+V[\x90`\xA4``\x96\x83\x88\x8A\x01R\x015\x94`\x80\x97\x86\x89\x82\x01RQ\x16\x99Q\x97Qa\x08\r\x82Q\x99\x89\x8B\x9A\x8BR\x8A\x01\x90a\x0E\xEBV[\x93\x88\x01R\x86\x83\x03\x90\x87\x01Ra\x0E\xEBV[\x91\x83\x01R\x03\x90\xA2`\0\x19\x81\x14a\x085W`\x01\x01a\x07\x17V[cNH{q`\xE0\x1B\x8BR`\x11\x88R\x89\x8B\xFD[P\x8F\x80\xFD[\x8E\x80\xFD[\x8C`A\x8CcNH{q`\xE0\x1B`\0RR`\0\xFD[\x8C\x80\xFD[\x8B\x80\xFD[\x84Qb\x82\xB4)`\xE8\x1B\x81R\x86\x90\xFD[\x86\x80\xFD[\x85\x80\xFD[\x83\x80\xFD[\x83\x834a\x01:W` 6`\x03\x19\x01\x12a\x01:Wa\x08\xA2a\x0E5V[3\x83R`\x02` R`\xFF\x82\x84 T\x16\x15a\tPW`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x83R`\x02` R`\xFF\x81\x84 T\x16a\tAW\x82T`\xFF\x81\x16`\xFF\x81\x14a\t.W`\xFF\x19\x91\x82\x16`\x01\x91\x82\x01`\xFF\x16\x17\x85U\x83\x85R`\x02` R\x91\x84 \x80T\x90\x91\x16\x90\x91\x17\x90U\x7FD\xD6\xD2Yc\xF0\x97\xAD\x14\xF2\x9F\x06\x85J\x01\xF5ud\x8A\x1E\xF8/0\xE5b\xCC\xD3\x88\x97\x17\xE39\x82\x80\xA2\x80\xF3[cNH{q`\xE0\x1B\x85R`\x11\x86R`$\x85\xFD[Qc\xA7A\xA0E`\xE0\x1B\x81R\x83\x90\xFD[\x81Qb\x82\xB4)`\xE8\x1B\x81R\x84\x90\xFD[\x90P4a\x04?W` 6`\x03\x19\x01\x12a\x04?W\x805\x913\x84R`\x01` R`\xFF\x81\x85 T\x16\x15a\nUWG\x83\x11a\nHW\x90\x83\x91`\x01`\x01`\x80\x1B\x03\x80\x85\x16\x90\x83T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x83\x85\x81\x84\x16\x01\x16\x91\x16\x17\x85U`\x03T\x92\x83`\x80\x1C\x01`\x80\x1B\x16\x91\x16\x17`\x03U`\x01`\x01`\xA0\x1B\x03`\x06T\x16\x91\x82;\x15a\x08\x83W\x81QcU\x8C\xB7\xF7`\xE0\x1B\x81R\x92\x84\x91\x84\x91\x82\x90\x88\x90Z\xF1\x90\x81\x15a\n?WPa\n+W[P\x80\x7F\xB0\xC6Z[20\"\xD9&\xC4V\xE1VM\x86\xF0\xBD\xA4\x01`\xA7\x81\xC8 \xF5q\xD7c[4\x88\x01\x91\xA2\x80\xF3[a\n4\x90a\x0EKV[a\x01:W\x818a\n\x03V[Q=\x84\x82>=\x90\xFD[Qc\xF1JB\xB7`\xE0\x1B\x81R\xFD[Qb\x82\xB4)`\xE8\x1B\x81R\xFD[PP4a\x01:W\x81`\x03\x196\x01\x12a\x01:W` \x90Qh\x01\xBC\x16\xD6t\xEC\x80\0\0\x81R\xF3[\x91\x90P4a\x04?W\x82`\x03\x196\x01\x12a\x04?W`\x01`\x01`\xA0\x1B\x03`\x06T\x163\x03a\x0B\x04WP\x80T`\x01`\x01`\x80\x1B\x03\x19h\x01\xBC\x16\xD6t\xEC\x80\0\0\x92`\x01`\x01`\x80\x1B\x03\x92\x83\x83\x86\x83`\x80\x1C\x01`\x80\x1B\x16\x91\x16\x17\x90U`\x03T\x92\x83`\x80\x1C\x01`\x80\x1B\x16\x91\x16\x17`\x03U`\x05T\x80a\n\xFAWP\x80\xF3[`\0\x19\x01`\x05U\x80\xF3[Qc\xE3'+\xBB`\xE0\x1B\x81R\xFD[\x90P4a\x04?W` 6`\x03\x19\x01\x12a\x04?Wa\x0B,a\x0E5V[3\x84R`\x02` R`\xFF\x83\x85 T\x16\x15a\x0B\x98W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15a\x0B\x8BWPP\x80`\x01`\x01`\xA0\x1B\x03\x19`\x06T\x16\x17`\x06U\x7F\x05\xBF\\\xA0b)=\xE5g\xF8C\xF2\xE7\xE8\xB0$\xD0\xB1Ukg\x98T\xE7\x9FS\xC1\x94\x02\xA8;\x07\x82\x80\xA2\x80\xF3[Qc\xD9.#=`\xE0\x1B\x81R\xFD[P\x90Qb\x82\xB4)`\xE8\x1B\x81R\xFD[PP4a\x01:W` 6`\x03\x19\x01\x12a\x01:W`\xFF\x81` \x93`\x01`\x01`\xA0\x1B\x03a\x0B\xCFa\x0E5V[\x16\x81R`\x02\x85R T\x16\x90Q\x90\x15\x15\x81R\xF3[\x90P4a\x04?W` 6`\x03\x19\x01\x12a\x04?Wa\x0B\xFDa\x0E5V[3\x84R`\x02` R`\xFF\x83\x85 T\x16\x15a\x0B\x98W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x84R`\x02` R`\xFF\x81\x85 T\x16\x15a\x0C\xB0W\x83T\x91`\xFF\x83\x16\x80\x15a\x0C\x9DW`\xFF\x90`\0\x19\x01\x16\x85`\xFF\x19\x94\x82\x86\x82\x16\x17\x82U\x16\x17\x15a\x0C\x8FWP\x82\x84R`\x02` R\x83 \x90\x81T\x16\x90U\x7F\x98\x9D\xDF\xCE\x05}\xAD!\x9E\n\xE1oi\x1B\x12\x1B\xB0\xE3H\xF0\xD8\xAE\n\xD4\0\xB4\xD5\xAC\x8Dal\x8B\x82\x80\xA2\x80\xF3[\x90Qc\x1F\x8C\x1D\xBD`\xE1\x1B\x81R\xFD[cNH{q`\xE0\x1B\x86R`\x11\x82R`$\x86\xFD[Qc\xA7A\xA0E`\xE0\x1B\x81R\xFD[\x91\x90P4a\x04?W\x82`\x03\x196\x01\x12a\x04?W`\x80\x92P`\x03T\x91T\x90\x80Q\x92`\x01`\x01`\x80\x1B\x03\x90\x81\x81\x16\x85R\x85\x1C` \x85\x01R\x82\x16\x90\x83\x01R\x82\x1C``\x82\x01R\xF3[\x83\x834a\x01:W\x81`\x03\x196\x01\x12a\x01:W3\x82R`\x01` R`\xFF\x81\x83 T\x16\x15a\r\xEDWh\x01\xBC\x16\xD6t\xEC\x80\0\0\x92G\x84\x11a\r\xDFW`\x01`\x01`\xA0\x1B\x03`\x06T\x16\x90\x81;\x15a\x08\x83W\x82Qc\xFE\x182\x11`\xE0\x1B\x81R\x91\x84\x91\x83\x91\x82\x90\x88\x90Z\xF1\x80\x15a\r\xD3Wa\r\xA2W[P\x7F\x12\xB9d\xA3\x99=\x15\x98\xDD\x8A;bz;\x90\xB4\xBCkz\x8FO\x8B\xB6\xAF\xDE\x02\xA3\r\x17\x8E(\xEF\x91\x92\x81Q\x903\x82R` \x82\x01R\xA1\x80\xF3[\x91a\r\xCD\x7F\x12\xB9d\xA3\x99=\x15\x98\xDD\x8A;bz;\x90\xB4\xBCkz\x8FO\x8B\xB6\xAF\xDE\x02\xA3\r\x17\x8E(\xEF\x93a\x0EKV[\x91a\roV[PPQ\x90=\x90\x82>=\x90\xFD[\x90Qc\xF1JB\xB7`\xE0\x1B\x81R\xFD[Qb\x82\xB4)`\xE8\x1B\x81R\x90P\xFD[\x84\x90\x844a\x04?W` 6`\x03\x19\x01\x12a\x04?W`\xFF\x90` \x93`\x01`\x01`\xA0\x1B\x03a\x0E%a\x0E5V[\x16\x81R`\x01\x85R T\x16\x15\x15\x81R\xF3[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x05\xE1WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0E_W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E_W`@RV[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x05\xE1W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x05\xE1W` \x01\x91\x816\x03\x83\x13a\x05\xE1WV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x0F\x17WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x0E\xF6V[\x81`\x1F\x82\x01\x12\x15a\x05\xE1W\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0E_W`@Q\x92a\x0F``\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\x0EuV[\x82\x84R` \x83\x83\x01\x01\x11a\x05\xE1W\x81`\0\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V\xFE\xA2dipfsX\"\x12 n\xEF\x08\xE0p:\xECj0D,s\xF8\xE6\x06{\xAA|l\xC3\xCFg@\x7Fer[\\\x06\xEA]\xEFdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static WAGYUSTAKER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x91\x826\x10\x15a\0\"W[PPP6\x15a\0 W`\0\x80\xFD[\0[`\0\x92\x835`\xE0\x1C\x91\x82c\x13\xE7\xC9\xD8\x14a\r\xFBWP\x81c\x14\xB7\x1A\x83\x14a\r\x01W\x81c&l\xF1\t\x14a\x0C\xBDW\x81c'\xE1\xF7\xDF\x14a\x0B\xE2W\x81cB\x9Bb\xE5\x14a\x0B\xA6W\x81cD.I=\x14a\x0B\x11W\x81cJ\xD8\xD3K\x14a\n\x85W\x81cUR\xAAe\x14a\naW\x81c`\xE3\xA0\xAC\x14a\t_W\x81cpH\x02u\x14a\x08\x87W\x81c\x83+|k\x14a\x06pW\x81c\x98p\xD7\xFE\x14a\x05\xE6W\x81c\xA7\"\x9F\xD9\x14a\x04\xE6W\x81c\xB4\t\x92\xA1\x14a\x04CW\x81c\xB7x\xA3\xA7\x14a\x01\x83WP\x80c\xBA\xDF&c\x14a\x01\\W\x80c\xCA\x1Ex\x19\x14a\x01>Wc\xCC\xEA\xB7P\x14a\0\xF8W\x80a\0\x12V[4a\x01:W\x81`\x03\x196\x01\x12a\x01:W` \x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P\x80\xFD[P4a\x01:W\x81`\x03\x196\x01\x12a\x01:W` \x90`\x05T\x90Q\x90\x81R\xF3[P4a\x01:W\x81`\x03\x196\x01\x12a\x01:W` \x90`\x01`\x01`\xA0\x1B\x03`\x06T\x16\x90Q\x90\x81R\xF3[\x83\x83`\x03\x19\x92\x81\x846\x01\x12a\x04?W\x805\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x11a\x04;W`\xA0\x86\x84\x01\x92\x876\x03\x01\x12a\x04;W`\x01`\x01`\xA0\x1B\x03\x93\x84`\x06T\x163\x03a\x04,Wh\x01\xBC\x16\xD6t\xEC\x80\0\0\x91\x824\x03a\x04\x1CW\x81Qc\xC5\xF2\x89/`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x16\x92` \x92\x90\x91\x83\x81\x89\x81\x88Z\xFA\x90\x81\x15a\x04\x12W\x8A\x91a\x03\xE1W[P`$5\x03a\x03\xD1W`\x03T`\x01`\x01`\x80\x1B\x03\x86\x81\x83\x16\x01\x16\x90`\x01`\x01`\x80\x1B\x03\x19\x16\x17`\x03U`\x01`\x05T\x01`\x05U`$\x8A\x01\x93a\x02e\x85\x88a\x0E\x97V[\x90\x96`D\x8D\x015\x99\x85Q\x90\x8B\x88\x83\x01R\x87\x82R\x86\x82\x01\x95\x82\x87\x10\x90\x87\x11\x17a\x03\xBCWP\x92\x8C\x9D\x92`\x84\x92\x86\x95\x8C\x9D\x9E\x9F\x97\x89Ra\x02\xA6`d\x87\x01\x80\x9Ea\x0E\x97V[\x95\x90\x96\x015\x9B\x84;\x15a\x03\x9AWc\x04Q*#`\xE3\x1B\x88R`\x80`D\x83\x01R\x88\x96\x88\x96\x87\x95a\x03\x03\x93a\x02\xDC\x91`\xC4\x87\x01\x91a\x0E\xCAV[\x91a\x02\xF4`C\x19\x93\x84\x87\x82\x03\x01`d\x88\x01R\x86a\x0E\xEBV[\x92\x85\x84\x03\x01`\x84\x86\x01Ra\x0E\xCAV[`\xA4\x82\x01\x8D\x90R\x03`?\x19\x01\x92Z\xF1\x80\x15a\x03\xB2Wa\x03\x9EW[PP\x855\x97\x88\x16\x80\x98\x03a\x03\x9AW\x7F\xFC\x94\"\xD7\x96S6\x11\xE7A\xD6\xD7tB6go!\x14\xB7\xE0\x1C\xB7\xF2\xCCA+\x12\xCE\x1D\xA5V\x96a\x03ea\x03]a\x03\x8E\x95\x89a\x0E\x97V[\x97\x90\x98a\x0E\x97V[\x92\x90\x91a\x03~\x82Q\x99\x8A\x99`\x80\x8BR`\x80\x8B\x01\x91a\x0E\xCAV[\x94\x88\x01R\x86\x84\x03\x90\x87\x01Ra\x0E\xCAV[\x90``\x83\x01R\x03\x90\xA2\x80\xF3[\x88\x80\xFD[a\x03\xA7\x90a\x0EKV[a\x03\x9AW\x88\x8Aa\x03\x1DV[\x83Q=\x84\x82>=\x90\xFD[`A\x90cNH{q`\xE0\x1B`\0RR`$`\0\xFD[\x81Qc\xC6\x1C\xF4\xC5`\xE0\x1B\x81R\x87\x90\xFD[\x90P\x83\x81\x81=\x83\x11a\x04\x0BW[a\x03\xF8\x81\x83a\x0EuV[\x81\x01\x03\x12a\x04\x07WQ\x8Ba\x02$V[\x89\x80\xFD[P=a\x03\xEEV[\x83Q=\x8C\x82>=\x90\xFD[\x81Qco\xF0\xAC\xF9`\xE1\x1B\x81R\x85\x90\xFD[Qc\xE3'+\xBB`\xE0\x1B\x81R\x83\x90\xFD[\x84\x80\xFD[\x82\x80\xFD[\x91\x90P4a\x04?W` 6`\x03\x19\x01\x12a\x04?Wa\x04_a\x0E5V[3\x84R`\x02` R`\xFF\x82\x85 T\x16\x15a\x04\xD9W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x84R`\x01` R`\xFF\x82\x85 T\x16\x15a\x04\xCBWP\x81\x83R`\x01` R\x82 \x80T`\xFF\x19\x16\x90U\x7Fi\xDF,^\xC2\xEAM\x1F\xBE\x1EP5$\xF5\x93\xB3V\x16,\xA7\x10g\x12c\x82\x7F.\x19\x92\xB9Z\xE1\x82\x80\xA2\x80\xF3[\x90Qc\xA7A\xA0E`\xE0\x1B\x81R\xFD[PQb\x82\xB4)`\xE8\x1B\x81R\xFD[\x91\x90P4a\x04?W``6`\x03\x19\x01\x12a\x04?Wa\x05\x02a\x0E5V[\x91`$5\x90`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x80\x93\x03a\x05\xE1W`D5\x943\x87R`\x02` R`\xFF\x85\x88 T\x16\x15a\x05\xD2W\x16\x92` \x86`D\x83Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x86\x86\x82\x01R\x88`$\x82\x01R\x82\x88Z\xF1=\x15`\x1F=\x11`\x01\x89Q\x14\x16\x17\x16\x15a\x05\x8FWPP\x7F\x87\x9F\x92\xDD\xED\x0F&\xB8<>\0\xB1.\x03\x95\xDCr\xCF\xC3\x07sC\xD1\x85N\xD6\x98\x8E\xDD\x1F\x90\x96\x84\x80\xA4\x80\xF3[\x90` `d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R\xFD[\x84Qb\x82\xB4)`\xE8\x1B\x81R\x83\x90\xFD[`\0\x80\xFD[\x91\x90P4a\x04?W` 6`\x03\x19\x01\x12a\x04?Wa\x06\x02a\x0E5V[3\x84R`\x02` R`\xFF\x82\x85 T\x16\x15a\x04\xD9W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x84R`\x01` R`\xFF\x82\x85 T\x16a\x04\xCBWP\x81\x83R`\x01` R\x82 `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\xACo\xA8X\xE95\nF\xCE\xC1e9\x92n\x0F\xDE%\xB7b\x9F\x84\xB5\xA7+\xFF\xAA\xE4\xDF\x88\x8A\xE8m\x82\x80\xA2\x80\xF3[\x91\x90P4a\x04?W` \x91\x82`\x03\x196\x01\x12a\x08\x83W\x805\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11a\x08\x7FW6`#\x85\x01\x12\x15a\x08\x7FW\x83\x83\x015\x94\x81\x86\x11a\x08{W`$\x95`\x05\x956\x88\x83\x89\x1B\x83\x01\x01\x11a\x03\x9AW3\x89R`\x02\x83R`\xFF\x85\x8A T\x16\x15a\x08lW`\x03\x96\x91\x96T`\x01`\x01`\x80\x1B\x03\x80h\x01\xBC\x16\xD6t\xEC\x80\0\0\x8A\x02\x16\x81\x83\x16\x01\x16\x90`\x01`\x01`\x80\x1B\x03\x19\x16\x17`\x03U\x86\x82T\x01\x82U\x88\x91`\xC2\x19\x826\x03\x01\x92[\x88\x81\x10a\x07\"W\x8A\x80\xF3[\x89\x81\x83\x1B\x84\x01\x015\x84\x81\x12\x15a\x08hW\x83\x01`\xA0\x90\x81`#\x19\x826\x03\x01\x12a\x08dW\x88Q\x91\x82\x01\x82\x81\x10\x89\x82\x11\x17a\x08PW\x89R`\x01`\x01`\xA0\x1B\x03\x90\x80\x8D\x015\x82\x81\x16\x81\x03a\x05\xE1W\x83R`D\x81\x015\x89\x81\x11a\x08LWa\x07\x89\x90\x8E6\x91\x84\x01\x01a\x0F+V[\x92\x88\x81\x01\x93\x84R\x8A\x81\x01\x91`d\x81\x015\x83R`\x84\x81\x015\x93\x8B\x85\x11a\x08GW\x8F\x8B\x8Ea\x07\xDE\x7F\xFC\x94\"\xD7\x96S6\x11\xE7A\xD6\xD7tB6go!\x14\xB7\xE0\x1C\xB7\xF2\xCCA+\x12\xCE\x1D\xA5V\x98a\x08\x1D\x946\x91\x88\x01\x01a\x0F+V[\x90`\xA4``\x96\x83\x88\x8A\x01R\x015\x94`\x80\x97\x86\x89\x82\x01RQ\x16\x99Q\x97Qa\x08\r\x82Q\x99\x89\x8B\x9A\x8BR\x8A\x01\x90a\x0E\xEBV[\x93\x88\x01R\x86\x83\x03\x90\x87\x01Ra\x0E\xEBV[\x91\x83\x01R\x03\x90\xA2`\0\x19\x81\x14a\x085W`\x01\x01a\x07\x17V[cNH{q`\xE0\x1B\x8BR`\x11\x88R\x89\x8B\xFD[P\x8F\x80\xFD[\x8E\x80\xFD[\x8C`A\x8CcNH{q`\xE0\x1B`\0RR`\0\xFD[\x8C\x80\xFD[\x8B\x80\xFD[\x84Qb\x82\xB4)`\xE8\x1B\x81R\x86\x90\xFD[\x86\x80\xFD[\x85\x80\xFD[\x83\x80\xFD[\x83\x834a\x01:W` 6`\x03\x19\x01\x12a\x01:Wa\x08\xA2a\x0E5V[3\x83R`\x02` R`\xFF\x82\x84 T\x16\x15a\tPW`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x83R`\x02` R`\xFF\x81\x84 T\x16a\tAW\x82T`\xFF\x81\x16`\xFF\x81\x14a\t.W`\xFF\x19\x91\x82\x16`\x01\x91\x82\x01`\xFF\x16\x17\x85U\x83\x85R`\x02` R\x91\x84 \x80T\x90\x91\x16\x90\x91\x17\x90U\x7FD\xD6\xD2Yc\xF0\x97\xAD\x14\xF2\x9F\x06\x85J\x01\xF5ud\x8A\x1E\xF8/0\xE5b\xCC\xD3\x88\x97\x17\xE39\x82\x80\xA2\x80\xF3[cNH{q`\xE0\x1B\x85R`\x11\x86R`$\x85\xFD[Qc\xA7A\xA0E`\xE0\x1B\x81R\x83\x90\xFD[\x81Qb\x82\xB4)`\xE8\x1B\x81R\x84\x90\xFD[\x90P4a\x04?W` 6`\x03\x19\x01\x12a\x04?W\x805\x913\x84R`\x01` R`\xFF\x81\x85 T\x16\x15a\nUWG\x83\x11a\nHW\x90\x83\x91`\x01`\x01`\x80\x1B\x03\x80\x85\x16\x90\x83T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x83\x85\x81\x84\x16\x01\x16\x91\x16\x17\x85U`\x03T\x92\x83`\x80\x1C\x01`\x80\x1B\x16\x91\x16\x17`\x03U`\x01`\x01`\xA0\x1B\x03`\x06T\x16\x91\x82;\x15a\x08\x83W\x81QcU\x8C\xB7\xF7`\xE0\x1B\x81R\x92\x84\x91\x84\x91\x82\x90\x88\x90Z\xF1\x90\x81\x15a\n?WPa\n+W[P\x80\x7F\xB0\xC6Z[20\"\xD9&\xC4V\xE1VM\x86\xF0\xBD\xA4\x01`\xA7\x81\xC8 \xF5q\xD7c[4\x88\x01\x91\xA2\x80\xF3[a\n4\x90a\x0EKV[a\x01:W\x818a\n\x03V[Q=\x84\x82>=\x90\xFD[Qc\xF1JB\xB7`\xE0\x1B\x81R\xFD[Qb\x82\xB4)`\xE8\x1B\x81R\xFD[PP4a\x01:W\x81`\x03\x196\x01\x12a\x01:W` \x90Qh\x01\xBC\x16\xD6t\xEC\x80\0\0\x81R\xF3[\x91\x90P4a\x04?W\x82`\x03\x196\x01\x12a\x04?W`\x01`\x01`\xA0\x1B\x03`\x06T\x163\x03a\x0B\x04WP\x80T`\x01`\x01`\x80\x1B\x03\x19h\x01\xBC\x16\xD6t\xEC\x80\0\0\x92`\x01`\x01`\x80\x1B\x03\x92\x83\x83\x86\x83`\x80\x1C\x01`\x80\x1B\x16\x91\x16\x17\x90U`\x03T\x92\x83`\x80\x1C\x01`\x80\x1B\x16\x91\x16\x17`\x03U`\x05T\x80a\n\xFAWP\x80\xF3[`\0\x19\x01`\x05U\x80\xF3[Qc\xE3'+\xBB`\xE0\x1B\x81R\xFD[\x90P4a\x04?W` 6`\x03\x19\x01\x12a\x04?Wa\x0B,a\x0E5V[3\x84R`\x02` R`\xFF\x83\x85 T\x16\x15a\x0B\x98W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x15a\x0B\x8BWPP\x80`\x01`\x01`\xA0\x1B\x03\x19`\x06T\x16\x17`\x06U\x7F\x05\xBF\\\xA0b)=\xE5g\xF8C\xF2\xE7\xE8\xB0$\xD0\xB1Ukg\x98T\xE7\x9FS\xC1\x94\x02\xA8;\x07\x82\x80\xA2\x80\xF3[Qc\xD9.#=`\xE0\x1B\x81R\xFD[P\x90Qb\x82\xB4)`\xE8\x1B\x81R\xFD[PP4a\x01:W` 6`\x03\x19\x01\x12a\x01:W`\xFF\x81` \x93`\x01`\x01`\xA0\x1B\x03a\x0B\xCFa\x0E5V[\x16\x81R`\x02\x85R T\x16\x90Q\x90\x15\x15\x81R\xF3[\x90P4a\x04?W` 6`\x03\x19\x01\x12a\x04?Wa\x0B\xFDa\x0E5V[3\x84R`\x02` R`\xFF\x83\x85 T\x16\x15a\x0B\x98W`\x01`\x01`\xA0\x1B\x03\x16\x91\x82\x84R`\x02` R`\xFF\x81\x85 T\x16\x15a\x0C\xB0W\x83T\x91`\xFF\x83\x16\x80\x15a\x0C\x9DW`\xFF\x90`\0\x19\x01\x16\x85`\xFF\x19\x94\x82\x86\x82\x16\x17\x82U\x16\x17\x15a\x0C\x8FWP\x82\x84R`\x02` R\x83 \x90\x81T\x16\x90U\x7F\x98\x9D\xDF\xCE\x05}\xAD!\x9E\n\xE1oi\x1B\x12\x1B\xB0\xE3H\xF0\xD8\xAE\n\xD4\0\xB4\xD5\xAC\x8Dal\x8B\x82\x80\xA2\x80\xF3[\x90Qc\x1F\x8C\x1D\xBD`\xE1\x1B\x81R\xFD[cNH{q`\xE0\x1B\x86R`\x11\x82R`$\x86\xFD[Qc\xA7A\xA0E`\xE0\x1B\x81R\xFD[\x91\x90P4a\x04?W\x82`\x03\x196\x01\x12a\x04?W`\x80\x92P`\x03T\x91T\x90\x80Q\x92`\x01`\x01`\x80\x1B\x03\x90\x81\x81\x16\x85R\x85\x1C` \x85\x01R\x82\x16\x90\x83\x01R\x82\x1C``\x82\x01R\xF3[\x83\x834a\x01:W\x81`\x03\x196\x01\x12a\x01:W3\x82R`\x01` R`\xFF\x81\x83 T\x16\x15a\r\xEDWh\x01\xBC\x16\xD6t\xEC\x80\0\0\x92G\x84\x11a\r\xDFW`\x01`\x01`\xA0\x1B\x03`\x06T\x16\x90\x81;\x15a\x08\x83W\x82Qc\xFE\x182\x11`\xE0\x1B\x81R\x91\x84\x91\x83\x91\x82\x90\x88\x90Z\xF1\x80\x15a\r\xD3Wa\r\xA2W[P\x7F\x12\xB9d\xA3\x99=\x15\x98\xDD\x8A;bz;\x90\xB4\xBCkz\x8FO\x8B\xB6\xAF\xDE\x02\xA3\r\x17\x8E(\xEF\x91\x92\x81Q\x903\x82R` \x82\x01R\xA1\x80\xF3[\x91a\r\xCD\x7F\x12\xB9d\xA3\x99=\x15\x98\xDD\x8A;bz;\x90\xB4\xBCkz\x8FO\x8B\xB6\xAF\xDE\x02\xA3\r\x17\x8E(\xEF\x93a\x0EKV[\x91a\roV[PPQ\x90=\x90\x82>=\x90\xFD[\x90Qc\xF1JB\xB7`\xE0\x1B\x81R\xFD[Qb\x82\xB4)`\xE8\x1B\x81R\x90P\xFD[\x84\x90\x844a\x04?W` 6`\x03\x19\x01\x12a\x04?W`\xFF\x90` \x93`\x01`\x01`\xA0\x1B\x03a\x0E%a\x0E5V[\x16\x81R`\x01\x85R T\x16\x15\x15\x81R\xF3[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x05\xE1WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0E_W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E_W`@RV[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x05\xE1W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x05\xE1W` \x01\x91\x816\x03\x83\x13a\x05\xE1WV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x0F\x17WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x0E\xF6V[\x81`\x1F\x82\x01\x12\x15a\x05\xE1W\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0E_W`@Q\x92a\x0F``\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\x0EuV[\x82\x84R` \x83\x83\x01\x01\x11a\x05\xE1W\x81`\0\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V\xFE\xA2dipfsX\"\x12 n\xEF\x08\xE0p:\xECj0D,s\xF8\xE6\x06{\xAA|l\xC3\xCFg@\x7Fer[\\\x06\xEA]\xEFdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static WAGYUSTAKER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct WagyuStaker<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for WagyuStaker<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for WagyuStaker<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for WagyuStaker<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for WagyuStaker<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(WagyuStaker))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> WagyuStaker<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    WAGYUSTAKER_ABI.clone(),
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
                WAGYUSTAKER_ABI.clone(),
                WAGYUSTAKER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BEACON_CHAIN_DEPOSIT_CONTRACT` (0xcceab750) function
        pub fn beacon_chain_deposit_contract(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([204, 234, 183, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `VALIDATOR_DEPOSIT_SIZE` (0x5552aa65) function
        pub fn validator_deposit_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([85, 82, 170, 101], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `batchMigrate` (0x832b7c6b) function
        pub fn batch_migrate(
            &self,
            batch_data: ::std::vec::Vec<ValidatorData>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 43, 124, 107], batch_data)
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
        ///Calls the contract's `deposit` (0xb778a3a7) function
        pub fn deposit(
            &self,
            data: ValidatorData,
            latest_deposit_root: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 120, 163, 167], (data, latest_deposit_root))
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
        ///Calls the contract's `payRewards` (0x60e3a0ac) function
        pub fn pay_rewards(
            &self,
            rewards: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 227, 160, 172], rewards)
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
        ///Calls the contract's `record` (0x266cf109) function
        pub fn record(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128, u128, u128)> {
            self.0
                .method_hash([38, 108, 241, 9], ())
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
        ///Calls the contract's `registerExit` (0x4ad8d34b) function
        pub fn register_exit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 216, 211, 75], ())
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
        ///Calls the contract's `validators` (0xca1e7819) function
        pub fn validators(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 30, 120, 25], ())
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
        ///Gets the contract's `NewValidator` event
        pub fn new_validator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewValidatorFilter,
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
            WagyuStakerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for WagyuStaker<M> {
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
    ///Custom Error type `DepositWasFrontrun` with signature `DepositWasFrontrun()` and selector `0xc61cf4c5`
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
    #[etherror(name = "DepositWasFrontrun", abi = "DepositWasFrontrun()")]
    pub struct DepositWasFrontrun;
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
    ///Custom Error type `UnAuthorizedCaller` with signature `UnAuthorizedCaller()` and selector `0xe3272bbb`
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
    #[etherror(name = "UnAuthorizedCaller", abi = "UnAuthorizedCaller()")]
    pub struct UnAuthorizedCaller;
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
    ///Custom Error type `WrongDepositAmount` with signature `WrongDepositAmount()` and selector `0xdfe159f2`
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
    #[etherror(name = "WrongDepositAmount", abi = "WrongDepositAmount()")]
    pub struct WrongDepositAmount;
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
    pub enum WagyuStakerErrors {
        AlreadySet(AlreadySet),
        DepositWasFrontrun(DepositWasFrontrun),
        NoAdmin(NoAdmin),
        NotEnoughEth(NotEnoughEth),
        UnAuthorizedCaller(UnAuthorizedCaller),
        Unauthorized(Unauthorized),
        WrongDepositAmount(WrongDepositAmount),
        ZeroAddress(ZeroAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for WagyuStakerErrors {
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
                = <DepositWasFrontrun as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositWasFrontrun(decoded));
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
                = <UnAuthorizedCaller as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnAuthorizedCaller(decoded));
            }
            if let Ok(decoded)
                = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unauthorized(decoded));
            }
            if let Ok(decoded)
                = <WrongDepositAmount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongDepositAmount(decoded));
            }
            if let Ok(decoded)
                = <ZeroAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WagyuStakerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AlreadySet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositWasFrontrun(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotEnoughEth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnAuthorizedCaller(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrongDepositAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for WagyuStakerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AlreadySet as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <DepositWasFrontrun as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoAdmin as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotEnoughEth as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <UnAuthorizedCaller as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <WrongDepositAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroAddress as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for WagyuStakerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AlreadySet(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositWasFrontrun(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotEnoughEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnAuthorizedCaller(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongDepositAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ZeroAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for WagyuStakerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AlreadySet> for WagyuStakerErrors {
        fn from(value: AlreadySet) -> Self {
            Self::AlreadySet(value)
        }
    }
    impl ::core::convert::From<DepositWasFrontrun> for WagyuStakerErrors {
        fn from(value: DepositWasFrontrun) -> Self {
            Self::DepositWasFrontrun(value)
        }
    }
    impl ::core::convert::From<NoAdmin> for WagyuStakerErrors {
        fn from(value: NoAdmin) -> Self {
            Self::NoAdmin(value)
        }
    }
    impl ::core::convert::From<NotEnoughEth> for WagyuStakerErrors {
        fn from(value: NotEnoughEth) -> Self {
            Self::NotEnoughEth(value)
        }
    }
    impl ::core::convert::From<UnAuthorizedCaller> for WagyuStakerErrors {
        fn from(value: UnAuthorizedCaller) -> Self {
            Self::UnAuthorizedCaller(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for WagyuStakerErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
        }
    }
    impl ::core::convert::From<WrongDepositAmount> for WagyuStakerErrors {
        fn from(value: WrongDepositAmount) -> Self {
            Self::WrongDepositAmount(value)
        }
    }
    impl ::core::convert::From<ZeroAddress> for WagyuStakerErrors {
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
    #[ethevent(
        name = "NewValidator",
        abi = "NewValidator(address,bytes,bytes32,bytes,bytes32)"
    )]
    pub struct NewValidatorFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub pubkey: ::ethers::core::types::Bytes,
        pub withdrawal_credentials: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
        pub deposit_data_root: [u8; 32],
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
    #[ethevent(name = "RewardsPaid", abi = "RewardsPaid(uint256)")]
    pub struct RewardsPaidFilter {
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
    pub enum WagyuStakerEvents {
        AdminAddedFilter(AdminAddedFilter),
        AdminDeletedFilter(AdminDeletedFilter),
        MevEthUpdatedFilter(MevEthUpdatedFilter),
        NewValidatorFilter(NewValidatorFilter),
        OperatorAddedFilter(OperatorAddedFilter),
        OperatorDeletedFilter(OperatorDeletedFilter),
        RewardsPaidFilter(RewardsPaidFilter),
        TokenRecoveredFilter(TokenRecoveredFilter),
        ValidatorWithdrawFilter(ValidatorWithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for WagyuStakerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminAddedFilter::decode_log(log) {
                return Ok(WagyuStakerEvents::AdminAddedFilter(decoded));
            }
            if let Ok(decoded) = AdminDeletedFilter::decode_log(log) {
                return Ok(WagyuStakerEvents::AdminDeletedFilter(decoded));
            }
            if let Ok(decoded) = MevEthUpdatedFilter::decode_log(log) {
                return Ok(WagyuStakerEvents::MevEthUpdatedFilter(decoded));
            }
            if let Ok(decoded) = NewValidatorFilter::decode_log(log) {
                return Ok(WagyuStakerEvents::NewValidatorFilter(decoded));
            }
            if let Ok(decoded) = OperatorAddedFilter::decode_log(log) {
                return Ok(WagyuStakerEvents::OperatorAddedFilter(decoded));
            }
            if let Ok(decoded) = OperatorDeletedFilter::decode_log(log) {
                return Ok(WagyuStakerEvents::OperatorDeletedFilter(decoded));
            }
            if let Ok(decoded) = RewardsPaidFilter::decode_log(log) {
                return Ok(WagyuStakerEvents::RewardsPaidFilter(decoded));
            }
            if let Ok(decoded) = TokenRecoveredFilter::decode_log(log) {
                return Ok(WagyuStakerEvents::TokenRecoveredFilter(decoded));
            }
            if let Ok(decoded) = ValidatorWithdrawFilter::decode_log(log) {
                return Ok(WagyuStakerEvents::ValidatorWithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for WagyuStakerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminDeletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MevEthUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewValidatorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorDeletedFilter(element) => {
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
    impl ::core::convert::From<AdminAddedFilter> for WagyuStakerEvents {
        fn from(value: AdminAddedFilter) -> Self {
            Self::AdminAddedFilter(value)
        }
    }
    impl ::core::convert::From<AdminDeletedFilter> for WagyuStakerEvents {
        fn from(value: AdminDeletedFilter) -> Self {
            Self::AdminDeletedFilter(value)
        }
    }
    impl ::core::convert::From<MevEthUpdatedFilter> for WagyuStakerEvents {
        fn from(value: MevEthUpdatedFilter) -> Self {
            Self::MevEthUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<NewValidatorFilter> for WagyuStakerEvents {
        fn from(value: NewValidatorFilter) -> Self {
            Self::NewValidatorFilter(value)
        }
    }
    impl ::core::convert::From<OperatorAddedFilter> for WagyuStakerEvents {
        fn from(value: OperatorAddedFilter) -> Self {
            Self::OperatorAddedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorDeletedFilter> for WagyuStakerEvents {
        fn from(value: OperatorDeletedFilter) -> Self {
            Self::OperatorDeletedFilter(value)
        }
    }
    impl ::core::convert::From<RewardsPaidFilter> for WagyuStakerEvents {
        fn from(value: RewardsPaidFilter) -> Self {
            Self::RewardsPaidFilter(value)
        }
    }
    impl ::core::convert::From<TokenRecoveredFilter> for WagyuStakerEvents {
        fn from(value: TokenRecoveredFilter) -> Self {
            Self::TokenRecoveredFilter(value)
        }
    }
    impl ::core::convert::From<ValidatorWithdrawFilter> for WagyuStakerEvents {
        fn from(value: ValidatorWithdrawFilter) -> Self {
            Self::ValidatorWithdrawFilter(value)
        }
    }
    ///Container type for all input parameters for the `BEACON_CHAIN_DEPOSIT_CONTRACT` function with signature `BEACON_CHAIN_DEPOSIT_CONTRACT()` and selector `0xcceab750`
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
        name = "BEACON_CHAIN_DEPOSIT_CONTRACT",
        abi = "BEACON_CHAIN_DEPOSIT_CONTRACT()"
    )]
    pub struct BeaconChainDepositContractCall;
    ///Container type for all input parameters for the `VALIDATOR_DEPOSIT_SIZE` function with signature `VALIDATOR_DEPOSIT_SIZE()` and selector `0x5552aa65`
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
    #[ethcall(name = "VALIDATOR_DEPOSIT_SIZE", abi = "VALIDATOR_DEPOSIT_SIZE()")]
    pub struct ValidatorDepositSizeCall;
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
    ///Container type for all input parameters for the `batchMigrate` function with signature `batchMigrate((address,bytes,bytes32,bytes,bytes32)[])` and selector `0x832b7c6b`
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
        name = "batchMigrate",
        abi = "batchMigrate((address,bytes,bytes32,bytes,bytes32)[])"
    )]
    pub struct BatchMigrateCall {
        pub batch_data: ::std::vec::Vec<ValidatorData>,
    }
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
    ///Container type for all input parameters for the `deposit` function with signature `deposit((address,bytes,bytes32,bytes,bytes32),bytes32)` and selector `0xb778a3a7`
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
        name = "deposit",
        abi = "deposit((address,bytes,bytes32,bytes,bytes32),bytes32)"
    )]
    pub struct DepositCall {
        pub data: ValidatorData,
        pub latest_deposit_root: [u8; 32],
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
    ///Container type for all input parameters for the `payRewards` function with signature `payRewards(uint256)` and selector `0x60e3a0ac`
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
    #[ethcall(name = "payRewards", abi = "payRewards(uint256)")]
    pub struct PayRewardsCall {
        pub rewards: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `record` function with signature `record()` and selector `0x266cf109`
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
    #[ethcall(name = "record", abi = "record()")]
    pub struct RecordCall;
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
    ///Container type for all input parameters for the `registerExit` function with signature `registerExit()` and selector `0x4ad8d34b`
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
    #[ethcall(name = "registerExit", abi = "registerExit()")]
    pub struct RegisterExitCall;
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
    ///Container type for all input parameters for the `validators` function with signature `validators()` and selector `0xca1e7819`
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
    #[ethcall(name = "validators", abi = "validators()")]
    pub struct ValidatorsCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum WagyuStakerCalls {
        BeaconChainDepositContract(BeaconChainDepositContractCall),
        ValidatorDepositSize(ValidatorDepositSizeCall),
        AddAdmin(AddAdminCall),
        AddOperator(AddOperatorCall),
        Admins(AdminsCall),
        BatchMigrate(BatchMigrateCall),
        DeleteAdmin(DeleteAdminCall),
        DeleteOperator(DeleteOperatorCall),
        Deposit(DepositCall),
        MevEth(MevEthCall),
        Operators(OperatorsCall),
        PayRewards(PayRewardsCall),
        PayValidatorWithdraw(PayValidatorWithdrawCall),
        Record(RecordCall),
        RecoverToken(RecoverTokenCall),
        RegisterExit(RegisterExitCall),
        SetNewMevEth(SetNewMevEthCall),
        Validators(ValidatorsCall),
    }
    impl ::ethers::core::abi::AbiDecode for WagyuStakerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BeaconChainDepositContractCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BeaconChainDepositContract(decoded));
            }
            if let Ok(decoded)
                = <ValidatorDepositSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ValidatorDepositSize(decoded));
            }
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
                = <BatchMigrateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BatchMigrate(decoded));
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
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
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
                = <RecordCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Record(decoded));
            }
            if let Ok(decoded)
                = <RecoverTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RecoverToken(decoded));
            }
            if let Ok(decoded)
                = <RegisterExitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RegisterExit(decoded));
            }
            if let Ok(decoded)
                = <SetNewMevEthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetNewMevEth(decoded));
            }
            if let Ok(decoded)
                = <ValidatorsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Validators(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WagyuStakerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BeaconChainDepositContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidatorDepositSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admins(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BatchMigrate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeleteAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeleteOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Record(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecoverToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterExit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetNewMevEth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Validators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for WagyuStakerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BeaconChainDepositContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorDepositSize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admins(element) => ::core::fmt::Display::fmt(element, f),
                Self::BatchMigrate(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::MevEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::Operators(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayValidatorWithdraw(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Record(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecoverToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterExit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNewMevEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::Validators(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BeaconChainDepositContractCall> for WagyuStakerCalls {
        fn from(value: BeaconChainDepositContractCall) -> Self {
            Self::BeaconChainDepositContract(value)
        }
    }
    impl ::core::convert::From<ValidatorDepositSizeCall> for WagyuStakerCalls {
        fn from(value: ValidatorDepositSizeCall) -> Self {
            Self::ValidatorDepositSize(value)
        }
    }
    impl ::core::convert::From<AddAdminCall> for WagyuStakerCalls {
        fn from(value: AddAdminCall) -> Self {
            Self::AddAdmin(value)
        }
    }
    impl ::core::convert::From<AddOperatorCall> for WagyuStakerCalls {
        fn from(value: AddOperatorCall) -> Self {
            Self::AddOperator(value)
        }
    }
    impl ::core::convert::From<AdminsCall> for WagyuStakerCalls {
        fn from(value: AdminsCall) -> Self {
            Self::Admins(value)
        }
    }
    impl ::core::convert::From<BatchMigrateCall> for WagyuStakerCalls {
        fn from(value: BatchMigrateCall) -> Self {
            Self::BatchMigrate(value)
        }
    }
    impl ::core::convert::From<DeleteAdminCall> for WagyuStakerCalls {
        fn from(value: DeleteAdminCall) -> Self {
            Self::DeleteAdmin(value)
        }
    }
    impl ::core::convert::From<DeleteOperatorCall> for WagyuStakerCalls {
        fn from(value: DeleteOperatorCall) -> Self {
            Self::DeleteOperator(value)
        }
    }
    impl ::core::convert::From<DepositCall> for WagyuStakerCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<MevEthCall> for WagyuStakerCalls {
        fn from(value: MevEthCall) -> Self {
            Self::MevEth(value)
        }
    }
    impl ::core::convert::From<OperatorsCall> for WagyuStakerCalls {
        fn from(value: OperatorsCall) -> Self {
            Self::Operators(value)
        }
    }
    impl ::core::convert::From<PayRewardsCall> for WagyuStakerCalls {
        fn from(value: PayRewardsCall) -> Self {
            Self::PayRewards(value)
        }
    }
    impl ::core::convert::From<PayValidatorWithdrawCall> for WagyuStakerCalls {
        fn from(value: PayValidatorWithdrawCall) -> Self {
            Self::PayValidatorWithdraw(value)
        }
    }
    impl ::core::convert::From<RecordCall> for WagyuStakerCalls {
        fn from(value: RecordCall) -> Self {
            Self::Record(value)
        }
    }
    impl ::core::convert::From<RecoverTokenCall> for WagyuStakerCalls {
        fn from(value: RecoverTokenCall) -> Self {
            Self::RecoverToken(value)
        }
    }
    impl ::core::convert::From<RegisterExitCall> for WagyuStakerCalls {
        fn from(value: RegisterExitCall) -> Self {
            Self::RegisterExit(value)
        }
    }
    impl ::core::convert::From<SetNewMevEthCall> for WagyuStakerCalls {
        fn from(value: SetNewMevEthCall) -> Self {
            Self::SetNewMevEth(value)
        }
    }
    impl ::core::convert::From<ValidatorsCall> for WagyuStakerCalls {
        fn from(value: ValidatorsCall) -> Self {
            Self::Validators(value)
        }
    }
    ///Container type for all return fields from the `BEACON_CHAIN_DEPOSIT_CONTRACT` function with signature `BEACON_CHAIN_DEPOSIT_CONTRACT()` and selector `0xcceab750`
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
    pub struct BeaconChainDepositContractReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `VALIDATOR_DEPOSIT_SIZE` function with signature `VALIDATOR_DEPOSIT_SIZE()` and selector `0x5552aa65`
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
    pub struct ValidatorDepositSizeReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `record` function with signature `record()` and selector `0x266cf109`
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
    pub struct RecordReturn {
        pub total_deposited: u128,
        pub total_withdrawn: u128,
        pub total_rewards_paid: u128,
        pub total_validator_exits_paid: u128,
    }
    ///Container type for all return fields from the `validators` function with signature `validators()` and selector `0xca1e7819`
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
    pub struct ValidatorsReturn(pub ::ethers::core::types::U256);
}
