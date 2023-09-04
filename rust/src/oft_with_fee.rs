pub use oft_with_fee::*;
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
pub mod oft_with_fee {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_symbol"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("decimals"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint8"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_sharedDecimals"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint8"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("authority"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_lzEndpoint"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
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
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
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
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                    ::std::borrow::ToOwned::to_owned("InsufficientAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientAllowance",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientBalance",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("SharedDecimalsTooLarge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SharedDecimalsTooLarge",
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OFTWITHFEE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01@`@\x90\x80\x82R4b\0\x05\xB2Wb\0=S\x808\x03\x80\x91b\0\0#\x82\x85b\0\x05\xB7V[\x839\x81\x01\x91`\xC0\x82\x84\x03\x12b\0\x05\xB2W\x81Q`\x01`\x01`@\x1B\x03\x90\x81\x81\x11b\0\x05\xB2W\x84b\0\0T\x91\x85\x01b\0\x05\xF1V[` \x94\x85\x85\x01Q\x90\x83\x82\x11b\0\x05\xB2Wb\0\0q\x91\x86\x01b\0\x05\xF1V[\x94b\0\0\x7F\x84\x86\x01b\0\x06hV[\x95b\0\0\x8E``\x87\x01b\0\x06hV[\x92b\0\0\xAB`\xA0b\0\0\xA3`\x80\x8A\x01b\0\x06wV[\x98\x01b\0\x06wV[\x90`\x01\x80`\xA0\x1B\x03\x92`\0\x98\x84\x81\x16\x8AR`\x02\x86R\x88\x8A \x94`\x01\x94`\xFF\x19\x96\x86\x88\x82T\x16\x17\x90U\x8BT\x87`\xFF\x88\x81\x84\x16\x01\x16\x91\x16\x17\x8CU\x85\x88R\x8A\x8C \x86\x88\x82T\x16\x17\x90U\x16`\x80R\x86`\xA0R`\x0BT\x90b\x01\0\0`\x01`\xB0\x1B\x03\x90`\x10\x1B\x16\x90b\x01\0\0`\x01`\xB0\x1B\x03\x19\x16\x17`\x0BU\x81Q\x91\x87\x83\x11b\0\x05\x9EW`\x0C\x92\x80b\0\x018\x85Tb\0\x06\x8CV[\x92`\x1F\x93\x84\x81\x11b\0\x05HW[P\x88\x90\x8D\x85\x84\x11`\x01\x14b\0\x04\xE4W\x92b\0\x04\xD8W[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x85\x1B\x17\x83U[\x81Q\x91\x88\x83\x11b\0\x04\xC4W\x90\x82\x91b\0\x01\x8A`\rTb\0\x06\x8CV[\x82\x81\x11b\0\x04nW[P\x87\x91\x83\x11`\x01\x14b\0\x04\x08W\x8B\x92b\0\x03\xFCW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x83\x1B\x17`\rU[\x88`\xC0RF`\xE0R\x86Q\x80\x92\x89\x90\x83T\x93b\0\x01\xD9\x85b\0\x06\x8CV[\x94\x85\x85R\x88\x80\x86\x01\x98\x84\x83\x16\x92\x83`\0\x14b\0\x03\xDCWPPP`\x01\x14b\0\x03\x9CW[PPb\0\x02\x0B\x92P\x03\x82b\0\x05\xB7V[Q\x90 \x90\x84Q\x90\x81\x01\x91\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R\x85\x82\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81R`\xC0\x81\x01\x93\x81\x85\x10\x90\x85\x11\x17b\0\x03\x88W`\xFF\x91\x82\x91\x85\x87RQ\x90 \x96a\x01\0\x97\x88R\x16\x91\x16\x91\x81\x83\x11b\0\x03yWP\x03`\xFF\x81\x11b\0\x03eW`\xFF\x16\x91`M\x83\x11b\0\x03QWPa\x01 \x91`\n\n\x82RQ\x90a6\x89\x92\x83b\0\x06\xCA\x849`\x80Q\x83\x81\x81a\x04\xC7\x01R\x81\x81a\x08<\x01R\x81\x81a\t\xDA\x01R\x81\x81a\x0FN\x01R\x81\x81a\x11\x07\x01R\x81\x81a\x1E=\x01R\x81\x81a A\x01R\x81\x81a(\xFF\x01Ra2(\x01R`\xA0Q\x83a\x18\xBB\x01R`\xC0Q\x83a\x0E)\x01R`\xE0Q\x83a*P\x01RQ\x82a*w\x01RQ\x81\x81\x81a\x14\x96\x01R\x81\x81a!\xF7\x01R\x81\x81a2\xC2\x01Ra3\x17\x01R\xF3[cNH{q`\xE0\x1B\x81R`\x11`\x04R`$\x90\xFD[cNH{q`\xE0\x1B\x83R`\x11`\x04R`$\x83\xFD[cG\x12r\r`\xE0\x1B\x81R`\x04\x90\xFD[cNH{q`\xE0\x1B\x86R`A`\x04R`$\x86\xFD[\x87\x92P\x8BR\x81\x8B \x90\x8B\x91[\x85\x83\x10b\0\x03\xC3WPPb\0\x02\x0B\x93P\x82\x01\x018\x80b\0\x01\xFBV[\x80T\x83\x88\x01\x85\x01R\x86\x94P\x88\x93\x90\x92\x01\x91\x81\x01b\0\x03\xA8V[\x92P\x92P\x93Pb\0\x02\x0B\x95\x92P\x16\x86R\x15\x15`\x05\x1B\x82\x01\x018\x80b\0\x01\xFBV[\x01Q\x90P8\x80b\0\x01\xA8V[`\r\x8CR\x87\x8C \x86\x94P\x91\x90`\x1F\x19\x84\x16\x8D[\x8A\x82\x82\x10b\0\x04WWPP\x84\x11b\0\x04=W[PPP\x81\x1B\x01`\rUb\0\x01\xBDV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80b\0\x04.V[\x83\x85\x01Q\x86U\x89\x97\x90\x95\x01\x94\x93\x84\x01\x93\x01b\0\x04\x1BV[\x90\x91\x92P`\r\x8CR\x87\x8C \x83\x80\x86\x01`\x05\x1C\x82\x01\x92\x8A\x87\x10b\0\x04\xBAW[\x91\x86\x95\x89\x92\x95\x94\x93\x01`\x05\x1C\x01\x91[\x82\x81\x10b\0\x04\xABWPPb\0\x01\x93V[\x8E\x81U\x86\x95P\x88\x91\x01b\0\x04\x9BV[\x92P\x81\x92b\0\x04\x8CV[cNH{q`\xE0\x1B\x8BR`A`\x04R`$\x8B\xFD[\x01Q\x90P8\x80b\0\x01[V[\x91\x90\x88\x94P`\x1F\x19\x84\x16\x88\x84R\x8B\x84 \x93[\x8C\x82\x82\x10b\0\x051WPP\x84\x11b\0\x05\x17W[PPP\x81\x1B\x01\x83Ub\0\x01oV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80b\0\x05\tV[\x83\x85\x01Q\x86U\x8B\x97\x90\x95\x01\x94\x93\x84\x01\x93\x01b\0\x04\xF6V[\x90\x91P\x85\x8DR\x88\x8D \x84\x80\x85\x01`\x05\x1C\x82\x01\x92\x8B\x86\x10b\0\x05\x94W[\x85\x94\x93\x91\x01`\x05\x1C\x90\x91\x01\x90\x88\x90\x8F[\x83\x82\x10b\0\x05\x85WPPPb\0\x01EV[\x81U\x85\x94P\x89\x91\x01\x8Fb\0\x05tV[\x92P\x81\x92b\0\x05dV[cNH{q`\xE0\x1B\x8AR`A`\x04R`$\x8A\xFD[`\0\x80\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17b\0\x05\xDBW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x91\x90\x80`\x1F\x84\x01\x12\x15b\0\x05\xB2W\x82Q\x90`\x01`\x01`@\x1B\x03\x82\x11b\0\x05\xDBW`@Q\x91` \x91b\0\x06-`\x1F\x83\x01`\x1F\x19\x16\x84\x01\x85b\0\x05\xB7V[\x81\x84R\x82\x82\x87\x01\x01\x11b\0\x05\xB2W`\0[\x81\x81\x10b\0\x06TWP\x82`\0\x93\x94\x95P\x01\x01R\x90V[\x85\x81\x01\x83\x01Q\x84\x82\x01\x84\x01R\x82\x01b\0\x06>V[Q\x90`\xFF\x82\x16\x82\x03b\0\x05\xB2WV[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03b\0\x05\xB2WV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15b\0\x06\xBEW[` \x83\x10\x14b\0\x06\xA8WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91b\0\x06\x9CV\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80b\x1D5g\x14a\x03\xD7W\x80c\x01\xFF\xC9\xA7\x14a\x03\xD2W\x80c\x06\xFD\xDE\x03\x14a\x03\xCDW\x80c\x07\xE0\xDB\x17\x14a\x03\xC8W\x80c\t^\xA7\xB3\x14a\x03\xC3W\x80c\r\xF3t\x83\x14a\x03\xBEW\x80c\x10\xDD\xB17\x14a\x03\xB9W\x80c\x13\xE7\xC9\xD8\x14a\x03\xB4W\x80c\x18\x16\r\xDD\x14a\x03AW\x80c#\xB8r\xDD\x14a\x03\xAFW\x80c'\xE1\xF7\xDF\x14a\x03\xAAW\x80c,\xDF\x0B\x95\x14a\x03\xA5W\x80c1<\xE5g\x14a\x03\xA0W\x80c6D\xE5\x15\x14a\x03\x9BW\x80c6R`\xB4\x14a\x03\x96W\x80c=\x8B8\xF6\x14a\x03\x91W\x80c?\x1FO\xA4\x14a\x03\x8CW\x80cB\x9Bb\xE5\x14a\x03\x87W\x80cB\xD6Z\x8D\x14a\x03\x82W\x80cDw\x05\x15\x14a\x03xW\x80cK\x10N\xFF\x14a\x03}W\x80cLB\x89\x9A\x14a\x03xW\x80cZ5\x9D\xC5\x14a\x03sW\x80c[\x8CA\xE6\x14a\x03nW\x80cf\xAD\\\x8A\x14a\x03iW\x80cpH\x02u\x14a\x03dW\x80cp\xA0\x821\x14a\x03_W\x80cu3\xD7\x88\x14a\x03ZW\x80cy\xC0\xADK\x14a\x03UW\x80c~\xCE\xBE\0\x14a\x03PW\x80c\x85wI\xB0\x14a\x03KW\x80c\x8C\xFD\x8F\\\x14a\x03FW\x80c\x93X\x92\x8B\x14a\x03AW\x80c\x95\x0C\x8At\x14a\x03<W\x80c\x95\xD8\x9BA\x14a\x037W\x80c\x98p\xD7\xFE\x14a\x032W\x80c\x9B\xDB\x98\x12\x14a\x03-W\x80c\x9F86\x9A\x14a\x03(W\x80c\xA6\xC3\xD1e\x14a\x03#W\x80c\xA9\x05\x9C\xBB\x14a\x03\x1EW\x80c\xAB\xE6\x85\xCD\x14a\x03\x05W\x80c\xB3S\xAA\xA7\x14a\x03\x19W\x80c\xB4\t\x92\xA1\x14a\x03\x14W\x80c\xB9\x81\x8B\xE1\x14a\x03\x0FW\x80c\xBA\xF3)-\x14a\x03\nW\x80c\xC4F\x184\x14a\x03\x05W\x80c\xC830\xCE\x14a\x03\0W\x80c\xCB\xED\x8B\x9C\x14a\x02\xFBW\x80c\xD1\xDE\xBA\x1F\x14a\x02\xF6W\x80c\xD5\x05\xAC\xCF\x14a\x02\xF1W\x80c\xD8\x88)h\x14a\x02\xECW\x80c\xDDb\xED>\x14a\x02\xE7W\x80c\xDF*[;\x14a\x02\xE2W\x80c\xE6\xA2\n\xE6\x14a\x02\xDDW\x80c\xEA\xB4]\x9C\x14a\x02\xD8W\x80c\xEA\xFF\xD4\x9A\x14a\x02\xD3W\x80c\xEB\x8Dr\xB7\x14a\x02\xCEW\x80c\xEC\xD8\xF2\x12\x14a\x02\xC9W\x80c\xEDb\x9C\\\x14a\x02\xC4W\x80c\xF5\xEC\xBD\xBC\x14a\x02\xBFWc\xFC\x0CTj\x14a\x02\xBAW`\0\x80\xFD[a)iV[a(\x9DV[a(zV[a(SV[a&\xFBV[a&uV[a&\x04V[a%\xE8V[a%\x1FV[a$\xC3V[a$\xA1V[a\"\xA7V[a \xD5V[a\x1F\xD8V[a\x1F\x95V[a\x1E\0V[a\x1F\x19V[a\x1E\xEFV[a\x1EaV[a\x1E\x1DV[a\x1DoV[a\x1B\xE7V[a\x1A\xD2V[a\x1A\x87V[a\x19\xF7V[a\x19PV[a\x19)V[a\ntV[a\x18\xDFV[a\x18\xA1V[a\x18cV[a\x17GV[a\x16\xF4V[a\x16\x03V[a\x157V[a\x13\xE5V[a\x13}V[a\x12EV[a\x11gV[a\x11\x83V[a\x10\xD3V[a\x10\x90V[a\x10[V[a\x0F\xFFV[a\x0E\x8EV[a\x0EMV[a\x0E\x0FV[a\r\x10V[a\x0C\0V[a\n\x92V[a\n1V[a\t\x9FV[a\tXV[a\x08\xCAV[a\x08\x01V[a\x07\x1CV[a\x05\x9CV[a\x04\xAAV[`\x045\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xEDWV[`\0\x80\xFD[`$5\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xEDWV[`D5\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xEDWV[\x91\x81`\x1F\x84\x01\x12\x15a\x03\xEDW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\xEDW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03\xEDWV[\x90`\x80`\x03\x19\x83\x01\x12a\x03\xEDW`\x045a\xFF\xFF\x81\x16\x81\x03a\x03\xEDW\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90`$5\x82\x81\x11a\x03\xEDW\x81a\x04\x7F\x91`\x04\x01a\x04\x14V[\x93\x90\x93\x92`D5\x81\x81\x16\x81\x03a\x03\xEDW\x92`d5\x91\x82\x11a\x03\xEDWa\x04\xA6\x91`\x04\x01a\x04\x14V[\x90\x91V[4a\x03\xEDWa\x04\xB86a\x04BV[\x91\x92\x94\x93\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x05\x8AWa\x05\ra\x05\x08\x86a\xFF\xFF\x16`\0R`\x03` R`@`\0 \x90V[a\x16\xD7V[\x80Q\x90\x81\x88\x14\x91\x82\x15\x92a\x05\x81W[P\x81\x15a\x05]W[Pa\x05KWa\x05;a\x05C\x92a\x05I\x976\x91a\x12\xEAV[\x926\x91a\x12\xEAV[\x92a,\xC0V[\0[`@Qc\x195\xE2\x81`\xE1\x1B\x81R`\x04\x90\xFD[\x90Pa\x05j6\x88\x85a\x12\xEAV[` \x81Q\x91\x01 \x90` \x81Q\x91\x01 \x14\x158a\x05$V[\x15\x91P8a\x05\x1CV[`@Qc\r\x1A\xD4\xCD`\xE0\x1B\x81R`\x04\x90\xFD[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045c\xFF\xFF\xFF\xFF`\xE0\x1B\x81\x16\x80\x91\x03a\x03\xEDW` \x90c,\xDF\x0B\x95`\xE0\x1B\x81\x14\x90\x81\x15a\x05\xE1W[P`@Q\x90\x15\x15\x81R\xF3[c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x90P8a\x05\xD6V[`\0\x91\x03\x12a\x03\xEDWV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x06-W[` \x83\x10\x14a\x06\x17WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x06\x0CV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06aW`@RV[a\x067V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x06aW`@RV[`\xC0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x06aW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x06aW`@RV[`\0[\x83\x81\x10a\x06\xD3WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xC3V[\x90` \x91a\x06\xFC\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x06\xC0V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x07\x19\x92\x81\x81R\x01\x90a\x06\xE3V[\x90V[4a\x03\xEDW`\0\x80`\x03\x196\x01\x12a\x07\xFEW`@Q\x90\x80`\x0CTa\x07?\x81a\x05\xFDV[\x80\x85R\x91`\x01\x91\x80\x83\x16\x90\x81\x15a\x07\xD4WP`\x01\x14a\x07yW[a\x07u\x85a\x07i\x81\x87\x03\x82a\x06\x9EV[`@Q\x91\x82\x91\x82a\x07\x08V[\x03\x90\xF3[\x92P`\x0C\x83R\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7[\x82\x84\x10a\x07\xBCWPPP\x81\x01` \x01a\x07i\x82a\x07ua\x07YV[\x80T` \x85\x87\x01\x81\x01\x91\x90\x91R\x90\x93\x01\x92\x81\x01a\x07\xA1V[\x86\x95Pa\x07u\x96\x93P` \x92Pa\x07i\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x93a\x07YV[\x80\xFD[4a\x03\xEDW`\0` 6`\x03\x19\x01\x12a\x07\xFEWa\x08\x1Ca\x03\xDCV[3\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8W\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x82;\x15a\x08\xA4W`$a\xFF\xFF\x91\x83`@Q\x95\x86\x94\x85\x93c\x07\xE0\xDB\x17`\xE0\x1B\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a\x08\x9FWa\x08\x93WP\x80\xF3[a\x08\x9C\x90a\x06MV[\x80\xF3[a)\xF3V[P\x80\xFD[`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\xEDWV[4a\x03\xEDW`@6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x08\xE7\x81a\x08\xB9V[`\x01`\x01`\xA0\x1B\x03`$5\x913`\0R`\x10` R\x82a\t\x1E\x82`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[U`@Q\x92\x83R\x16\x90\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` 3\x92\xA3` `@Q`\x01\x81R\xF3[4a\x03\xEDW`@6`\x03\x19\x01\x12a\x03\xEDWa\tqa\x03\xDCV[`\0\x903\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8Wa\xFF\xFF\x16\x81R`\x05` R`$5`@\x82 U\x80\xF3[4a\x03\xEDW`\0` 6`\x03\x19\x01\x12a\x07\xFEWa\t\xBAa\x03\xDCV[3\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8W\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x82;\x15a\x08\xA4W`$a\xFF\xFF\x91\x83`@Q\x95\x86\x94\x85\x93c\x10\xDD\xB17`\xE0\x1B\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a\x08\x9FWa\x08\x93WP\x80\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x01`\x01`\xA0\x1B\x03`\x045a\nV\x81a\x08\xB9V[\x16`\0R`\x01` R` `\xFF`@`\0 T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `\x0ET`@Q\x90\x81R\xF3[4a\x03\xEDW``6`\x03\x19\x01\x12a\x03\xEDW`\x045a\n\xAF\x81a\x08\xB9V[`$5a\n\xBB\x81a\x08\xB9V[`D5\x91`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x92\x83`\0R`\x10` Ra\n\xF4`@`\0 3`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T`\x01\x81\x01a\x0B\x8AW[Pa\x0B=\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0F` R`@`\0 \x90V[a\x0BH\x86\x82Ta)\x9AV[\x90Ua\x0Bg\x81`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0F` R`@`\0 \x90V[\x80T\x86\x01\x90U`@Q\x94\x85R\x16\x92\x80` \x81\x01[\x03\x90\xA3`@Q`\x01\x81R` \x90\xF3[\x85\x81\x03\x90\x81\x11a\x0B\xFBW\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x0B=\x91a\x0B\xF33a\x0B\xDB\x84`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x10` R`@`\0 \x90V[\x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[U\x93Pa\n\xFEV[a)\x84V[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x0C\x1D\x81a\x08\xB9V[`\0\x903\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8W`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x83R`\x02` Ra\x0C^a\x0CZ`@\x85 `\xFF\x90T\x16\x90V[\x15\x90V[a\x0C\xFEWa\x0C\x88a\x0Cxa\x0Cs\x85T`\xFF\x16\x90V[a6#V[`\xFF\x16`\xFF\x19`\0T\x16\x17`\0UV[`\xFFa\x0C\x95\x84T`\xFF\x16\x90V[\x16\x15a\x0C\xECWa\x0C\xBBa\x0C\xC5\x91`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x80T`\xFF\x19\x16\x90UV[\x7F\x98\x9D\xDF\xCE\x05}\xAD!\x9E\n\xE1oi\x1B\x12\x1B\xB0\xE3H\xF0\xD8\xAE\n\xD4\0\xB4\xD5\xAC\x8Dal\x8B\x82\x80\xA2\x80\xF3[`@Qc\x1F\x8C\x1D\xBD`\xE1\x1B\x81R`\x04\x90\xFD[`@Qc\xA7A\xA0E`\xE0\x1B\x81R`\x04\x90\xFD[`\x03\x19`\xC06\x82\x01\x12a\x03\xEDW`\x045a\r)\x81a\x08\xB9V[a\r1a\x03\xF2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92`\xA45\x90`d5\x85\x83\x11a\x03\xEDW``\x836\x03\x92\x83\x01\x12a\x03\xEDWa\r`\x81\x85a.\xB9V[\x80\x82\x03\x91\x82\x11a\x0B\xFBW\x80a\r\xEFW[P\x82`\x04\x015\x91a\r\x80\x83a\x08\xB9V[`$\x84\x015\x93a\r\x8F\x85a\x08\xB9V[`D\x81\x015\x91`\"\x19\x01\x82\x12\x15a\x03\xEDW\x01`\x04\x81\x015\x96\x87\x11a\x03\xEDW`$\x01\x94\x866\x03\x86\x13a\x03\xEDWa\r\xCCa\r\xD6\x96`\x845\x986\x91a\x12\xEAV[\x94`D5\x91a0+V[\x10a\r\xDDW\0[`@Qc@\x84GY`\xE0\x1B\x81R`\x04\x90\xFD[a\x0E\x08\x90`\x01`\x01`\xA0\x1B\x03`\x0BT`\x10\x1C\x16\x87a4kV[P8a\rpV[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` a\x0Eha*KV[`@Q\x90\x81R\xF3[`d5\x90\x81\x15\x15\x82\x03a\x03\xEDWV[`$5\x90\x81\x15\x15\x82\x03a\x03\xEDWV[4a\x03\xEDW`\x03\x19`\xA06\x82\x01\x12a\x03\xEDWa\x0E\xA8a\x03\xDCV[a\x0E\xB0a\x0EpV[`\x845\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\xEDWa\x0FBa\xFF\xFF\x92a\x0E\xE4a\x0E\xDD`@\x966\x90`\x04\x01a\x04\x14V[6\x91a\x12\xEAV[a\x0E\xFAa\x0E\xF2`D5a2\xC0V[`$5a3LV[\x96a\x0F,\x87Q\x98\x89\x97\x88\x97c\x04\n{\xB1`\xE4\x1B\x89R\x16`\x04\x88\x01R0`$\x88\x01R`\xA0`D\x88\x01R`\xA4\x87\x01\x90a\x06\xE3V[\x92\x15\x15`d\x86\x01R\x84\x83\x03\x01`\x84\x85\x01Ra\x06\xE3V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x08\x9FW`\0\x90\x81\x92a\x0F\x93W[P`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xF3[\x90Pa\x0F\xB6\x91P`@=\x81\x11a\x0F\xBDW[a\x0F\xAE\x81\x83a\x06\x9EV[\x81\x01\x90a0\x15V[\x908a\x0F\x81V[P=a\x0F\xA4V[\x90`@`\x03\x19\x83\x01\x12a\x03\xEDW`\x045a\xFF\xFF\x81\x16\x81\x03a\x03\xEDW\x91`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03\xEDWa\x04\xA6\x91`\x04\x01a\x04\x14V[4a\x03\xEDW` a\xFF\xFFa\x10La\x10\x156a\x0F\xC4V[\x93\x90\x91\x16`\0R`\x03\x84Ra\x107a\x10>`@`\0 `@Q\x92\x83\x80\x92a\x16AV[\x03\x82a\x06\x9EV[\x84\x81Q\x91\x01 \x926\x91a\x12\xEAV[\x82\x81Q\x91\x01 \x14`@Q\x90\x81R\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDWa\xFF\xFFa\x10wa\x03\xDCV[\x16`\0R`\x05` R` `@`\0 T`@Q\x90\x81R\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x01`\x01`\xA0\x1B\x03`\x045a\x10\xB5\x81a\x08\xB9V[\x16`\0R`\x02` R` `\xFF`@`\0 T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xEDWa\x10\xE16a\x0F\xC4V[\x91\x90`\0\x923\x84R`\x02` R`\xFF`@\x85 T\x16\x15a\x08\xA8W\x83\x91`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x81;\x15a\x11cW\x83a\x11Q\x95`@Q\x96\x87\x95\x86\x94\x85\x93cB\xD6Z\x8D`\xE0\x1B\x85R`\x04\x85\x01a,QV[\x03\x92Z\xF1\x80\x15a\x08\x9FWa\x08\x93WP\x80\xF3[\x83\x80\xFD[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Q`\0\x81R\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x11\xA0\x81a\x08\xB9V[3`\0R`\x02` R`\xFF`@`\0 T\x16\x15a\x08\xA8W`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a\x123W\x7F\x04y\x12c\x1A\xFAVN\xEB\xD3\xDB.\xFE\x19\x1A\r\xECb\xDA\x1F\xED\xE6\xBB\xBC\x1F\xFC\x89\xD8xE\xB1\xB5\x91` \x91u\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0`\x0BT\x91`\x10\x1B\x16\x90u\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x19\x16\x17`\x0BU`@Q\x90\x81R\xA1\0[`@Qc\xA6\xAF\xC5=`\xE0\x1B\x81R`\x04\x90\xFD[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDWa\x12^a\x03\xDCV[3`\0R`\x02` R`\xFF`@`\0 T\x16\x15a\x08\xA8Wa\xFF\xFF\x16a'\x10\x81\x11a\x12\xBCW` \x81\x7F\xD2`0\xEFJ\x8C\"^\xE1+dn\xB4Fj\xCBA\xFB\x96\xB6\xCDF`\xB2-\x0B\xA0\x12N{\xDCt\x92a\xFF\xFF\x19`\x0BT\x16\x17`\x0BU`@Q\x90\x81R\xA1\0[`@Qc\x0F\xC0\x0F\x19`\xE1\x1B\x81R`\x04\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06aW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x12\xF6\x82a\x12\xCEV[\x91a\x13\x04`@Q\x93\x84a\x06\x9EV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03\xEDW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[```\x03\x19\x82\x01\x12a\x03\xEDW`\x045a\xFF\xFF\x81\x16\x81\x03a\x03\xEDW\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x03\xEDW\x80`#\x83\x01\x12\x15a\x03\xEDW\x81`$a\x13m\x93`\x04\x015\x91\x01a\x12\xEAV[\x91`D5\x90\x81\x16\x81\x03a\x03\xEDW\x90V[4a\x03\xEDW` a\x13\xDCa\xFF\xFFa\x13\xBA\x83a\x13\x976a\x13!V[\x94\x90\x91\x16`\0R`\x07\x82R`@`\0 \x82`@Q\x94\x83\x86\x80\x95Q\x93\x84\x92\x01a\x06\xC0V[\x82\x01\x90\x81R\x03\x01\x90 \x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R` R`@`\0 \x90V[T`@Q\x90\x81R\xF3[4a\x03\xEDWa\x13\xF36a\x04BV[\x91P\x9103\x03a\x15&Wa\x14\x14\x93a\x14\x0C\x916\x91a\x12\xEAV[P6\x91a\x12\xEAV[`\xFFa\x14\x1F\x82a63V[\x16a\x15\x14W`\xFFa\x14/\x82a63V[\x16\x15\x80\x15\x90a\x15\x08W[a\x14\xF6W`!\x81Q\x10a\x14\xE4Wa\x14W`-\x82\x01Q``\x1C\x91a6CV[\x81\x15a\x14\xDAW[`\x01`\x01`\xA0\x1B\x03a\x14\xBC\x7F\xBFU\x1E\xC98Y\xB1p\xF9\xB2\x14\x1B\xD9)\x8B\xF3\xF6C\"\xC6\xF7\xBE\xB2T:\x0C\xB6i\x83A\x18\xBF\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x16a.\x90V[\x92a\x14\xC7\x84\x82a4\x10V[`@Q\x93\x84R\x16\x92a\xFF\xFF\x16\x91` \x90\xA3\0[a\xDE\xAD\x91Pa\x14^V[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc|iS\xF9`\xE0\x1B\x81R`\x04\x90\xFD[P`)\x81Q\x14\x15a\x149V[`@Qc\xFE>\x83'`\xE0\x1B\x81R`\x04\x90\xFD[`@Qb\xE4\xF8\x15`\xE5\x1B\x81R`\x04\x90\xFD[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x15T\x81a\x08\xB9V[`\0\x903\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8W`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x83R`\x02` R`\xFF`@\x84 T\x16a\x0C\xFEW`\xFF\x83T\x16`\xFF\x81\x14a\x0B\xFBWa\x15\xDC\x91a\x15\xB6`\x01a\x15\xCF\x93\x01`\xFF\x16`\xFF\x19`\0T\x16\x17`\0UV[`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[\x7FD\xD6\xD2Yc\xF0\x97\xAD\x14\xF2\x9F\x06\x85J\x01\xF5ud\x8A\x1E\xF8/0\xE5b\xCC\xD3\x88\x97\x17\xE39\x82\x80\xA2\x80\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x01`\x01`\xA0\x1B\x03`\x045a\x16(\x81a\x08\xB9V[\x16`\0R`\x0F` R` `@`\0 T`@Q\x90\x81R\xF3[\x90`\0\x92\x91\x80T\x91a\x16R\x83a\x05\xFDV[\x91\x82\x82R`\x01\x93\x84\x81\x16\x90\x81`\0\x14a\x16\xB4WP`\x01\x14a\x16tW[PPPPV[\x90\x91\x93\x94P`\0R` \x92\x83`\0 \x92\x84`\0\x94[\x83\x86\x10a\x16\xA0WPPPP\x01\x01\x908\x80\x80\x80a\x16nV[\x80T\x85\x87\x01\x83\x01R\x94\x01\x93\x85\x90\x82\x01a\x16\x89V[\x92\x94PPP` \x93\x94P`\xFF\x19\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x908\x80\x80\x80a\x16nV[\x90a\x16\xF2a\x16\xEB\x92`@Q\x93\x84\x80\x92a\x16AV[\x03\x83a\x06\x9EV[V[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDWa\xFF\xFFa\x17\x10a\x03\xDCV[\x16`\0R`\x03` Ra\x07ua\x107a\x173`@`\0 `@Q\x92\x83\x80\x92a\x16AV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\xE3V[4a\x03\xEDW``6`\x03\x19\x01\x12a\x03\xEDWa\x17`a\x03\xDCV[a\x17ha\x0E\x7FV[\x90a\x17qa\x04\x03V[\x91`\0\x913\x83R`\x02` R`@\x93`\xFF\x85\x85 T\x16\x15a\x18SWa\xFF\xFF\x80\x82\x16\x90a'\x10\x82\x11a\x18BW\x95a\x18<\x92\x91\x7F\xDD\x9C\x96\x85\xAF>m\xCBV\xD8\xF4\xB8\x8D%\x95\xD4\xAD\xD6\x83z\x15\x004\xE7x\x1CF\xB6\xDC\xF8\xAA\xAB\x96\x97\x82Q\x91a\x17\xD1\x83a\x06fV[\x82Ra\x18\x02` \x83\x01\x91\x88\x15\x15\x83R\x80\x88\x16\x8BR`\n` R\x84\x8B \x93Q\x16\x83\x90a\xFF\xFF\x16a\xFF\xFF\x19\x82T\x16\x17\x90UV[Q\x81Tb\xFF\0\0\x19\x16\x90\x15\x15`\x10\x1Bb\xFF\0\0\x16\x17\x90UQa\xFF\xFF\x93\x84\x16\x81R\x93\x15\x15` \x85\x01R\x91\x90\x91\x16`@\x83\x01R\x81\x90``\x82\x01\x90V[\x03\x90\xA1\x80\xF3[\x86Qc\x0F\xC0\x0F\x19`\xE1\x1B\x81R`\x04\x90\xFD[\x84Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x90\xFD[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x01`\x01`\xA0\x1B\x03`\x045a\x18\x88\x81a\x08\xB9V[\x16`\0R`\x11` R` `@`\0 T`@Q\x90\x81R\xF3[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xEDW`@6`\x03\x19\x01\x12a\x03\xEDW` a\x13\xDCa\x18\xFDa\x03\xDCV[a\xFF\xFFa\x19\x08a\x03\xF2V[\x91\x16`\0R`\x04\x83R`@`\0 \x90a\xFF\xFF\x16`\0R` R`@`\0 \x90V[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `\x01`\x01`\xA0\x1B\x03`\x06T\x16`@Q\x90\x81R\xF3[4a\x03\xEDW`\0\x80`\x03\x196\x01\x12a\x07\xFEW`@Q\x90\x80`\rTa\x19s\x81a\x05\xFDV[\x80\x85R\x91`\x01\x91\x80\x83\x16\x90\x81\x15a\x07\xD4WP`\x01\x14a\x19\x9CWa\x07u\x85a\x07i\x81\x87\x03\x82a\x06\x9EV[\x92P`\r\x83R\x7F\xD7\xB6\x99\x01\x05q\x91\x01\xDA\xBE\xB7qD\xF2\xA38\\\x803\xAC\xD3\xAF\x97\xE9B:i^\x81\xAD\x1E\xB5[\x82\x84\x10a\x19\xDFWPPP\x81\x01` \x01a\x07i\x82a\x07ua\x07YV[\x80T` \x85\x87\x01\x81\x01\x91\x90\x91R\x90\x93\x01\x92\x81\x01a\x19\xC4V[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x1A\x14\x81a\x08\xB9V[`\0\x903\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8W`\x01`\x01`\xA0\x1B\x03\x16\x80\x82R`\x01` R`\xFF`@\x83 T\x16a\x0C\xFEW\x80\x82R`\x01` R`@\x82 `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\xACo\xA8X\xE95\nF\xCE\xC1e9\x92n\x0F\xDE%\xB7b\x9F\x84\xB5\xA7+\xFF\xAA\xE4\xDF\x88\x8A\xE8m\x82\x80\xA2\x80\xF3[4a\x03\xEDW` `\xFFa\x1A\xC6a\xFF\xFFa\x13\xBA\x84a\x1A\xA36a\x13!V[\x94\x90\x91\x16`\0R`\t\x82R`@`\0 \x82`@Q\x94\x83\x86\x80\x95Q\x93\x84\x92\x01a\x06\xC0V[T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xEDW` \x80`\x03\x196\x01\x12a\x03\xEDWa\x1B\x0F\x90a\xFF\xFFa\x1A\xF3a\x03\xDCV[\x16`\0R`\x03\x81R`@a\x1B\x16\x81`\0 \x82Q\x94\x85\x80\x92a\x16AV[\x03\x84a\x06\x9EV[\x82Q\x15a\x1B\xD7W\x82Q`\x13\x19\x93\x84\x82\x01\x90\x82\x82\x11a\x0B\xFBW\x81a\x1B8\x81a2\xB2V[\x10a\x1B\xC6W\x81\x81Q\x10a\x1B\xB5W\x81a\x1BhWPPPa\x07u\x92P\x80Q\x91`\0\x83R\x82\x01\x81R[Q\x91\x82\x91\x82a\x07\x08V[\x83\x95\x94\x95Q\x94`\x1F\x83\x16\x80\x15`\x05\x1B\x91\x82\x82\x89\x01\x01\x95\x86\x01\x01\x92\x01\x01\x90[\x80\x84\x10a\x1B\xA4WPP\x83R`\x1F\x01`\x1F\x19\x16\x81Ra\x07u\x92Pa\x1B^V[\x81Q\x84R\x92\x86\x01\x92\x90\x86\x01\x90a\x1B\x86V[\x83Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[\x83Qc#\xD5x=`\xE1\x1B\x81R`\x04\x90\xFD[Qc\x05(La`\xE3\x1B\x81R`\x04\x90\xFD[4a\x03\xEDWa\x1B\xF56a\x0F\xC4V[\x91\x90`\0\x913\x83R` `\x02\x81R`\xFF`@\x85 T\x16\x15a\x08\xA8W`@Q\x85\x84\x83\x83\x017a\x1C8`4\x82\x88\x81\x010``\x1B\x86\x82\x01R\x03`\x14\x81\x01\x84R\x01\x82a\x06\x9EV[a\xFF\xFF\x83\x16\x85R`\x03\x82R`@\x85 \x91\x81Q\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x06aWa\x1Cn\x83a\x1Ch\x86Ta\x05\xFDV[\x86a,lV[\x81`\x1F\x84\x11`\x01\x14a\x1C\xD9WP\x91\x80a\x18<\x94\x92\x88\x99\x94\x7F\x8C\x04\0\xCF\xE2\xD1\x19\x9B\x1Ar\\x\x96\x0B\xCC*4M\x86\x9B\x80Y\r\x0F+\xD0\x05\xDB\x15\xA5r\xCE\x99\x92a\x1C\xCEW[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[`@Q\x93\x84\x93\x84a,QV[\x01Q\x90P8\x80a\x1C\xADV[\x91\x90`\x1F\x19\x84\x16a\x1C\xEF\x86`\0R` `\0 \x90V[\x93\x89\x90[\x82\x82\x10a\x1DWWPP\x92`\x01\x92\x85\x92\x7F\x8C\x04\0\xCF\xE2\xD1\x19\x9B\x1Ar\\x\x96\x0B\xCC*4M\x86\x9B\x80Y\r\x0F+\xD0\x05\xDB\x15\xA5r\xCE\x9A\x9B\x96a\x18<\x98\x96\x10a\x1D>W[PPP\x81\x1B\x01\x90Ua\x1C\xC2V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\x1D1V[\x80`\x01\x86\x97\x82\x94\x97\x87\x01Q\x81U\x01\x96\x01\x94\x01\x90a\x1C\xF3V[4a\x03\xEDW`@6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x1D\x8C\x81a\x08\xB9V[`$5\x903`\0R`\x0F` R`@`\0 \x90\x81T\x83\x81\x03\x90\x81\x11a\x0B\xFBW`\x01`\x01`\xA0\x1B\x03\x92U\x16\x90\x81`\0R`\x0F` R`@`\0 \x81\x81T\x01\x90U\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`@Q\x80a\x0B{3\x94\x82\x91\x90` \x83\x01\x92RV[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Qa'\x10\x81R\xF3[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x1E~\x81a\x08\xB9V[`\0\x903\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8W`\x01`\x01`\xA0\x1B\x03\x16\x80\x82R`\x01` R`\xFF`@\x83 T\x16\x15a\x0C\xFEW\x80\x82R`\x01` R`@\x82 \x80T`\xFF\x19\x16\x90U\x7Fi\xDF,^\xC2\xEAM\x1F\xBE\x1EP5$\xF5\x93\xB3V\x16,\xA7\x10g\x12c\x82\x7F.\x19\x92\xB9Z\xE1\x82\x80\xA2\x80\xF3[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `\x01`\x01`\xA0\x1B\x03`\x0BT`\x10\x1C\x16`@Q\x90\x81R\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x1F6\x81a\x08\xB9V[3`\0R`\x02` R`\xFF`@`\0 T\x16\x15a\x08\xA8W` `\x01`\x01`\xA0\x1B\x03\x7F]\xB7X\xE9\x95\xA1~\xC1\xAD\x84\xBD\xEF~\x8C2\x93\xA0\xBDay\xBC\xCE@\r\xFF]L=\x87\xDBrk\x92\x16\x80`\x01`\x01`\xA0\x1B\x03\x19`\x06T\x16\x17`\x06U`@Q\x90\x81R\xA1\0[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`@a\xFF\xFF\x80a\x1F\xB4a\x03\xDCV[\x16`\0R`\n` R`\xFF\x82`\0 T\x83Q\x92\x81\x16\x83R`\x10\x1C\x16\x15\x15` \x82\x01R\xF3[4a\x03\xEDW`\x806`\x03\x19\x01\x12a\x03\xEDWa\x1F\xF1a\x03\xDCV[a\x1F\xF9a\x03\xF2V[\x90`d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\xEDWa \x1A\x906\x90`\x04\x01a\x04\x14V[`\0\x92\x91\x92\x933\x85R`\x02` R`\xFF`@\x86 T\x16\x15a\x08\xA8W\x84\x92`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x82;\x15a \xD1W\x84\x90a \xAC`@Q\x97\x88\x96\x87\x95\x86\x94c2\xFBb\xE7`\xE2\x1B\x86Ra\xFF\xFF\x80\x92\x16`\x04\x87\x01R\x16`$\x85\x01R`D5`D\x85\x01R`\x80`d\x85\x01R`\x84\x84\x01\x91a,0V[\x03\x92Z\xF1\x80\x15a\x08\x9FWa \xBEWP\x80\xF3[\x80a \xCBa\x08\x9C\x92a\x06MV[\x80a\x05\xF2V[\x84\x80\xFD[a \xDE6a\x04BV[a!\x1F\x83a!\x06a \xFF\x89\x97\x99a\xFF\xFF\x16`\0R`\x07` R`@`\0 \x90V[\x89\x89a.?V[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R` R`@`\0 \x90V[T\x91\x82\x15a\"\x95W\x82a!36\x84\x84a\x12\xEAV[` \x81Q\x91\x01 \x03a\x14\xF6Wa!w\x91`\0a!k\x86a!\x06a!d\x8Aa\xFF\xFF\x16`\0R`\x07` R`@`\0 \x90V[\x8C\x8Ca.?V[Ua\x14\x0C6\x89\x89a\x12\xEAV[\x91`\xFFa!\x83\x84a63V[\x16a\x15\x14W`\xFFa!\x93\x84a63V[\x16\x15\x80\x15\x90a\"\x89W[a\x14\xF6W`!\x83Q\x10a\x14\xE4W\x7F\xC2d\xD9\x1F:\xDCU\x88%\x0E\x15Q\xF5Gu,\xA0\xCF\xA8\xF6\xB50\xD2C\xB9\xF9\xF4\xCA\xB1\x0E\xA8\xE5\x95\x83a!\xE1`-a\"z\x96\x01Q``\x1C\x91a6CV[\x81\x15a\"\x7FW[a\"\x1D\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x16a.\x90V[a\"'\x81\x83a4\x10V[\x7F\xBFU\x1E\xC98Y\xB1p\xF9\xB2\x14\x1B\xD9)\x8B\xF3\xF6C\"\xC6\xF7\xBE\xB2T:\x0C\xB6i\x83A\x18\xBF`\x01`\x01`\xA0\x1B\x03`@Q\x93\x16\x92\x80a\"ka\xFF\xFF\x8B\x16\x94\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA3`@Q\x95\x86\x95\x86a.XV[\x03\x90\xA1\0[a\xDE\xAD\x91Pa!\xE8V[P`)\x83Q\x14\x15a!\x9DV[`@Qc+\x96\xC9\x85`\xE2\x1B\x81R`\x04\x90\xFD[4a\x03\xEDW`\xE06`\x03\x19\x01\x12a\x03\xEDW`\x045a\"\xC4\x81a\x08\xB9V[`$5\x90a\"\xD1\x82a\x08\xB9V[`D5`d5\x92`\x845\x93`\xFF\x85\x16\x85\x03a\x03\xEDWa$\t` \x91a\"\xF8B\x82\x10\x15a)\xA7V[a#\xD0a#\xDCa#\x06a*KV[\x92\x88a#%\x81`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x11` R`@`\0 \x90V[\x80T\x90`\x01\x82\x01\x90Ua#\x93`@Q\x93\x84\x92\x8C\x8C\x8C\x86\x01\x96\x87\x91\x95\x94\x93\x90\x92`\xA0\x93`\xC0\x84\x01\x97\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x85R`\x01`\x01`\xA0\x1B\x03\x80\x92\x16` \x86\x01R\x16`@\x84\x01R``\x83\x01R`\x80\x82\x01R\x01RV[\x03\x91a#\xA7`\x1F\x19\x93\x84\x81\x01\x83R\x82a\x06\x9EV[Q\x90 `@Q\x93\x84\x91\x88\x83\x01\x96\x87\x90\x91`B\x92a\x19\x01`\xF0\x1B\x83R`\x02\x83\x01R`\"\x82\x01R\x01\x90V[\x03\x90\x81\x01\x83R\x82a\x06\x9EV[Q\x90 `@\x80Q\x91\x82R`\xFF\x90\x97\x16` \x82\x01R`\xA45\x96\x81\x01\x96\x90\x96R`\xC45``\x87\x01R`\x80\x86\x01\x90V[\x85`\0\x96\x87\x92\x83\x80R\x03\x90`\x01Z\xFA\x15a\x08\x9FW\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90\x84Q\x90\x83a$\x82\x82a\x0B\xDB`\x01`\x01`\xA0\x1B\x03\x95a$i\x87\x82\x16\x80\x15\x15\x90\x81a$\x95W[Pa)\xFFV[`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x10` R`@`\0 \x90V[U`@Q\x93\x84R\x81\x16\x93\x16\x91` \x90\xA3\x80\xF3[\x90P\x88\x8C\x16\x148a$cV[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` a\xFF\xFF`\x0BT\x16`@Q\x90\x81R\xF3[4a\x03\xEDW`@6`\x03\x19\x01\x12a\x03\xEDW` a\x13\xDC`\x045a$\xE5\x81a\x08\xB9V[`\x01`\x01`\xA0\x1B\x03`$5\x91a$\xFA\x83a\x08\xB9V[\x16`\0R`\x10\x83R`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[4a\x03\xEDW``6`\x03\x19\x01\x12a\x03\xEDWa%8a\x03\xDCV[a%@a\x03\xF2V[`D5`\0\x923\x84R`\x02` R`@\x90`\xFF\x82\x86 T\x16\x15a%\xD8W\x82\x15a%\xC7W\x91\x7F\x9D\\|\x0B\x93M\xA8\xFE\xFA\x9Cw`\xC9\x83\x83w\x8A\x12\xDF\xBF\xC0\xC3\xB3\x10e\x18\xF4?\xB9P\x8A\xC0\x93\x91``\x93a\xFF\xFF\x80\x91\x16\x93\x84\x88R`\x04` R\x83a%\xB4\x82\x85\x8B \x90a\xFF\xFF\x16`\0R` R`@`\0 \x90V[U\x82Q\x94\x85R\x16` \x84\x01R\x82\x01R\xA1\x80\xF3[\x81Qc\xE4\xAC;?`\xE0\x1B\x81R`\x04\x90\xFD[\x81Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x90\xFD[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Q`\x01\x81R\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045\x80\x15\x15\x80\x91\x03a\x03\xEDW3`\0R`\x02` R`\xFF`@`\0 T\x16\x15a\x08\xA8W` \x7F\x15\x84\xADYJp\xCB\xE1\xE6QU\x92\xE1'*\x98}\x92+\t~\xAD\x87Pi\xCE\xBE\x8B@\xC0\x04\xA4\x91`\xFF\x19`\x08T\x16`\xFF\x82\x16\x17`\x08U`@Q\x90\x81R\xA1\0[4a\x03\xEDWa\x01\x006`\x03\x19\x01\x12a\x03\xEDWa&\x8Fa\x03\xDCV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90`$5\x82\x81\x11a\x03\xEDWa&\xB1\x906\x90`\x04\x01a\x04\x14V[\x91\x90`D5\x90\x84\x82\x16\x82\x03a\x03\xEDW`\x845a&\xCC\x81a\x08\xB9V[`\xC45\x95\x86\x11a\x03\xEDWa&\xE7a\x05I\x966\x90`\x04\x01a\x04\x14V[\x94\x90\x93`\xE45\x96`\xA45\x94`d5\x93a/&V[4a\x03\xEDWa'\t6a\x0F\xC4V[\x91\x90`\0\x913\x83R` `\x02\x81R`\xFF`@\x85 T\x16\x15a\x08\xA8Wa\xFF\xFF\x82\x16\x84R`\x03\x81R`@\x84 \x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11a\x06aWa'W\x86a'Q\x84Ta\x05\xFDV[\x84a,lV[\x84\x90`\x1F\x87\x11`\x01\x14a'\xBFWP\x94a\x18<\x91\x81\x86\x97\x7F\xFAAHz\xD5\xD6r\x8F\x0B\x19'o\xA1\xED\xDC\x16U\x85x\xF5\x10\x9F\xC3\x9D-\xC3<20G\r\xAB\x97\x91a'\xB4W[P\x82`\x01\x1B\x90`\0\x19\x84`\x03\x1B\x1C\x19\x16\x17\x90U`@Q\x93\x84\x93\x84a,QV[\x90P\x85\x0158a'\x95V[\x90`\x1F\x19\x87\x16a'\xD4\x84`\0R` `\0 \x90V[\x92\x87\x90[\x82\x82\x10a(;WPP\x91a\x18<\x93\x91\x88\x7F\xFAAHz\xD5\xD6r\x8F\x0B\x19'o\xA1\xED\xDC\x16U\x85x\xF5\x10\x9F\xC3\x9D-\xC3<20G\r\xAB\x98\x99\x94\x10a(!W[PP`\x01\x82\x81\x1B\x01\x90Ua\x1C\xC2V[\x86\x015`\0\x19`\x03\x85\x90\x1B`\xF8\x16\x1C\x19\x16\x90U8\x80a(\x12V[\x80`\x01\x85\x96\x82\x94\x96\x8B\x015\x81U\x01\x95\x01\x93\x01\x90a'\xD8V[4a\x03\xEDW`@6`\x03\x19\x01\x12a\x03\xEDW` a\x0Eha(qa\x03\xDCV[`$5\x90a.\xB9V[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `\xFF`\x08T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xEDW`\x806`\x03\x19\x01\x12a\x03\xEDWa(\xB6a\x03\xDCV[a(\xBEa\x03\xF2V[\x90a(\xCA`D5a\x08\xB9V[`@Qc={/o`\xE2\x1B\x81Ra\xFF\xFF\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01R0`D\x82\x01R`d\x805\x90\x82\x01R`\0\x81`\x84\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x08\x9FWa\x07u\x91`\0\x91a)HW[P`@Q\x91\x82\x91\x82a\x07\x08V[a)c\x91=\x80\x91\x83>a)[\x81\x83a\x06\x9EV[\x81\x01\x90a+\xD1V[8a);V[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Q0\x81R\xF3[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x0B\xFBWV[\x15a)\xAEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`@Q=`\0\x82>=\x90\xFD[\x15a*\x06WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FINVALID_SIGNER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`\0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a*\x99WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q`\x0CT\x91\x90\x81\x81a*\xAC\x85a\x05\xFDV[\x91\x82\x82R` \x95\x86\x83\x01\x95`\x01\x91\x88\x83\x82\x16\x91\x82`\0\x14a+\xB1WPP`\x01\x14a+WW[PPa*\xDF\x92P\x03\x82a\x06\x9EV[Q\x90 \x90`@Q\x90\x81\x01\x91\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R`@\x82\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra+Q\x81a\x06\x82V[Q\x90 \x90V[\x90\x87\x92P`\x0C\x82R\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7[\x85\x83\x10a+\x99WPPa*\xDF\x93P\x82\x01\x018\x80a*\xD1V[\x80T\x83\x88\x01\x85\x01R\x86\x94P\x88\x93\x90\x92\x01\x91\x81\x01a+\x81V[\x92P\x93PPa*\xDF\x94\x91P`\xFF\x19\x16\x86R\x15\x15`\x05\x1B\x82\x01\x018\x80a*\xD1V[` \x81\x83\x03\x12a\x03\xEDW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03\xEDW\x01\x81`\x1F\x82\x01\x12\x15a\x03\xEDW\x80Qa,\x04\x81a\x12\xCEV[\x92a,\x12`@Q\x94\x85a\x06\x9EV[\x81\x84R` \x82\x84\x01\x01\x11a\x03\xEDWa\x07\x19\x91` \x80\x85\x01\x91\x01a\x06\xC0V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90a\xFF\xFFa\x07\x19\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a,0V[\x90`\x1F\x81\x11a,zWPPPV[`\0\x91\x82R` \x82 \x90` `\x1F\x85\x01`\x05\x1C\x83\x01\x94\x10a,\xB6W[`\x1F\x01`\x05\x1C\x01\x91[\x82\x81\x10a,\xABWPPPV[\x81\x81U`\x01\x01a,\x9FV[\x90\x92P\x82\x90a,\x96V[\x92\x90\x91Z`@Qc3V\xAEE`\xE1\x1B` \x82\x01\x90\x81Ra\xFF\xFF\x87\x16`$\x83\x01R`\x80`D\x83\x01R\x94\x91a--\x82a-\x1Fa,\xFD`\xA4\x83\x01\x87a\x06\xE3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16`d\x84\x01R\x82\x81\x03`#\x19\x01`\x84\x84\x01R\x88a\x06\xE3V[\x03`\x1F\x19\x81\x01\x84R\x83a\x06\x9EV[`\0\x80\x91`@Q\x97a->\x89a\x06\x82V[`\x96\x89R\x82` \x8A\x01\x95`\xA06\x887Q\x920\x90\xF1\x90=\x90`\x96\x82\x11a-\x85W[`\0\x90\x82\x88R>\x15a-rW[PPPPPV[a-{\x94a-\x8EV[8\x80\x80\x80\x80a-kV[`\x96\x91Pa-^V[\x91\x93a.,\x7F\xE1\x83\xF3=\xE2\x83w\x95R[G\x92\xCAL\xD6\x055\xBDw\xC5;~p0\x06\x0B\xFC\xF5sMk\x0C\x95a.:\x93\x95a\xFF\xFF\x81Q` \x83\x01 \x96\x16\x95\x86`\0R`\x07` Ra-\xF2\x83a\x13\xBA` \x8B`@`\0 \x82`@Q\x94\x83\x86\x80\x95Q\x93\x84\x92\x01a\x06\xC0V[Ug\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa.\x18`@Q\x98\x89\x98\x89R`\xA0` \x8A\x01R`\xA0\x89\x01\x90a\x06\xE3V[\x92\x16`@\x87\x01R\x85\x82\x03``\x87\x01Ra\x06\xE3V[\x90\x83\x82\x03`\x80\x85\x01Ra\x06\xE3V[\x03\x90\xA1V[` \x91\x92\x83`@Q\x94\x85\x93\x847\x82\x01\x90\x81R\x03\x01\x90 \x90V[\x91a.\x85\x90``\x94a\xFF\xFFg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x99\x98\x97\x99\x16\x85R`\x80` \x86\x01R`\x80\x85\x01\x91a,0V[\x95\x16`@\x82\x01R\x01RV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x0B\xFBWV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[a\xFF\xFF\x80\x91\x16`\0R`\n` R`@`\0 `\xFF`@Q\x91a.\xDB\x83a\x06fV[T\x83\x81\x16\x83R`\x10\x1C\x16\x15\x80\x15` \x83\x01Ra/\x06W\x91a/\x02\x91a'\x10\x93Q\x16\x90a.\x90V[\x04\x90V[P`\x0BT\x16\x90\x81\x15a/\x1FWa'\x10\x91a/\x02\x91a.\x90V[PP`\0\x90V[\x93\x90\x96\x97\x98\x95\x94\x91\x9403\x03a0\x03Wa\xFF\xFFa/L`\x01`\x01`\xA0\x1B\x03\x92\x850a4kV[\x95\x16\x92\x16\x92\x83\x83\x7F\xBFU\x1E\xC98Y\xB1p\xF9\xB2\x14\x1B\xD9)\x8B\xF3\xF6C\"\xC6\xF7\xBE\xB2T:\x0C\xB6i\x83A\x18\xBF` `@Q\x89\x81R\xA3\x83;\x15a\x03\xEDWa/\xC4\x99`\0\x99\x8A\x96a/\xE6\x94g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x9E\x8F\x9D\x8E\x9C\x8D\x9Ac?\xE7\x9A\xED`\xE1\x1B\x8CR`\x04\x8C\x01R`\xC0`$\x8C\x01R`\xC4\x8B\x01\x91a,0V[\x95\x16`D\x88\x01R`d\x87\x01R`\x84\x86\x01R\x84\x83\x03`\x03\x19\x01`\xA4\x86\x01Ra,0V[\x03\x93\xF1\x80\x15a\x08\x9FWa/\xF6WPV[\x80a \xCBa\x16\xF2\x92a\x06MV[`@Qc \xAAl#`\xE2\x1B\x81R`\x04\x90\xFD[\x91\x90\x82`@\x91\x03\x12a\x03\xEDW` \x82Q\x92\x01Q\x90V[\x92\x96\x95\x96\x94\x91\x94\x93\x90\x93`\xFF`\x08T\x16`\0\x14a1KW`\"\x88Q\x10a19W`\"\x88\x01Qa\xFF\xFF\x86\x16`\0R`\x04` Ra0t`@`\0 `\0\x80R` R`@`\0 \x90V[T\x90\x81\x15a1'W\x10a1\x15Wa0\x8Da0\x94\x91a3\x15V[P\x84a3\x94V[\x96\x87\x15a1\x03Wa0\xBA\x92a0\xB1a0\xAB\x8Aa2\xC0V[\x88a3LV[\x924\x93\x87a1\xC6V[\x7F\xD8\x1F\xC9\xB8R14\xEDa8p\xED\x02\x9Dap\xCB\xB7:\xA6\xA6\xBC1\x1B\x9Ad&\x89\xFB\x9D\xF5\x9Aa\xFF\xFF`\x01`\x01`\xA0\x1B\x03`@Q\x93\x16\x93\x16\x91\x80a0\xFE\x88\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA4V[`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x90\xFD[`@Qcv\xA1\xE1\xD3`\xE1\x1B\x81R`\x04\x90\xFD[`@Qc\x1F>\xC9\xD5`\xE1\x1B\x81R`\x04\x90\xFD[`@Qc\xCE\xF8\x0E\xA3`\xE0\x1B\x81R`\x04\x90\xFD[\x87Qa1]Wa0\x8Da0\x94\x91a3\x15V[`@Qc\x8F\xAD\xCA\xDB`\xE0\x1B\x81R`\x04\x90\xFD[\x92a1\x94a\x07\x19\x97\x95\x93a\xFF\xFFa1\xA2\x94\x16\x86R`\xC0` \x87\x01R`\xC0\x86\x01\x90a\x06\xE3V[\x90\x84\x82\x03`@\x86\x01Ra\x06\xE3V[\x93`\x01`\x01`\xA0\x1B\x03\x80\x92\x16``\x84\x01R\x16`\x80\x82\x01R`\xA0\x81\x84\x03\x91\x01Ra\x06\xE3V[\x94a\x1B\x0F\x91\x93\x92\x95a1\xF5a1\xE9\x82a\xFF\xFF\x16`\0R`\x03` R`@`\0 \x90V[`@Q\x94\x85\x80\x92a\x16AV[\x82Q\x15a2\xA0W\x84Qa\xFF\xFF\x82\x16`\0R`\x05` R`@`\0 T\x90\x81\x15a2\x96W[\x11a2\x84W`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x84;\x15a\x03\xEDW`\0\x96a2s\x91`@Q\x99\x8A\x98\x89\x97\x88\x96b\xC5\x801`\xE8\x1B\x88R`\x04\x88\x01a1oV[\x03\x92Z\xF1\x80\x15a\x08\x9FWa/\xF6WPV[`@Qc\"\x0B\t3`\xE1\x1B\x81R`\x04\x90\xFD[a'\x10\x91Pa2\x19V[`@Qc&\xBA|\xFB`\xE0\x1B\x81R`\x04\x90\xFD[\x90`\x1F\x82\x01\x80\x92\x11a\x0B\xFBWV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x15a3\x10W\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x81\x11a2\xFEW\x16\x90V[`@Qc1$\x99\x8D`\xE1\x1B\x81R`\x04\x90\xFD[a.\xA3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x15a3\x10W\x81\x06\x90\x81\x81\x03\x90\x81\x11a\x0B\xFBW\x91V[\x90`@Q\x91`\0` \x84\x01R`!\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xC0\x1B\x90`\xC0\x1B\x16`A\x82\x01R`)\x81R``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x06aW`@R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x823\x83\x03a3\xFEW[PP\x80`\0R`\x0F` R`@`\0 \x90\x81T\x83\x81\x03\x90\x81\x11a\x0B\xFBW`\0\x92U\x82`\x0ET\x03`\x0EU\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF` `@Q\x85\x81R\xA3\x90V[a4\t\x913\x90a5wV[8\x82a3\xA8V[`\x0ET\x82\x81\x01\x80\x91\x11a\x0B\xFBW` `\x01`\x01`\xA0\x1B\x03`\0\x93\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93`\x0EU\x16\x93\x84\x84R`\x0F\x82R`@\x84 \x81\x81T\x01\x90U`@Q\x90\x81R\xA3V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x81\x16\x910\x83\x14\x15\x80a5mW[a5]W[\x82\x15\x80\x15a5SW[a5AW\x84a4\xB2\x83`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0F` R`@`\0 \x90V[T\x10a5/Wa4\xF6\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0F` R`@`\0 \x90V[\x85\x81T\x03\x90Ua5\x19\x84`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0F` R`@`\0 \x90V[\x80T\x86\x01\x90U`@Q\x85\x81R\x93\x16\x92` \x90\xA3\x90V[`@Qc\x1E\x9A\xCF\x17`\xE3\x1B\x81R`\x04\x90\xFD[`@Qc\xD9.#=`\xE0\x1B\x81R`\x04\x90\xFD[P\x80\x84\x16\x15a4\x90V[a5h\x853\x84a5wV[a4\x87V[P3\x83\x14\x15a4\x82V[\x91\x90`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x81`\0R`\x10` Ra5\xAF\x83`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T\x91`\x01\x83\x01a5\xC2W[PPPPPPV[\x84\x83\x10a6\x11W\x15\x90\x81\x15a6\x06W[Pa5AWa5\xFA\x92a\x0B\xDB\x91\x03\x93`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x10` R`@`\0 \x90V[U8\x80\x80\x80\x80\x80a5\xBAV[\x90P\x82\x16\x158a5\xD2V[`@Qc\x13\xBE%+`\xE0\x1B\x81R`\x04\x90\xFD[`\xFF\x16\x80\x15a\x0B\xFBW`\0\x19\x01\x90V[`\x01\x81Q\x10a\x14\xE4W`\x01\x01Q\x90V[`)\x81Q\x10a\x14\xE4W`)\x01Q\x90V\xFE\xA2dipfsX\"\x12 c\xCB\xF8\xA8\0\x8Cd\x9F6MP\xD5\xBF\xD9,\x86Cw\xBB\xB4I\xDA\x95\xC2\xC2W\xE9\x83\x08r\xDF|dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static OFTWITHFEE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80b\x1D5g\x14a\x03\xD7W\x80c\x01\xFF\xC9\xA7\x14a\x03\xD2W\x80c\x06\xFD\xDE\x03\x14a\x03\xCDW\x80c\x07\xE0\xDB\x17\x14a\x03\xC8W\x80c\t^\xA7\xB3\x14a\x03\xC3W\x80c\r\xF3t\x83\x14a\x03\xBEW\x80c\x10\xDD\xB17\x14a\x03\xB9W\x80c\x13\xE7\xC9\xD8\x14a\x03\xB4W\x80c\x18\x16\r\xDD\x14a\x03AW\x80c#\xB8r\xDD\x14a\x03\xAFW\x80c'\xE1\xF7\xDF\x14a\x03\xAAW\x80c,\xDF\x0B\x95\x14a\x03\xA5W\x80c1<\xE5g\x14a\x03\xA0W\x80c6D\xE5\x15\x14a\x03\x9BW\x80c6R`\xB4\x14a\x03\x96W\x80c=\x8B8\xF6\x14a\x03\x91W\x80c?\x1FO\xA4\x14a\x03\x8CW\x80cB\x9Bb\xE5\x14a\x03\x87W\x80cB\xD6Z\x8D\x14a\x03\x82W\x80cDw\x05\x15\x14a\x03xW\x80cK\x10N\xFF\x14a\x03}W\x80cLB\x89\x9A\x14a\x03xW\x80cZ5\x9D\xC5\x14a\x03sW\x80c[\x8CA\xE6\x14a\x03nW\x80cf\xAD\\\x8A\x14a\x03iW\x80cpH\x02u\x14a\x03dW\x80cp\xA0\x821\x14a\x03_W\x80cu3\xD7\x88\x14a\x03ZW\x80cy\xC0\xADK\x14a\x03UW\x80c~\xCE\xBE\0\x14a\x03PW\x80c\x85wI\xB0\x14a\x03KW\x80c\x8C\xFD\x8F\\\x14a\x03FW\x80c\x93X\x92\x8B\x14a\x03AW\x80c\x95\x0C\x8At\x14a\x03<W\x80c\x95\xD8\x9BA\x14a\x037W\x80c\x98p\xD7\xFE\x14a\x032W\x80c\x9B\xDB\x98\x12\x14a\x03-W\x80c\x9F86\x9A\x14a\x03(W\x80c\xA6\xC3\xD1e\x14a\x03#W\x80c\xA9\x05\x9C\xBB\x14a\x03\x1EW\x80c\xAB\xE6\x85\xCD\x14a\x03\x05W\x80c\xB3S\xAA\xA7\x14a\x03\x19W\x80c\xB4\t\x92\xA1\x14a\x03\x14W\x80c\xB9\x81\x8B\xE1\x14a\x03\x0FW\x80c\xBA\xF3)-\x14a\x03\nW\x80c\xC4F\x184\x14a\x03\x05W\x80c\xC830\xCE\x14a\x03\0W\x80c\xCB\xED\x8B\x9C\x14a\x02\xFBW\x80c\xD1\xDE\xBA\x1F\x14a\x02\xF6W\x80c\xD5\x05\xAC\xCF\x14a\x02\xF1W\x80c\xD8\x88)h\x14a\x02\xECW\x80c\xDDb\xED>\x14a\x02\xE7W\x80c\xDF*[;\x14a\x02\xE2W\x80c\xE6\xA2\n\xE6\x14a\x02\xDDW\x80c\xEA\xB4]\x9C\x14a\x02\xD8W\x80c\xEA\xFF\xD4\x9A\x14a\x02\xD3W\x80c\xEB\x8Dr\xB7\x14a\x02\xCEW\x80c\xEC\xD8\xF2\x12\x14a\x02\xC9W\x80c\xEDb\x9C\\\x14a\x02\xC4W\x80c\xF5\xEC\xBD\xBC\x14a\x02\xBFWc\xFC\x0CTj\x14a\x02\xBAW`\0\x80\xFD[a)iV[a(\x9DV[a(zV[a(SV[a&\xFBV[a&uV[a&\x04V[a%\xE8V[a%\x1FV[a$\xC3V[a$\xA1V[a\"\xA7V[a \xD5V[a\x1F\xD8V[a\x1F\x95V[a\x1E\0V[a\x1F\x19V[a\x1E\xEFV[a\x1EaV[a\x1E\x1DV[a\x1DoV[a\x1B\xE7V[a\x1A\xD2V[a\x1A\x87V[a\x19\xF7V[a\x19PV[a\x19)V[a\ntV[a\x18\xDFV[a\x18\xA1V[a\x18cV[a\x17GV[a\x16\xF4V[a\x16\x03V[a\x157V[a\x13\xE5V[a\x13}V[a\x12EV[a\x11gV[a\x11\x83V[a\x10\xD3V[a\x10\x90V[a\x10[V[a\x0F\xFFV[a\x0E\x8EV[a\x0EMV[a\x0E\x0FV[a\r\x10V[a\x0C\0V[a\n\x92V[a\n1V[a\t\x9FV[a\tXV[a\x08\xCAV[a\x08\x01V[a\x07\x1CV[a\x05\x9CV[a\x04\xAAV[`\x045\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xEDWV[`\0\x80\xFD[`$5\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xEDWV[`D5\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xEDWV[\x91\x81`\x1F\x84\x01\x12\x15a\x03\xEDW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\xEDW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03\xEDWV[\x90`\x80`\x03\x19\x83\x01\x12a\x03\xEDW`\x045a\xFF\xFF\x81\x16\x81\x03a\x03\xEDW\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90`$5\x82\x81\x11a\x03\xEDW\x81a\x04\x7F\x91`\x04\x01a\x04\x14V[\x93\x90\x93\x92`D5\x81\x81\x16\x81\x03a\x03\xEDW\x92`d5\x91\x82\x11a\x03\xEDWa\x04\xA6\x91`\x04\x01a\x04\x14V[\x90\x91V[4a\x03\xEDWa\x04\xB86a\x04BV[\x91\x92\x94\x93\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x05\x8AWa\x05\ra\x05\x08\x86a\xFF\xFF\x16`\0R`\x03` R`@`\0 \x90V[a\x16\xD7V[\x80Q\x90\x81\x88\x14\x91\x82\x15\x92a\x05\x81W[P\x81\x15a\x05]W[Pa\x05KWa\x05;a\x05C\x92a\x05I\x976\x91a\x12\xEAV[\x926\x91a\x12\xEAV[\x92a,\xC0V[\0[`@Qc\x195\xE2\x81`\xE1\x1B\x81R`\x04\x90\xFD[\x90Pa\x05j6\x88\x85a\x12\xEAV[` \x81Q\x91\x01 \x90` \x81Q\x91\x01 \x14\x158a\x05$V[\x15\x91P8a\x05\x1CV[`@Qc\r\x1A\xD4\xCD`\xE0\x1B\x81R`\x04\x90\xFD[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045c\xFF\xFF\xFF\xFF`\xE0\x1B\x81\x16\x80\x91\x03a\x03\xEDW` \x90c,\xDF\x0B\x95`\xE0\x1B\x81\x14\x90\x81\x15a\x05\xE1W[P`@Q\x90\x15\x15\x81R\xF3[c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x90P8a\x05\xD6V[`\0\x91\x03\x12a\x03\xEDWV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x06-W[` \x83\x10\x14a\x06\x17WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x06\x0CV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06aW`@RV[a\x067V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x06aW`@RV[`\xC0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x06aW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x06aW`@RV[`\0[\x83\x81\x10a\x06\xD3WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xC3V[\x90` \x91a\x06\xFC\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x06\xC0V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x07\x19\x92\x81\x81R\x01\x90a\x06\xE3V[\x90V[4a\x03\xEDW`\0\x80`\x03\x196\x01\x12a\x07\xFEW`@Q\x90\x80`\x0CTa\x07?\x81a\x05\xFDV[\x80\x85R\x91`\x01\x91\x80\x83\x16\x90\x81\x15a\x07\xD4WP`\x01\x14a\x07yW[a\x07u\x85a\x07i\x81\x87\x03\x82a\x06\x9EV[`@Q\x91\x82\x91\x82a\x07\x08V[\x03\x90\xF3[\x92P`\x0C\x83R\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7[\x82\x84\x10a\x07\xBCWPPP\x81\x01` \x01a\x07i\x82a\x07ua\x07YV[\x80T` \x85\x87\x01\x81\x01\x91\x90\x91R\x90\x93\x01\x92\x81\x01a\x07\xA1V[\x86\x95Pa\x07u\x96\x93P` \x92Pa\x07i\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x93a\x07YV[\x80\xFD[4a\x03\xEDW`\0` 6`\x03\x19\x01\x12a\x07\xFEWa\x08\x1Ca\x03\xDCV[3\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8W\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x82;\x15a\x08\xA4W`$a\xFF\xFF\x91\x83`@Q\x95\x86\x94\x85\x93c\x07\xE0\xDB\x17`\xE0\x1B\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a\x08\x9FWa\x08\x93WP\x80\xF3[a\x08\x9C\x90a\x06MV[\x80\xF3[a)\xF3V[P\x80\xFD[`@Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\xEDWV[4a\x03\xEDW`@6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x08\xE7\x81a\x08\xB9V[`\x01`\x01`\xA0\x1B\x03`$5\x913`\0R`\x10` R\x82a\t\x1E\x82`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[U`@Q\x92\x83R\x16\x90\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` 3\x92\xA3` `@Q`\x01\x81R\xF3[4a\x03\xEDW`@6`\x03\x19\x01\x12a\x03\xEDWa\tqa\x03\xDCV[`\0\x903\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8Wa\xFF\xFF\x16\x81R`\x05` R`$5`@\x82 U\x80\xF3[4a\x03\xEDW`\0` 6`\x03\x19\x01\x12a\x07\xFEWa\t\xBAa\x03\xDCV[3\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8W\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x82;\x15a\x08\xA4W`$a\xFF\xFF\x91\x83`@Q\x95\x86\x94\x85\x93c\x10\xDD\xB17`\xE0\x1B\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a\x08\x9FWa\x08\x93WP\x80\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x01`\x01`\xA0\x1B\x03`\x045a\nV\x81a\x08\xB9V[\x16`\0R`\x01` R` `\xFF`@`\0 T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `\x0ET`@Q\x90\x81R\xF3[4a\x03\xEDW``6`\x03\x19\x01\x12a\x03\xEDW`\x045a\n\xAF\x81a\x08\xB9V[`$5a\n\xBB\x81a\x08\xB9V[`D5\x91`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x92\x83`\0R`\x10` Ra\n\xF4`@`\0 3`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T`\x01\x81\x01a\x0B\x8AW[Pa\x0B=\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0F` R`@`\0 \x90V[a\x0BH\x86\x82Ta)\x9AV[\x90Ua\x0Bg\x81`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0F` R`@`\0 \x90V[\x80T\x86\x01\x90U`@Q\x94\x85R\x16\x92\x80` \x81\x01[\x03\x90\xA3`@Q`\x01\x81R` \x90\xF3[\x85\x81\x03\x90\x81\x11a\x0B\xFBW\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93a\x0B=\x91a\x0B\xF33a\x0B\xDB\x84`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x10` R`@`\0 \x90V[\x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[U\x93Pa\n\xFEV[a)\x84V[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x0C\x1D\x81a\x08\xB9V[`\0\x903\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8W`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x83R`\x02` Ra\x0C^a\x0CZ`@\x85 `\xFF\x90T\x16\x90V[\x15\x90V[a\x0C\xFEWa\x0C\x88a\x0Cxa\x0Cs\x85T`\xFF\x16\x90V[a6#V[`\xFF\x16`\xFF\x19`\0T\x16\x17`\0UV[`\xFFa\x0C\x95\x84T`\xFF\x16\x90V[\x16\x15a\x0C\xECWa\x0C\xBBa\x0C\xC5\x91`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x80T`\xFF\x19\x16\x90UV[\x7F\x98\x9D\xDF\xCE\x05}\xAD!\x9E\n\xE1oi\x1B\x12\x1B\xB0\xE3H\xF0\xD8\xAE\n\xD4\0\xB4\xD5\xAC\x8Dal\x8B\x82\x80\xA2\x80\xF3[`@Qc\x1F\x8C\x1D\xBD`\xE1\x1B\x81R`\x04\x90\xFD[`@Qc\xA7A\xA0E`\xE0\x1B\x81R`\x04\x90\xFD[`\x03\x19`\xC06\x82\x01\x12a\x03\xEDW`\x045a\r)\x81a\x08\xB9V[a\r1a\x03\xF2V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92`\xA45\x90`d5\x85\x83\x11a\x03\xEDW``\x836\x03\x92\x83\x01\x12a\x03\xEDWa\r`\x81\x85a.\xB9V[\x80\x82\x03\x91\x82\x11a\x0B\xFBW\x80a\r\xEFW[P\x82`\x04\x015\x91a\r\x80\x83a\x08\xB9V[`$\x84\x015\x93a\r\x8F\x85a\x08\xB9V[`D\x81\x015\x91`\"\x19\x01\x82\x12\x15a\x03\xEDW\x01`\x04\x81\x015\x96\x87\x11a\x03\xEDW`$\x01\x94\x866\x03\x86\x13a\x03\xEDWa\r\xCCa\r\xD6\x96`\x845\x986\x91a\x12\xEAV[\x94`D5\x91a0+V[\x10a\r\xDDW\0[`@Qc@\x84GY`\xE0\x1B\x81R`\x04\x90\xFD[a\x0E\x08\x90`\x01`\x01`\xA0\x1B\x03`\x0BT`\x10\x1C\x16\x87a4kV[P8a\rpV[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` a\x0Eha*KV[`@Q\x90\x81R\xF3[`d5\x90\x81\x15\x15\x82\x03a\x03\xEDWV[`$5\x90\x81\x15\x15\x82\x03a\x03\xEDWV[4a\x03\xEDW`\x03\x19`\xA06\x82\x01\x12a\x03\xEDWa\x0E\xA8a\x03\xDCV[a\x0E\xB0a\x0EpV[`\x845\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\xEDWa\x0FBa\xFF\xFF\x92a\x0E\xE4a\x0E\xDD`@\x966\x90`\x04\x01a\x04\x14V[6\x91a\x12\xEAV[a\x0E\xFAa\x0E\xF2`D5a2\xC0V[`$5a3LV[\x96a\x0F,\x87Q\x98\x89\x97\x88\x97c\x04\n{\xB1`\xE4\x1B\x89R\x16`\x04\x88\x01R0`$\x88\x01R`\xA0`D\x88\x01R`\xA4\x87\x01\x90a\x06\xE3V[\x92\x15\x15`d\x86\x01R\x84\x83\x03\x01`\x84\x85\x01Ra\x06\xE3V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x08\x9FW`\0\x90\x81\x92a\x0F\x93W[P`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xF3[\x90Pa\x0F\xB6\x91P`@=\x81\x11a\x0F\xBDW[a\x0F\xAE\x81\x83a\x06\x9EV[\x81\x01\x90a0\x15V[\x908a\x0F\x81V[P=a\x0F\xA4V[\x90`@`\x03\x19\x83\x01\x12a\x03\xEDW`\x045a\xFF\xFF\x81\x16\x81\x03a\x03\xEDW\x91`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03\xEDWa\x04\xA6\x91`\x04\x01a\x04\x14V[4a\x03\xEDW` a\xFF\xFFa\x10La\x10\x156a\x0F\xC4V[\x93\x90\x91\x16`\0R`\x03\x84Ra\x107a\x10>`@`\0 `@Q\x92\x83\x80\x92a\x16AV[\x03\x82a\x06\x9EV[\x84\x81Q\x91\x01 \x926\x91a\x12\xEAV[\x82\x81Q\x91\x01 \x14`@Q\x90\x81R\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDWa\xFF\xFFa\x10wa\x03\xDCV[\x16`\0R`\x05` R` `@`\0 T`@Q\x90\x81R\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x01`\x01`\xA0\x1B\x03`\x045a\x10\xB5\x81a\x08\xB9V[\x16`\0R`\x02` R` `\xFF`@`\0 T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xEDWa\x10\xE16a\x0F\xC4V[\x91\x90`\0\x923\x84R`\x02` R`\xFF`@\x85 T\x16\x15a\x08\xA8W\x83\x91`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x81;\x15a\x11cW\x83a\x11Q\x95`@Q\x96\x87\x95\x86\x94\x85\x93cB\xD6Z\x8D`\xE0\x1B\x85R`\x04\x85\x01a,QV[\x03\x92Z\xF1\x80\x15a\x08\x9FWa\x08\x93WP\x80\xF3[\x83\x80\xFD[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Q`\0\x81R\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x11\xA0\x81a\x08\xB9V[3`\0R`\x02` R`\xFF`@`\0 T\x16\x15a\x08\xA8W`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x15a\x123W\x7F\x04y\x12c\x1A\xFAVN\xEB\xD3\xDB.\xFE\x19\x1A\r\xECb\xDA\x1F\xED\xE6\xBB\xBC\x1F\xFC\x89\xD8xE\xB1\xB5\x91` \x91u\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0`\x0BT\x91`\x10\x1B\x16\x90u\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x19\x16\x17`\x0BU`@Q\x90\x81R\xA1\0[`@Qc\xA6\xAF\xC5=`\xE0\x1B\x81R`\x04\x90\xFD[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDWa\x12^a\x03\xDCV[3`\0R`\x02` R`\xFF`@`\0 T\x16\x15a\x08\xA8Wa\xFF\xFF\x16a'\x10\x81\x11a\x12\xBCW` \x81\x7F\xD2`0\xEFJ\x8C\"^\xE1+dn\xB4Fj\xCBA\xFB\x96\xB6\xCDF`\xB2-\x0B\xA0\x12N{\xDCt\x92a\xFF\xFF\x19`\x0BT\x16\x17`\x0BU`@Q\x90\x81R\xA1\0[`@Qc\x0F\xC0\x0F\x19`\xE1\x1B\x81R`\x04\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x06aW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x12\xF6\x82a\x12\xCEV[\x91a\x13\x04`@Q\x93\x84a\x06\x9EV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03\xEDW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[```\x03\x19\x82\x01\x12a\x03\xEDW`\x045a\xFF\xFF\x81\x16\x81\x03a\x03\xEDW\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x03\xEDW\x80`#\x83\x01\x12\x15a\x03\xEDW\x81`$a\x13m\x93`\x04\x015\x91\x01a\x12\xEAV[\x91`D5\x90\x81\x16\x81\x03a\x03\xEDW\x90V[4a\x03\xEDW` a\x13\xDCa\xFF\xFFa\x13\xBA\x83a\x13\x976a\x13!V[\x94\x90\x91\x16`\0R`\x07\x82R`@`\0 \x82`@Q\x94\x83\x86\x80\x95Q\x93\x84\x92\x01a\x06\xC0V[\x82\x01\x90\x81R\x03\x01\x90 \x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R` R`@`\0 \x90V[T`@Q\x90\x81R\xF3[4a\x03\xEDWa\x13\xF36a\x04BV[\x91P\x9103\x03a\x15&Wa\x14\x14\x93a\x14\x0C\x916\x91a\x12\xEAV[P6\x91a\x12\xEAV[`\xFFa\x14\x1F\x82a63V[\x16a\x15\x14W`\xFFa\x14/\x82a63V[\x16\x15\x80\x15\x90a\x15\x08W[a\x14\xF6W`!\x81Q\x10a\x14\xE4Wa\x14W`-\x82\x01Q``\x1C\x91a6CV[\x81\x15a\x14\xDAW[`\x01`\x01`\xA0\x1B\x03a\x14\xBC\x7F\xBFU\x1E\xC98Y\xB1p\xF9\xB2\x14\x1B\xD9)\x8B\xF3\xF6C\"\xC6\xF7\xBE\xB2T:\x0C\xB6i\x83A\x18\xBF\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x16a.\x90V[\x92a\x14\xC7\x84\x82a4\x10V[`@Q\x93\x84R\x16\x92a\xFF\xFF\x16\x91` \x90\xA3\0[a\xDE\xAD\x91Pa\x14^V[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc|iS\xF9`\xE0\x1B\x81R`\x04\x90\xFD[P`)\x81Q\x14\x15a\x149V[`@Qc\xFE>\x83'`\xE0\x1B\x81R`\x04\x90\xFD[`@Qb\xE4\xF8\x15`\xE5\x1B\x81R`\x04\x90\xFD[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x15T\x81a\x08\xB9V[`\0\x903\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8W`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x83R`\x02` R`\xFF`@\x84 T\x16a\x0C\xFEW`\xFF\x83T\x16`\xFF\x81\x14a\x0B\xFBWa\x15\xDC\x91a\x15\xB6`\x01a\x15\xCF\x93\x01`\xFF\x16`\xFF\x19`\0T\x16\x17`\0UV[`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[\x7FD\xD6\xD2Yc\xF0\x97\xAD\x14\xF2\x9F\x06\x85J\x01\xF5ud\x8A\x1E\xF8/0\xE5b\xCC\xD3\x88\x97\x17\xE39\x82\x80\xA2\x80\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x01`\x01`\xA0\x1B\x03`\x045a\x16(\x81a\x08\xB9V[\x16`\0R`\x0F` R` `@`\0 T`@Q\x90\x81R\xF3[\x90`\0\x92\x91\x80T\x91a\x16R\x83a\x05\xFDV[\x91\x82\x82R`\x01\x93\x84\x81\x16\x90\x81`\0\x14a\x16\xB4WP`\x01\x14a\x16tW[PPPPV[\x90\x91\x93\x94P`\0R` \x92\x83`\0 \x92\x84`\0\x94[\x83\x86\x10a\x16\xA0WPPPP\x01\x01\x908\x80\x80\x80a\x16nV[\x80T\x85\x87\x01\x83\x01R\x94\x01\x93\x85\x90\x82\x01a\x16\x89V[\x92\x94PPP` \x93\x94P`\xFF\x19\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x908\x80\x80\x80a\x16nV[\x90a\x16\xF2a\x16\xEB\x92`@Q\x93\x84\x80\x92a\x16AV[\x03\x83a\x06\x9EV[V[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDWa\xFF\xFFa\x17\x10a\x03\xDCV[\x16`\0R`\x03` Ra\x07ua\x107a\x173`@`\0 `@Q\x92\x83\x80\x92a\x16AV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\xE3V[4a\x03\xEDW``6`\x03\x19\x01\x12a\x03\xEDWa\x17`a\x03\xDCV[a\x17ha\x0E\x7FV[\x90a\x17qa\x04\x03V[\x91`\0\x913\x83R`\x02` R`@\x93`\xFF\x85\x85 T\x16\x15a\x18SWa\xFF\xFF\x80\x82\x16\x90a'\x10\x82\x11a\x18BW\x95a\x18<\x92\x91\x7F\xDD\x9C\x96\x85\xAF>m\xCBV\xD8\xF4\xB8\x8D%\x95\xD4\xAD\xD6\x83z\x15\x004\xE7x\x1CF\xB6\xDC\xF8\xAA\xAB\x96\x97\x82Q\x91a\x17\xD1\x83a\x06fV[\x82Ra\x18\x02` \x83\x01\x91\x88\x15\x15\x83R\x80\x88\x16\x8BR`\n` R\x84\x8B \x93Q\x16\x83\x90a\xFF\xFF\x16a\xFF\xFF\x19\x82T\x16\x17\x90UV[Q\x81Tb\xFF\0\0\x19\x16\x90\x15\x15`\x10\x1Bb\xFF\0\0\x16\x17\x90UQa\xFF\xFF\x93\x84\x16\x81R\x93\x15\x15` \x85\x01R\x91\x90\x91\x16`@\x83\x01R\x81\x90``\x82\x01\x90V[\x03\x90\xA1\x80\xF3[\x86Qc\x0F\xC0\x0F\x19`\xE1\x1B\x81R`\x04\x90\xFD[\x84Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x90\xFD[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x01`\x01`\xA0\x1B\x03`\x045a\x18\x88\x81a\x08\xB9V[\x16`\0R`\x11` R` `@`\0 T`@Q\x90\x81R\xF3[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xEDW`@6`\x03\x19\x01\x12a\x03\xEDW` a\x13\xDCa\x18\xFDa\x03\xDCV[a\xFF\xFFa\x19\x08a\x03\xF2V[\x91\x16`\0R`\x04\x83R`@`\0 \x90a\xFF\xFF\x16`\0R` R`@`\0 \x90V[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `\x01`\x01`\xA0\x1B\x03`\x06T\x16`@Q\x90\x81R\xF3[4a\x03\xEDW`\0\x80`\x03\x196\x01\x12a\x07\xFEW`@Q\x90\x80`\rTa\x19s\x81a\x05\xFDV[\x80\x85R\x91`\x01\x91\x80\x83\x16\x90\x81\x15a\x07\xD4WP`\x01\x14a\x19\x9CWa\x07u\x85a\x07i\x81\x87\x03\x82a\x06\x9EV[\x92P`\r\x83R\x7F\xD7\xB6\x99\x01\x05q\x91\x01\xDA\xBE\xB7qD\xF2\xA38\\\x803\xAC\xD3\xAF\x97\xE9B:i^\x81\xAD\x1E\xB5[\x82\x84\x10a\x19\xDFWPPP\x81\x01` \x01a\x07i\x82a\x07ua\x07YV[\x80T` \x85\x87\x01\x81\x01\x91\x90\x91R\x90\x93\x01\x92\x81\x01a\x19\xC4V[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x1A\x14\x81a\x08\xB9V[`\0\x903\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8W`\x01`\x01`\xA0\x1B\x03\x16\x80\x82R`\x01` R`\xFF`@\x83 T\x16a\x0C\xFEW\x80\x82R`\x01` R`@\x82 `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\xACo\xA8X\xE95\nF\xCE\xC1e9\x92n\x0F\xDE%\xB7b\x9F\x84\xB5\xA7+\xFF\xAA\xE4\xDF\x88\x8A\xE8m\x82\x80\xA2\x80\xF3[4a\x03\xEDW` `\xFFa\x1A\xC6a\xFF\xFFa\x13\xBA\x84a\x1A\xA36a\x13!V[\x94\x90\x91\x16`\0R`\t\x82R`@`\0 \x82`@Q\x94\x83\x86\x80\x95Q\x93\x84\x92\x01a\x06\xC0V[T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xEDW` \x80`\x03\x196\x01\x12a\x03\xEDWa\x1B\x0F\x90a\xFF\xFFa\x1A\xF3a\x03\xDCV[\x16`\0R`\x03\x81R`@a\x1B\x16\x81`\0 \x82Q\x94\x85\x80\x92a\x16AV[\x03\x84a\x06\x9EV[\x82Q\x15a\x1B\xD7W\x82Q`\x13\x19\x93\x84\x82\x01\x90\x82\x82\x11a\x0B\xFBW\x81a\x1B8\x81a2\xB2V[\x10a\x1B\xC6W\x81\x81Q\x10a\x1B\xB5W\x81a\x1BhWPPPa\x07u\x92P\x80Q\x91`\0\x83R\x82\x01\x81R[Q\x91\x82\x91\x82a\x07\x08V[\x83\x95\x94\x95Q\x94`\x1F\x83\x16\x80\x15`\x05\x1B\x91\x82\x82\x89\x01\x01\x95\x86\x01\x01\x92\x01\x01\x90[\x80\x84\x10a\x1B\xA4WPP\x83R`\x1F\x01`\x1F\x19\x16\x81Ra\x07u\x92Pa\x1B^V[\x81Q\x84R\x92\x86\x01\x92\x90\x86\x01\x90a\x1B\x86V[\x83Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[\x83Qc#\xD5x=`\xE1\x1B\x81R`\x04\x90\xFD[Qc\x05(La`\xE3\x1B\x81R`\x04\x90\xFD[4a\x03\xEDWa\x1B\xF56a\x0F\xC4V[\x91\x90`\0\x913\x83R` `\x02\x81R`\xFF`@\x85 T\x16\x15a\x08\xA8W`@Q\x85\x84\x83\x83\x017a\x1C8`4\x82\x88\x81\x010``\x1B\x86\x82\x01R\x03`\x14\x81\x01\x84R\x01\x82a\x06\x9EV[a\xFF\xFF\x83\x16\x85R`\x03\x82R`@\x85 \x91\x81Q\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x06aWa\x1Cn\x83a\x1Ch\x86Ta\x05\xFDV[\x86a,lV[\x81`\x1F\x84\x11`\x01\x14a\x1C\xD9WP\x91\x80a\x18<\x94\x92\x88\x99\x94\x7F\x8C\x04\0\xCF\xE2\xD1\x19\x9B\x1Ar\\x\x96\x0B\xCC*4M\x86\x9B\x80Y\r\x0F+\xD0\x05\xDB\x15\xA5r\xCE\x99\x92a\x1C\xCEW[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90U[`@Q\x93\x84\x93\x84a,QV[\x01Q\x90P8\x80a\x1C\xADV[\x91\x90`\x1F\x19\x84\x16a\x1C\xEF\x86`\0R` `\0 \x90V[\x93\x89\x90[\x82\x82\x10a\x1DWWPP\x92`\x01\x92\x85\x92\x7F\x8C\x04\0\xCF\xE2\xD1\x19\x9B\x1Ar\\x\x96\x0B\xCC*4M\x86\x9B\x80Y\r\x0F+\xD0\x05\xDB\x15\xA5r\xCE\x9A\x9B\x96a\x18<\x98\x96\x10a\x1D>W[PPP\x81\x1B\x01\x90Ua\x1C\xC2V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\x1D1V[\x80`\x01\x86\x97\x82\x94\x97\x87\x01Q\x81U\x01\x96\x01\x94\x01\x90a\x1C\xF3V[4a\x03\xEDW`@6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x1D\x8C\x81a\x08\xB9V[`$5\x903`\0R`\x0F` R`@`\0 \x90\x81T\x83\x81\x03\x90\x81\x11a\x0B\xFBW`\x01`\x01`\xA0\x1B\x03\x92U\x16\x90\x81`\0R`\x0F` R`@`\0 \x81\x81T\x01\x90U\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`@Q\x80a\x0B{3\x94\x82\x91\x90` \x83\x01\x92RV[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Qa'\x10\x81R\xF3[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x1E~\x81a\x08\xB9V[`\0\x903\x82R`\x02` R`\xFF`@\x83 T\x16\x15a\x08\xA8W`\x01`\x01`\xA0\x1B\x03\x16\x80\x82R`\x01` R`\xFF`@\x83 T\x16\x15a\x0C\xFEW\x80\x82R`\x01` R`@\x82 \x80T`\xFF\x19\x16\x90U\x7Fi\xDF,^\xC2\xEAM\x1F\xBE\x1EP5$\xF5\x93\xB3V\x16,\xA7\x10g\x12c\x82\x7F.\x19\x92\xB9Z\xE1\x82\x80\xA2\x80\xF3[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `\x01`\x01`\xA0\x1B\x03`\x0BT`\x10\x1C\x16`@Q\x90\x81R\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045a\x1F6\x81a\x08\xB9V[3`\0R`\x02` R`\xFF`@`\0 T\x16\x15a\x08\xA8W` `\x01`\x01`\xA0\x1B\x03\x7F]\xB7X\xE9\x95\xA1~\xC1\xAD\x84\xBD\xEF~\x8C2\x93\xA0\xBDay\xBC\xCE@\r\xFF]L=\x87\xDBrk\x92\x16\x80`\x01`\x01`\xA0\x1B\x03\x19`\x06T\x16\x17`\x06U`@Q\x90\x81R\xA1\0[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`@a\xFF\xFF\x80a\x1F\xB4a\x03\xDCV[\x16`\0R`\n` R`\xFF\x82`\0 T\x83Q\x92\x81\x16\x83R`\x10\x1C\x16\x15\x15` \x82\x01R\xF3[4a\x03\xEDW`\x806`\x03\x19\x01\x12a\x03\xEDWa\x1F\xF1a\x03\xDCV[a\x1F\xF9a\x03\xF2V[\x90`d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03\xEDWa \x1A\x906\x90`\x04\x01a\x04\x14V[`\0\x92\x91\x92\x933\x85R`\x02` R`\xFF`@\x86 T\x16\x15a\x08\xA8W\x84\x92`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x82;\x15a \xD1W\x84\x90a \xAC`@Q\x97\x88\x96\x87\x95\x86\x94c2\xFBb\xE7`\xE2\x1B\x86Ra\xFF\xFF\x80\x92\x16`\x04\x87\x01R\x16`$\x85\x01R`D5`D\x85\x01R`\x80`d\x85\x01R`\x84\x84\x01\x91a,0V[\x03\x92Z\xF1\x80\x15a\x08\x9FWa \xBEWP\x80\xF3[\x80a \xCBa\x08\x9C\x92a\x06MV[\x80a\x05\xF2V[\x84\x80\xFD[a \xDE6a\x04BV[a!\x1F\x83a!\x06a \xFF\x89\x97\x99a\xFF\xFF\x16`\0R`\x07` R`@`\0 \x90V[\x89\x89a.?V[\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0R` R`@`\0 \x90V[T\x91\x82\x15a\"\x95W\x82a!36\x84\x84a\x12\xEAV[` \x81Q\x91\x01 \x03a\x14\xF6Wa!w\x91`\0a!k\x86a!\x06a!d\x8Aa\xFF\xFF\x16`\0R`\x07` R`@`\0 \x90V[\x8C\x8Ca.?V[Ua\x14\x0C6\x89\x89a\x12\xEAV[\x91`\xFFa!\x83\x84a63V[\x16a\x15\x14W`\xFFa!\x93\x84a63V[\x16\x15\x80\x15\x90a\"\x89W[a\x14\xF6W`!\x83Q\x10a\x14\xE4W\x7F\xC2d\xD9\x1F:\xDCU\x88%\x0E\x15Q\xF5Gu,\xA0\xCF\xA8\xF6\xB50\xD2C\xB9\xF9\xF4\xCA\xB1\x0E\xA8\xE5\x95\x83a!\xE1`-a\"z\x96\x01Q``\x1C\x91a6CV[\x81\x15a\"\x7FW[a\"\x1D\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x16a.\x90V[a\"'\x81\x83a4\x10V[\x7F\xBFU\x1E\xC98Y\xB1p\xF9\xB2\x14\x1B\xD9)\x8B\xF3\xF6C\"\xC6\xF7\xBE\xB2T:\x0C\xB6i\x83A\x18\xBF`\x01`\x01`\xA0\x1B\x03`@Q\x93\x16\x92\x80a\"ka\xFF\xFF\x8B\x16\x94\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA3`@Q\x95\x86\x95\x86a.XV[\x03\x90\xA1\0[a\xDE\xAD\x91Pa!\xE8V[P`)\x83Q\x14\x15a!\x9DV[`@Qc+\x96\xC9\x85`\xE2\x1B\x81R`\x04\x90\xFD[4a\x03\xEDW`\xE06`\x03\x19\x01\x12a\x03\xEDW`\x045a\"\xC4\x81a\x08\xB9V[`$5\x90a\"\xD1\x82a\x08\xB9V[`D5`d5\x92`\x845\x93`\xFF\x85\x16\x85\x03a\x03\xEDWa$\t` \x91a\"\xF8B\x82\x10\x15a)\xA7V[a#\xD0a#\xDCa#\x06a*KV[\x92\x88a#%\x81`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x11` R`@`\0 \x90V[\x80T\x90`\x01\x82\x01\x90Ua#\x93`@Q\x93\x84\x92\x8C\x8C\x8C\x86\x01\x96\x87\x91\x95\x94\x93\x90\x92`\xA0\x93`\xC0\x84\x01\x97\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x85R`\x01`\x01`\xA0\x1B\x03\x80\x92\x16` \x86\x01R\x16`@\x84\x01R``\x83\x01R`\x80\x82\x01R\x01RV[\x03\x91a#\xA7`\x1F\x19\x93\x84\x81\x01\x83R\x82a\x06\x9EV[Q\x90 `@Q\x93\x84\x91\x88\x83\x01\x96\x87\x90\x91`B\x92a\x19\x01`\xF0\x1B\x83R`\x02\x83\x01R`\"\x82\x01R\x01\x90V[\x03\x90\x81\x01\x83R\x82a\x06\x9EV[Q\x90 `@\x80Q\x91\x82R`\xFF\x90\x97\x16` \x82\x01R`\xA45\x96\x81\x01\x96\x90\x96R`\xC45``\x87\x01R`\x80\x86\x01\x90V[\x85`\0\x96\x87\x92\x83\x80R\x03\x90`\x01Z\xFA\x15a\x08\x9FW\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90\x84Q\x90\x83a$\x82\x82a\x0B\xDB`\x01`\x01`\xA0\x1B\x03\x95a$i\x87\x82\x16\x80\x15\x15\x90\x81a$\x95W[Pa)\xFFV[`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x10` R`@`\0 \x90V[U`@Q\x93\x84R\x81\x16\x93\x16\x91` \x90\xA3\x80\xF3[\x90P\x88\x8C\x16\x148a$cV[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` a\xFF\xFF`\x0BT\x16`@Q\x90\x81R\xF3[4a\x03\xEDW`@6`\x03\x19\x01\x12a\x03\xEDW` a\x13\xDC`\x045a$\xE5\x81a\x08\xB9V[`\x01`\x01`\xA0\x1B\x03`$5\x91a$\xFA\x83a\x08\xB9V[\x16`\0R`\x10\x83R`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[4a\x03\xEDW``6`\x03\x19\x01\x12a\x03\xEDWa%8a\x03\xDCV[a%@a\x03\xF2V[`D5`\0\x923\x84R`\x02` R`@\x90`\xFF\x82\x86 T\x16\x15a%\xD8W\x82\x15a%\xC7W\x91\x7F\x9D\\|\x0B\x93M\xA8\xFE\xFA\x9Cw`\xC9\x83\x83w\x8A\x12\xDF\xBF\xC0\xC3\xB3\x10e\x18\xF4?\xB9P\x8A\xC0\x93\x91``\x93a\xFF\xFF\x80\x91\x16\x93\x84\x88R`\x04` R\x83a%\xB4\x82\x85\x8B \x90a\xFF\xFF\x16`\0R` R`@`\0 \x90V[U\x82Q\x94\x85R\x16` \x84\x01R\x82\x01R\xA1\x80\xF3[\x81Qc\xE4\xAC;?`\xE0\x1B\x81R`\x04\x90\xFD[\x81Qb\x82\xB4)`\xE8\x1B\x81R`\x04\x90\xFD[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Q`\x01\x81R\xF3[4a\x03\xEDW` 6`\x03\x19\x01\x12a\x03\xEDW`\x045\x80\x15\x15\x80\x91\x03a\x03\xEDW3`\0R`\x02` R`\xFF`@`\0 T\x16\x15a\x08\xA8W` \x7F\x15\x84\xADYJp\xCB\xE1\xE6QU\x92\xE1'*\x98}\x92+\t~\xAD\x87Pi\xCE\xBE\x8B@\xC0\x04\xA4\x91`\xFF\x19`\x08T\x16`\xFF\x82\x16\x17`\x08U`@Q\x90\x81R\xA1\0[4a\x03\xEDWa\x01\x006`\x03\x19\x01\x12a\x03\xEDWa&\x8Fa\x03\xDCV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90`$5\x82\x81\x11a\x03\xEDWa&\xB1\x906\x90`\x04\x01a\x04\x14V[\x91\x90`D5\x90\x84\x82\x16\x82\x03a\x03\xEDW`\x845a&\xCC\x81a\x08\xB9V[`\xC45\x95\x86\x11a\x03\xEDWa&\xE7a\x05I\x966\x90`\x04\x01a\x04\x14V[\x94\x90\x93`\xE45\x96`\xA45\x94`d5\x93a/&V[4a\x03\xEDWa'\t6a\x0F\xC4V[\x91\x90`\0\x913\x83R` `\x02\x81R`\xFF`@\x85 T\x16\x15a\x08\xA8Wa\xFF\xFF\x82\x16\x84R`\x03\x81R`@\x84 \x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x11a\x06aWa'W\x86a'Q\x84Ta\x05\xFDV[\x84a,lV[\x84\x90`\x1F\x87\x11`\x01\x14a'\xBFWP\x94a\x18<\x91\x81\x86\x97\x7F\xFAAHz\xD5\xD6r\x8F\x0B\x19'o\xA1\xED\xDC\x16U\x85x\xF5\x10\x9F\xC3\x9D-\xC3<20G\r\xAB\x97\x91a'\xB4W[P\x82`\x01\x1B\x90`\0\x19\x84`\x03\x1B\x1C\x19\x16\x17\x90U`@Q\x93\x84\x93\x84a,QV[\x90P\x85\x0158a'\x95V[\x90`\x1F\x19\x87\x16a'\xD4\x84`\0R` `\0 \x90V[\x92\x87\x90[\x82\x82\x10a(;WPP\x91a\x18<\x93\x91\x88\x7F\xFAAHz\xD5\xD6r\x8F\x0B\x19'o\xA1\xED\xDC\x16U\x85x\xF5\x10\x9F\xC3\x9D-\xC3<20G\r\xAB\x98\x99\x94\x10a(!W[PP`\x01\x82\x81\x1B\x01\x90Ua\x1C\xC2V[\x86\x015`\0\x19`\x03\x85\x90\x1B`\xF8\x16\x1C\x19\x16\x90U8\x80a(\x12V[\x80`\x01\x85\x96\x82\x94\x96\x8B\x015\x81U\x01\x95\x01\x93\x01\x90a'\xD8V[4a\x03\xEDW`@6`\x03\x19\x01\x12a\x03\xEDW` a\x0Eha(qa\x03\xDCV[`$5\x90a.\xB9V[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `\xFF`\x08T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xEDW`\x806`\x03\x19\x01\x12a\x03\xEDWa(\xB6a\x03\xDCV[a(\xBEa\x03\xF2V[\x90a(\xCA`D5a\x08\xB9V[`@Qc={/o`\xE2\x1B\x81Ra\xFF\xFF\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01R0`D\x82\x01R`d\x805\x90\x82\x01R`\0\x81`\x84\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x08\x9FWa\x07u\x91`\0\x91a)HW[P`@Q\x91\x82\x91\x82a\x07\x08V[a)c\x91=\x80\x91\x83>a)[\x81\x83a\x06\x9EV[\x81\x01\x90a+\xD1V[8a);V[4a\x03\xEDW`\x006`\x03\x19\x01\x12a\x03\xEDW` `@Q0\x81R\xF3[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x0B\xFBWV[\x15a)\xAEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`@Q=`\0\x82>=\x90\xFD[\x15a*\x06WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FINVALID_SIGNER\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`\0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a*\x99WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q`\x0CT\x91\x90\x81\x81a*\xAC\x85a\x05\xFDV[\x91\x82\x82R` \x95\x86\x83\x01\x95`\x01\x91\x88\x83\x82\x16\x91\x82`\0\x14a+\xB1WPP`\x01\x14a+WW[PPa*\xDF\x92P\x03\x82a\x06\x9EV[Q\x90 \x90`@Q\x90\x81\x01\x91\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R`@\x82\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra+Q\x81a\x06\x82V[Q\x90 \x90V[\x90\x87\x92P`\x0C\x82R\x7F\xDFif\xC9q\x05\x1C=T\xECY\x16&\x06S\x14\x93\xA5\x14\x04\xA0\x02\x84/V\0\x9D~\\\xF4\xA8\xC7[\x85\x83\x10a+\x99WPPa*\xDF\x93P\x82\x01\x018\x80a*\xD1V[\x80T\x83\x88\x01\x85\x01R\x86\x94P\x88\x93\x90\x92\x01\x91\x81\x01a+\x81V[\x92P\x93PPa*\xDF\x94\x91P`\xFF\x19\x16\x86R\x15\x15`\x05\x1B\x82\x01\x018\x80a*\xD1V[` \x81\x83\x03\x12a\x03\xEDW\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03\xEDW\x01\x81`\x1F\x82\x01\x12\x15a\x03\xEDW\x80Qa,\x04\x81a\x12\xCEV[\x92a,\x12`@Q\x94\x85a\x06\x9EV[\x81\x84R` \x82\x84\x01\x01\x11a\x03\xEDWa\x07\x19\x91` \x80\x85\x01\x91\x01a\x06\xC0V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90a\xFF\xFFa\x07\x19\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a,0V[\x90`\x1F\x81\x11a,zWPPPV[`\0\x91\x82R` \x82 \x90` `\x1F\x85\x01`\x05\x1C\x83\x01\x94\x10a,\xB6W[`\x1F\x01`\x05\x1C\x01\x91[\x82\x81\x10a,\xABWPPPV[\x81\x81U`\x01\x01a,\x9FV[\x90\x92P\x82\x90a,\x96V[\x92\x90\x91Z`@Qc3V\xAEE`\xE1\x1B` \x82\x01\x90\x81Ra\xFF\xFF\x87\x16`$\x83\x01R`\x80`D\x83\x01R\x94\x91a--\x82a-\x1Fa,\xFD`\xA4\x83\x01\x87a\x06\xE3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16`d\x84\x01R\x82\x81\x03`#\x19\x01`\x84\x84\x01R\x88a\x06\xE3V[\x03`\x1F\x19\x81\x01\x84R\x83a\x06\x9EV[`\0\x80\x91`@Q\x97a->\x89a\x06\x82V[`\x96\x89R\x82` \x8A\x01\x95`\xA06\x887Q\x920\x90\xF1\x90=\x90`\x96\x82\x11a-\x85W[`\0\x90\x82\x88R>\x15a-rW[PPPPPV[a-{\x94a-\x8EV[8\x80\x80\x80\x80a-kV[`\x96\x91Pa-^V[\x91\x93a.,\x7F\xE1\x83\xF3=\xE2\x83w\x95R[G\x92\xCAL\xD6\x055\xBDw\xC5;~p0\x06\x0B\xFC\xF5sMk\x0C\x95a.:\x93\x95a\xFF\xFF\x81Q` \x83\x01 \x96\x16\x95\x86`\0R`\x07` Ra-\xF2\x83a\x13\xBA` \x8B`@`\0 \x82`@Q\x94\x83\x86\x80\x95Q\x93\x84\x92\x01a\x06\xC0V[Ug\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa.\x18`@Q\x98\x89\x98\x89R`\xA0` \x8A\x01R`\xA0\x89\x01\x90a\x06\xE3V[\x92\x16`@\x87\x01R\x85\x82\x03``\x87\x01Ra\x06\xE3V[\x90\x83\x82\x03`\x80\x85\x01Ra\x06\xE3V[\x03\x90\xA1V[` \x91\x92\x83`@Q\x94\x85\x93\x847\x82\x01\x90\x81R\x03\x01\x90 \x90V[\x91a.\x85\x90``\x94a\xFF\xFFg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x99\x98\x97\x99\x16\x85R`\x80` \x86\x01R`\x80\x85\x01\x91a,0V[\x95\x16`@\x82\x01R\x01RV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x0B\xFBWV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[a\xFF\xFF\x80\x91\x16`\0R`\n` R`@`\0 `\xFF`@Q\x91a.\xDB\x83a\x06fV[T\x83\x81\x16\x83R`\x10\x1C\x16\x15\x80\x15` \x83\x01Ra/\x06W\x91a/\x02\x91a'\x10\x93Q\x16\x90a.\x90V[\x04\x90V[P`\x0BT\x16\x90\x81\x15a/\x1FWa'\x10\x91a/\x02\x91a.\x90V[PP`\0\x90V[\x93\x90\x96\x97\x98\x95\x94\x91\x9403\x03a0\x03Wa\xFF\xFFa/L`\x01`\x01`\xA0\x1B\x03\x92\x850a4kV[\x95\x16\x92\x16\x92\x83\x83\x7F\xBFU\x1E\xC98Y\xB1p\xF9\xB2\x14\x1B\xD9)\x8B\xF3\xF6C\"\xC6\xF7\xBE\xB2T:\x0C\xB6i\x83A\x18\xBF` `@Q\x89\x81R\xA3\x83;\x15a\x03\xEDWa/\xC4\x99`\0\x99\x8A\x96a/\xE6\x94g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Q\x9E\x8F\x9D\x8E\x9C\x8D\x9Ac?\xE7\x9A\xED`\xE1\x1B\x8CR`\x04\x8C\x01R`\xC0`$\x8C\x01R`\xC4\x8B\x01\x91a,0V[\x95\x16`D\x88\x01R`d\x87\x01R`\x84\x86\x01R\x84\x83\x03`\x03\x19\x01`\xA4\x86\x01Ra,0V[\x03\x93\xF1\x80\x15a\x08\x9FWa/\xF6WPV[\x80a \xCBa\x16\xF2\x92a\x06MV[`@Qc \xAAl#`\xE2\x1B\x81R`\x04\x90\xFD[\x91\x90\x82`@\x91\x03\x12a\x03\xEDW` \x82Q\x92\x01Q\x90V[\x92\x96\x95\x96\x94\x91\x94\x93\x90\x93`\xFF`\x08T\x16`\0\x14a1KW`\"\x88Q\x10a19W`\"\x88\x01Qa\xFF\xFF\x86\x16`\0R`\x04` Ra0t`@`\0 `\0\x80R` R`@`\0 \x90V[T\x90\x81\x15a1'W\x10a1\x15Wa0\x8Da0\x94\x91a3\x15V[P\x84a3\x94V[\x96\x87\x15a1\x03Wa0\xBA\x92a0\xB1a0\xAB\x8Aa2\xC0V[\x88a3LV[\x924\x93\x87a1\xC6V[\x7F\xD8\x1F\xC9\xB8R14\xEDa8p\xED\x02\x9Dap\xCB\xB7:\xA6\xA6\xBC1\x1B\x9Ad&\x89\xFB\x9D\xF5\x9Aa\xFF\xFF`\x01`\x01`\xA0\x1B\x03`@Q\x93\x16\x93\x16\x91\x80a0\xFE\x88\x82\x91\x90` \x83\x01\x92RV[\x03\x90\xA4V[`@Qcaz\xB1-`\xE1\x1B\x81R`\x04\x90\xFD[`@Qcv\xA1\xE1\xD3`\xE1\x1B\x81R`\x04\x90\xFD[`@Qc\x1F>\xC9\xD5`\xE1\x1B\x81R`\x04\x90\xFD[`@Qc\xCE\xF8\x0E\xA3`\xE0\x1B\x81R`\x04\x90\xFD[\x87Qa1]Wa0\x8Da0\x94\x91a3\x15V[`@Qc\x8F\xAD\xCA\xDB`\xE0\x1B\x81R`\x04\x90\xFD[\x92a1\x94a\x07\x19\x97\x95\x93a\xFF\xFFa1\xA2\x94\x16\x86R`\xC0` \x87\x01R`\xC0\x86\x01\x90a\x06\xE3V[\x90\x84\x82\x03`@\x86\x01Ra\x06\xE3V[\x93`\x01`\x01`\xA0\x1B\x03\x80\x92\x16``\x84\x01R\x16`\x80\x82\x01R`\xA0\x81\x84\x03\x91\x01Ra\x06\xE3V[\x94a\x1B\x0F\x91\x93\x92\x95a1\xF5a1\xE9\x82a\xFF\xFF\x16`\0R`\x03` R`@`\0 \x90V[`@Q\x94\x85\x80\x92a\x16AV[\x82Q\x15a2\xA0W\x84Qa\xFF\xFF\x82\x16`\0R`\x05` R`@`\0 T\x90\x81\x15a2\x96W[\x11a2\x84W`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x84;\x15a\x03\xEDW`\0\x96a2s\x91`@Q\x99\x8A\x98\x89\x97\x88\x96b\xC5\x801`\xE8\x1B\x88R`\x04\x88\x01a1oV[\x03\x92Z\xF1\x80\x15a\x08\x9FWa/\xF6WPV[`@Qc\"\x0B\t3`\xE1\x1B\x81R`\x04\x90\xFD[a'\x10\x91Pa2\x19V[`@Qc&\xBA|\xFB`\xE0\x1B\x81R`\x04\x90\xFD[\x90`\x1F\x82\x01\x80\x92\x11a\x0B\xFBWV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x81\x15a3\x10W\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x81\x11a2\xFEW\x16\x90V[`@Qc1$\x99\x8D`\xE1\x1B\x81R`\x04\x90\xFD[a.\xA3V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x15a3\x10W\x81\x06\x90\x81\x81\x03\x90\x81\x11a\x0B\xFBW\x91V[\x90`@Q\x91`\0` \x84\x01R`!\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xC0\x1B\x90`\xC0\x1B\x16`A\x82\x01R`)\x81R``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x06aW`@R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x823\x83\x03a3\xFEW[PP\x80`\0R`\x0F` R`@`\0 \x90\x81T\x83\x81\x03\x90\x81\x11a\x0B\xFBW`\0\x92U\x82`\x0ET\x03`\x0EU\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF` `@Q\x85\x81R\xA3\x90V[a4\t\x913\x90a5wV[8\x82a3\xA8V[`\x0ET\x82\x81\x01\x80\x91\x11a\x0B\xFBW` `\x01`\x01`\xA0\x1B\x03`\0\x93\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x93`\x0EU\x16\x93\x84\x84R`\x0F\x82R`@\x84 \x81\x81T\x01\x90U`@Q\x90\x81R\xA3V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x81\x16\x910\x83\x14\x15\x80a5mW[a5]W[\x82\x15\x80\x15a5SW[a5AW\x84a4\xB2\x83`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0F` R`@`\0 \x90V[T\x10a5/Wa4\xF6\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0F` R`@`\0 \x90V[\x85\x81T\x03\x90Ua5\x19\x84`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x0F` R`@`\0 \x90V[\x80T\x86\x01\x90U`@Q\x85\x81R\x93\x16\x92` \x90\xA3\x90V[`@Qc\x1E\x9A\xCF\x17`\xE3\x1B\x81R`\x04\x90\xFD[`@Qc\xD9.#=`\xE0\x1B\x81R`\x04\x90\xFD[P\x80\x84\x16\x15a4\x90V[a5h\x853\x84a5wV[a4\x87V[P3\x83\x14\x15a4\x82V[\x91\x90`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x81`\0R`\x10` Ra5\xAF\x83`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T\x91`\x01\x83\x01a5\xC2W[PPPPPPV[\x84\x83\x10a6\x11W\x15\x90\x81\x15a6\x06W[Pa5AWa5\xFA\x92a\x0B\xDB\x91\x03\x93`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x10` R`@`\0 \x90V[U8\x80\x80\x80\x80\x80a5\xBAV[\x90P\x82\x16\x158a5\xD2V[`@Qc\x13\xBE%+`\xE0\x1B\x81R`\x04\x90\xFD[`\xFF\x16\x80\x15a\x0B\xFBW`\0\x19\x01\x90V[`\x01\x81Q\x10a\x14\xE4W`\x01\x01Q\x90V[`)\x81Q\x10a\x14\xE4W`)\x01Q\x90V\xFE\xA2dipfsX\"\x12 c\xCB\xF8\xA8\0\x8Cd\x9F6MP\xD5\xBF\xD9,\x86Cw\xBB\xB4I\xDA\x95\xC2\xC2W\xE9\x83\x08r\xDF|dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static OFTWITHFEE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct OFTWithFee<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OFTWithFee<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OFTWithFee<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OFTWithFee<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OFTWithFee<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OFTWithFee)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OFTWithFee<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OFTWITHFEE_ABI.clone(),
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
                OFTWITHFEE_ABI.clone(),
                OFTWITHFEE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
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
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
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
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
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
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
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
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
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
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
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
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
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
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
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
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OFTWithFeeEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for OFTWithFee<M> {
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
    ///Custom Error type `InsufficientAllowance` with signature `InsufficientAllowance()` and selector `0x13be252b`
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
    #[etherror(name = "InsufficientAllowance", abi = "InsufficientAllowance()")]
    pub struct InsufficientAllowance;
    ///Custom Error type `InsufficientBalance` with signature `InsufficientBalance()` and selector `0xf4d678b8`
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
    #[etherror(name = "InsufficientBalance", abi = "InsufficientBalance()")]
    pub struct InsufficientBalance;
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
    ///Custom Error type `SharedDecimalsTooLarge` with signature `SharedDecimalsTooLarge()` and selector `0x4712720d`
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
    #[etherror(name = "SharedDecimalsTooLarge", abi = "SharedDecimalsTooLarge()")]
    pub struct SharedDecimalsTooLarge;
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
    pub enum OFTWithFeeErrors {
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
        InsufficientAllowance(InsufficientAllowance),
        InsufficientBalance(InsufficientBalance),
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
        SharedDecimalsTooLarge(SharedDecimalsTooLarge),
        SliceOverflow(SliceOverflow),
        Unauthorized(Unauthorized),
        UnknownPacketType(UnknownPacketType),
        ZeroAddress(ZeroAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for OFTWithFeeErrors {
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
                = <InsufficientAllowance as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InsufficientAllowance(decoded));
            }
            if let Ok(decoded)
                = <InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InsufficientBalance(decoded));
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
                = <SharedDecimalsTooLarge as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SharedDecimalsTooLarge(decoded));
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
            if let Ok(decoded)
                = <ZeroAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OFTWithFeeErrors {
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
                Self::InsufficientAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientBalance(element) => {
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
                Self::SharedDecimalsTooLarge(element) => {
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
                Self::ZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for OFTWithFeeErrors {
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
                    == <InsufficientAllowance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientBalance as ::ethers::contract::EthError>::selector() => {
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
                    == <SharedDecimalsTooLarge as ::ethers::contract::EthError>::selector() => {
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
                _ if selector
                    == <ZeroAddress as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for OFTWithFeeErrors {
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
                Self::InsufficientAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::SharedDecimalsTooLarge(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SliceOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnknownPacketType(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for OFTWithFeeErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AdapterParamsMustBeEmpty> for OFTWithFeeErrors {
        fn from(value: AdapterParamsMustBeEmpty) -> Self {
            Self::AdapterParamsMustBeEmpty(value)
        }
    }
    impl ::core::convert::From<AlreadySet> for OFTWithFeeErrors {
        fn from(value: AlreadySet) -> Self {
            Self::AlreadySet(value)
        }
    }
    impl ::core::convert::From<AmountLessThanMinAmount> for OFTWithFeeErrors {
        fn from(value: AmountLessThanMinAmount) -> Self {
            Self::AmountLessThanMinAmount(value)
        }
    }
    impl ::core::convert::From<AmountSDOverflow> for OFTWithFeeErrors {
        fn from(value: AmountSDOverflow) -> Self {
            Self::AmountSDOverflow(value)
        }
    }
    impl ::core::convert::From<AmountTooSmall> for OFTWithFeeErrors {
        fn from(value: AmountTooSmall) -> Self {
            Self::AmountTooSmall(value)
        }
    }
    impl ::core::convert::From<CallerMustBeLzApp> for OFTWithFeeErrors {
        fn from(value: CallerMustBeLzApp) -> Self {
            Self::CallerMustBeLzApp(value)
        }
    }
    impl ::core::convert::From<CallerMustBeOFTCore> for OFTWithFeeErrors {
        fn from(value: CallerMustBeOFTCore) -> Self {
            Self::CallerMustBeOFTCore(value)
        }
    }
    impl ::core::convert::From<DestinationChainNotTrusted> for OFTWithFeeErrors {
        fn from(value: DestinationChainNotTrusted) -> Self {
            Self::DestinationChainNotTrusted(value)
        }
    }
    impl ::core::convert::From<FeeBpTooLarge> for OFTWithFeeErrors {
        fn from(value: FeeBpTooLarge) -> Self {
            Self::FeeBpTooLarge(value)
        }
    }
    impl ::core::convert::From<FeeOwnerNotSet> for OFTWithFeeErrors {
        fn from(value: FeeOwnerNotSet) -> Self {
            Self::FeeOwnerNotSet(value)
        }
    }
    impl ::core::convert::From<GasLimitTooLow> for OFTWithFeeErrors {
        fn from(value: GasLimitTooLow) -> Self {
            Self::GasLimitTooLow(value)
        }
    }
    impl ::core::convert::From<InsufficientAllowance> for OFTWithFeeErrors {
        fn from(value: InsufficientAllowance) -> Self {
            Self::InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<InsufficientBalance> for OFTWithFeeErrors {
        fn from(value: InsufficientBalance) -> Self {
            Self::InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<InvalidAdapterParams> for OFTWithFeeErrors {
        fn from(value: InvalidAdapterParams) -> Self {
            Self::InvalidAdapterParams(value)
        }
    }
    impl ::core::convert::From<InvalidEndpointCaller> for OFTWithFeeErrors {
        fn from(value: InvalidEndpointCaller) -> Self {
            Self::InvalidEndpointCaller(value)
        }
    }
    impl ::core::convert::From<InvalidMinGas> for OFTWithFeeErrors {
        fn from(value: InvalidMinGas) -> Self {
            Self::InvalidMinGas(value)
        }
    }
    impl ::core::convert::From<InvalidPayload> for OFTWithFeeErrors {
        fn from(value: InvalidPayload) -> Self {
            Self::InvalidPayload(value)
        }
    }
    impl ::core::convert::From<InvalidSourceSendingContract> for OFTWithFeeErrors {
        fn from(value: InvalidSourceSendingContract) -> Self {
            Self::InvalidSourceSendingContract(value)
        }
    }
    impl ::core::convert::From<MinGasLimitNotSet> for OFTWithFeeErrors {
        fn from(value: MinGasLimitNotSet) -> Self {
            Self::MinGasLimitNotSet(value)
        }
    }
    impl ::core::convert::From<NoAdmin> for OFTWithFeeErrors {
        fn from(value: NoAdmin) -> Self {
            Self::NoAdmin(value)
        }
    }
    impl ::core::convert::From<NoStoredMessage> for OFTWithFeeErrors {
        fn from(value: NoStoredMessage) -> Self {
            Self::NoStoredMessage(value)
        }
    }
    impl ::core::convert::From<NoTrustedPath> for OFTWithFeeErrors {
        fn from(value: NoTrustedPath) -> Self {
            Self::NoTrustedPath(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for OFTWithFeeErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    impl ::core::convert::From<PayloadSizeTooLarge> for OFTWithFeeErrors {
        fn from(value: PayloadSizeTooLarge) -> Self {
            Self::PayloadSizeTooLarge(value)
        }
    }
    impl ::core::convert::From<SharedDecimalsTooLarge> for OFTWithFeeErrors {
        fn from(value: SharedDecimalsTooLarge) -> Self {
            Self::SharedDecimalsTooLarge(value)
        }
    }
    impl ::core::convert::From<SliceOverflow> for OFTWithFeeErrors {
        fn from(value: SliceOverflow) -> Self {
            Self::SliceOverflow(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for OFTWithFeeErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
        }
    }
    impl ::core::convert::From<UnknownPacketType> for OFTWithFeeErrors {
        fn from(value: UnknownPacketType) -> Self {
            Self::UnknownPacketType(value)
        }
    }
    impl ::core::convert::From<ZeroAddress> for OFTWithFeeErrors {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OFTWithFeeEvents {
        AdminAddedFilter(AdminAddedFilter),
        AdminDeletedFilter(AdminDeletedFilter),
        ApprovalFilter(ApprovalFilter),
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
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for OFTWithFeeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminAddedFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::AdminAddedFilter(decoded));
            }
            if let Ok(decoded) = AdminDeletedFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::AdminDeletedFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = CallOFTReceivedSuccessFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::CallOFTReceivedSuccessFilter(decoded));
            }
            if let Ok(decoded) = MessageFailedFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::MessageFailedFilter(decoded));
            }
            if let Ok(decoded) = NonContractAddressFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::NonContractAddressFilter(decoded));
            }
            if let Ok(decoded) = OperatorAddedFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::OperatorAddedFilter(decoded));
            }
            if let Ok(decoded) = OperatorDeletedFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::OperatorDeletedFilter(decoded));
            }
            if let Ok(decoded) = ReceiveFromChainFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::ReceiveFromChainFilter(decoded));
            }
            if let Ok(decoded) = RetryMessageSuccessFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::RetryMessageSuccessFilter(decoded));
            }
            if let Ok(decoded) = SendToChainFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::SendToChainFilter(decoded));
            }
            if let Ok(decoded) = SetDefaultFeeBpFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::SetDefaultFeeBpFilter(decoded));
            }
            if let Ok(decoded) = SetFeeBpFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::SetFeeBpFilter(decoded));
            }
            if let Ok(decoded) = SetFeeOwnerFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::SetFeeOwnerFilter(decoded));
            }
            if let Ok(decoded) = SetMinDstGasFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::SetMinDstGasFilter(decoded));
            }
            if let Ok(decoded) = SetPrecrimeFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::SetPrecrimeFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedRemoteFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::SetTrustedRemoteFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedRemoteAddressFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::SetTrustedRemoteAddressFilter(decoded));
            }
            if let Ok(decoded) = SetUseCustomAdapterParamsFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::SetUseCustomAdapterParamsFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(OFTWithFeeEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for OFTWithFeeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminDeletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminAddedFilter> for OFTWithFeeEvents {
        fn from(value: AdminAddedFilter) -> Self {
            Self::AdminAddedFilter(value)
        }
    }
    impl ::core::convert::From<AdminDeletedFilter> for OFTWithFeeEvents {
        fn from(value: AdminDeletedFilter) -> Self {
            Self::AdminDeletedFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalFilter> for OFTWithFeeEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<CallOFTReceivedSuccessFilter> for OFTWithFeeEvents {
        fn from(value: CallOFTReceivedSuccessFilter) -> Self {
            Self::CallOFTReceivedSuccessFilter(value)
        }
    }
    impl ::core::convert::From<MessageFailedFilter> for OFTWithFeeEvents {
        fn from(value: MessageFailedFilter) -> Self {
            Self::MessageFailedFilter(value)
        }
    }
    impl ::core::convert::From<NonContractAddressFilter> for OFTWithFeeEvents {
        fn from(value: NonContractAddressFilter) -> Self {
            Self::NonContractAddressFilter(value)
        }
    }
    impl ::core::convert::From<OperatorAddedFilter> for OFTWithFeeEvents {
        fn from(value: OperatorAddedFilter) -> Self {
            Self::OperatorAddedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorDeletedFilter> for OFTWithFeeEvents {
        fn from(value: OperatorDeletedFilter) -> Self {
            Self::OperatorDeletedFilter(value)
        }
    }
    impl ::core::convert::From<ReceiveFromChainFilter> for OFTWithFeeEvents {
        fn from(value: ReceiveFromChainFilter) -> Self {
            Self::ReceiveFromChainFilter(value)
        }
    }
    impl ::core::convert::From<RetryMessageSuccessFilter> for OFTWithFeeEvents {
        fn from(value: RetryMessageSuccessFilter) -> Self {
            Self::RetryMessageSuccessFilter(value)
        }
    }
    impl ::core::convert::From<SendToChainFilter> for OFTWithFeeEvents {
        fn from(value: SendToChainFilter) -> Self {
            Self::SendToChainFilter(value)
        }
    }
    impl ::core::convert::From<SetDefaultFeeBpFilter> for OFTWithFeeEvents {
        fn from(value: SetDefaultFeeBpFilter) -> Self {
            Self::SetDefaultFeeBpFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeBpFilter> for OFTWithFeeEvents {
        fn from(value: SetFeeBpFilter) -> Self {
            Self::SetFeeBpFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeOwnerFilter> for OFTWithFeeEvents {
        fn from(value: SetFeeOwnerFilter) -> Self {
            Self::SetFeeOwnerFilter(value)
        }
    }
    impl ::core::convert::From<SetMinDstGasFilter> for OFTWithFeeEvents {
        fn from(value: SetMinDstGasFilter) -> Self {
            Self::SetMinDstGasFilter(value)
        }
    }
    impl ::core::convert::From<SetPrecrimeFilter> for OFTWithFeeEvents {
        fn from(value: SetPrecrimeFilter) -> Self {
            Self::SetPrecrimeFilter(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteFilter> for OFTWithFeeEvents {
        fn from(value: SetTrustedRemoteFilter) -> Self {
            Self::SetTrustedRemoteFilter(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteAddressFilter> for OFTWithFeeEvents {
        fn from(value: SetTrustedRemoteAddressFilter) -> Self {
            Self::SetTrustedRemoteAddressFilter(value)
        }
    }
    impl ::core::convert::From<SetUseCustomAdapterParamsFilter> for OFTWithFeeEvents {
        fn from(value: SetUseCustomAdapterParamsFilter) -> Self {
            Self::SetUseCustomAdapterParamsFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for OFTWithFeeEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
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
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
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
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
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
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
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
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
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
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
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
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
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
    pub enum OFTWithFeeCalls {
        BpDenominator(BpDenominatorCall),
        DefaultPayloadSizeLimit(DefaultPayloadSizeLimitCall),
        DomainSeparator(DomainSeparatorCall),
        NoExtraGas(NoExtraGasCall),
        PtSend(PtSendCall),
        PtSendAndCall(PtSendAndCallCall),
        AddAdmin(AddAdminCall),
        AddOperator(AddOperatorCall),
        Admins(AdminsCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        CallOnOFTReceived(CallOnOFTReceivedCall),
        ChainIdToFeeBps(ChainIdToFeeBpsCall),
        CirculatingSupply(CirculatingSupplyCall),
        CreditedPackets(CreditedPacketsCall),
        Decimals(DecimalsCall),
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
        Name(NameCall),
        NonblockingLzReceive(NonblockingLzReceiveCall),
        Nonces(NoncesCall),
        Operators(OperatorsCall),
        PayloadSizeLimitLookup(PayloadSizeLimitLookupCall),
        Permit(PermitCall),
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
        Symbol(SymbolCall),
        Token(TokenCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TrustedRemoteLookup(TrustedRemoteLookupCall),
        UseCustomAdapterParams(UseCustomAdapterParamsCall),
    }
    impl ::ethers::core::abi::AbiDecode for OFTWithFeeCalls {
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
                = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DomainSeparator(decoded));
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
                = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
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
                = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
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
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <NonblockingLzReceiveCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NonblockingLzReceive(decoded));
            }
            if let Ok(decoded)
                = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
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
                = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permit(decoded));
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
                = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded)
                = <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
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
    impl ::ethers::core::abi::AbiEncode for OFTWithFeeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BpDenominator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultPayloadSizeLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
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
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::Decimals(element) => {
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
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NonblockingLzReceive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Operators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PayloadSizeLimitLookup(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TrustedRemoteLookup(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UseCustomAdapterParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for OFTWithFeeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BpDenominator(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultPayloadSizeLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoExtraGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::PtSend(element) => ::core::fmt::Display::fmt(element, f),
                Self::PtSendAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admins(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallOnOFTReceived(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainIdToFeeBps(element) => ::core::fmt::Display::fmt(element, f),
                Self::CirculatingSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreditedPackets(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::NonblockingLzReceive(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Operators(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayloadSizeLimitLookup(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrustedRemoteLookup(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UseCustomAdapterParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BpDenominatorCall> for OFTWithFeeCalls {
        fn from(value: BpDenominatorCall) -> Self {
            Self::BpDenominator(value)
        }
    }
    impl ::core::convert::From<DefaultPayloadSizeLimitCall> for OFTWithFeeCalls {
        fn from(value: DefaultPayloadSizeLimitCall) -> Self {
            Self::DefaultPayloadSizeLimit(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for OFTWithFeeCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<NoExtraGasCall> for OFTWithFeeCalls {
        fn from(value: NoExtraGasCall) -> Self {
            Self::NoExtraGas(value)
        }
    }
    impl ::core::convert::From<PtSendCall> for OFTWithFeeCalls {
        fn from(value: PtSendCall) -> Self {
            Self::PtSend(value)
        }
    }
    impl ::core::convert::From<PtSendAndCallCall> for OFTWithFeeCalls {
        fn from(value: PtSendAndCallCall) -> Self {
            Self::PtSendAndCall(value)
        }
    }
    impl ::core::convert::From<AddAdminCall> for OFTWithFeeCalls {
        fn from(value: AddAdminCall) -> Self {
            Self::AddAdmin(value)
        }
    }
    impl ::core::convert::From<AddOperatorCall> for OFTWithFeeCalls {
        fn from(value: AddOperatorCall) -> Self {
            Self::AddOperator(value)
        }
    }
    impl ::core::convert::From<AdminsCall> for OFTWithFeeCalls {
        fn from(value: AdminsCall) -> Self {
            Self::Admins(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for OFTWithFeeCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for OFTWithFeeCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for OFTWithFeeCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<CallOnOFTReceivedCall> for OFTWithFeeCalls {
        fn from(value: CallOnOFTReceivedCall) -> Self {
            Self::CallOnOFTReceived(value)
        }
    }
    impl ::core::convert::From<ChainIdToFeeBpsCall> for OFTWithFeeCalls {
        fn from(value: ChainIdToFeeBpsCall) -> Self {
            Self::ChainIdToFeeBps(value)
        }
    }
    impl ::core::convert::From<CirculatingSupplyCall> for OFTWithFeeCalls {
        fn from(value: CirculatingSupplyCall) -> Self {
            Self::CirculatingSupply(value)
        }
    }
    impl ::core::convert::From<CreditedPacketsCall> for OFTWithFeeCalls {
        fn from(value: CreditedPacketsCall) -> Self {
            Self::CreditedPackets(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for OFTWithFeeCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DefaultFeeBpCall> for OFTWithFeeCalls {
        fn from(value: DefaultFeeBpCall) -> Self {
            Self::DefaultFeeBp(value)
        }
    }
    impl ::core::convert::From<DeleteAdminCall> for OFTWithFeeCalls {
        fn from(value: DeleteAdminCall) -> Self {
            Self::DeleteAdmin(value)
        }
    }
    impl ::core::convert::From<DeleteOperatorCall> for OFTWithFeeCalls {
        fn from(value: DeleteOperatorCall) -> Self {
            Self::DeleteOperator(value)
        }
    }
    impl ::core::convert::From<EstimateSendFeeCall> for OFTWithFeeCalls {
        fn from(value: EstimateSendFeeCall) -> Self {
            Self::EstimateSendFee(value)
        }
    }
    impl ::core::convert::From<FailedMessagesCall> for OFTWithFeeCalls {
        fn from(value: FailedMessagesCall) -> Self {
            Self::FailedMessages(value)
        }
    }
    impl ::core::convert::From<FeeOwnerCall> for OFTWithFeeCalls {
        fn from(value: FeeOwnerCall) -> Self {
            Self::FeeOwner(value)
        }
    }
    impl ::core::convert::From<ForceResumeReceiveCall> for OFTWithFeeCalls {
        fn from(value: ForceResumeReceiveCall) -> Self {
            Self::ForceResumeReceive(value)
        }
    }
    impl ::core::convert::From<GetConfigCall> for OFTWithFeeCalls {
        fn from(value: GetConfigCall) -> Self {
            Self::GetConfig(value)
        }
    }
    impl ::core::convert::From<GetTrustedRemoteAddressCall> for OFTWithFeeCalls {
        fn from(value: GetTrustedRemoteAddressCall) -> Self {
            Self::GetTrustedRemoteAddress(value)
        }
    }
    impl ::core::convert::From<IsTrustedRemoteCall> for OFTWithFeeCalls {
        fn from(value: IsTrustedRemoteCall) -> Self {
            Self::IsTrustedRemote(value)
        }
    }
    impl ::core::convert::From<LzEndpointCall> for OFTWithFeeCalls {
        fn from(value: LzEndpointCall) -> Self {
            Self::LzEndpoint(value)
        }
    }
    impl ::core::convert::From<LzReceiveCall> for OFTWithFeeCalls {
        fn from(value: LzReceiveCall) -> Self {
            Self::LzReceive(value)
        }
    }
    impl ::core::convert::From<MinDstGasLookupCall> for OFTWithFeeCalls {
        fn from(value: MinDstGasLookupCall) -> Self {
            Self::MinDstGasLookup(value)
        }
    }
    impl ::core::convert::From<NameCall> for OFTWithFeeCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NonblockingLzReceiveCall> for OFTWithFeeCalls {
        fn from(value: NonblockingLzReceiveCall) -> Self {
            Self::NonblockingLzReceive(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for OFTWithFeeCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<OperatorsCall> for OFTWithFeeCalls {
        fn from(value: OperatorsCall) -> Self {
            Self::Operators(value)
        }
    }
    impl ::core::convert::From<PayloadSizeLimitLookupCall> for OFTWithFeeCalls {
        fn from(value: PayloadSizeLimitLookupCall) -> Self {
            Self::PayloadSizeLimitLookup(value)
        }
    }
    impl ::core::convert::From<PermitCall> for OFTWithFeeCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<PrecrimeCall> for OFTWithFeeCalls {
        fn from(value: PrecrimeCall) -> Self {
            Self::Precrime(value)
        }
    }
    impl ::core::convert::From<QuoteOFTFeeCall> for OFTWithFeeCalls {
        fn from(value: QuoteOFTFeeCall) -> Self {
            Self::QuoteOFTFee(value)
        }
    }
    impl ::core::convert::From<RetryMessageCall> for OFTWithFeeCalls {
        fn from(value: RetryMessageCall) -> Self {
            Self::RetryMessage(value)
        }
    }
    impl ::core::convert::From<SendFromCall> for OFTWithFeeCalls {
        fn from(value: SendFromCall) -> Self {
            Self::SendFrom(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for OFTWithFeeCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetDefaultFeeBpCall> for OFTWithFeeCalls {
        fn from(value: SetDefaultFeeBpCall) -> Self {
            Self::SetDefaultFeeBp(value)
        }
    }
    impl ::core::convert::From<SetFeeBpCall> for OFTWithFeeCalls {
        fn from(value: SetFeeBpCall) -> Self {
            Self::SetFeeBp(value)
        }
    }
    impl ::core::convert::From<SetFeeOwnerCall> for OFTWithFeeCalls {
        fn from(value: SetFeeOwnerCall) -> Self {
            Self::SetFeeOwner(value)
        }
    }
    impl ::core::convert::From<SetMinDstGasCall> for OFTWithFeeCalls {
        fn from(value: SetMinDstGasCall) -> Self {
            Self::SetMinDstGas(value)
        }
    }
    impl ::core::convert::From<SetPayloadSizeLimitCall> for OFTWithFeeCalls {
        fn from(value: SetPayloadSizeLimitCall) -> Self {
            Self::SetPayloadSizeLimit(value)
        }
    }
    impl ::core::convert::From<SetPrecrimeCall> for OFTWithFeeCalls {
        fn from(value: SetPrecrimeCall) -> Self {
            Self::SetPrecrime(value)
        }
    }
    impl ::core::convert::From<SetReceiveVersionCall> for OFTWithFeeCalls {
        fn from(value: SetReceiveVersionCall) -> Self {
            Self::SetReceiveVersion(value)
        }
    }
    impl ::core::convert::From<SetSendVersionCall> for OFTWithFeeCalls {
        fn from(value: SetSendVersionCall) -> Self {
            Self::SetSendVersion(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteCall> for OFTWithFeeCalls {
        fn from(value: SetTrustedRemoteCall) -> Self {
            Self::SetTrustedRemote(value)
        }
    }
    impl ::core::convert::From<SetTrustedRemoteAddressCall> for OFTWithFeeCalls {
        fn from(value: SetTrustedRemoteAddressCall) -> Self {
            Self::SetTrustedRemoteAddress(value)
        }
    }
    impl ::core::convert::From<SetUseCustomAdapterParamsCall> for OFTWithFeeCalls {
        fn from(value: SetUseCustomAdapterParamsCall) -> Self {
            Self::SetUseCustomAdapterParams(value)
        }
    }
    impl ::core::convert::From<SharedDecimalsCall> for OFTWithFeeCalls {
        fn from(value: SharedDecimalsCall) -> Self {
            Self::SharedDecimals(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for OFTWithFeeCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for OFTWithFeeCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenCall> for OFTWithFeeCalls {
        fn from(value: TokenCall) -> Self {
            Self::Token(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for OFTWithFeeCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for OFTWithFeeCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for OFTWithFeeCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TrustedRemoteLookupCall> for OFTWithFeeCalls {
        fn from(value: TrustedRemoteLookupCall) -> Self {
            Self::TrustedRemoteLookup(value)
        }
    }
    impl ::core::convert::From<UseCustomAdapterParamsCall> for OFTWithFeeCalls {
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
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
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
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
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
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
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
