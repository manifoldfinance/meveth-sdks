pub use base_oft_with_fee::*;
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
pub mod base_oft_with_fee {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BP_DENOMINATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BP_DENOMINATOR"),
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
                    ::std::borrow::ToOwned::to_owned("DEFAULT_PAYLOAD_SIZE_LIMIT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DEFAULT_PAYLOAD_SIZE_LIMIT",
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
                    ::std::borrow::ToOwned::to_owned("NO_EXTRA_GAS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("NO_EXTRA_GAS"),
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
                    ::std::borrow::ToOwned::to_owned("PT_SEND"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PT_SEND"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PT_SEND_AND_CALL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PT_SEND_AND_CALL"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("callOnOFTReceived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("callOnOFTReceived"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasForCall"),
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
                    ::std::borrow::ToOwned::to_owned("chainIdToFeeBps"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("chainIdToFeeBps"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeBP"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
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
                    ::std::borrow::ToOwned::to_owned("circulatingSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("circulatingSupply"),
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
                    ::std::borrow::ToOwned::to_owned("creditedPackets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("creditedPackets"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("defaultFeeBp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defaultFeeBp"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("estimateSendFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("estimateSendFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_toAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_useZro"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_adapterParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nativeFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("zroFee"),
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
                    ::std::borrow::ToOwned::to_owned("failedMessages"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failedMessages"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feeOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeOwner"),
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
                    ::std::borrow::ToOwned::to_owned("forceResumeReceive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("forceResumeReceive"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("getConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getConfig"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_configType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTrustedRemoteAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTrustedRemoteAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isTrustedRemote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isTrustedRemote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("lzEndpoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lzEndpoint"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ILayerZeroEndpoint",
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
                    ::std::borrow::ToOwned::to_owned("lzReceive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lzReceive"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("minDstGasLookup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("minDstGasLookup"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("nonblockingLzReceive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "nonblockingLzReceive",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("payloadSizeLimitLookup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "payloadSizeLimitLookup",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("precrime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("precrime"),
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
                    ::std::borrow::ToOwned::to_owned("quoteOFTFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quoteOFTFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
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
                    ::std::borrow::ToOwned::to_owned("retryMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("retryMessage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("sendFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_toAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_callParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ICommonOFT.LzCallParams",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("setConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setConfig"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_configType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_config"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("setDefaultFeeBp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDefaultFeeBp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_feeBp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("setFeeBp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFeeBp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_feeBp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("setFeeOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFeeOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_feeOwner"),
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
                    ::std::borrow::ToOwned::to_owned("setMinDstGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMinDstGas"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_packetType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minGas"),
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
                    ::std::borrow::ToOwned::to_owned("setPayloadSizeLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setPayloadSizeLimit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_size"),
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
                    ::std::borrow::ToOwned::to_owned("setPrecrime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPrecrime"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_precrime"),
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
                    ::std::borrow::ToOwned::to_owned("setReceiveVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setReceiveVersion"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("setSendVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSendVersion"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("setTrustedRemote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTrustedRemote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("setTrustedRemoteAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setTrustedRemoteAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("setUseCustomAdapterParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setUseCustomAdapterParams",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_useCustomAdapterParams",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("sharedDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sharedDecimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                    ::std::borrow::ToOwned::to_owned("token"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("trustedRemoteLookup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "trustedRemoteLookup",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("useCustomAdapterParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "useCustomAdapterParams",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("CallOFTReceivedSuccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CallOFTReceivedSuccess",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_hash"),
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
                    ::std::borrow::ToOwned::to_owned("MessageFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MessageFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NonContractAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NonContractAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_address"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("ReceiveFromChain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReceiveFromChain"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                (
                    ::std::borrow::ToOwned::to_owned("RetryMessageSuccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RetryMessageSuccess",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_payloadHash"),
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
                    ::std::borrow::ToOwned::to_owned("SendToChain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SendToChain"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_toAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_amount"),
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
                (
                    ::std::borrow::ToOwned::to_owned("SetDefaultFeeBp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetDefaultFeeBp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feeBp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetFeeBp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetFeeBp"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dstchainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feeBp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetFeeOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetFeeOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feeOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetMinDstGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetMinDstGas"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_type"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_minDstGas"),
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
                (
                    ::std::borrow::ToOwned::to_owned("SetPrecrime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetPrecrime"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("precrime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetTrustedRemote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetTrustedRemote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetTrustedRemoteAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SetTrustedRemoteAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_remoteAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetUseCustomAdapterParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SetUseCustomAdapterParams",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_useCustomAdapterParams",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
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
                    ::std::borrow::ToOwned::to_owned("AdapterParamsMustBeEmpty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AdapterParamsMustBeEmpty",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("AmountLessThanMinAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AmountLessThanMinAmount",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AmountSDOverflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AmountSDOverflow"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AmountTooSmall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AmountTooSmall"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CallerMustBeLzApp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("CallerMustBeLzApp"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CallerMustBeOFTCore"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CallerMustBeOFTCore",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DestinationChainNotTrusted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DestinationChainNotTrusted",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeBpTooLarge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FeeBpTooLarge"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeOwnerNotSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FeeOwnerNotSet"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GasLimitTooLow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("GasLimitTooLow"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidAdapterParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidAdapterParams",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidEndpointCaller"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidEndpointCaller",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMinGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidMinGas"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPayload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidPayload"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSourceSendingContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidSourceSendingContract",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MinGasLimitNotSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("MinGasLimitNotSet"),
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
                    ::std::borrow::ToOwned::to_owned("NoStoredMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NoStoredMessage"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoTrustedPath"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NoTrustedPath"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PayloadSizeTooLarge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PayloadSizeTooLarge",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SliceOverflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SliceOverflow"),
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
                    ::std::borrow::ToOwned::to_owned("UnknownPacketType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnknownPacketType"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BASEOFTWITHFEE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct BaseOFTWithFee<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BaseOFTWithFee<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BaseOFTWithFee<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BaseOFTWithFee<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BaseOFTWithFee<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BaseOFTWithFee))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BaseOFTWithFee<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BASEOFTWITHFEE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `BP_DENOMINATOR` (0xabe685cd) function
        pub fn bp_denominator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([171, 230, 133, 205], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEFAULT_PAYLOAD_SIZE_LIMIT` (0xc4461834) function
        pub fn default_payload_size_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([196, 70, 24, 52], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `NO_EXTRA_GAS` (0x44770515) function
        pub fn no_extra_gas(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([68, 119, 5, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PT_SEND` (0x4c42899a) function
        pub fn pt_send(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([76, 66, 137, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PT_SEND_AND_CALL` (0xe6a20ae6) function
        pub fn pt_send_and_call(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([230, 162, 10, 230], ())
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
        ///Calls the contract's `callOnOFTReceived` (0xeaffd49a) function
        pub fn call_on_oft_received(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
            nonce: u64,
            from: [u8; 32],
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            payload: ::ethers::core::types::Bytes,
            gas_for_call: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [234, 255, 212, 154],
                    (
                        src_chain_id,
                        src_address,
                        nonce,
                        from,
                        to,
                        amount,
                        payload,
                        gas_for_call,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainIdToFeeBps` (0xc83330ce) function
        pub fn chain_id_to_fee_bps(
            &self,
            p0: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, (u16, bool)> {
            self.0
                .method_hash([200, 51, 48, 206], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `circulatingSupply` (0x9358928b) function
        pub fn circulating_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([147, 88, 146, 139], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `creditedPackets` (0x9bdb9812) function
        pub fn credited_packets(
            &self,
            p0: u16,
            p1: ::ethers::core::types::Bytes,
            p2: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([155, 219, 152, 18], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultFeeBp` (0xd8882968) function
        pub fn default_fee_bp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([216, 136, 41, 104], ())
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
        ///Calls the contract's `estimateSendFee` (0x365260b4) function
        pub fn estimate_send_fee(
            &self,
            dst_chain_id: u16,
            to_address: [u8; 32],
            amount: ::ethers::core::types::U256,
            use_zro: bool,
            adapter_params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [54, 82, 96, 180],
                    (dst_chain_id, to_address, amount, use_zro, adapter_params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedMessages` (0x5b8c41e6) function
        pub fn failed_messages(
            &self,
            p0: u16,
            p1: ::ethers::core::types::Bytes,
            p2: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([91, 140, 65, 230], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeOwner` (0xb9818be1) function
        pub fn fee_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([185, 129, 139, 225], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `forceResumeReceive` (0x42d65a8d) function
        pub fn force_resume_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 214, 90, 141], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConfig` (0xf5ecbdbc) function
        pub fn get_config(
            &self,
            version: u16,
            chain_id: u16,
            p2: ::ethers::core::types::Address,
            config_type: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([245, 236, 189, 188], (version, chain_id, p2, config_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTrustedRemoteAddress` (0x9f38369a) function
        pub fn get_trusted_remote_address(
            &self,
            remote_chain_id: u16,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([159, 56, 54, 154], remote_chain_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isTrustedRemote` (0x3d8b38f6) function
        pub fn is_trusted_remote(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([61, 139, 56, 246], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lzEndpoint` (0xb353aaa7) function
        pub fn lz_endpoint(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([179, 83, 170, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lzReceive` (0x001d3567) function
        pub fn lz_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
            nonce: u64,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [0, 29, 53, 103],
                    (src_chain_id, src_address, nonce, payload),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minDstGasLookup` (0x8cfd8f5c) function
        pub fn min_dst_gas_lookup(
            &self,
            p0: u16,
            p1: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([140, 253, 143, 92], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonblockingLzReceive` (0x66ad5c8a) function
        pub fn nonblocking_lz_receive(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
            nonce: u64,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [102, 173, 92, 138],
                    (src_chain_id, src_address, nonce, payload),
                )
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
        ///Calls the contract's `payloadSizeLimitLookup` (0x3f1f4fa4) function
        pub fn payload_size_limit_lookup(
            &self,
            p0: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([63, 31, 79, 164], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `precrime` (0x950c8a74) function
        pub fn precrime(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([149, 12, 138, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteOFTFee` (0xecd8f212) function
        pub fn quote_oft_fee(
            &self,
            dst_chain_id: u16,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([236, 216, 242, 18], (dst_chain_id, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `retryMessage` (0xd1deba1f) function
        pub fn retry_message(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
            nonce: u64,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [209, 222, 186, 31],
                    (src_chain_id, src_address, nonce, payload),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendFrom` (0x2cdf0b95) function
        pub fn send_from(
            &self,
            from: ::ethers::core::types::Address,
            dst_chain_id: u16,
            to_address: [u8; 32],
            amount: ::ethers::core::types::U256,
            min_amount: ::ethers::core::types::U256,
            call_params: LzCallParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [44, 223, 11, 149],
                    (from, dst_chain_id, to_address, amount, min_amount, call_params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0xcbed8b9c) function
        pub fn set_config(
            &self,
            version: u16,
            chain_id: u16,
            config_type: ::ethers::core::types::U256,
            config: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [203, 237, 139, 156],
                    (version, chain_id, config_type, config),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDefaultFeeBp` (0x5a359dc5) function
        pub fn set_default_fee_bp(
            &self,
            fee_bp: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 53, 157, 197], fee_bp)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeBp` (0x79c0ad4b) function
        pub fn set_fee_bp(
            &self,
            dst_chain_id: u16,
            enabled: bool,
            fee_bp: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 192, 173, 75], (dst_chain_id, enabled, fee_bp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeOwner` (0x4b104eff) function
        pub fn set_fee_owner(
            &self,
            fee_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 16, 78, 255], fee_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMinDstGas` (0xdf2a5b3b) function
        pub fn set_min_dst_gas(
            &self,
            dst_chain_id: u16,
            packet_type: u16,
            min_gas: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 42, 91, 59], (dst_chain_id, packet_type, min_gas))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPayloadSizeLimit` (0x0df37483) function
        pub fn set_payload_size_limit(
            &self,
            dst_chain_id: u16,
            size: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 243, 116, 131], (dst_chain_id, size))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPrecrime` (0xbaf3292d) function
        pub fn set_precrime(
            &self,
            precrime: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 243, 41, 45], precrime)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReceiveVersion` (0x10ddb137) function
        pub fn set_receive_version(
            &self,
            version: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 221, 177, 55], version)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSendVersion` (0x07e0db17) function
        pub fn set_send_version(
            &self,
            version: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 224, 219, 23], version)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTrustedRemote` (0xeb8d72b7) function
        pub fn set_trusted_remote(
            &self,
            remote_chain_id: u16,
            path: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 141, 114, 183], (remote_chain_id, path))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTrustedRemoteAddress` (0xa6c3d165) function
        pub fn set_trusted_remote_address(
            &self,
            remote_chain_id: u16,
            remote_address: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 195, 209, 101], (remote_chain_id, remote_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUseCustomAdapterParams` (0xeab45d9c) function
        pub fn set_use_custom_adapter_params(
            &self,
            use_custom_adapter_params: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 180, 93, 156], use_custom_adapter_params)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sharedDecimals` (0x857749b0) function
        pub fn shared_decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([133, 119, 73, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token` (0xfc0c546a) function
        pub fn token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trustedRemoteLookup` (0x7533d788) function
        pub fn trusted_remote_lookup(
            &self,
            p0: u16,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([117, 51, 215, 136], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `useCustomAdapterParams` (0xed629c5c) function
        pub fn use_custom_adapter_params(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([237, 98, 156, 92], ())
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
        ///Gets the contract's `CallOFTReceivedSuccess` event
        pub fn call_oft_received_success_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CallOFTReceivedSuccessFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MessageFailed` event
        pub fn message_failed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MessageFailedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NonContractAddress` event
        pub fn non_contract_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NonContractAddressFilter,
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
        ///Gets the contract's `ReceiveFromChain` event
        pub fn receive_from_chain_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReceiveFromChainFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RetryMessageSuccess` event
        pub fn retry_message_success_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RetryMessageSuccessFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SendToChain` event
        pub fn send_to_chain_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SendToChainFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetDefaultFeeBp` event
        pub fn set_default_fee_bp_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetDefaultFeeBpFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetFeeBp` event
        pub fn set_fee_bp_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetFeeBpFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetFeeOwner` event
        pub fn set_fee_owner_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetFeeOwnerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetMinDstGas` event
        pub fn set_min_dst_gas_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetMinDstGasFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetPrecrime` event
        pub fn set_precrime_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetPrecrimeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetTrustedRemote` event
        pub fn set_trusted_remote_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetTrustedRemoteFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetTrustedRemoteAddress` event
        pub fn set_trusted_remote_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetTrustedRemoteAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetUseCustomAdapterParams` event
        pub fn set_use_custom_adapter_params_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetUseCustomAdapterParamsFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BaseOFTWithFeeEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BaseOFTWithFee<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AdapterParamsMustBeEmpty` with signature `AdapterParamsMustBeEmpty()` and selector `0x8fadcadb`
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
    #[etherror(name = "AdapterParamsMustBeEmpty", abi = "AdapterParamsMustBeEmpty()")]
    pub struct AdapterParamsMustBeEmpty;
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
    ///Custom Error type `AmountLessThanMinAmount` with signature `AmountLessThanMinAmount()` and selector `0x40844759`
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
    #[etherror(name = "AmountLessThanMinAmount", abi = "AmountLessThanMinAmount()")]
    pub struct AmountLessThanMinAmount;
    ///Custom Error type `AmountSDOverflow` with signature `AmountSDOverflow()` and selector `0x6249331a`
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
    #[etherror(name = "AmountSDOverflow", abi = "AmountSDOverflow()")]
    pub struct AmountSDOverflow;
    ///Custom Error type `AmountTooSmall` with signature `AmountTooSmall()` and selector `0xc2f5625a`
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
    #[etherror(name = "AmountTooSmall", abi = "AmountTooSmall()")]
    pub struct AmountTooSmall;
    ///Custom Error type `CallerMustBeLzApp` with signature `CallerMustBeLzApp()` and selector `0x1c9f02a0`
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
    #[etherror(name = "CallerMustBeLzApp", abi = "CallerMustBeLzApp()")]
    pub struct CallerMustBeLzApp;
    ///Custom Error type `CallerMustBeOFTCore` with signature `CallerMustBeOFTCore()` and selector `0x82a9b08c`
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
    #[etherror(name = "CallerMustBeOFTCore", abi = "CallerMustBeOFTCore()")]
    pub struct CallerMustBeOFTCore;
    ///Custom Error type `DestinationChainNotTrusted` with signature `DestinationChainNotTrusted()` and selector `0x26ba7cfb`
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
    #[etherror(
        name = "DestinationChainNotTrusted",
        abi = "DestinationChainNotTrusted()"
    )]
    pub struct DestinationChainNotTrusted;
    ///Custom Error type `FeeBpTooLarge` with signature `FeeBpTooLarge()` and selector `0x1f801e32`
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
    #[etherror(name = "FeeBpTooLarge", abi = "FeeBpTooLarge()")]
    pub struct FeeBpTooLarge;
    ///Custom Error type `FeeOwnerNotSet` with signature `FeeOwnerNotSet()` and selector `0xa6afc53d`
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
    #[etherror(name = "FeeOwnerNotSet", abi = "FeeOwnerNotSet()")]
    pub struct FeeOwnerNotSet;
    ///Custom Error type `GasLimitTooLow` with signature `GasLimitTooLow()` and selector `0xed43c3a6`
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
    #[etherror(name = "GasLimitTooLow", abi = "GasLimitTooLow()")]
    pub struct GasLimitTooLow;
    ///Custom Error type `InvalidAdapterParams` with signature `InvalidAdapterParams()` and selector `0xcef80ea3`
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
    #[etherror(name = "InvalidAdapterParams", abi = "InvalidAdapterParams()")]
    pub struct InvalidAdapterParams;
    ///Custom Error type `InvalidEndpointCaller` with signature `InvalidEndpointCaller()` and selector `0x0d1ad4cd`
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
    #[etherror(name = "InvalidEndpointCaller", abi = "InvalidEndpointCaller()")]
    pub struct InvalidEndpointCaller;
    ///Custom Error type `InvalidMinGas` with signature `InvalidMinGas()` and selector `0xe4ac3b3f`
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
    #[etherror(name = "InvalidMinGas", abi = "InvalidMinGas()")]
    pub struct InvalidMinGas;
    ///Custom Error type `InvalidPayload` with signature `InvalidPayload()` and selector `0x7c6953f9`
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
    #[etherror(name = "InvalidPayload", abi = "InvalidPayload()")]
    pub struct InvalidPayload;
    ///Custom Error type `InvalidSourceSendingContract` with signature `InvalidSourceSendingContract()` and selector `0x326bc502`
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
    #[etherror(
        name = "InvalidSourceSendingContract",
        abi = "InvalidSourceSendingContract()"
    )]
    pub struct InvalidSourceSendingContract;
    ///Custom Error type `MinGasLimitNotSet` with signature `MinGasLimitNotSet()` and selector `0x3e7d93aa`
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
    #[etherror(name = "MinGasLimitNotSet", abi = "MinGasLimitNotSet()")]
    pub struct MinGasLimitNotSet;
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
    ///Custom Error type `NoStoredMessage` with signature `NoStoredMessage()` and selector `0xae5b2614`
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
    #[etherror(name = "NoStoredMessage", abi = "NoStoredMessage()")]
    pub struct NoStoredMessage;
    ///Custom Error type `NoTrustedPath` with signature `NoTrustedPath()` and selector `0x29426308`
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
    #[etherror(name = "NoTrustedPath", abi = "NoTrustedPath()")]
    pub struct NoTrustedPath;
    ///Custom Error type `OutOfBounds` with signature `OutOfBounds()` and selector `0xb4120f14`
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
    #[etherror(name = "OutOfBounds", abi = "OutOfBounds()")]
    pub struct OutOfBounds;
    ///Custom Error type `PayloadSizeTooLarge` with signature `PayloadSizeTooLarge()` and selector `0x44161266`
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
    #[etherror(name = "PayloadSizeTooLarge", abi = "PayloadSizeTooLarge()")]
    pub struct PayloadSizeTooLarge;
    ///Custom Error type `SliceOverflow` with signature `SliceOverflow()` and selector `0x47aaf07a`
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
    #[etherror(name = "SliceOverflow", abi = "SliceOverflow()")]
    pub struct SliceOverflow;
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
    ///Custom Error type `UnknownPacketType` with signature `UnknownPacketType()` and selector `0xfe3e8327`
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
    #[etherror(name = "UnknownPacketType", abi = "UnknownPacketType()")]
    pub struct UnknownPacketType;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BaseOFTWithFeeErrors {
        AdapterParamsMustBeEmpty(AdapterParamsMustBeEmpty),
        AlreadySet(AlreadySet),
        AmountLessThanMinAmount(AmountLessThanMinAmount),
        AmountSDOverflow(AmountSDOverflow),
        AmountTooSmall(AmountTooSmall),
        CallerMustBeLzApp(CallerMustBeLzApp),
        CallerMustBeOFTCore(CallerMustBeOFTCore),
        DestinationChainNotTrusted(DestinationChainNotTrusted),
        FeeBpTooLarge(FeeBpTooLarge),
        FeeOwnerNotSet(FeeOwnerNotSet),
        GasLimitTooLow(GasLimitTooLow),
        InvalidAdapterParams(InvalidAdapterParams),
        InvalidEndpointCaller(InvalidEndpointCaller),
        InvalidMinGas(InvalidMinGas),
        InvalidPayload(InvalidPayload),
        InvalidSourceSendingContract(InvalidSourceSendingContract),
        MinGasLimitNotSet(MinGasLimitNotSet),
        NoAdmin(NoAdmin),
        NoStoredMessage(NoStoredMessage),
        NoTrustedPath(NoTrustedPath),
        OutOfBounds(OutOfBounds),
        PayloadSizeTooLarge(PayloadSizeTooLarge),
        SliceOverflow(SliceOverflow),
        Unauthorized(Unauthorized),
        UnknownPacketType(UnknownPacketType),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BaseOFTWithFeeErrors {
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
                = <AdapterParamsMustBeEmpty as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AdapterParamsMustBeEmpty(decoded));
            }
            if let Ok(decoded)
                = <AlreadySet as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AlreadySet(decoded));
            }
            if let Ok(decoded)
                = <AmountLessThanMinAmount as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AmountLessThanMinAmount(decoded));
            }
            if let Ok(decoded)
                = <AmountSDOverflow as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AmountSDOverflow(decoded));
            }
            if let Ok(decoded)
                = <AmountTooSmall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AmountTooSmall(decoded));
            }
            if let Ok(decoded)
                = <CallerMustBeLzApp as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallerMustBeLzApp(decoded));
            }
            if let Ok(decoded)
                = <CallerMustBeOFTCore as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallerMustBeOFTCore(decoded));
            }
            if let Ok(decoded)
                = <DestinationChainNotTrusted as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DestinationChainNotTrusted(decoded));
            }
            if let Ok(decoded)
                = <FeeBpTooLarge as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeBpTooLarge(decoded));
            }
            if let Ok(decoded)
                = <FeeOwnerNotSet as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeOwnerNotSet(decoded));
            }
            if let Ok(decoded)
                = <GasLimitTooLow as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GasLimitTooLow(decoded));
            }
            if let Ok(decoded)
                = <InvalidAdapterParams as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidAdapterParams(decoded));
            }
            if let Ok(decoded)
                = <InvalidEndpointCaller as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidEndpointCaller(decoded));
            }
            if let Ok(decoded)
                = <InvalidMinGas as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidMinGas(decoded));
            }
            if let Ok(decoded)
                = <InvalidPayload as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidPayload(decoded));
            }
            if let Ok(decoded)
                = <InvalidSourceSendingContract as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidSourceSendingContract(decoded));
            }
            if let Ok(decoded)
                = <MinGasLimitNotSet as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MinGasLimitNotSet(decoded));
            }
            if let Ok(decoded)
                = <NoAdmin as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoAdmin(decoded));
            }
            if let Ok(decoded)
                = <NoStoredMessage as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoStoredMessage(decoded));
            }
            if let Ok(decoded)
                = <NoTrustedPath as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoTrustedPath(decoded));
            }
            if let Ok(decoded)
                = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutOfBounds(decoded));
            }
            if let Ok(decoded)
                = <PayloadSizeTooLarge as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PayloadSizeTooLarge(decoded));
            }
            if let Ok(decoded)
                = <SliceOverflow as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SliceOverflow(decoded));
            }
            if let Ok(decoded)
                = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unauthorized(decoded));
            }
            if let Ok(decoded)
                = <UnknownPacketType as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnknownPacketType(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BaseOFTWithFeeErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AdapterParamsMustBeEmpty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AlreadySet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AmountLessThanMinAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AmountSDOverflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AmountTooSmall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerMustBeLzApp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallerMustBeOFTCore(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DestinationChainNotTrusted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeBpTooLarge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeOwnerNotSet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GasLimitTooLow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAdapterParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidEndpointCaller(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMinGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPayload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSourceSendingContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinGasLimitNotSet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NoStoredMessage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoTrustedPath(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PayloadSizeTooLarge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SliceOverflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnknownPacketType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for BaseOFTWithFeeErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AdapterParamsMustBeEmpty as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AlreadySet as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <AmountLessThanMinAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AmountSDOverflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AmountTooSmall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CallerMustBeLzApp as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CallerMustBeOFTCore as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DestinationChainNotTrusted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FeeBpTooLarge as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FeeOwnerNotSet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <GasLimitTooLow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidAdapterParams as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidEndpointCaller as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMinGas as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPayload as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSourceSendingContract as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MinGasLimitNotSet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoAdmin as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NoStoredMessage as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoTrustedPath as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <PayloadSizeTooLarge as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SliceOverflow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <UnknownPacketType as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for BaseOFTWithFeeErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdapterParamsMustBeEmpty(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AlreadySet(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountLessThanMinAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AmountSDOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountTooSmall(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallerMustBeLzApp(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallerMustBeOFTCore(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DestinationChainNotTrusted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeBpTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeOwnerNotSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::GasLimitTooLow(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAdapterParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidEndpointCaller(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMinGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidPayload(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSourceSendingContract(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinGasLimitNotSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoStoredMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoTrustedPath(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayloadSizeTooLarge(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SliceOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnknownPacketType(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for BaseOFTWithFeeErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AdapterParamsMustBeEmpty> for BaseOFTWithFeeErrors {
        fn from(value: AdapterParamsMustBeEmpty) -> Self {
            Self::AdapterParamsMustBeEmpty(value)
        }
    }
    impl ::core::convert::From<AlreadySet> for BaseOFTWithFeeErrors {
        fn from(value: AlreadySet) -> Self {
            Self::AlreadySet(value)
        }
    }
    impl ::core::convert::From<AmountLessThanMinAmount> for BaseOFTWithFeeErrors {
        fn from(value: AmountLessThanMinAmount) -> Self {
            Self::AmountLessThanMinAmount(value)
        }
    }
    impl ::core::convert::From<AmountSDOverflow> for BaseOFTWithFeeErrors {
        fn from(value: AmountSDOverflow) -> Self {
            Self::AmountSDOverflow(value)
        }
    }
    impl ::core::convert::From<AmountTooSmall> for BaseOFTWithFeeErrors {
        fn from(value: AmountTooSmall) -> Self {
            Self::AmountTooSmall(value)
        }
    }
    impl ::core::convert::From<CallerMustBeLzApp> for BaseOFTWithFeeErrors {
        fn from(value: CallerMustBeLzApp) -> Self {
            Self::CallerMustBeLzApp(value)
        }
    }
    impl ::core::convert::From<CallerMustBeOFTCore> for BaseOFTWithFeeErrors {
        fn from(value: CallerMustBeOFTCore) -> Self {
            Self::CallerMustBeOFTCore(value)
        }
    }
    impl ::core::convert::From<DestinationChainNotTrusted> for BaseOFTWithFeeErrors {
        fn from(value: DestinationChainNotTrusted) -> Self {
            Self::DestinationChainNotTrusted(value)
        }
    }
    impl ::core::convert::From<FeeBpTooLarge> for BaseOFTWithFeeErrors {
        fn from(value: FeeBpTooLarge) -> Self {
            Self::FeeBpTooLarge(value)
        }
    }
    impl ::core::convert::From<FeeOwnerNotSet> for BaseOFTWithFeeErrors {
        fn from(value: FeeOwnerNotSet) -> Self {
            Self::FeeOwnerNotSet(value)
        }
    }
    impl ::core::convert::From<GasLimitTooLow> for BaseOFTWithFeeErrors {
        fn from(value: GasLimitTooLow) -> Self {
            Self::GasLimitTooLow(value)
        }
    }
    impl ::core::convert::From<InvalidAdapterParams> for BaseOFTWithFeeErrors {
        fn from(value: InvalidAdapterParams) -> Self {
            Self::InvalidAdapterParams(value)
        }
    }
    impl ::core::convert::From<InvalidEndpointCaller> for BaseOFTWithFeeErrors {
        fn from(value: InvalidEndpointCaller) -> Self {
            Self::InvalidEndpointCaller(value)
        }
    }
    impl ::core::convert::From<InvalidMinGas> for BaseOFTWithFeeErrors {
        fn from(value: InvalidMinGas) -> Self {
            Self::InvalidMinGas(value)
        }
    }
    impl ::core::convert::From<InvalidPayload> for BaseOFTWithFeeErrors {
        fn from(value: InvalidPayload) -> Self {
            Self::InvalidPayload(value)
        }
    }
    impl ::core::convert::From<InvalidSourceSendingContract> for BaseOFTWithFeeErrors {
        fn from(value: InvalidSourceSendingContract) -> Self {
            Self::InvalidSourceSendingContract(value)
        }
    }
    impl ::core::convert::From<MinGasLimitNotSet> for BaseOFTWithFeeErrors {
        fn from(value: MinGasLimitNotSet) -> Self {
            Self::MinGasLimitNotSet(value)
        }
    }
    impl ::core::convert::From<NoAdmin> for BaseOFTWithFeeErrors {
        fn from(value: NoAdmin) -> Self {
            Self::NoAdmin(value)
        }
    }
    impl ::core::convert::From<NoStoredMessage> for BaseOFTWithFeeErrors {
        fn from(value: NoStoredMessage) -> Self {
            Self::NoStoredMessage(value)
        }
    }
    impl ::core::convert::From<NoTrustedPath> for BaseOFTWithFeeErrors {
        fn from(value: NoTrustedPath) -> Self {
            Self::NoTrustedPath(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for BaseOFTWithFeeErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    impl ::core::convert::From<PayloadSizeTooLarge> for BaseOFTWithFeeErrors {
        fn from(value: PayloadSizeTooLarge) -> Self {
            Self::PayloadSizeTooLarge(value)
        }
    }
    impl ::core::convert::From<SliceOverflow> for BaseOFTWithFeeErrors {
        fn from(value: SliceOverflow) -> Self {
            Self::SliceOverflow(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for BaseOFTWithFeeErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
        }
    }
    impl ::core::convert::From<UnknownPacketType> for BaseOFTWithFeeErrors {
        fn from(value: UnknownPacketType) -> Self {
            Self::UnknownPacketType(value)
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
    #[ethevent(
        name = "CallOFTReceivedSuccess",
        abi = "CallOFTReceivedSuccess(uint16,bytes,uint64,bytes32)"
    )]
    pub struct CallOFTReceivedSuccessFilter {
        #[ethevent(indexed)]
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub nonce: u64,
        pub hash: [u8; 32],
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
        name = "MessageFailed",
        abi = "MessageFailed(uint16,bytes,uint64,bytes,bytes)"
    )]
    pub struct MessageFailedFilter {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ::ethers::core::types::Bytes,
        pub reason: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "NonContractAddress", abi = "NonContractAddress(address)")]
    pub struct NonContractAddressFilter {
        pub address: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "ReceiveFromChain",
        abi = "ReceiveFromChain(uint16,address,uint256)"
    )]
    pub struct ReceiveFromChainFilter {
        #[ethevent(indexed)]
        pub src_chain_id: u16,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "RetryMessageSuccess",
        abi = "RetryMessageSuccess(uint16,bytes,uint64,bytes32)"
    )]
    pub struct RetryMessageSuccessFilter {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload_hash: [u8; 32],
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
        name = "SendToChain",
        abi = "SendToChain(uint16,address,bytes32,uint256)"
    )]
    pub struct SendToChainFilter {
        #[ethevent(indexed)]
        pub dst_chain_id: u16,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to_address: [u8; 32],
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
    #[ethevent(name = "SetDefaultFeeBp", abi = "SetDefaultFeeBp(uint16)")]
    pub struct SetDefaultFeeBpFilter {
        pub fee_bp: u16,
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
    #[ethevent(name = "SetFeeBp", abi = "SetFeeBp(uint16,bool,uint16)")]
    pub struct SetFeeBpFilter {
        pub dstchain_id: u16,
        pub enabled: bool,
        pub fee_bp: u16,
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
    #[ethevent(name = "SetFeeOwner", abi = "SetFeeOwner(address)")]
    pub struct SetFeeOwnerFilter {
        pub fee_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "SetMinDstGas", abi = "SetMinDstGas(uint16,uint16,uint256)")]
    pub struct SetMinDstGasFilter {
        pub dst_chain_id: u16,
        pub type_: u16,
        pub min_dst_gas: ::ethers::core::types::U256,
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
    #[ethevent(name = "SetPrecrime", abi = "SetPrecrime(address)")]
    pub struct SetPrecrimeFilter {
        pub precrime: ::ethers::core::types::Address,
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
    #[ethevent(name = "SetTrustedRemote", abi = "SetTrustedRemote(uint16,bytes)")]
    pub struct SetTrustedRemoteFilter {
        pub remote_chain_id: u16,
        pub path: ::ethers::core::types::Bytes,
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
        name = "SetTrustedRemoteAddress",
        abi = "SetTrustedRemoteAddress(uint16,bytes)"
    )]
    pub struct SetTrustedRemoteAddressFilter {
        pub remote_chain_id: u16,
        pub remote_address: ::ethers::core::types::Bytes,
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
        name = "SetUseCustomAdapterParams",
        abi = "SetUseCustomAdapterParams(bool)"
    )]
    pub struct SetUseCustomAdapterParamsFilter {
        pub use_custom_adapter_params: bool,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BaseOFTWithFeeEvents {
        AdminAddedFilter(AdminAddedFilter),
        AdminDeletedFilter(AdminDeletedFilter),
        CallOFTReceivedSuccessFilter(CallOFTReceivedSuccessFilter),
        MessageFailedFilter(MessageFailedFilter),
        NonContractAddressFilter(NonContractAddressFilter),
        OperatorAddedFilter(OperatorAddedFilter),
        OperatorDeletedFilter(OperatorDeletedFilter),
        ReceiveFromChainFilter(ReceiveFromChainFilter),
        RetryMessageSuccessFilter(RetryMessageSuccessFilter),
        SendToChainFilter(SendToChainFilter),
        SetDefaultFeeBpFilter(SetDefaultFeeBpFilter),
        SetFeeBpFilter(SetFeeBpFilter),
        SetFeeOwnerFilter(SetFeeOwnerFilter),
        SetMinDstGasFilter(SetMinDstGasFilter),
        SetPrecrimeFilter(SetPrecrimeFilter),
        SetTrustedRemoteFilter(SetTrustedRemoteFilter),
        SetTrustedRemoteAddressFilter(SetTrustedRemoteAddressFilter),
        SetUseCustomAdapterParamsFilter(SetUseCustomAdapterParamsFilter),
    }
    impl ::ethers::contract::EthLogDecode for BaseOFTWithFeeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminAddedFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::AdminAddedFilter(decoded));
            }
            if let Ok(decoded) = AdminDeletedFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::AdminDeletedFilter(decoded));
            }
            if let Ok(decoded) = CallOFTReceivedSuccessFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::CallOFTReceivedSuccessFilter(decoded));
            }
            if let Ok(decoded) = MessageFailedFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::MessageFailedFilter(decoded));
            }
            if let Ok(decoded) = NonContractAddressFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::NonContractAddressFilter(decoded));
            }
            if let Ok(decoded) = OperatorAddedFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::OperatorAddedFilter(decoded));
            }
            if let Ok(decoded) = OperatorDeletedFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::OperatorDeletedFilter(decoded));
            }
            if let Ok(decoded) = ReceiveFromChainFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::ReceiveFromChainFilter(decoded));
            }
            if let Ok(decoded) = RetryMessageSuccessFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::RetryMessageSuccessFilter(decoded));
            }
            if let Ok(decoded) = SendToChainFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::SendToChainFilter(decoded));
            }
            if let Ok(decoded) = SetDefaultFeeBpFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::SetDefaultFeeBpFilter(decoded));
            }
            if let Ok(decoded) = SetFeeBpFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::SetFeeBpFilter(decoded));
            }
            if let Ok(decoded) = SetFeeOwnerFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::SetFeeOwnerFilter(decoded));
            }
            if let Ok(decoded) = SetMinDstGasFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::SetMinDstGasFilter(decoded));
            }
            if let Ok(decoded) = SetPrecrimeFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::SetPrecrimeFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedRemoteFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::SetTrustedRemoteFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedRemoteAddressFilter::decode_log(log) {
                return Ok(BaseOFTWithFeeEvents::SetTrustedRemoteAddressFilter(decoded));
            }
            if let Ok(decoded) = SetUseCustomAdapterParamsFilter::decode_log(log) {
                return Ok(
                    BaseOFTWithFeeEvents::SetUseCustomAdapterParamsFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for BaseOFTWithFeeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminDeletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallOFTReceivedSuccessFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MessageFailedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NonContractAddressFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorDeletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReceiveFromChainFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RetryMessageSuccessFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendToChainFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDefaultFeeBpFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetFeeBpFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeOwnerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinDstGasFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPrecrimeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTrustedRemoteFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetTrustedRemoteAddressFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetUseCustomAdapterParamsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AdminAddedFilter> for BaseOFTWithFeeEvents {
        fn from(value: AdminAddedFilter) -> Self {
            Self::AdminAddedFilter(value)
        }
    }
    impl ::core::convert::From<AdminDeletedFilter> for BaseOFTWithFeeEvents {
        fn from(value: AdminDeletedFilter) -> Self {
            Self::AdminDeletedFilter(value)
        }
    }
    impl ::core::convert::From<CallOFTReceivedSuccessFilter> for BaseOFTWithFeeEvents {
        fn from(value: CallOFTReceivedSuccessFilter) -> Self {
            Self::CallOFTReceivedSuccessFilter(value)
        }
    }
    impl ::core::convert::From<MessageFailedFilter> for BaseOFTWithFeeEvents {
        fn from(value: MessageFailedFilter) -> Self {
            Self::MessageFailedFilter(value)
        }
    }
    impl ::core::convert::From<NonContractAddressFilter> for BaseOFTWithFeeEvents {
        fn from(value: NonContractAddressFilter) -> Self {
            Self::NonContractAddressFilter(value)
        }
    }
    impl ::core::convert::From<OperatorAddedFilter> for BaseOFTWithFeeEvents {
        fn from(value: OperatorAddedFilter) -> Self {
            Self::OperatorAddedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorDeletedFilter> for BaseOFTWithFeeEvents {
        fn from(value: OperatorDeletedFilter) -> Self {
            Self::OperatorDeletedFilter(value)
        }
    }
    impl ::core::convert::From<ReceiveFromChainFilter> for BaseOFTWithFeeEvents {
        fn from(value: ReceiveFromChainFilter) -> Self {
            Self::ReceiveFromChainFilter(value)
        }
    }
    impl ::core::convert::From<RetryMessageSuccessFilter> for BaseOFTWithFeeEvents {
        fn from(value: RetryMessageSuccessFilter) -> Self {
            Self::RetryMessageSuccessFilter(value)
        }
    }
    impl ::core::convert::From<SendToChainFilter> for BaseOFTWithFeeEvents {
        fn from(value: SendToChainFilter) -> Self {
            Self::SendToChainFilter(value)
        }
    }
    impl ::core::convert::From<SetDefaultFeeBpFilter> for BaseOFTWithFeeEvents {
        fn from(value: SetDefaultFeeBpFilter) -> Self {
            Self::SetDefaultFeeBpFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeBpFilter> for BaseOFTWithFeeEvents {
        fn from(value: SetFeeBpFilter) -> Self {
            Self::SetFeeBpFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeOwnerFilter> for BaseOFTWithFeeEvents {
        fn from(value: SetFeeOwnerFilter) -> Self {
            Self::SetFeeOwnerFilter(value)
        }
    }
    impl ::core::convert::From<SetMinDstGasFilter> for BaseOFTWithFeeEvents {
        fn from(value: SetMinDstGasFilter) -> Self {
            Self::SetMinDstGasFilter(value)
        }
    }
    impl ::core::convert::From<SetPrecrimeFilter> for BaseOFTWithFeeEvents {
        fn from(value: SetPrecrimeFilter) -> Self {
            Self::SetPrecrimeFilter(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteFilter> for BaseOFTWithFeeEvents {
        fn from(value: SetTrustedRemoteFilter) -> Self {
            Self::SetTrustedRemoteFilter(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteAddressFilter> for BaseOFTWithFeeEvents {
        fn from(value: SetTrustedRemoteAddressFilter) -> Self {
            Self::SetTrustedRemoteAddressFilter(value)
        }
    }
    impl ::core::convert::From<SetUseCustomAdapterParamsFilter>
    for BaseOFTWithFeeEvents {
        fn from(value: SetUseCustomAdapterParamsFilter) -> Self {
            Self::SetUseCustomAdapterParamsFilter(value)
        }
    }
    ///Container type for all input parameters for the `BP_DENOMINATOR` function with signature `BP_DENOMINATOR()` and selector `0xabe685cd`
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
    #[ethcall(name = "BP_DENOMINATOR", abi = "BP_DENOMINATOR()")]
    pub struct BpDenominatorCall;
    ///Container type for all input parameters for the `DEFAULT_PAYLOAD_SIZE_LIMIT` function with signature `DEFAULT_PAYLOAD_SIZE_LIMIT()` and selector `0xc4461834`
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
    #[ethcall(name = "DEFAULT_PAYLOAD_SIZE_LIMIT", abi = "DEFAULT_PAYLOAD_SIZE_LIMIT()")]
    pub struct DefaultPayloadSizeLimitCall;
    ///Container type for all input parameters for the `NO_EXTRA_GAS` function with signature `NO_EXTRA_GAS()` and selector `0x44770515`
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
    #[ethcall(name = "NO_EXTRA_GAS", abi = "NO_EXTRA_GAS()")]
    pub struct NoExtraGasCall;
    ///Container type for all input parameters for the `PT_SEND` function with signature `PT_SEND()` and selector `0x4c42899a`
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
    #[ethcall(name = "PT_SEND", abi = "PT_SEND()")]
    pub struct PtSendCall;
    ///Container type for all input parameters for the `PT_SEND_AND_CALL` function with signature `PT_SEND_AND_CALL()` and selector `0xe6a20ae6`
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
    #[ethcall(name = "PT_SEND_AND_CALL", abi = "PT_SEND_AND_CALL()")]
    pub struct PtSendAndCallCall;
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
    ///Container type for all input parameters for the `callOnOFTReceived` function with signature `callOnOFTReceived(uint16,bytes,uint64,bytes32,address,uint256,bytes,uint256)` and selector `0xeaffd49a`
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
        name = "callOnOFTReceived",
        abi = "callOnOFTReceived(uint16,bytes,uint64,bytes32,address,uint256,bytes,uint256)"
    )]
    pub struct CallOnOFTReceivedCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub nonce: u64,
        pub from: [u8; 32],
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub payload: ::ethers::core::types::Bytes,
        pub gas_for_call: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `chainIdToFeeBps` function with signature `chainIdToFeeBps(uint16)` and selector `0xc83330ce`
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
    #[ethcall(name = "chainIdToFeeBps", abi = "chainIdToFeeBps(uint16)")]
    pub struct ChainIdToFeeBpsCall(pub u16);
    ///Container type for all input parameters for the `circulatingSupply` function with signature `circulatingSupply()` and selector `0x9358928b`
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
    #[ethcall(name = "circulatingSupply", abi = "circulatingSupply()")]
    pub struct CirculatingSupplyCall;
    ///Container type for all input parameters for the `creditedPackets` function with signature `creditedPackets(uint16,bytes,uint64)` and selector `0x9bdb9812`
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
    #[ethcall(name = "creditedPackets", abi = "creditedPackets(uint16,bytes,uint64)")]
    pub struct CreditedPacketsCall(pub u16, pub ::ethers::core::types::Bytes, pub u64);
    ///Container type for all input parameters for the `defaultFeeBp` function with signature `defaultFeeBp()` and selector `0xd8882968`
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
    #[ethcall(name = "defaultFeeBp", abi = "defaultFeeBp()")]
    pub struct DefaultFeeBpCall;
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
    ///Container type for all input parameters for the `estimateSendFee` function with signature `estimateSendFee(uint16,bytes32,uint256,bool,bytes)` and selector `0x365260b4`
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
        name = "estimateSendFee",
        abi = "estimateSendFee(uint16,bytes32,uint256,bool,bytes)"
    )]
    pub struct EstimateSendFeeCall {
        pub dst_chain_id: u16,
        pub to_address: [u8; 32],
        pub amount: ::ethers::core::types::U256,
        pub use_zro: bool,
        pub adapter_params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `failedMessages` function with signature `failedMessages(uint16,bytes,uint64)` and selector `0x5b8c41e6`
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
    #[ethcall(name = "failedMessages", abi = "failedMessages(uint16,bytes,uint64)")]
    pub struct FailedMessagesCall(pub u16, pub ::ethers::core::types::Bytes, pub u64);
    ///Container type for all input parameters for the `feeOwner` function with signature `feeOwner()` and selector `0xb9818be1`
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
    #[ethcall(name = "feeOwner", abi = "feeOwner()")]
    pub struct FeeOwnerCall;
    ///Container type for all input parameters for the `forceResumeReceive` function with signature `forceResumeReceive(uint16,bytes)` and selector `0x42d65a8d`
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
    #[ethcall(name = "forceResumeReceive", abi = "forceResumeReceive(uint16,bytes)")]
    pub struct ForceResumeReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `0xf5ecbdbc`
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
    #[ethcall(name = "getConfig", abi = "getConfig(uint16,uint16,address,uint256)")]
    pub struct GetConfigCall {
        pub version: u16,
        pub chain_id: u16,
        pub p2: ::ethers::core::types::Address,
        pub config_type: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTrustedRemoteAddress` function with signature `getTrustedRemoteAddress(uint16)` and selector `0x9f38369a`
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
    #[ethcall(name = "getTrustedRemoteAddress", abi = "getTrustedRemoteAddress(uint16)")]
    pub struct GetTrustedRemoteAddressCall {
        pub remote_chain_id: u16,
    }
    ///Container type for all input parameters for the `isTrustedRemote` function with signature `isTrustedRemote(uint16,bytes)` and selector `0x3d8b38f6`
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
    #[ethcall(name = "isTrustedRemote", abi = "isTrustedRemote(uint16,bytes)")]
    pub struct IsTrustedRemoteCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `lzEndpoint` function with signature `lzEndpoint()` and selector `0xb353aaa7`
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
    #[ethcall(name = "lzEndpoint", abi = "lzEndpoint()")]
    pub struct LzEndpointCall;
    ///Container type for all input parameters for the `lzReceive` function with signature `lzReceive(uint16,bytes,uint64,bytes)` and selector `0x001d3567`
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
    #[ethcall(name = "lzReceive", abi = "lzReceive(uint16,bytes,uint64,bytes)")]
    pub struct LzReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `minDstGasLookup` function with signature `minDstGasLookup(uint16,uint16)` and selector `0x8cfd8f5c`
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
    #[ethcall(name = "minDstGasLookup", abi = "minDstGasLookup(uint16,uint16)")]
    pub struct MinDstGasLookupCall(pub u16, pub u16);
    ///Container type for all input parameters for the `nonblockingLzReceive` function with signature `nonblockingLzReceive(uint16,bytes,uint64,bytes)` and selector `0x66ad5c8a`
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
        name = "nonblockingLzReceive",
        abi = "nonblockingLzReceive(uint16,bytes,uint64,bytes)"
    )]
    pub struct NonblockingLzReceiveCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ::ethers::core::types::Bytes,
    }
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
    ///Container type for all input parameters for the `payloadSizeLimitLookup` function with signature `payloadSizeLimitLookup(uint16)` and selector `0x3f1f4fa4`
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
    #[ethcall(name = "payloadSizeLimitLookup", abi = "payloadSizeLimitLookup(uint16)")]
    pub struct PayloadSizeLimitLookupCall(pub u16);
    ///Container type for all input parameters for the `precrime` function with signature `precrime()` and selector `0x950c8a74`
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
    #[ethcall(name = "precrime", abi = "precrime()")]
    pub struct PrecrimeCall;
    ///Container type for all input parameters for the `quoteOFTFee` function with signature `quoteOFTFee(uint16,uint256)` and selector `0xecd8f212`
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
    #[ethcall(name = "quoteOFTFee", abi = "quoteOFTFee(uint16,uint256)")]
    pub struct QuoteOFTFeeCall {
        pub dst_chain_id: u16,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `retryMessage` function with signature `retryMessage(uint16,bytes,uint64,bytes)` and selector `0xd1deba1f`
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
    #[ethcall(name = "retryMessage", abi = "retryMessage(uint16,bytes,uint64,bytes)")]
    pub struct RetryMessageCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub nonce: u64,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `sendFrom` function with signature `sendFrom(address,uint16,bytes32,uint256,uint256,(address,address,bytes))` and selector `0x2cdf0b95`
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
        name = "sendFrom",
        abi = "sendFrom(address,uint16,bytes32,uint256,uint256,(address,address,bytes))"
    )]
    pub struct SendFromCall {
        pub from: ::ethers::core::types::Address,
        pub dst_chain_id: u16,
        pub to_address: [u8; 32],
        pub amount: ::ethers::core::types::U256,
        pub min_amount: ::ethers::core::types::U256,
        pub call_params: LzCallParams,
    }
    ///Container type for all input parameters for the `setConfig` function with signature `setConfig(uint16,uint16,uint256,bytes)` and selector `0xcbed8b9c`
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
    #[ethcall(name = "setConfig", abi = "setConfig(uint16,uint16,uint256,bytes)")]
    pub struct SetConfigCall {
        pub version: u16,
        pub chain_id: u16,
        pub config_type: ::ethers::core::types::U256,
        pub config: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setDefaultFeeBp` function with signature `setDefaultFeeBp(uint16)` and selector `0x5a359dc5`
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
    #[ethcall(name = "setDefaultFeeBp", abi = "setDefaultFeeBp(uint16)")]
    pub struct SetDefaultFeeBpCall {
        pub fee_bp: u16,
    }
    ///Container type for all input parameters for the `setFeeBp` function with signature `setFeeBp(uint16,bool,uint16)` and selector `0x79c0ad4b`
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
    #[ethcall(name = "setFeeBp", abi = "setFeeBp(uint16,bool,uint16)")]
    pub struct SetFeeBpCall {
        pub dst_chain_id: u16,
        pub enabled: bool,
        pub fee_bp: u16,
    }
    ///Container type for all input parameters for the `setFeeOwner` function with signature `setFeeOwner(address)` and selector `0x4b104eff`
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
    #[ethcall(name = "setFeeOwner", abi = "setFeeOwner(address)")]
    pub struct SetFeeOwnerCall {
        pub fee_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setMinDstGas` function with signature `setMinDstGas(uint16,uint16,uint256)` and selector `0xdf2a5b3b`
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
    #[ethcall(name = "setMinDstGas", abi = "setMinDstGas(uint16,uint16,uint256)")]
    pub struct SetMinDstGasCall {
        pub dst_chain_id: u16,
        pub packet_type: u16,
        pub min_gas: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPayloadSizeLimit` function with signature `setPayloadSizeLimit(uint16,uint256)` and selector `0x0df37483`
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
    #[ethcall(name = "setPayloadSizeLimit", abi = "setPayloadSizeLimit(uint16,uint256)")]
    pub struct SetPayloadSizeLimitCall {
        pub dst_chain_id: u16,
        pub size: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPrecrime` function with signature `setPrecrime(address)` and selector `0xbaf3292d`
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
    #[ethcall(name = "setPrecrime", abi = "setPrecrime(address)")]
    pub struct SetPrecrimeCall {
        pub precrime: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setReceiveVersion` function with signature `setReceiveVersion(uint16)` and selector `0x10ddb137`
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
    #[ethcall(name = "setReceiveVersion", abi = "setReceiveVersion(uint16)")]
    pub struct SetReceiveVersionCall {
        pub version: u16,
    }
    ///Container type for all input parameters for the `setSendVersion` function with signature `setSendVersion(uint16)` and selector `0x07e0db17`
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
    #[ethcall(name = "setSendVersion", abi = "setSendVersion(uint16)")]
    pub struct SetSendVersionCall {
        pub version: u16,
    }
    ///Container type for all input parameters for the `setTrustedRemote` function with signature `setTrustedRemote(uint16,bytes)` and selector `0xeb8d72b7`
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
    #[ethcall(name = "setTrustedRemote", abi = "setTrustedRemote(uint16,bytes)")]
    pub struct SetTrustedRemoteCall {
        pub remote_chain_id: u16,
        pub path: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setTrustedRemoteAddress` function with signature `setTrustedRemoteAddress(uint16,bytes)` and selector `0xa6c3d165`
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
        name = "setTrustedRemoteAddress",
        abi = "setTrustedRemoteAddress(uint16,bytes)"
    )]
    pub struct SetTrustedRemoteAddressCall {
        pub remote_chain_id: u16,
        pub remote_address: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setUseCustomAdapterParams` function with signature `setUseCustomAdapterParams(bool)` and selector `0xeab45d9c`
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
        name = "setUseCustomAdapterParams",
        abi = "setUseCustomAdapterParams(bool)"
    )]
    pub struct SetUseCustomAdapterParamsCall {
        pub use_custom_adapter_params: bool,
    }
    ///Container type for all input parameters for the `sharedDecimals` function with signature `sharedDecimals()` and selector `0x857749b0`
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
    #[ethcall(name = "sharedDecimals", abi = "sharedDecimals()")]
    pub struct SharedDecimalsCall;
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `token` function with signature `token()` and selector `0xfc0c546a`
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
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    ///Container type for all input parameters for the `trustedRemoteLookup` function with signature `trustedRemoteLookup(uint16)` and selector `0x7533d788`
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
    #[ethcall(name = "trustedRemoteLookup", abi = "trustedRemoteLookup(uint16)")]
    pub struct TrustedRemoteLookupCall(pub u16);
    ///Container type for all input parameters for the `useCustomAdapterParams` function with signature `useCustomAdapterParams()` and selector `0xed629c5c`
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
    #[ethcall(name = "useCustomAdapterParams", abi = "useCustomAdapterParams()")]
    pub struct UseCustomAdapterParamsCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BaseOFTWithFeeCalls {
        BpDenominator(BpDenominatorCall),
        DefaultPayloadSizeLimit(DefaultPayloadSizeLimitCall),
        NoExtraGas(NoExtraGasCall),
        PtSend(PtSendCall),
        PtSendAndCall(PtSendAndCallCall),
        AddAdmin(AddAdminCall),
        AddOperator(AddOperatorCall),
        Admins(AdminsCall),
        CallOnOFTReceived(CallOnOFTReceivedCall),
        ChainIdToFeeBps(ChainIdToFeeBpsCall),
        CirculatingSupply(CirculatingSupplyCall),
        CreditedPackets(CreditedPacketsCall),
        DefaultFeeBp(DefaultFeeBpCall),
        DeleteAdmin(DeleteAdminCall),
        DeleteOperator(DeleteOperatorCall),
        EstimateSendFee(EstimateSendFeeCall),
        FailedMessages(FailedMessagesCall),
        FeeOwner(FeeOwnerCall),
        ForceResumeReceive(ForceResumeReceiveCall),
        GetConfig(GetConfigCall),
        GetTrustedRemoteAddress(GetTrustedRemoteAddressCall),
        IsTrustedRemote(IsTrustedRemoteCall),
        LzEndpoint(LzEndpointCall),
        LzReceive(LzReceiveCall),
        MinDstGasLookup(MinDstGasLookupCall),
        NonblockingLzReceive(NonblockingLzReceiveCall),
        Operators(OperatorsCall),
        PayloadSizeLimitLookup(PayloadSizeLimitLookupCall),
        Precrime(PrecrimeCall),
        QuoteOFTFee(QuoteOFTFeeCall),
        RetryMessage(RetryMessageCall),
        SendFrom(SendFromCall),
        SetConfig(SetConfigCall),
        SetDefaultFeeBp(SetDefaultFeeBpCall),
        SetFeeBp(SetFeeBpCall),
        SetFeeOwner(SetFeeOwnerCall),
        SetMinDstGas(SetMinDstGasCall),
        SetPayloadSizeLimit(SetPayloadSizeLimitCall),
        SetPrecrime(SetPrecrimeCall),
        SetReceiveVersion(SetReceiveVersionCall),
        SetSendVersion(SetSendVersionCall),
        SetTrustedRemote(SetTrustedRemoteCall),
        SetTrustedRemoteAddress(SetTrustedRemoteAddressCall),
        SetUseCustomAdapterParams(SetUseCustomAdapterParamsCall),
        SharedDecimals(SharedDecimalsCall),
        SupportsInterface(SupportsInterfaceCall),
        Token(TokenCall),
        TrustedRemoteLookup(TrustedRemoteLookupCall),
        UseCustomAdapterParams(UseCustomAdapterParamsCall),
    }
    impl ::ethers::core::abi::AbiDecode for BaseOFTWithFeeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BpDenominatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BpDenominator(decoded));
            }
            if let Ok(decoded)
                = <DefaultPayloadSizeLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultPayloadSizeLimit(decoded));
            }
            if let Ok(decoded)
                = <NoExtraGasCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoExtraGas(decoded));
            }
            if let Ok(decoded)
                = <PtSendCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PtSend(decoded));
            }
            if let Ok(decoded)
                = <PtSendAndCallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PtSendAndCall(decoded));
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
                = <CallOnOFTReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CallOnOFTReceived(decoded));
            }
            if let Ok(decoded)
                = <ChainIdToFeeBpsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChainIdToFeeBps(decoded));
            }
            if let Ok(decoded)
                = <CirculatingSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CirculatingSupply(decoded));
            }
            if let Ok(decoded)
                = <CreditedPacketsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreditedPackets(decoded));
            }
            if let Ok(decoded)
                = <DefaultFeeBpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultFeeBp(decoded));
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
                = <EstimateSendFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EstimateSendFee(decoded));
            }
            if let Ok(decoded)
                = <FailedMessagesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedMessages(decoded));
            }
            if let Ok(decoded)
                = <FeeOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeOwner(decoded));
            }
            if let Ok(decoded)
                = <ForceResumeReceiveCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ForceResumeReceive(decoded));
            }
            if let Ok(decoded)
                = <GetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetConfig(decoded));
            }
            if let Ok(decoded)
                = <GetTrustedRemoteAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetTrustedRemoteAddress(decoded));
            }
            if let Ok(decoded)
                = <IsTrustedRemoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsTrustedRemote(decoded));
            }
            if let Ok(decoded)
                = <LzEndpointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LzEndpoint(decoded));
            }
            if let Ok(decoded)
                = <LzReceiveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LzReceive(decoded));
            }
            if let Ok(decoded)
                = <MinDstGasLookupCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MinDstGasLookup(decoded));
            }
            if let Ok(decoded)
                = <NonblockingLzReceiveCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NonblockingLzReceive(decoded));
            }
            if let Ok(decoded)
                = <OperatorsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Operators(decoded));
            }
            if let Ok(decoded)
                = <PayloadSizeLimitLookupCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PayloadSizeLimitLookup(decoded));
            }
            if let Ok(decoded)
                = <PrecrimeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Precrime(decoded));
            }
            if let Ok(decoded)
                = <QuoteOFTFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QuoteOFTFee(decoded));
            }
            if let Ok(decoded)
                = <RetryMessageCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RetryMessage(decoded));
            }
            if let Ok(decoded)
                = <SendFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendFrom(decoded));
            }
            if let Ok(decoded)
                = <SetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetConfig(decoded));
            }
            if let Ok(decoded)
                = <SetDefaultFeeBpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDefaultFeeBp(decoded));
            }
            if let Ok(decoded)
                = <SetFeeBpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFeeBp(decoded));
            }
            if let Ok(decoded)
                = <SetFeeOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFeeOwner(decoded));
            }
            if let Ok(decoded)
                = <SetMinDstGasCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMinDstGas(decoded));
            }
            if let Ok(decoded)
                = <SetPayloadSizeLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetPayloadSizeLimit(decoded));
            }
            if let Ok(decoded)
                = <SetPrecrimeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPrecrime(decoded));
            }
            if let Ok(decoded)
                = <SetReceiveVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReceiveVersion(decoded));
            }
            if let Ok(decoded)
                = <SetSendVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSendVersion(decoded));
            }
            if let Ok(decoded)
                = <SetTrustedRemoteCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetTrustedRemote(decoded));
            }
            if let Ok(decoded)
                = <SetTrustedRemoteAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetTrustedRemoteAddress(decoded));
            }
            if let Ok(decoded)
                = <SetUseCustomAdapterParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetUseCustomAdapterParams(decoded));
            }
            if let Ok(decoded)
                = <SharedDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SharedDecimals(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token(decoded));
            }
            if let Ok(decoded)
                = <TrustedRemoteLookupCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TrustedRemoteLookup(decoded));
            }
            if let Ok(decoded)
                = <UseCustomAdapterParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UseCustomAdapterParams(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BaseOFTWithFeeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BpDenominator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultPayloadSizeLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoExtraGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PtSend(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PtSendAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admins(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CallOnOFTReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChainIdToFeeBps(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CirculatingSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreditedPackets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultFeeBp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeleteAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeleteOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EstimateSendFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedMessages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ForceResumeReceive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTrustedRemoteAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsTrustedRemote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LzEndpoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LzReceive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinDstGasLookup(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NonblockingLzReceive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Operators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PayloadSizeLimitLookup(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Precrime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteOFTFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RetryMessage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDefaultFeeBp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeeBp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeeOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMinDstGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPayloadSizeLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPrecrime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetReceiveVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSendVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTrustedRemote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTrustedRemoteAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUseCustomAdapterParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SharedDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Token(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TrustedRemoteLookup(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UseCustomAdapterParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BaseOFTWithFeeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BpDenominator(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultPayloadSizeLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoExtraGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::PtSend(element) => ::core::fmt::Display::fmt(element, f),
                Self::PtSendAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admins(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallOnOFTReceived(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainIdToFeeBps(element) => ::core::fmt::Display::fmt(element, f),
                Self::CirculatingSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreditedPackets(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultFeeBp(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::EstimateSendFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedMessages(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForceResumeReceive(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTrustedRemoteAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsTrustedRemote(element) => ::core::fmt::Display::fmt(element, f),
                Self::LzEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::LzReceive(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinDstGasLookup(element) => ::core::fmt::Display::fmt(element, f),
                Self::NonblockingLzReceive(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Operators(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayloadSizeLimitLookup(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Precrime(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteOFTFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::RetryMessage(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDefaultFeeBp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeBp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinDstGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPayloadSizeLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPrecrime(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReceiveVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSendVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTrustedRemote(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTrustedRemoteAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetUseCustomAdapterParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SharedDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrustedRemoteLookup(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UseCustomAdapterParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BpDenominatorCall> for BaseOFTWithFeeCalls {
        fn from(value: BpDenominatorCall) -> Self {
            Self::BpDenominator(value)
        }
    }
    impl ::core::convert::From<DefaultPayloadSizeLimitCall> for BaseOFTWithFeeCalls {
        fn from(value: DefaultPayloadSizeLimitCall) -> Self {
            Self::DefaultPayloadSizeLimit(value)
        }
    }
    impl ::core::convert::From<NoExtraGasCall> for BaseOFTWithFeeCalls {
        fn from(value: NoExtraGasCall) -> Self {
            Self::NoExtraGas(value)
        }
    }
    impl ::core::convert::From<PtSendCall> for BaseOFTWithFeeCalls {
        fn from(value: PtSendCall) -> Self {
            Self::PtSend(value)
        }
    }
    impl ::core::convert::From<PtSendAndCallCall> for BaseOFTWithFeeCalls {
        fn from(value: PtSendAndCallCall) -> Self {
            Self::PtSendAndCall(value)
        }
    }
    impl ::core::convert::From<AddAdminCall> for BaseOFTWithFeeCalls {
        fn from(value: AddAdminCall) -> Self {
            Self::AddAdmin(value)
        }
    }
    impl ::core::convert::From<AddOperatorCall> for BaseOFTWithFeeCalls {
        fn from(value: AddOperatorCall) -> Self {
            Self::AddOperator(value)
        }
    }
    impl ::core::convert::From<AdminsCall> for BaseOFTWithFeeCalls {
        fn from(value: AdminsCall) -> Self {
            Self::Admins(value)
        }
    }
    impl ::core::convert::From<CallOnOFTReceivedCall> for BaseOFTWithFeeCalls {
        fn from(value: CallOnOFTReceivedCall) -> Self {
            Self::CallOnOFTReceived(value)
        }
    }
    impl ::core::convert::From<ChainIdToFeeBpsCall> for BaseOFTWithFeeCalls {
        fn from(value: ChainIdToFeeBpsCall) -> Self {
            Self::ChainIdToFeeBps(value)
        }
    }
    impl ::core::convert::From<CirculatingSupplyCall> for BaseOFTWithFeeCalls {
        fn from(value: CirculatingSupplyCall) -> Self {
            Self::CirculatingSupply(value)
        }
    }
    impl ::core::convert::From<CreditedPacketsCall> for BaseOFTWithFeeCalls {
        fn from(value: CreditedPacketsCall) -> Self {
            Self::CreditedPackets(value)
        }
    }
    impl ::core::convert::From<DefaultFeeBpCall> for BaseOFTWithFeeCalls {
        fn from(value: DefaultFeeBpCall) -> Self {
            Self::DefaultFeeBp(value)
        }
    }
    impl ::core::convert::From<DeleteAdminCall> for BaseOFTWithFeeCalls {
        fn from(value: DeleteAdminCall) -> Self {
            Self::DeleteAdmin(value)
        }
    }
    impl ::core::convert::From<DeleteOperatorCall> for BaseOFTWithFeeCalls {
        fn from(value: DeleteOperatorCall) -> Self {
            Self::DeleteOperator(value)
        }
    }
    impl ::core::convert::From<EstimateSendFeeCall> for BaseOFTWithFeeCalls {
        fn from(value: EstimateSendFeeCall) -> Self {
            Self::EstimateSendFee(value)
        }
    }
    impl ::core::convert::From<FailedMessagesCall> for BaseOFTWithFeeCalls {
        fn from(value: FailedMessagesCall) -> Self {
            Self::FailedMessages(value)
        }
    }
    impl ::core::convert::From<FeeOwnerCall> for BaseOFTWithFeeCalls {
        fn from(value: FeeOwnerCall) -> Self {
            Self::FeeOwner(value)
        }
    }
    impl ::core::convert::From<ForceResumeReceiveCall> for BaseOFTWithFeeCalls {
        fn from(value: ForceResumeReceiveCall) -> Self {
            Self::ForceResumeReceive(value)
        }
    }
    impl ::core::convert::From<GetConfigCall> for BaseOFTWithFeeCalls {
        fn from(value: GetConfigCall) -> Self {
            Self::GetConfig(value)
        }
    }
    impl ::core::convert::From<GetTrustedRemoteAddressCall> for BaseOFTWithFeeCalls {
        fn from(value: GetTrustedRemoteAddressCall) -> Self {
            Self::GetTrustedRemoteAddress(value)
        }
    }
    impl ::core::convert::From<IsTrustedRemoteCall> for BaseOFTWithFeeCalls {
        fn from(value: IsTrustedRemoteCall) -> Self {
            Self::IsTrustedRemote(value)
        }
    }
    impl ::core::convert::From<LzEndpointCall> for BaseOFTWithFeeCalls {
        fn from(value: LzEndpointCall) -> Self {
            Self::LzEndpoint(value)
        }
    }
    impl ::core::convert::From<LzReceiveCall> for BaseOFTWithFeeCalls {
        fn from(value: LzReceiveCall) -> Self {
            Self::LzReceive(value)
        }
    }
    impl ::core::convert::From<MinDstGasLookupCall> for BaseOFTWithFeeCalls {
        fn from(value: MinDstGasLookupCall) -> Self {
            Self::MinDstGasLookup(value)
        }
    }
    impl ::core::convert::From<NonblockingLzReceiveCall> for BaseOFTWithFeeCalls {
        fn from(value: NonblockingLzReceiveCall) -> Self {
            Self::NonblockingLzReceive(value)
        }
    }
    impl ::core::convert::From<OperatorsCall> for BaseOFTWithFeeCalls {
        fn from(value: OperatorsCall) -> Self {
            Self::Operators(value)
        }
    }
    impl ::core::convert::From<PayloadSizeLimitLookupCall> for BaseOFTWithFeeCalls {
        fn from(value: PayloadSizeLimitLookupCall) -> Self {
            Self::PayloadSizeLimitLookup(value)
        }
    }
    impl ::core::convert::From<PrecrimeCall> for BaseOFTWithFeeCalls {
        fn from(value: PrecrimeCall) -> Self {
            Self::Precrime(value)
        }
    }
    impl ::core::convert::From<QuoteOFTFeeCall> for BaseOFTWithFeeCalls {
        fn from(value: QuoteOFTFeeCall) -> Self {
            Self::QuoteOFTFee(value)
        }
    }
    impl ::core::convert::From<RetryMessageCall> for BaseOFTWithFeeCalls {
        fn from(value: RetryMessageCall) -> Self {
            Self::RetryMessage(value)
        }
    }
    impl ::core::convert::From<SendFromCall> for BaseOFTWithFeeCalls {
        fn from(value: SendFromCall) -> Self {
            Self::SendFrom(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for BaseOFTWithFeeCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetDefaultFeeBpCall> for BaseOFTWithFeeCalls {
        fn from(value: SetDefaultFeeBpCall) -> Self {
            Self::SetDefaultFeeBp(value)
        }
    }
    impl ::core::convert::From<SetFeeBpCall> for BaseOFTWithFeeCalls {
        fn from(value: SetFeeBpCall) -> Self {
            Self::SetFeeBp(value)
        }
    }
    impl ::core::convert::From<SetFeeOwnerCall> for BaseOFTWithFeeCalls {
        fn from(value: SetFeeOwnerCall) -> Self {
            Self::SetFeeOwner(value)
        }
    }
    impl ::core::convert::From<SetMinDstGasCall> for BaseOFTWithFeeCalls {
        fn from(value: SetMinDstGasCall) -> Self {
            Self::SetMinDstGas(value)
        }
    }
    impl ::core::convert::From<SetPayloadSizeLimitCall> for BaseOFTWithFeeCalls {
        fn from(value: SetPayloadSizeLimitCall) -> Self {
            Self::SetPayloadSizeLimit(value)
        }
    }
    impl ::core::convert::From<SetPrecrimeCall> for BaseOFTWithFeeCalls {
        fn from(value: SetPrecrimeCall) -> Self {
            Self::SetPrecrime(value)
        }
    }
    impl ::core::convert::From<SetReceiveVersionCall> for BaseOFTWithFeeCalls {
        fn from(value: SetReceiveVersionCall) -> Self {
            Self::SetReceiveVersion(value)
        }
    }
    impl ::core::convert::From<SetSendVersionCall> for BaseOFTWithFeeCalls {
        fn from(value: SetSendVersionCall) -> Self {
            Self::SetSendVersion(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteCall> for BaseOFTWithFeeCalls {
        fn from(value: SetTrustedRemoteCall) -> Self {
            Self::SetTrustedRemote(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteAddressCall> for BaseOFTWithFeeCalls {
        fn from(value: SetTrustedRemoteAddressCall) -> Self {
            Self::SetTrustedRemoteAddress(value)
        }
    }
    impl ::core::convert::From<SetUseCustomAdapterParamsCall> for BaseOFTWithFeeCalls {
        fn from(value: SetUseCustomAdapterParamsCall) -> Self {
            Self::SetUseCustomAdapterParams(value)
        }
    }
    impl ::core::convert::From<SharedDecimalsCall> for BaseOFTWithFeeCalls {
        fn from(value: SharedDecimalsCall) -> Self {
            Self::SharedDecimals(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for BaseOFTWithFeeCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TokenCall> for BaseOFTWithFeeCalls {
        fn from(value: TokenCall) -> Self {
            Self::Token(value)
        }
    }
    impl ::core::convert::From<TrustedRemoteLookupCall> for BaseOFTWithFeeCalls {
        fn from(value: TrustedRemoteLookupCall) -> Self {
            Self::TrustedRemoteLookup(value)
        }
    }
    impl ::core::convert::From<UseCustomAdapterParamsCall> for BaseOFTWithFeeCalls {
        fn from(value: UseCustomAdapterParamsCall) -> Self {
            Self::UseCustomAdapterParams(value)
        }
    }
    ///Container type for all return fields from the `BP_DENOMINATOR` function with signature `BP_DENOMINATOR()` and selector `0xabe685cd`
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
    pub struct BpDenominatorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `DEFAULT_PAYLOAD_SIZE_LIMIT` function with signature `DEFAULT_PAYLOAD_SIZE_LIMIT()` and selector `0xc4461834`
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
    pub struct DefaultPayloadSizeLimitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `NO_EXTRA_GAS` function with signature `NO_EXTRA_GAS()` and selector `0x44770515`
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
    pub struct NoExtraGasReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PT_SEND` function with signature `PT_SEND()` and selector `0x4c42899a`
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
    pub struct PtSendReturn(pub u8);
    ///Container type for all return fields from the `PT_SEND_AND_CALL` function with signature `PT_SEND_AND_CALL()` and selector `0xe6a20ae6`
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
    pub struct PtSendAndCallReturn(pub u8);
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
    ///Container type for all return fields from the `chainIdToFeeBps` function with signature `chainIdToFeeBps(uint16)` and selector `0xc83330ce`
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
    pub struct ChainIdToFeeBpsReturn {
        pub fee_bp: u16,
        pub enabled: bool,
    }
    ///Container type for all return fields from the `circulatingSupply` function with signature `circulatingSupply()` and selector `0x9358928b`
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
    pub struct CirculatingSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `creditedPackets` function with signature `creditedPackets(uint16,bytes,uint64)` and selector `0x9bdb9812`
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
    pub struct CreditedPacketsReturn(pub bool);
    ///Container type for all return fields from the `defaultFeeBp` function with signature `defaultFeeBp()` and selector `0xd8882968`
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
    pub struct DefaultFeeBpReturn(pub u16);
    ///Container type for all return fields from the `estimateSendFee` function with signature `estimateSendFee(uint16,bytes32,uint256,bool,bytes)` and selector `0x365260b4`
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
    pub struct EstimateSendFeeReturn {
        pub native_fee: ::ethers::core::types::U256,
        pub zro_fee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `failedMessages` function with signature `failedMessages(uint16,bytes,uint64)` and selector `0x5b8c41e6`
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
    pub struct FailedMessagesReturn(pub [u8; 32]);
    ///Container type for all return fields from the `feeOwner` function with signature `feeOwner()` and selector `0xb9818be1`
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
    pub struct FeeOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getConfig` function with signature `getConfig(uint16,uint16,address,uint256)` and selector `0xf5ecbdbc`
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
    pub struct GetConfigReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getTrustedRemoteAddress` function with signature `getTrustedRemoteAddress(uint16)` and selector `0x9f38369a`
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
    pub struct GetTrustedRemoteAddressReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `isTrustedRemote` function with signature `isTrustedRemote(uint16,bytes)` and selector `0x3d8b38f6`
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
    pub struct IsTrustedRemoteReturn(pub bool);
    ///Container type for all return fields from the `lzEndpoint` function with signature `lzEndpoint()` and selector `0xb353aaa7`
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
    pub struct LzEndpointReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `minDstGasLookup` function with signature `minDstGasLookup(uint16,uint16)` and selector `0x8cfd8f5c`
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
    pub struct MinDstGasLookupReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `payloadSizeLimitLookup` function with signature `payloadSizeLimitLookup(uint16)` and selector `0x3f1f4fa4`
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
    pub struct PayloadSizeLimitLookupReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `precrime` function with signature `precrime()` and selector `0x950c8a74`
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
    pub struct PrecrimeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `quoteOFTFee` function with signature `quoteOFTFee(uint16,uint256)` and selector `0xecd8f212`
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
    pub struct QuoteOFTFeeReturn {
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `sharedDecimals` function with signature `sharedDecimals()` and selector `0x857749b0`
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
    pub struct SharedDecimalsReturn(pub u8);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `token` function with signature `token()` and selector `0xfc0c546a`
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
    pub struct TokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `trustedRemoteLookup` function with signature `trustedRemoteLookup(uint16)` and selector `0x7533d788`
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
    pub struct TrustedRemoteLookupReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `useCustomAdapterParams` function with signature `useCustomAdapterParams()` and selector `0xed629c5c`
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
    pub struct UseCustomAdapterParamsReturn(pub bool);
}
