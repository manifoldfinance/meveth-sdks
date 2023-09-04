pub use lz_endpoint_mock::*;
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
pub mod lz_endpoint_mock {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_chainId"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint16"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("blockNextMsg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("blockNextMsg"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("defaultAdapterParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "defaultAdapterParams",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("estimateFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("estimateFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_userApplication"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_payInZRO"),
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
                    ::std::borrow::ToOwned::to_owned("getChainId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getChainId"),
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
                    ::std::borrow::ToOwned::to_owned("getConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getConfig"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInboundNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInboundNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_chainID"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLengthOfQueue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getLengthOfQueue"),
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
                    ::std::borrow::ToOwned::to_owned("getOutboundNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOutboundNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_chainID"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReceiveLibraryAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getReceiveLibraryAddress",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getReceiveVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReceiveVersion"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSendLibraryAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSendLibraryAddress",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getSendVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSendVersion"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasStoredPayload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasStoredPayload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
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
                    ::std::borrow::ToOwned::to_owned("inboundNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("inboundNonce"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isReceivingPayload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isReceivingPayload"),
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
                (
                    ::std::borrow::ToOwned::to_owned("isSendingPayload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isSendingPayload"),
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
                (
                    ::std::borrow::ToOwned::to_owned("lzEndpointLookup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lzEndpointLookup"),
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
                    ::std::borrow::ToOwned::to_owned("mockChainId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mockChainId"),
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
                    ::std::borrow::ToOwned::to_owned("msgsToDeliver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("msgsToDeliver"),
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
                                    name: ::std::borrow::ToOwned::to_owned("dstAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
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
                    ::std::borrow::ToOwned::to_owned("nextMsgBlocked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nextMsgBlocked"),
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
                (
                    ::std::borrow::ToOwned::to_owned("oracleFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("oracleFee"),
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
                    ::std::borrow::ToOwned::to_owned("outboundNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("outboundNonce"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("protocolFeeConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("protocolFeeConfig"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("zroFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nativeBP"),
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
                    ::std::borrow::ToOwned::to_owned("receivePayload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("receivePayload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_gasLimit"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("relayerFeeConfig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("relayerFeeConfig"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dstPriceRatio"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dstGasPriceInWei"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dstNativeAmtCap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasPerByte"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("retryPayload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("retryPayload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
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
                    ::std::borrow::ToOwned::to_owned("send"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("send"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_chainId"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_refundAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_zroPaymentAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("setDefaultAdapterParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setDefaultAdapterParams",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_adapterParams"),
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
                    ::std::borrow::ToOwned::to_owned("setDestLzEndpoint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDestLzEndpoint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("destAddr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lzEndpointAddr"),
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
                    ::std::borrow::ToOwned::to_owned("setOracleFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setOracleFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_oracleFee"),
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
                    ::std::borrow::ToOwned::to_owned("setProtocolFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setProtocolFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_zroFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nativeBP"),
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
                    ::std::borrow::ToOwned::to_owned("setReceiveVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setReceiveVersion"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("setRelayerPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setRelayerPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstPriceRatio"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstGasPriceInWei"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstNativeAmtCap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_baseGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasPerByte"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("storedPayload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("storedPayload"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payloadLength"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dstAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PayloadCleared"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PayloadCleared"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("srcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dstAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PayloadStored"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PayloadStored"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("srcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dstAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UaForceResumeReceive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UaForceResumeReceive",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("srcAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ValueTransferFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ValueTransferFailed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quantity"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("GasTooLow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("GasTooLow"),
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
                    ::std::borrow::ToOwned::to_owned("UnsupportedTxType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnsupportedTxType"),
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
    pub static LZENDPOINTMOCK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x804b\0\x02\xB5W`\x1F\x19b\0)\xDE8\x81\x90\x03`\x1F\x81\x01\x83\x16\x84\x01\x93`\x01`\x01`@\x1B\x03\x92\x90\x91\x82\x86\x10\x84\x87\x11\x17b\0\x02\x9FW\x80\x83\x92`@\x97\x88R\x839` \x92\x83\x91\x81\x01\x03\x12b\0\x02\xB5WQ\x90a\xFF\xFF\x82\x16\x80\x92\x03b\0\x02\xB5Wa\xFF\xFF\x19\x91a\x01\x01\x83`\x0CT\x16\x17`\x0CU`\x01\x92\x83T\x16\x17\x82U\x84Q`\xA0\x81\x01\x81\x81\x10\x85\x82\x11\x17b\0\x02\x9FW\x86Rd\x02T\x0B\xE4\0\x80\x82R\x82\x82\x01Rg\x8A\xC7#\x04\x89\xE8\0\0\x86\x82\x01R`d``\x82\x01R`\x80\x01\x82\x90Rt\x02T\x0B\xE4\0\0\0\0\0\0\0\0\0\0\0\0\x02T\x0B\xE4\0`\x02Ux\x01\0\0\0\0\0\0\0d\0\0\0\0\0\0\0\0\x8A\xC7#\x04\x89\xE8\0\0`\x03U\x84Q\x80\x86\x01\x90\x84\x82\x11\x81\x83\x10\x17b\0\x02\x9FW\x82\x91\x87Rg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81Ra\x03\xE8\x92\x83\x91\x01R`\x04U`\x05Uf#\x86\xF2o\xC1\0\0`\x06U\x84Q`\x01`\xF0\x1B\x82\x82\x01Rb\x03\r@`\"\x82\x01R`\"\x81R``\x81\x01\x81\x81\x10\x85\x82\x11\x17b\0\x02\x9FW\x86R\x80Q\x93\x84\x11b\0\x02\x9FW`\x07T\x83\x81\x81\x1C\x91\x16\x80\x15b\0\x02\x94W[\x83\x82\x10\x14b\0\x02~W`\x1F\x81\x11b\0\x02/W[P\x81`\x1F\x85\x11`\x01\x14b\0\x01\xC4WP\x83\x94P\x90\x83\x92\x91`\0\x94b\0\x01\xB8W[PP\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x07U[Qa'#\x90\x81b\0\x02\xBB\x829\xF3[\x01Q\x92P8\x80b\0\x01\x97V[\x92\x94\x84\x90\x81\x16`\x07`\0R\x84`\0 \x94`\0\x90[\x88\x83\x83\x10b\0\x02\x14WPPP\x10b\0\x01\xFAW[PPP\x81\x1B\x01`\x07Ub\0\x01\xAAV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80b\0\x01\xEBV[\x85\x87\x01Q\x88U\x90\x96\x01\x95\x94\x85\x01\x94\x87\x93P\x90\x81\x01\x90b\0\x01\xD8V[`\x07`\0R\x82`\0 `\x1F\x86\x01`\x05\x1C\x81\x01\x91\x84\x87\x10b\0\x02sW[`\x1F\x01`\x05\x1C\x01\x90\x84\x90[\x82\x81\x10b\0\x02fWPPb\0\x01xV[`\0\x81U\x01\x84\x90b\0\x02VV[\x90\x91P\x81\x90b\0\x02KV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x90`\x7F\x16\x90b\0\x01eV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80c\x07\xD3'\x7F\x14a\x02NW\x80c\x07\xE0\xDB\x17\x14a\x02DW\x80c\teh\xF6\x14a\x01\xD1W\x80c\x0E\xAFn\xA6\x14a\x02IW\x80c\x10\xDD\xB17\x14a\x02DW\x80c\x12\xA9\xEEk\x14a\x02?W\x80c$\r\xE2w\x14a\x02:W\x80c'+\xD3\x84\x14a\x025W\x80c,6^%\x14a\x020W\x80c4\x08\xE4p\x14a\x01\xCCW\x80c>\r\xD8>\x14a\x02+W\x80c@\xA7\xBB\x10\x14a\x02&W\x80cB\xD6Z\x8D\x14a\x02!W\x80cq\xBA/\xD6\x14a\x02\x08W\x80cv\xA3\x86\xDC\x14a\x02\x1CW\x80cz\x14WH\x14a\x01\xFEW\x80c\x7Fm\xF8\xE6\x14a\x02\x17W\x80c\x90|^~\x14a\x02\x12W\x80c\x99$\xD3;\x14a\x02\rW\x80c\x9Cr\x9D\xA1\x14a\x02\x08W\x80c\xAA\xFF_\x16\x14a\x02\x03W\x80c\xB2\x08d\x99\x14a\x01\xFEW\x80c\xB6\xD9\xEF`\x14a\x01\xF9W\x80c\xC0\x8F\x15\xA1\x14a\x01\xF4W\x80c\xC2\xFAH\x13\x14a\x01\xEFW\x80c\xC5\x801\0\x14a\x01\xEAW\x80c\xC8\x1B8:\x14a\x01\xE5W\x80c\xCA\x06k5\x14a\x01\xE0W\x80c\xCB\xED\x8B\x9C\x14a\x01\xDBW\x80c\xD21\x04\xF1\x14a\x01\xD6W\x80c\xDA\x1A|\x9A\x14a\x01\xD1W\x80c\xDB\x14\xF3\x05\x14a\x01\xCCW\x80c\xE9zD\x8A\x14a\x01\xC7W\x80c\xF5\xEC\xBD\xBC\x14a\x01\xC2W\x80c\xF9\xCD<\xEB\x14a\x01\xBDW\x80c\xFB\xBAb;\x14a\x01\xB8Wc\xFD\xC0|p\x14a\x01\xB3W`\0\x80\xFD[a\x19\xECV[a\x18\xDAV[a\x18\xBCV[a\x18gV[a\x18CV[a\t>V[a\x02\xDEV[a\x18\x1FV[a\x17\xDDV[a\x17\xB6V[a\x17vV[a\x143V[a\x0FnV[a\x0F\x10V[a\x0E\xF7V[a\x0B\xF9V[a\rFV[a\x0B(V[a\x0C\xF3V[a\x0C\x9DV[a\x0C^V[a\x0B\x88V[a\n\nV[a\t\x86V[a\t`V[a\x08bV[a\x07\xF0V[a\x07\xBBV[a\x07\x0BV[a\x02\xB2V[a\x03pV[a\x02cV[`\0\x91\x03\x12a\x02^WV[`\0\x80\xFD[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W`\x04T`\x05T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\xF3[\x03\x90\xF3[`\x045\x90a\xFF\xFF\x82\x16\x82\x03a\x02^WV[`$5\x90a\xFF\xFF\x82\x16\x82\x03a\x02^WV[4a\x02^W` 6`\x03\x19\x01\x12a\x02^Wa\x02\xCBa\x02\x90V[\0[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02^WV[4a\x02^W` 6`\x03\x19\x01\x12a\x02^Wa\x02\xFA`\x045a\x02\xCDV[` `@Q`\x01\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x02^W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02^W` \x83\x81\x86\x01\x95\x01\x01\x11a\x02^WV[\x90`@`\x03\x19\x83\x01\x12a\x02^W`\x045a\xFF\xFF\x81\x16\x81\x03a\x02^W\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02^Wa\x03l\x91`\x04\x01a\x03\x05V[\x90\x91V[4a\x02^W` `\x01a\xFF\xFF\x82a\x03\x866a\x032V[\x93\x90\x91\x16`\0R`\n\x82R`@`\0 \x83`@Q\x94\x85\x93\x847\x82\x01\x90\x81R\x03\x01\x90 \x01T\x15\x15`@Q\x90\x81R\xF3[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xDDW`@RV[a\x03\xB4V[` \x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xDDW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xDDW`@RV[`@Q\x90``\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xDDW`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xDDW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x04d\x82a\x04=V[\x91a\x04r`@Q\x93\x84a\x03\xFDV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02^W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x02^W\x81` a\x04\xAA\x935\x91\x01a\x04XV[\x90V[`\0[\x83\x81\x10a\x04\xC0WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\xB0V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80T\x15a\x04\xFBW`\0R` `\0 \x90`\0\x90V[a\x04\xD0V[\x80T\x82\x10\x15a\x04\xFBW`\0R` `\0 \x90`\x01\x1B\x01\x90`\0\x90V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x05LW[` \x83\x10\x14a\x056WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x05+V[`@Q\x90`\0\x82`\x07T\x91a\x05j\x83a\x05\x1CV[\x80\x83R\x92`\x01\x90\x81\x81\x16\x90\x81\x15a\x05\xF2WP`\x01\x14a\x05\x93W[Pa\x05\x91\x92P\x03\x83a\x03\xFDV[V[`\x07`\0\x90\x81R\x91P\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88[\x84\x83\x10a\x05\xD7WPa\x05\x91\x93PP\x81\x01` \x018a\x05\x84V[\x81\x93P\x90\x81` \x92T\x83\x85\x8A\x01\x01R\x01\x91\x01\x90\x91\x85\x92a\x05\xBEV[\x90P` \x92Pa\x05\x91\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x018a\x05\x84V[\x90`@Q\x91\x82`\0\x82T\x92a\x06(\x84a\x05\x1CV[\x90\x81\x84R`\x01\x94\x85\x81\x16\x90\x81`\0\x14a\x06\x95WP`\x01\x14a\x06RW[PPa\x05\x91\x92P\x03\x83a\x03\xFDV[\x90\x93\x91P`\0R` \x90\x81`\0 \x93`\0\x91[\x81\x83\x10a\x06}WPPa\x05\x91\x93P\x82\x01\x018\x80a\x06DV[\x85T\x88\x84\x01\x85\x01R\x94\x85\x01\x94\x87\x94P\x91\x83\x01\x91a\x06eV[\x91PPa\x05\x91\x94P` \x92P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x018\x80a\x06DV[\x90` \x91a\x06\xD1\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x04\xADV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\x01`\x01`@\x1B\x03a\x04\xAA\x94\x93`\x01`\x01`\xA0\x1B\x03``\x94\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a\x06\xB8V[4a\x02^W``6`\x03\x19\x01\x12a\x02^Wa\x07$a\x02\x90V[`\x01`\x01`@\x1B\x03`$5\x81\x81\x11a\x02^W` a\x07Ia\x07p\x926\x90`\x04\x01a\x04\x8FV[a\xFF\xFF`D5\x95\x16`\0R`\x0B\x82R`@`\0 \x82`@Q\x94\x83\x86\x80\x95Q\x93\x84\x92\x01a\x04\xADV[\x82\x01\x90\x81R\x03\x01\x90 \x91\x82T\x81\x10\x15a\x02^Wa\x07\x98a\x02\x8C\x91`\x01`\x01`\xA0\x1B\x03\x94a\x05\0V[Pa\x07\xA7`\x01\x82T\x92\x01a\x06\x14V[\x90`@Q\x94\x85\x94\x82`\xA0\x1C\x16\x91\x16\x84a\x06\xDDV[4a\x02^W`@6`\x03\x19\x01\x12a\x02^W`\x04\x805\x90U`$5`\x05U\0[cNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^Wa\x02\x8Ca\x08\x0Ca\x05VV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\xB8V[`D5\x90`\x01`\x01`\x80\x1B\x03\x82\x16\x82\x03a\x02^WV[`d5\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x02^WV[`\x845\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x02^WV[4a\x02^W`\xA06`\x03\x19\x01\x12a\x02^W`\x045`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x92\x03a\x02^W`$5\x90\x80\x82\x16\x82\x03a\x02^Wa\x02\xCB\x92a\x08\xA1a\x08 V[\x91a\x08\xAAa\x086V[\x91a\x08\xB3a\x08LV[`\x80\x95\x86\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x91\x17`\x02Uw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x93\x16\x16\x92\x1Bw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x90\x91\x17`\xC0\x91\x90\x91\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17`\x03UV[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W` a\xFF\xFF`\x01T\x16`@Q\x90\x81R\xF3[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W` `\xFF`\x01T`\x10\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02^W`\xA06`\x03\x19\x01\x12a\x02^Wa\t\x9Fa\x02\x90V[Pa\t\xAB`$5a\x02\xCDV[`\x01`\x01`@\x1B\x03`D5\x81\x81\x11a\x02^Wa\t\xCB\x906\x90`\x04\x01a\x04\x8FV[`d5\x80\x15\x15\x81\x03a\x02^W`\x845\x92\x83\x11a\x02^Wa\t\xF2a\t\xF8\x936\x90`\x04\x01a\x04\x8FV[\x91a!\xFCV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\xF3[4a\x02^Wa\n\x186a\x032V[\x90a\xFF\xFF\x83\x16`\0R`\n` R`@`\0 ` `@Q\x80\x92\x85\x85\x837\x85\x82\x01\x90\x81R\x03\x01\x90 \x92`\x01\x84\x01\x93a\nR\x85T\x15\x15a#4V[`\x01`\x01`\xA0\x1B\x03\x81T`@\x1C\x163\x03a\n\xE3W\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81Ua\x02\xCB\x94`\0\x91a\n\xA9\x90[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x16\x90UV[U\x7F#\xD2hO9n\x92\xA6\xE2\xFF-\x16\xF9\x8Eo\xEA\0\xD5\x0C\xB2zd\xB51\xBC\x07H\xF70!\x1F\x98`@Q\x80a\n\xDB\x86\x86\x86\x84a$\x0BV[\x03\x90\xA1a%XV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FLayerZeroMock: invalid caller\0\0\0`D\x82\x01R`d\x90\xFD[4a\x02^W` 6`\x03\x19\x01\x12a\x02^Wa\x0BD`\x045a\x02\xCDV[` `@Q0\x81R\xF3[\x90`@`\x03\x19\x83\x01\x12a\x02^W`\x045a\xFF\xFF\x81\x16\x81\x03a\x02^W\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02^Wa\x04\xAA\x91`\x04\x01a\x04\x8FV[4a\x02^W``a\x0B\xC1` a\xFF\xFFa\x0B\xA06a\x0BNV[\x91\x16`\0R`\n\x82R`@`\0 \x82`@Q\x94\x83\x86\x80\x95Q\x93\x84\x92\x01a\x04\xADV[\x82\x01\x90\x81R\x03\x01\x90 `\x01\x81T\x91\x01T`\x01`\x01`\xA0\x1B\x03`@Q\x92`\x01`\x01`@\x1B\x03\x81\x16\x84R`@\x1C\x16` \x83\x01R`@\x82\x01R\xF3[4a\x02^W`@6`\x03\x19\x01\x12a\x02^W` `\x01`\x01`@\x1B\x03a\x0CTa\x0C\x1Fa\x02\x90V[a\xFF\xFF`$5\x91a\x0C/\x83a\x02\xCDV[\x16`\0R`\t\x84R`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T\x16`@Q\x90\x81R\xF3[4a\x02^W` a\xFF\xFF\x81a\x0Cr6a\x032V[\x93\x90\x91\x16`\0R`\x0B\x82R`@`\0 \x83`@Q\x94\x85\x93\x847\x82\x01\x90\x81R\x03\x01\x90 T`@Q\x90\x81R\xF3[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W`\xA0`\x02T`\x03T`@Q\x91`\x01`\x01`\x80\x1B\x03\x90\x81\x81\x16\x84R`\x80\x1C` \x84\x01R\x81\x16`@\x83\x01R`\x01`\x01`@\x1B\x03\x81`\x80\x1C\x16``\x83\x01R`\xC0\x1C`\x80\x82\x01R\xF3[4a\x02^W` `\x01`\x01`@\x1B\x03a\r3\x82a\xFF\xFFa\r\x126a\x0BNV[\x91\x16`\0R`\x08\x82R`@`\0 \x82`@Q\x94\x83\x86\x80\x95Q\x93\x84\x92\x01a\x04\xADV[\x82\x01\x90\x81R\x03\x01\x90 T\x16`@Q\x90\x81R\xF3[4a\x02^W``6`\x03\x19\x01\x12a\x02^Wa\r_a\x02\x90V[`\x01`\x01`@\x1B\x03\x90`$5\x82\x81\x11a\x02^Wa\r\x80\x906\x90`\x04\x01a\x03\x05V[\x91`D5\x84\x81\x11a\x02^Wa\r\x99\x906\x90`\x04\x01a\x03\x05V[a\r\xBEa\r\xB7\x84\x96\x93\x96a\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x83\x86a\x1DEV[\x90a\x0E\x0C`\x01`\x01`\xA0\x1B\x03`\x01\x84\x01\x98a\r\xF3\x8ATa\r\xDF\x81\x15\x15a#4V[\x86T\x92\x83\x16\x86\x14\x90\x81a\x0E\xDCW[Pa#\x7FV[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x85U`@\x1C\x16\x92a\n\x81V[`\0\x80\x97U\x86a\x0EDa\x0E7a\x0E0\x87a\xFF\xFF\x16`\0R`\x08` R`@`\0 \x90V[\x86\x89a\x1DEV[T`\x01`\x01`@\x1B\x03\x16\x90V[\x91\x83;\x15a\x0E\xD8W`@Qb\x1D5g`\xE0\x1B\x81R\x97\x88\x91\x82\x91a\x0Eo\x91\x90\x86\x89\x8C\x8C`\x04\x88\x01a\x1D\xCBV[\x03\x81\x83\x86Z\xF1\x92\x83\x15a\x0E\xD3W\x7Fa$4\xF3\x95\x81\xC8\xE7\xD9\x97F\xC9\xC2\x0Cn\xB0\xCE\x8C\x0E\xB9\x9F\0|W\x19\xD6 \x84\x13p\x95}\x96a\x0E\xB4\x94a\x0E\xBAW[P`@Q\x95\x86\x95\x86a#\xCBV[\x03\x90\xA1\x80\xF3[\x80a\x0E\xC7a\x0E\xCD\x92a\x03\xCAV[\x80a\x02SV[8a\x0E\xA7V[a\x1C\x96V[P\x80\xFD[\x90Pa\x0E\xE96\x87\x8Da\x04XV[` \x81Q\x91\x01 \x148a\r\xEDV[4a\x02^W` 6`\x03\x19\x01\x12a\x02^W`\x045`\x06U\0[4a\x02^W`@6`\x03\x19\x01\x12a\x02^Wa\x02\xCB`\x045a\x0F0\x81a\x02\xCDV[`\x01`\x01`\xA0\x1B\x03`$5\x91a\x0FE\x83a\x02\xCDV[\x16`\0R`\0` R`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x82T\x16\x17\x90UV[4a\x02^W`\xC06`\x03\x19\x01\x12a\x02^Wa\x0F\x87a\x02\x90V[`\x01`\x01`@\x1B\x03`$5\x81\x81\x11a\x02^Wa\x0F\xA7\x906\x90`\x04\x01a\x03\x05V[\x91\x90\x92`D5\x92a\x0F\xB7\x84a\x02\xCDV[a\x0F\xBFa\x086V[\x94`\xA45\x84\x81\x11a\x02^Wa\x0F\xD8\x906\x90`\x04\x01a\x03\x05V[\x92\x90\x93`\x0CT\x95`\x01\x96\x87`\xFF\x82`\x08\x1C\x16\x03a\x13\xE2Wa\x02\0\x90a\xFF\0\x19\x16\x17`\x0CU\x86a\x10\x1Fa\x10\x18\x84a\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x85\x87a\x1DEV[a\x10\x88\x83a\x10Ea\x10>\x87a\xFF\xFF\x16`\0R`\x08` R`@`\0 \x90V[\x88\x8Aa\x1DEV[a\x10~a\x10aa\x10\\\x83T`\x01`\x01`@\x1B\x03\x16\x90V[a\x1B{V[\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x82\x16\x17\x90\x92UV[\x16\x84\x8D\x16\x14a\x1D^V[\x01T\x15a\x11\x8EWPa\x10\xED\x94\x93\x92a\x10\xE6\x92a\x10\xB5a\x10\xBB\x93a\xFF\xFF\x16`\0R`\x0B` R`@`\0 \x90V[\x91a\x1DEV[\x96a\x10\xD6a\x10\xC7a\x04\x1EV[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[`\x01`\x01`@\x1B\x03\x16` \x87\x01RV[6\x91a\x04XV[`@\x83\x01R\x82T\x15a\x11\x7FWa\x11\x03\x82\x84a PV[`\0\x81[a\x114W[PPa\x11\x1Aa\x11 \x92a\x04\xE6V[\x90a\x1F\x1DV[a\x02\xCBa\x01\0a\xFF\0\x19`\x0CT\x16\x17`\x0CUV[a\x11>\x84Ta\x1B\x98V[\x81\x10\x15a\x11zW\x80a\x11oa\x11Va\x11t\x93\x87a\x05\0V[Pa\x11ia\x11c\x84a \x87V[\x88a\x05\0V[\x90a!\x8FV[a xV[\x81a\x11\x07V[a\x11\x0CV[Pa\x11\x89\x91a PV[a\x11 V[\x90\x95a\x11\xA5\x90\x98\x92\x95\x93\x97\x98T`\xFF\x90`\x10\x1C\x16\x90V[\x15a\x12\xD0W\x90\x7F\x0F\x9EM\x95\xB6/\x08\"-a+Z\xB9 9\xCD\x8F\xBB\xBE\xA5P\xA9^\x8D\xF9\xF9'Ck\xBD\xF5\xDB\x97a\x12\xA7a\x12\xBC\x96\x95\x94\x93a\x11\xE26\x87\x87a\x04XV[` \x81Q\x91\x01 a\x12\x06a\x11\xF4a\x04\x1EV[\x92\x88\x16\x83\x90`\x01`\x01`@\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x84\x16` \x83\x01R`@\x82\x01Ra\x12<a\x125\x8Aa\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x88\x8Ca\x1DEV[\x81Q\x81T` \x84\x01Q`\x01`\x01`@\x1B\x03\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17`@\x91\x82\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16\x17\x82U\x90\x91\x01Q`\x01\x90\x91\x01UV[a\x12\xAFa\x1B\xB4V[\x94`@Q\x98\x89\x98\x89a\x1E\x0BV[\x03\x90\xA1a\x11\x89b\xFF\0\0\x19`\x01T\x16`\x01UV[\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x80;\x15a\x02^W`\0\x86\x83\x92\x8A\x83\x8Ba\x13\r\x8A\x8A`@Q\x99\x8A\x98\x89\x97\x88\x96b\x1D5g`\xE0\x1B\x88R`\x04\x88\x01a\x1D\xCBV[\x03\x92`\x845\xF1\x90\x81a\x13\xCFW[Pa\x13\xC2W\x7F\x0F\x9EM\x95\xB6/\x08\"-a+Z\xB9 9\xCD\x8F\xBB\xBE\xA5P\xA9^\x8D\xF9\xF9'Ck\xBD\xF5\xDB\x97a\x12\xBC\x95a\x13\xB6a\x13Pa\x1B\xC7V[\x96a\x13\\6\x88\x88a\x04XV[` \x81Q\x91\x01 a\x13\x80a\x13na\x04\x1EV[\x92\x89\x16\x83\x90`\x01`\x01`@\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x85\x16` \x83\x01R`@\x82\x01Ra\x12<a\x13\xAF\x8Ba\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x84\x8Da\x1DEV[`@Q\x98\x89\x98\x89a\x1E\x0BV[PPPPPPPPa\x11 V[\x80a\x0E\xC7a\x13\xDC\x92a\x03\xCAV[8a\x13\x1AV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FLayerZeroMock: no receive reentr`D\x82\x01Rcancy`\xE0\x1B`d\x82\x01R`\x84\x90\xFD[`\xC06`\x03\x19\x01\x12a\x02^Wa\x14Ga\x02\x90V[`\x01`\x01`@\x1B\x03`$5\x81\x81\x11a\x02^Wa\x14g\x906\x90`\x04\x01a\x04\x8FV[`D5\x82\x81\x11a\x02^Wa\x14\x7F\x906\x90`\x04\x01a\x03\x05V[`d\x94\x91\x945\x93a\x14\x8F\x85a\x02\xCDV[`\x845\x90a\x14\x9C\x82a\x02\xCDV[`\xA45\x90\x81\x11a\x02^Wa\x14\xB4\x906\x90`\x04\x01a\x04\x8FV[\x93`\x0CT`\x01`\xFF\x82\x16\x03a\x17'W\x87\x91\x84\x91`\xFF\x19\x16`\x02\x17`\x0CU\x80Q`(\x14a\x14\xDF\x90a\x1A4V[`\x14\x01Q\x95`\x01`\x01`\xA0\x1B\x03\x80\x98\x81\x92\x82a\x15\x0E\x8B`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[T`\x01`\x01`\xA0\x1B\x03\x16\x16\x98a\x15%\x8A\x15\x15a\x1A\x95V[\x81Q\x15\x15`\0\x14a\x15\xB4a\x15\xA5a\x10\\\x9Fa\x16:\x9Ba\x15\xE5\x9Aa\x15\xD7\x97a\x15f\x93a\x15\xD1\x97a\x17\x12Wa\x15a\x91\x9C\x8D\x94[\x16\x15\x15\x926\x91a\x04XV[a!\xFCV[P\x93a\x15t\x854\x10\x15a\x1B\x07V[a\x15\x8D3\x91a\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[\x9E\x8FT`\x01`\x01`@\x1B\x03\x16\x90V[\x8ETg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x82\x16\x17\x90\x9EUV[4a\x1B\xA7V[\x80a\x16\xEDW[PPPa\x1C\xA2V[\x90\x80\x93P\x98\x91\x98a\x16\xA2W[PPP`@Q\x92a\x10\xE6\x84a\x16,\x883` \x84\x01\x90`(\x92k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x92``\x1B\x16\x83R``\x1B\x16`\x14\x82\x01R\x01\x90V[\x03`\x1F\x19\x81\x01\x86R\x85a\x03\xFDV[`\x01Ta\xFF\xFF\x16\x83;\x15a\x02^Wa\x16n`\0\x96\x92\x87\x93`@Q\x99\x8A\x98\x89\x97\x88\x96c\xC2\xFAH\x13`\xE0\x1B\x88R`\x04\x88\x01a\x1CCV[\x03\x92Z\xF1\x80\x15a\x0E\xD3Wa\x16\x8FW[a\x02\xCB`\x01`\xFF\x19`\x0CT\x16\x17`\x0CUV[\x80a\x0E\xC7a\x16\x9C\x92a\x03\xCAV[8a\x16}V[\x16`\0\x80\x80\x80\x85\x85Z\xF1a\x16\xB4a\x1B\xC7V[P\x15a\x16\xC1W[\x80a\x15\xF1V[\x7F,z\x96L\xA3\xDE^\xC1\xD4-\x98\"\xF9\xBB\xD0\xEB\x14*Y\xCC\x9F\x85^\x9D\x93\x81;w1\x92\xC7\xA3`\0\x80\xA38\x80a\x16\xBBV[`\0\x80\x80\x93\x92a\x17\n\x95\x82\x94\x16Z\xF1a\x17\x04a\x1B\xC7V[Pa\x1B\xF7V[\x878\x80a\x15\xDDV[a\x15a\x91Pa\x17\x1Fa\x05VV[\x9C\x8D\x94a\x15VV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FLayerZeroMock: no send reentranc`D\x82\x01R`y`\xF8\x1B`d\x82\x01R`\x84\x90\xFD[4a\x02^W` 6`\x03\x19\x01\x12a\x02^W` `\x045a\x17\x95\x81a\x02\xCDV[`\x01`\x01`\xA0\x1B\x03\x80\x91\x16`\0R`\0\x82R`@`\0 T\x16`@Q\x90\x81R\xF3[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W` `\x02`\xFF`\x0CT`\x08\x1C\x16\x14`@Q\x90\x81R\xF3[4a\x02^W`\x806`\x03\x19\x01\x12a\x02^Wa\x17\xF6a\x02\x90V[Pa\x17\xFFa\x02\xA1V[P`d5`\x01`\x01`@\x1B\x03\x81\x11a\x02^Wa\x02\xCB\x906\x90`\x04\x01a\x04\x8FV[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W`\x01\x80Tb\xFF\0\0\x19\x16b\x01\0\0\x17\x90U\0[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W` `\x02`\xFF`\x0CT\x16\x14`@Q\x90\x81R\xF3[4a\x02^W`\x806`\x03\x19\x01\x12a\x02^Wa\x18\x80a\x02\x90V[Pa\x18\x89a\x02\xA1V[Pa\x18\x95`D5a\x02\xCDV[a\x02\x8C`@Qa\x18\xA4\x81a\x03\xE2V[`\0\x81R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\xB8V[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W` `\x06T`@Q\x90\x81R\xF3[4a\x02^W` \x80`\x03\x196\x01\x12a\x02^W`\x01`\x01`@\x1B\x03`\x045\x81\x81\x11a\x02^Wa\x19\x0C\x906\x90`\x04\x01a\x04\x8FV[\x91\x82Q\x91\x82\x11a\x03\xDDWa\x19*\x82a\x19%`\x07Ta\x05\x1CV[a\x1E\x82V[\x80`\x1F\x83\x11`\x01\x14a\x19eWP\x81\x92`\0\x92a\x19ZW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16`\x01\x91\x90\x91\x1B\x17`\x07U\0[\x01Q\x90P8\x80a\x19AV[\x90`\x1F\x19\x83\x16\x93a\x19\x98`\x07`\0R\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88\x90V[\x92`\0\x90[\x86\x82\x10a\x19\xD4WPP\x83`\x01\x95\x10a\x19\xBBW[PPP\x81\x1B\x01`\x07U\0[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\x19\xB0V[\x80`\x01\x85\x96\x82\x94\x96\x86\x01Q\x81U\x01\x95\x01\x93\x01\x90a\x19\x9DV[4a\x02^W` `\x01`\x01`@\x1B\x03a\xFF\xFF\x82a\x1A\x086a\x032V[\x93\x90\x91\x16`\0R`\x08\x82R`@`\0 \x83`@Q\x94\x85\x93\x847\x82\x01\x90\x81R\x03\x01\x90 T\x16`@Q\x90\x81R\xF3[\x15a\x1A;WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FLayerZeroMock: incorrect remote `D\x82\x01Rkaddress size`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[\x15a\x1A\x9CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FLayerZeroMock: destination Layer`D\x82\x01R\x7FZero Endpoint not found\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a\x1B\x0EWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FLayerZeroMock: not enough native`D\x82\x01Rh for fees`\xB8\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x80\x91\x16\x90\x81\x14a\x1B\x93W`\x01\x01\x90V[a\x1BeV[`\0\x19\x81\x01\x91\x90\x82\x11a\x1B\x93WV[\x91\x90\x82\x03\x91\x82\x11a\x1B\x93WV[`@Q\x90a\x1B\xC1\x82a\x03\xE2V[`\0\x82RV[=\x15a\x1B\xF2W=\x90a\x1B\xD8\x82a\x04=V[\x91a\x1B\xE6`@Q\x93\x84a\x03\xFDV[\x82R=`\0` \x84\x01>V[``\x90V[\x15a\x1B\xFEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FLayerZeroMock: failed to refund\0`D\x82\x01R`d\x90\xFD[\x92\x94\x91\x93`\x01`\x01`\xA0\x1B\x03a\x1Cwa\x04\xAA\x98\x96a\xFF\xFF`\x01`\x01`@\x1B\x03\x95\x16\x87R`\xC0` \x88\x01R`\xC0\x87\x01\x90a\x06\xB8V[\x96\x16`@\x85\x01R\x16``\x83\x01R`\x80\x82\x01R`\xA0\x81\x84\x03\x91\x01Ra\x06\xB8V[`@Q=`\0\x82>=\x90\xFD[`\0\x80\x82Q`\"\x81\x14\x15\x90\x81a\x1D:W[Pa\x1D(W`\x02\x83\x01Q\x93`\"\x84\x01Q\x93a\xFF\xFF\x86\x16`\x01\x81\x14\x15\x80a\x1D\x1DW[a\x1D\x0BW\x85\x15a\x1C\xF9W`\x02\x14a\x1C\xE8WPV[\x92P\x90P`V`B\x83\x01Q\x92\x01Q\x90V[`@Qc!c\"]`\xE2\x1B\x81R`\x04\x90\xFD[`@Qco\xC3\xDA\xA3`\xE1\x1B\x81R`\x04\x90\xFD[P`\x02\x81\x14\x15a\x1C\xD4V[`@Qc\xCE\xF8\x0E\xA3`\xE0\x1B\x81R`\x04\x90\xFD[`B\x91P\x108a\x1C\xB3V[` \x91\x92\x83`@Q\x94\x85\x93\x847\x82\x01\x90\x81R\x03\x01\x90 \x90V[\x15a\x1DeWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FLayerZeroMock: wrong nonce\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91a\x1D\xF7\x90a\x04\xAA\x97\x95\x96\x94a\xFF\xFF`\x01`\x01`@\x1B\x03\x94\x16\x85R`\x80` \x86\x01R`\x80\x85\x01\x91a\x1D\xAAV[\x94\x16`@\x82\x01R``\x81\x85\x03\x91\x01Ra\x1D\xAAV[\x96\x92\x94`\x01`\x01`\xA0\x1B\x03a\x1ECa\x04\xAA\x9A\x98\x94a\x1E]\x98a\xFF\xFF`\x01`\x01`@\x1B\x03\x96\x16\x8CR`\xC0` \x8D\x01R`\xC0\x8C\x01\x91a\x1D\xAAV[\x95\x16`@\x89\x01R\x16``\x87\x01R\x85\x83\x03`\x80\x87\x01Ra\x1D\xAAV[\x91`\xA0\x81\x84\x03\x91\x01Ra\x06\xB8V[\x81\x81\x10a\x1EvWPPV[`\0\x81U`\x01\x01a\x1EkV[\x90`\x1F\x82\x11a\x1E\x8FWPPV[a\x05\x91\x91`\x07`\0R\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88\x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x1E\xD9W[`\x1F\x01`\x05\x1C\x01\x90a\x1EkV[\x90\x91P\x81\x90a\x1E\xCCV[\x91\x90`\x1F\x81\x11a\x1E\xF2WPPPV[a\x05\x91\x92`\0R` `\0 \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x1E\xD9W`\x1F\x01`\x05\x1C\x01\x90a\x1EkV[\x90a KW\x81Q\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x81U` \x82\x81\x01Q\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x82U`\x01`\x01`@\x1B\x03\x90`@`\x01\x80\x94\x01\x94\x01Q\x80Q\x92\x83\x11a\x03\xDDWa\x1F\xA8\x83a\x1F\xA2\x87Ta\x05\x1CV[\x87a\x1E\xE3V[\x81`\x1F\x84\x11`\x01\x14a\x1F\xE1WP\x92\x82\x93\x91\x83\x92`\0\x94a\x1F\xD6W[PP\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90UV[\x01Q\x92P8\x80a\x1F\xC3V[\x91\x90\x83`\x1F\x19\x81\x16a\x1F\xF8\x88`\0R` `\0 \x90V[\x94`\0\x90[\x88\x83\x83\x10a 1WPPP\x10a \x18W[PPP\x81\x1B\x01\x90UV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a \x0EV[\x85\x87\x01Q\x88U\x90\x96\x01\x95\x94\x85\x01\x94\x87\x93P\x90\x81\x01\x90a\x1F\xFDV[a\x07\xDAV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x03\xDDW\x82a\x11\x1A\x91`\x01a\x05\x91\x95\x01\x81Ua\x05\0V[`\0\x19\x81\x14a\x1B\x93W`\x01\x01\x90V[\x90`\x01\x82\x01\x80\x92\x11a\x1B\x93WV[\x91\x90\x82\x01\x80\x92\x11a\x1B\x93WV[\x91\x90\x91\x82\x81\x14a!\x8AWa \xB6\x83Ta\x05\x1CV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xDDWa \xD8\x81a \xD2\x84Ta\x05\x1CV[\x84a\x1E\xE3V[`\0\x93`\x1F\x82\x11`\x01\x14a!\x14W\x93\x81\x92\x93\x94`\0\x92a!\tW[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90UV[\x01T\x90P8\x80a \xF3V[a!(`\x1F\x19\x83\x16\x91`\0R` `\0 \x90V[\x94a!8\x84`\0R` `\0 \x90V[\x91\x81[\x81\x81\x10a!rWP\x95\x83`\x01\x95\x96\x97\x10a!YWPPP\x81\x1B\x01\x90UV[\x01T`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a \x0EV[\x87\x83\x01T\x84U`\x01\x93\x84\x01\x93\x90\x92\x01\x91` \x01a!;V[P\x90PV[\x90a KW\x81\x81\x03a!\x9FWPPV[`\x01\x80\x83a!\xD0`\x01`\x01`\xA0\x1B\x03a\x05\x91\x96T\x16\x85\x90`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x82T\x16\x17\x90UV[\x80T\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x90\x91\x16\x17\x84U\x01\x91\x01a \xA2V[\x82Q\x91\x92`\0\x92\x83\x92\x91\x90\x15a#\"Wa\"\x19\x90\x91[Q\x91a\x1C\xA2V[P`\x02a\xFF\xFF`\0\x95\x94\x93\x95\x93\x16\x14a#\x03W[P`\x02T\x90\x81`\x80\x1C\x90`\x03T\x94\x85`\x80\x1C`\x01`\x01`@\x1B\x03\x16\x90a\"R\x91a \x95V[a\"\\\x90\x83a$\xAFV[a\"e\x91a \x95V[\x91`\x01`\x01`\x80\x1B\x03\x16\x91\x82a\"z\x91a$\xAFV[d\x02T\x0B\xE4\0\x90\x04\x93`\xC0\x1Ca\"\x8F\x91a&\xCFV[\x90a\"\x99\x91a&\xCFV[d\x02T\x0B\xE4\0`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x04`\x01`\x01`\x80\x1B\x03\x16a\"\xBD\x91a$\xAFV[a\"\xC6\x91a \x95V[\x92`\x06T\x91a\"\xD6\x83\x86\x84a&GV[\x91\x15a\"\xF4WPa\"\xF1\x92\x93a\"\xEC\x91\x94a \x95V[a \x95V[\x91V[\x93a\"\xF1\x93Pa\"\xEC\x91a \x95V[\x90Pa#\x1C\x81`\x01`\x01`\x80\x1B\x03`\x03T\x16\x10\x15a&tV[8a\"-V[Pa\"\x19a#.a\x05VV[\x91a\"\x12V[\x15a#;WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FLayerZeroMock: no stored payload`D\x82\x01R\xFD[\x15a#\x86WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FLayerZeroMock: invalid payload\0\0`D\x82\x01R`d\x90\xFD[\x93\x95\x94\x90a#\xFE`\x01`\x01`\xA0\x1B\x03\x93``\x95a\xFF\xFF`\x01`\x01`@\x1B\x03\x94\x16\x88R`\x80` \x89\x01R`\x80\x88\x01\x91a\x1D\xAAV[\x96\x16`@\x85\x01R\x16\x91\x01RV[`@\x90a\xFF\xFFa\x04\xAA\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a\x1D\xAAV[\x90`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x93\x82\x82\x10\x85\x83\x11\x17a\x03\xDDW`\x01a$l\x91`@\x93\x84R\x84\x96\x81T`\x01`\x01`\xA0\x1B\x03\x81\x16\x87R`\xA0\x1C\x16` \x86\x01R\x01a\x06\x14V[\x91\x01RV[\x91a$\x9B\x90a\x04\xAA\x96\x94a\xFF\xFF`\x01`\x01`@\x1B\x03\x94\x16\x85R`\x80` \x86\x01R`\x80\x85\x01\x91a\x1D\xAAV[\x93\x16`@\x82\x01R``\x81\x84\x03\x91\x01Ra\x06\xB8V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x1B\x93WV[\x80T\x80\x15a%BW`\0\x19\x01\x90a$\xD9\x82\x82a\x05\0V[a KW\x80`\0`\x01\x92U\x01a$\xEF\x81Ta\x05\x1CV[\x90\x81a$\xFAWPPUV[`\x1F\x82\x11`\x01\x14a%\rW`\0\x91PUUV[a%/a%?\x92\x82`\0R`\x01`\x1F` `\0 \x92\x01`\x05\x1C\x82\x01\x91\x01a\x1EkV[`\0\x90\x80\x82R\x81` \x81 \x91UUV[UV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[\x92\x91a%|a%u\x85a\xFF\xFF\x16`\0R`\x0B` R`@`\0 \x90V[\x82\x84a\x1DEV[\x92[\x83T\x80\x15a&?Wa%\x9Ba%\x95a%\xA1\x92a\x1B\x98V[\x86a\x05\0V[Pa$&V[a%\xC4a%\xB8a%\xB8\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90a%\xD9` \x82\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x91`@\x80\x92\x01Q\x90\x80;\x15a\x02^Wa&\x0E\x93\x89\x88\x94Q\x80\x96\x81\x95\x82\x94b\x1D5g`\xE0\x1B\x84R\x8B`\0\x99\x8A\x96`\x04\x87\x01a$qV[\x03\x92Z\xF1\x80\x15a\x0E\xD3Wa&,W[Pa&'\x84a$\xC2V[a%~V[\x80a\x0E\xC7a&9\x92a\x03\xCAV[8a&\x1DV[P\x93PPPPV[\x90\x91\x90\x15a&WWPP`\x04T\x90V[\x81\x01\x80\x91\x11a\x1B\x93Wa&pa'\x10\x91`\x05T\x90a$\xAFV[\x04\x90V[\x15a&{WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FLayerZeroMock: dstNativeAmt too `D\x82\x01Re\x03c\x0B\x93;)`\xD5\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x91`\x01`\x01`\x80\x1B\x03\x80\x80\x94\x16\x91\x16\x02\x91\x82\x16\x91\x82\x03a\x1B\x93WV\xFE\xA2dipfsX\"\x12 \n\xC6\xEE\xC7\xD2W\xAE\xCBb\xEB\xB2B\xF5\x0F\xFFXz\xFF\x88\xD3\xCCN\xE6$\x14y\r\xC7\xE5\xB0\xBC\x8CdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static LZENDPOINTMOCK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80c\x07\xD3'\x7F\x14a\x02NW\x80c\x07\xE0\xDB\x17\x14a\x02DW\x80c\teh\xF6\x14a\x01\xD1W\x80c\x0E\xAFn\xA6\x14a\x02IW\x80c\x10\xDD\xB17\x14a\x02DW\x80c\x12\xA9\xEEk\x14a\x02?W\x80c$\r\xE2w\x14a\x02:W\x80c'+\xD3\x84\x14a\x025W\x80c,6^%\x14a\x020W\x80c4\x08\xE4p\x14a\x01\xCCW\x80c>\r\xD8>\x14a\x02+W\x80c@\xA7\xBB\x10\x14a\x02&W\x80cB\xD6Z\x8D\x14a\x02!W\x80cq\xBA/\xD6\x14a\x02\x08W\x80cv\xA3\x86\xDC\x14a\x02\x1CW\x80cz\x14WH\x14a\x01\xFEW\x80c\x7Fm\xF8\xE6\x14a\x02\x17W\x80c\x90|^~\x14a\x02\x12W\x80c\x99$\xD3;\x14a\x02\rW\x80c\x9Cr\x9D\xA1\x14a\x02\x08W\x80c\xAA\xFF_\x16\x14a\x02\x03W\x80c\xB2\x08d\x99\x14a\x01\xFEW\x80c\xB6\xD9\xEF`\x14a\x01\xF9W\x80c\xC0\x8F\x15\xA1\x14a\x01\xF4W\x80c\xC2\xFAH\x13\x14a\x01\xEFW\x80c\xC5\x801\0\x14a\x01\xEAW\x80c\xC8\x1B8:\x14a\x01\xE5W\x80c\xCA\x06k5\x14a\x01\xE0W\x80c\xCB\xED\x8B\x9C\x14a\x01\xDBW\x80c\xD21\x04\xF1\x14a\x01\xD6W\x80c\xDA\x1A|\x9A\x14a\x01\xD1W\x80c\xDB\x14\xF3\x05\x14a\x01\xCCW\x80c\xE9zD\x8A\x14a\x01\xC7W\x80c\xF5\xEC\xBD\xBC\x14a\x01\xC2W\x80c\xF9\xCD<\xEB\x14a\x01\xBDW\x80c\xFB\xBAb;\x14a\x01\xB8Wc\xFD\xC0|p\x14a\x01\xB3W`\0\x80\xFD[a\x19\xECV[a\x18\xDAV[a\x18\xBCV[a\x18gV[a\x18CV[a\t>V[a\x02\xDEV[a\x18\x1FV[a\x17\xDDV[a\x17\xB6V[a\x17vV[a\x143V[a\x0FnV[a\x0F\x10V[a\x0E\xF7V[a\x0B\xF9V[a\rFV[a\x0B(V[a\x0C\xF3V[a\x0C\x9DV[a\x0C^V[a\x0B\x88V[a\n\nV[a\t\x86V[a\t`V[a\x08bV[a\x07\xF0V[a\x07\xBBV[a\x07\x0BV[a\x02\xB2V[a\x03pV[a\x02cV[`\0\x91\x03\x12a\x02^WV[`\0\x80\xFD[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W`\x04T`\x05T`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\xF3[\x03\x90\xF3[`\x045\x90a\xFF\xFF\x82\x16\x82\x03a\x02^WV[`$5\x90a\xFF\xFF\x82\x16\x82\x03a\x02^WV[4a\x02^W` 6`\x03\x19\x01\x12a\x02^Wa\x02\xCBa\x02\x90V[\0[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02^WV[4a\x02^W` 6`\x03\x19\x01\x12a\x02^Wa\x02\xFA`\x045a\x02\xCDV[` `@Q`\x01\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x02^W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02^W` \x83\x81\x86\x01\x95\x01\x01\x11a\x02^WV[\x90`@`\x03\x19\x83\x01\x12a\x02^W`\x045a\xFF\xFF\x81\x16\x81\x03a\x02^W\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02^Wa\x03l\x91`\x04\x01a\x03\x05V[\x90\x91V[4a\x02^W` `\x01a\xFF\xFF\x82a\x03\x866a\x032V[\x93\x90\x91\x16`\0R`\n\x82R`@`\0 \x83`@Q\x94\x85\x93\x847\x82\x01\x90\x81R\x03\x01\x90 \x01T\x15\x15`@Q\x90\x81R\xF3[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xDDW`@RV[a\x03\xB4V[` \x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xDDW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xDDW`@RV[`@Q\x90``\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\xDDW`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xDDW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x04d\x82a\x04=V[\x91a\x04r`@Q\x93\x84a\x03\xFDV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x02^W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x02^W\x81` a\x04\xAA\x935\x91\x01a\x04XV[\x90V[`\0[\x83\x81\x10a\x04\xC0WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x04\xB0V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80T\x15a\x04\xFBW`\0R` `\0 \x90`\0\x90V[a\x04\xD0V[\x80T\x82\x10\x15a\x04\xFBW`\0R` `\0 \x90`\x01\x1B\x01\x90`\0\x90V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x05LW[` \x83\x10\x14a\x056WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x05+V[`@Q\x90`\0\x82`\x07T\x91a\x05j\x83a\x05\x1CV[\x80\x83R\x92`\x01\x90\x81\x81\x16\x90\x81\x15a\x05\xF2WP`\x01\x14a\x05\x93W[Pa\x05\x91\x92P\x03\x83a\x03\xFDV[V[`\x07`\0\x90\x81R\x91P\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88[\x84\x83\x10a\x05\xD7WPa\x05\x91\x93PP\x81\x01` \x018a\x05\x84V[\x81\x93P\x90\x81` \x92T\x83\x85\x8A\x01\x01R\x01\x91\x01\x90\x91\x85\x92a\x05\xBEV[\x90P` \x92Pa\x05\x91\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x018a\x05\x84V[\x90`@Q\x91\x82`\0\x82T\x92a\x06(\x84a\x05\x1CV[\x90\x81\x84R`\x01\x94\x85\x81\x16\x90\x81`\0\x14a\x06\x95WP`\x01\x14a\x06RW[PPa\x05\x91\x92P\x03\x83a\x03\xFDV[\x90\x93\x91P`\0R` \x90\x81`\0 \x93`\0\x91[\x81\x83\x10a\x06}WPPa\x05\x91\x93P\x82\x01\x018\x80a\x06DV[\x85T\x88\x84\x01\x85\x01R\x94\x85\x01\x94\x87\x94P\x91\x83\x01\x91a\x06eV[\x91PPa\x05\x91\x94P` \x92P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x018\x80a\x06DV[\x90` \x91a\x06\xD1\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x04\xADV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\x01`\x01`@\x1B\x03a\x04\xAA\x94\x93`\x01`\x01`\xA0\x1B\x03``\x94\x16\x83R\x16` \x82\x01R\x81`@\x82\x01R\x01\x90a\x06\xB8V[4a\x02^W``6`\x03\x19\x01\x12a\x02^Wa\x07$a\x02\x90V[`\x01`\x01`@\x1B\x03`$5\x81\x81\x11a\x02^W` a\x07Ia\x07p\x926\x90`\x04\x01a\x04\x8FV[a\xFF\xFF`D5\x95\x16`\0R`\x0B\x82R`@`\0 \x82`@Q\x94\x83\x86\x80\x95Q\x93\x84\x92\x01a\x04\xADV[\x82\x01\x90\x81R\x03\x01\x90 \x91\x82T\x81\x10\x15a\x02^Wa\x07\x98a\x02\x8C\x91`\x01`\x01`\xA0\x1B\x03\x94a\x05\0V[Pa\x07\xA7`\x01\x82T\x92\x01a\x06\x14V[\x90`@Q\x94\x85\x94\x82`\xA0\x1C\x16\x91\x16\x84a\x06\xDDV[4a\x02^W`@6`\x03\x19\x01\x12a\x02^W`\x04\x805\x90U`$5`\x05U\0[cNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^Wa\x02\x8Ca\x08\x0Ca\x05VV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\xB8V[`D5\x90`\x01`\x01`\x80\x1B\x03\x82\x16\x82\x03a\x02^WV[`d5\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x02^WV[`\x845\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x02^WV[4a\x02^W`\xA06`\x03\x19\x01\x12a\x02^W`\x045`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x92\x03a\x02^W`$5\x90\x80\x82\x16\x82\x03a\x02^Wa\x02\xCB\x92a\x08\xA1a\x08 V[\x91a\x08\xAAa\x086V[\x91a\x08\xB3a\x08LV[`\x80\x95\x86\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x91\x17`\x02Uw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x93\x16\x16\x92\x1Bw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x90\x91\x17`\xC0\x91\x90\x91\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17`\x03UV[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W` a\xFF\xFF`\x01T\x16`@Q\x90\x81R\xF3[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W` `\xFF`\x01T`\x10\x1C\x16`@Q\x90\x15\x15\x81R\xF3[4a\x02^W`\xA06`\x03\x19\x01\x12a\x02^Wa\t\x9Fa\x02\x90V[Pa\t\xAB`$5a\x02\xCDV[`\x01`\x01`@\x1B\x03`D5\x81\x81\x11a\x02^Wa\t\xCB\x906\x90`\x04\x01a\x04\x8FV[`d5\x80\x15\x15\x81\x03a\x02^W`\x845\x92\x83\x11a\x02^Wa\t\xF2a\t\xF8\x936\x90`\x04\x01a\x04\x8FV[\x91a!\xFCV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\xF3[4a\x02^Wa\n\x186a\x032V[\x90a\xFF\xFF\x83\x16`\0R`\n` R`@`\0 ` `@Q\x80\x92\x85\x85\x837\x85\x82\x01\x90\x81R\x03\x01\x90 \x92`\x01\x84\x01\x93a\nR\x85T\x15\x15a#4V[`\x01`\x01`\xA0\x1B\x03\x81T`@\x1C\x163\x03a\n\xE3W\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81Ua\x02\xCB\x94`\0\x91a\n\xA9\x90[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81T\x16\x90UV[U\x7F#\xD2hO9n\x92\xA6\xE2\xFF-\x16\xF9\x8Eo\xEA\0\xD5\x0C\xB2zd\xB51\xBC\x07H\xF70!\x1F\x98`@Q\x80a\n\xDB\x86\x86\x86\x84a$\x0BV[\x03\x90\xA1a%XV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FLayerZeroMock: invalid caller\0\0\0`D\x82\x01R`d\x90\xFD[4a\x02^W` 6`\x03\x19\x01\x12a\x02^Wa\x0BD`\x045a\x02\xCDV[` `@Q0\x81R\xF3[\x90`@`\x03\x19\x83\x01\x12a\x02^W`\x045a\xFF\xFF\x81\x16\x81\x03a\x02^W\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02^Wa\x04\xAA\x91`\x04\x01a\x04\x8FV[4a\x02^W``a\x0B\xC1` a\xFF\xFFa\x0B\xA06a\x0BNV[\x91\x16`\0R`\n\x82R`@`\0 \x82`@Q\x94\x83\x86\x80\x95Q\x93\x84\x92\x01a\x04\xADV[\x82\x01\x90\x81R\x03\x01\x90 `\x01\x81T\x91\x01T`\x01`\x01`\xA0\x1B\x03`@Q\x92`\x01`\x01`@\x1B\x03\x81\x16\x84R`@\x1C\x16` \x83\x01R`@\x82\x01R\xF3[4a\x02^W`@6`\x03\x19\x01\x12a\x02^W` `\x01`\x01`@\x1B\x03a\x0CTa\x0C\x1Fa\x02\x90V[a\xFF\xFF`$5\x91a\x0C/\x83a\x02\xCDV[\x16`\0R`\t\x84R`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T\x16`@Q\x90\x81R\xF3[4a\x02^W` a\xFF\xFF\x81a\x0Cr6a\x032V[\x93\x90\x91\x16`\0R`\x0B\x82R`@`\0 \x83`@Q\x94\x85\x93\x847\x82\x01\x90\x81R\x03\x01\x90 T`@Q\x90\x81R\xF3[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W`\xA0`\x02T`\x03T`@Q\x91`\x01`\x01`\x80\x1B\x03\x90\x81\x81\x16\x84R`\x80\x1C` \x84\x01R\x81\x16`@\x83\x01R`\x01`\x01`@\x1B\x03\x81`\x80\x1C\x16``\x83\x01R`\xC0\x1C`\x80\x82\x01R\xF3[4a\x02^W` `\x01`\x01`@\x1B\x03a\r3\x82a\xFF\xFFa\r\x126a\x0BNV[\x91\x16`\0R`\x08\x82R`@`\0 \x82`@Q\x94\x83\x86\x80\x95Q\x93\x84\x92\x01a\x04\xADV[\x82\x01\x90\x81R\x03\x01\x90 T\x16`@Q\x90\x81R\xF3[4a\x02^W``6`\x03\x19\x01\x12a\x02^Wa\r_a\x02\x90V[`\x01`\x01`@\x1B\x03\x90`$5\x82\x81\x11a\x02^Wa\r\x80\x906\x90`\x04\x01a\x03\x05V[\x91`D5\x84\x81\x11a\x02^Wa\r\x99\x906\x90`\x04\x01a\x03\x05V[a\r\xBEa\r\xB7\x84\x96\x93\x96a\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x83\x86a\x1DEV[\x90a\x0E\x0C`\x01`\x01`\xA0\x1B\x03`\x01\x84\x01\x98a\r\xF3\x8ATa\r\xDF\x81\x15\x15a#4V[\x86T\x92\x83\x16\x86\x14\x90\x81a\x0E\xDCW[Pa#\x7FV[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x85U`@\x1C\x16\x92a\n\x81V[`\0\x80\x97U\x86a\x0EDa\x0E7a\x0E0\x87a\xFF\xFF\x16`\0R`\x08` R`@`\0 \x90V[\x86\x89a\x1DEV[T`\x01`\x01`@\x1B\x03\x16\x90V[\x91\x83;\x15a\x0E\xD8W`@Qb\x1D5g`\xE0\x1B\x81R\x97\x88\x91\x82\x91a\x0Eo\x91\x90\x86\x89\x8C\x8C`\x04\x88\x01a\x1D\xCBV[\x03\x81\x83\x86Z\xF1\x92\x83\x15a\x0E\xD3W\x7Fa$4\xF3\x95\x81\xC8\xE7\xD9\x97F\xC9\xC2\x0Cn\xB0\xCE\x8C\x0E\xB9\x9F\0|W\x19\xD6 \x84\x13p\x95}\x96a\x0E\xB4\x94a\x0E\xBAW[P`@Q\x95\x86\x95\x86a#\xCBV[\x03\x90\xA1\x80\xF3[\x80a\x0E\xC7a\x0E\xCD\x92a\x03\xCAV[\x80a\x02SV[8a\x0E\xA7V[a\x1C\x96V[P\x80\xFD[\x90Pa\x0E\xE96\x87\x8Da\x04XV[` \x81Q\x91\x01 \x148a\r\xEDV[4a\x02^W` 6`\x03\x19\x01\x12a\x02^W`\x045`\x06U\0[4a\x02^W`@6`\x03\x19\x01\x12a\x02^Wa\x02\xCB`\x045a\x0F0\x81a\x02\xCDV[`\x01`\x01`\xA0\x1B\x03`$5\x91a\x0FE\x83a\x02\xCDV[\x16`\0R`\0` R`@`\0 \x90`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x82T\x16\x17\x90UV[4a\x02^W`\xC06`\x03\x19\x01\x12a\x02^Wa\x0F\x87a\x02\x90V[`\x01`\x01`@\x1B\x03`$5\x81\x81\x11a\x02^Wa\x0F\xA7\x906\x90`\x04\x01a\x03\x05V[\x91\x90\x92`D5\x92a\x0F\xB7\x84a\x02\xCDV[a\x0F\xBFa\x086V[\x94`\xA45\x84\x81\x11a\x02^Wa\x0F\xD8\x906\x90`\x04\x01a\x03\x05V[\x92\x90\x93`\x0CT\x95`\x01\x96\x87`\xFF\x82`\x08\x1C\x16\x03a\x13\xE2Wa\x02\0\x90a\xFF\0\x19\x16\x17`\x0CU\x86a\x10\x1Fa\x10\x18\x84a\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x85\x87a\x1DEV[a\x10\x88\x83a\x10Ea\x10>\x87a\xFF\xFF\x16`\0R`\x08` R`@`\0 \x90V[\x88\x8Aa\x1DEV[a\x10~a\x10aa\x10\\\x83T`\x01`\x01`@\x1B\x03\x16\x90V[a\x1B{V[\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x82\x16\x17\x90\x92UV[\x16\x84\x8D\x16\x14a\x1D^V[\x01T\x15a\x11\x8EWPa\x10\xED\x94\x93\x92a\x10\xE6\x92a\x10\xB5a\x10\xBB\x93a\xFF\xFF\x16`\0R`\x0B` R`@`\0 \x90V[\x91a\x1DEV[\x96a\x10\xD6a\x10\xC7a\x04\x1EV[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[`\x01`\x01`@\x1B\x03\x16` \x87\x01RV[6\x91a\x04XV[`@\x83\x01R\x82T\x15a\x11\x7FWa\x11\x03\x82\x84a PV[`\0\x81[a\x114W[PPa\x11\x1Aa\x11 \x92a\x04\xE6V[\x90a\x1F\x1DV[a\x02\xCBa\x01\0a\xFF\0\x19`\x0CT\x16\x17`\x0CUV[a\x11>\x84Ta\x1B\x98V[\x81\x10\x15a\x11zW\x80a\x11oa\x11Va\x11t\x93\x87a\x05\0V[Pa\x11ia\x11c\x84a \x87V[\x88a\x05\0V[\x90a!\x8FV[a xV[\x81a\x11\x07V[a\x11\x0CV[Pa\x11\x89\x91a PV[a\x11 V[\x90\x95a\x11\xA5\x90\x98\x92\x95\x93\x97\x98T`\xFF\x90`\x10\x1C\x16\x90V[\x15a\x12\xD0W\x90\x7F\x0F\x9EM\x95\xB6/\x08\"-a+Z\xB9 9\xCD\x8F\xBB\xBE\xA5P\xA9^\x8D\xF9\xF9'Ck\xBD\xF5\xDB\x97a\x12\xA7a\x12\xBC\x96\x95\x94\x93a\x11\xE26\x87\x87a\x04XV[` \x81Q\x91\x01 a\x12\x06a\x11\xF4a\x04\x1EV[\x92\x88\x16\x83\x90`\x01`\x01`@\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x84\x16` \x83\x01R`@\x82\x01Ra\x12<a\x125\x8Aa\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x88\x8Ca\x1DEV[\x81Q\x81T` \x84\x01Q`\x01`\x01`@\x1B\x03\x90\x92\x16\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x17`@\x91\x82\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16\x17\x82U\x90\x91\x01Q`\x01\x90\x91\x01UV[a\x12\xAFa\x1B\xB4V[\x94`@Q\x98\x89\x98\x89a\x1E\x0BV[\x03\x90\xA1a\x11\x89b\xFF\0\0\x19`\x01T\x16`\x01UV[\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x80;\x15a\x02^W`\0\x86\x83\x92\x8A\x83\x8Ba\x13\r\x8A\x8A`@Q\x99\x8A\x98\x89\x97\x88\x96b\x1D5g`\xE0\x1B\x88R`\x04\x88\x01a\x1D\xCBV[\x03\x92`\x845\xF1\x90\x81a\x13\xCFW[Pa\x13\xC2W\x7F\x0F\x9EM\x95\xB6/\x08\"-a+Z\xB9 9\xCD\x8F\xBB\xBE\xA5P\xA9^\x8D\xF9\xF9'Ck\xBD\xF5\xDB\x97a\x12\xBC\x95a\x13\xB6a\x13Pa\x1B\xC7V[\x96a\x13\\6\x88\x88a\x04XV[` \x81Q\x91\x01 a\x13\x80a\x13na\x04\x1EV[\x92\x89\x16\x83\x90`\x01`\x01`@\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x85\x16` \x83\x01R`@\x82\x01Ra\x12<a\x13\xAF\x8Ba\xFF\xFF\x16`\0R`\n` R`@`\0 \x90V[\x84\x8Da\x1DEV[`@Q\x98\x89\x98\x89a\x1E\x0BV[PPPPPPPPa\x11 V[\x80a\x0E\xC7a\x13\xDC\x92a\x03\xCAV[8a\x13\x1AV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FLayerZeroMock: no receive reentr`D\x82\x01Rcancy`\xE0\x1B`d\x82\x01R`\x84\x90\xFD[`\xC06`\x03\x19\x01\x12a\x02^Wa\x14Ga\x02\x90V[`\x01`\x01`@\x1B\x03`$5\x81\x81\x11a\x02^Wa\x14g\x906\x90`\x04\x01a\x04\x8FV[`D5\x82\x81\x11a\x02^Wa\x14\x7F\x906\x90`\x04\x01a\x03\x05V[`d\x94\x91\x945\x93a\x14\x8F\x85a\x02\xCDV[`\x845\x90a\x14\x9C\x82a\x02\xCDV[`\xA45\x90\x81\x11a\x02^Wa\x14\xB4\x906\x90`\x04\x01a\x04\x8FV[\x93`\x0CT`\x01`\xFF\x82\x16\x03a\x17'W\x87\x91\x84\x91`\xFF\x19\x16`\x02\x17`\x0CU\x80Q`(\x14a\x14\xDF\x90a\x1A4V[`\x14\x01Q\x95`\x01`\x01`\xA0\x1B\x03\x80\x98\x81\x92\x82a\x15\x0E\x8B`\x01`\x01`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[T`\x01`\x01`\xA0\x1B\x03\x16\x16\x98a\x15%\x8A\x15\x15a\x1A\x95V[\x81Q\x15\x15`\0\x14a\x15\xB4a\x15\xA5a\x10\\\x9Fa\x16:\x9Ba\x15\xE5\x9Aa\x15\xD7\x97a\x15f\x93a\x15\xD1\x97a\x17\x12Wa\x15a\x91\x9C\x8D\x94[\x16\x15\x15\x926\x91a\x04XV[a!\xFCV[P\x93a\x15t\x854\x10\x15a\x1B\x07V[a\x15\x8D3\x91a\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x90`\x01`\x01`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[\x9E\x8FT`\x01`\x01`@\x1B\x03\x16\x90V[\x8ETg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x82\x16\x17\x90\x9EUV[4a\x1B\xA7V[\x80a\x16\xEDW[PPPa\x1C\xA2V[\x90\x80\x93P\x98\x91\x98a\x16\xA2W[PPP`@Q\x92a\x10\xE6\x84a\x16,\x883` \x84\x01\x90`(\x92k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x92``\x1B\x16\x83R``\x1B\x16`\x14\x82\x01R\x01\x90V[\x03`\x1F\x19\x81\x01\x86R\x85a\x03\xFDV[`\x01Ta\xFF\xFF\x16\x83;\x15a\x02^Wa\x16n`\0\x96\x92\x87\x93`@Q\x99\x8A\x98\x89\x97\x88\x96c\xC2\xFAH\x13`\xE0\x1B\x88R`\x04\x88\x01a\x1CCV[\x03\x92Z\xF1\x80\x15a\x0E\xD3Wa\x16\x8FW[a\x02\xCB`\x01`\xFF\x19`\x0CT\x16\x17`\x0CUV[\x80a\x0E\xC7a\x16\x9C\x92a\x03\xCAV[8a\x16}V[\x16`\0\x80\x80\x80\x85\x85Z\xF1a\x16\xB4a\x1B\xC7V[P\x15a\x16\xC1W[\x80a\x15\xF1V[\x7F,z\x96L\xA3\xDE^\xC1\xD4-\x98\"\xF9\xBB\xD0\xEB\x14*Y\xCC\x9F\x85^\x9D\x93\x81;w1\x92\xC7\xA3`\0\x80\xA38\x80a\x16\xBBV[`\0\x80\x80\x93\x92a\x17\n\x95\x82\x94\x16Z\xF1a\x17\x04a\x1B\xC7V[Pa\x1B\xF7V[\x878\x80a\x15\xDDV[a\x15a\x91Pa\x17\x1Fa\x05VV[\x9C\x8D\x94a\x15VV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FLayerZeroMock: no send reentranc`D\x82\x01R`y`\xF8\x1B`d\x82\x01R`\x84\x90\xFD[4a\x02^W` 6`\x03\x19\x01\x12a\x02^W` `\x045a\x17\x95\x81a\x02\xCDV[`\x01`\x01`\xA0\x1B\x03\x80\x91\x16`\0R`\0\x82R`@`\0 T\x16`@Q\x90\x81R\xF3[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W` `\x02`\xFF`\x0CT`\x08\x1C\x16\x14`@Q\x90\x81R\xF3[4a\x02^W`\x806`\x03\x19\x01\x12a\x02^Wa\x17\xF6a\x02\x90V[Pa\x17\xFFa\x02\xA1V[P`d5`\x01`\x01`@\x1B\x03\x81\x11a\x02^Wa\x02\xCB\x906\x90`\x04\x01a\x04\x8FV[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W`\x01\x80Tb\xFF\0\0\x19\x16b\x01\0\0\x17\x90U\0[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W` `\x02`\xFF`\x0CT\x16\x14`@Q\x90\x81R\xF3[4a\x02^W`\x806`\x03\x19\x01\x12a\x02^Wa\x18\x80a\x02\x90V[Pa\x18\x89a\x02\xA1V[Pa\x18\x95`D5a\x02\xCDV[a\x02\x8C`@Qa\x18\xA4\x81a\x03\xE2V[`\0\x81R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\xB8V[4a\x02^W`\x006`\x03\x19\x01\x12a\x02^W` `\x06T`@Q\x90\x81R\xF3[4a\x02^W` \x80`\x03\x196\x01\x12a\x02^W`\x01`\x01`@\x1B\x03`\x045\x81\x81\x11a\x02^Wa\x19\x0C\x906\x90`\x04\x01a\x04\x8FV[\x91\x82Q\x91\x82\x11a\x03\xDDWa\x19*\x82a\x19%`\x07Ta\x05\x1CV[a\x1E\x82V[\x80`\x1F\x83\x11`\x01\x14a\x19eWP\x81\x92`\0\x92a\x19ZW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16`\x01\x91\x90\x91\x1B\x17`\x07U\0[\x01Q\x90P8\x80a\x19AV[\x90`\x1F\x19\x83\x16\x93a\x19\x98`\x07`\0R\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88\x90V[\x92`\0\x90[\x86\x82\x10a\x19\xD4WPP\x83`\x01\x95\x10a\x19\xBBW[PPP\x81\x1B\x01`\x07U\0[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a\x19\xB0V[\x80`\x01\x85\x96\x82\x94\x96\x86\x01Q\x81U\x01\x95\x01\x93\x01\x90a\x19\x9DV[4a\x02^W` `\x01`\x01`@\x1B\x03a\xFF\xFF\x82a\x1A\x086a\x032V[\x93\x90\x91\x16`\0R`\x08\x82R`@`\0 \x83`@Q\x94\x85\x93\x847\x82\x01\x90\x81R\x03\x01\x90 T\x16`@Q\x90\x81R\xF3[\x15a\x1A;WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FLayerZeroMock: incorrect remote `D\x82\x01Rkaddress size`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[\x15a\x1A\x9CWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FLayerZeroMock: destination Layer`D\x82\x01R\x7FZero Endpoint not found\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a\x1B\x0EWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FLayerZeroMock: not enough native`D\x82\x01Rh for fees`\xB8\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x80\x91\x16\x90\x81\x14a\x1B\x93W`\x01\x01\x90V[a\x1BeV[`\0\x19\x81\x01\x91\x90\x82\x11a\x1B\x93WV[\x91\x90\x82\x03\x91\x82\x11a\x1B\x93WV[`@Q\x90a\x1B\xC1\x82a\x03\xE2V[`\0\x82RV[=\x15a\x1B\xF2W=\x90a\x1B\xD8\x82a\x04=V[\x91a\x1B\xE6`@Q\x93\x84a\x03\xFDV[\x82R=`\0` \x84\x01>V[``\x90V[\x15a\x1B\xFEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FLayerZeroMock: failed to refund\0`D\x82\x01R`d\x90\xFD[\x92\x94\x91\x93`\x01`\x01`\xA0\x1B\x03a\x1Cwa\x04\xAA\x98\x96a\xFF\xFF`\x01`\x01`@\x1B\x03\x95\x16\x87R`\xC0` \x88\x01R`\xC0\x87\x01\x90a\x06\xB8V[\x96\x16`@\x85\x01R\x16``\x83\x01R`\x80\x82\x01R`\xA0\x81\x84\x03\x91\x01Ra\x06\xB8V[`@Q=`\0\x82>=\x90\xFD[`\0\x80\x82Q`\"\x81\x14\x15\x90\x81a\x1D:W[Pa\x1D(W`\x02\x83\x01Q\x93`\"\x84\x01Q\x93a\xFF\xFF\x86\x16`\x01\x81\x14\x15\x80a\x1D\x1DW[a\x1D\x0BW\x85\x15a\x1C\xF9W`\x02\x14a\x1C\xE8WPV[\x92P\x90P`V`B\x83\x01Q\x92\x01Q\x90V[`@Qc!c\"]`\xE2\x1B\x81R`\x04\x90\xFD[`@Qco\xC3\xDA\xA3`\xE1\x1B\x81R`\x04\x90\xFD[P`\x02\x81\x14\x15a\x1C\xD4V[`@Qc\xCE\xF8\x0E\xA3`\xE0\x1B\x81R`\x04\x90\xFD[`B\x91P\x108a\x1C\xB3V[` \x91\x92\x83`@Q\x94\x85\x93\x847\x82\x01\x90\x81R\x03\x01\x90 \x90V[\x15a\x1DeWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FLayerZeroMock: wrong nonce\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91a\x1D\xF7\x90a\x04\xAA\x97\x95\x96\x94a\xFF\xFF`\x01`\x01`@\x1B\x03\x94\x16\x85R`\x80` \x86\x01R`\x80\x85\x01\x91a\x1D\xAAV[\x94\x16`@\x82\x01R``\x81\x85\x03\x91\x01Ra\x1D\xAAV[\x96\x92\x94`\x01`\x01`\xA0\x1B\x03a\x1ECa\x04\xAA\x9A\x98\x94a\x1E]\x98a\xFF\xFF`\x01`\x01`@\x1B\x03\x96\x16\x8CR`\xC0` \x8D\x01R`\xC0\x8C\x01\x91a\x1D\xAAV[\x95\x16`@\x89\x01R\x16``\x87\x01R\x85\x83\x03`\x80\x87\x01Ra\x1D\xAAV[\x91`\xA0\x81\x84\x03\x91\x01Ra\x06\xB8V[\x81\x81\x10a\x1EvWPPV[`\0\x81U`\x01\x01a\x1EkV[\x90`\x1F\x82\x11a\x1E\x8FWPPV[a\x05\x91\x91`\x07`\0R\x7F\xA6l\xC9(\xB5\xED\xB8*\xF9\xBDI\x92)T\x15Z\xB7\xB0\x94&\x94\xBE\xA4\xCEDf\x1D\x9A\x876\xC6\x88\x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x1E\xD9W[`\x1F\x01`\x05\x1C\x01\x90a\x1EkV[\x90\x91P\x81\x90a\x1E\xCCV[\x91\x90`\x1F\x81\x11a\x1E\xF2WPPPV[a\x05\x91\x92`\0R` `\0 \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a\x1E\xD9W`\x1F\x01`\x05\x1C\x01\x90a\x1EkV[\x90a KW\x81Q\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x81U` \x82\x81\x01Q\x82Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x82U`\x01`\x01`@\x1B\x03\x90`@`\x01\x80\x94\x01\x94\x01Q\x80Q\x92\x83\x11a\x03\xDDWa\x1F\xA8\x83a\x1F\xA2\x87Ta\x05\x1CV[\x87a\x1E\xE3V[\x81`\x1F\x84\x11`\x01\x14a\x1F\xE1WP\x92\x82\x93\x91\x83\x92`\0\x94a\x1F\xD6W[PP\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90UV[\x01Q\x92P8\x80a\x1F\xC3V[\x91\x90\x83`\x1F\x19\x81\x16a\x1F\xF8\x88`\0R` `\0 \x90V[\x94`\0\x90[\x88\x83\x83\x10a 1WPPP\x10a \x18W[PPP\x81\x1B\x01\x90UV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a \x0EV[\x85\x87\x01Q\x88U\x90\x96\x01\x95\x94\x85\x01\x94\x87\x93P\x90\x81\x01\x90a\x1F\xFDV[a\x07\xDAV[\x90\x81T\x91h\x01\0\0\0\0\0\0\0\0\x83\x10\x15a\x03\xDDW\x82a\x11\x1A\x91`\x01a\x05\x91\x95\x01\x81Ua\x05\0V[`\0\x19\x81\x14a\x1B\x93W`\x01\x01\x90V[\x90`\x01\x82\x01\x80\x92\x11a\x1B\x93WV[\x91\x90\x82\x01\x80\x92\x11a\x1B\x93WV[\x91\x90\x91\x82\x81\x14a!\x8AWa \xB6\x83Ta\x05\x1CV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\xDDWa \xD8\x81a \xD2\x84Ta\x05\x1CV[\x84a\x1E\xE3V[`\0\x93`\x1F\x82\x11`\x01\x14a!\x14W\x93\x81\x92\x93\x94`\0\x92a!\tW[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90UV[\x01T\x90P8\x80a \xF3V[a!(`\x1F\x19\x83\x16\x91`\0R` `\0 \x90V[\x94a!8\x84`\0R` `\0 \x90V[\x91\x81[\x81\x81\x10a!rWP\x95\x83`\x01\x95\x96\x97\x10a!YWPPP\x81\x1B\x01\x90UV[\x01T`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80a \x0EV[\x87\x83\x01T\x84U`\x01\x93\x84\x01\x93\x90\x92\x01\x91` \x01a!;V[P\x90PV[\x90a KW\x81\x81\x03a!\x9FWPPV[`\x01\x80\x83a!\xD0`\x01`\x01`\xA0\x1B\x03a\x05\x91\x96T\x16\x85\x90`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x82T\x16\x17\x90UV[\x80T\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x90\x91\x16\x17\x84U\x01\x91\x01a \xA2V[\x82Q\x91\x92`\0\x92\x83\x92\x91\x90\x15a#\"Wa\"\x19\x90\x91[Q\x91a\x1C\xA2V[P`\x02a\xFF\xFF`\0\x95\x94\x93\x95\x93\x16\x14a#\x03W[P`\x02T\x90\x81`\x80\x1C\x90`\x03T\x94\x85`\x80\x1C`\x01`\x01`@\x1B\x03\x16\x90a\"R\x91a \x95V[a\"\\\x90\x83a$\xAFV[a\"e\x91a \x95V[\x91`\x01`\x01`\x80\x1B\x03\x16\x91\x82a\"z\x91a$\xAFV[d\x02T\x0B\xE4\0\x90\x04\x93`\xC0\x1Ca\"\x8F\x91a&\xCFV[\x90a\"\x99\x91a&\xCFV[d\x02T\x0B\xE4\0`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x04`\x01`\x01`\x80\x1B\x03\x16a\"\xBD\x91a$\xAFV[a\"\xC6\x91a \x95V[\x92`\x06T\x91a\"\xD6\x83\x86\x84a&GV[\x91\x15a\"\xF4WPa\"\xF1\x92\x93a\"\xEC\x91\x94a \x95V[a \x95V[\x91V[\x93a\"\xF1\x93Pa\"\xEC\x91a \x95V[\x90Pa#\x1C\x81`\x01`\x01`\x80\x1B\x03`\x03T\x16\x10\x15a&tV[8a\"-V[Pa\"\x19a#.a\x05VV[\x91a\"\x12V[\x15a#;WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FLayerZeroMock: no stored payload`D\x82\x01R\xFD[\x15a#\x86WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FLayerZeroMock: invalid payload\0\0`D\x82\x01R`d\x90\xFD[\x93\x95\x94\x90a#\xFE`\x01`\x01`\xA0\x1B\x03\x93``\x95a\xFF\xFF`\x01`\x01`@\x1B\x03\x94\x16\x88R`\x80` \x89\x01R`\x80\x88\x01\x91a\x1D\xAAV[\x96\x16`@\x85\x01R\x16\x91\x01RV[`@\x90a\xFF\xFFa\x04\xAA\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a\x1D\xAAV[\x90`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x93\x82\x82\x10\x85\x83\x11\x17a\x03\xDDW`\x01a$l\x91`@\x93\x84R\x84\x96\x81T`\x01`\x01`\xA0\x1B\x03\x81\x16\x87R`\xA0\x1C\x16` \x86\x01R\x01a\x06\x14V[\x91\x01RV[\x91a$\x9B\x90a\x04\xAA\x96\x94a\xFF\xFF`\x01`\x01`@\x1B\x03\x94\x16\x85R`\x80` \x86\x01R`\x80\x85\x01\x91a\x1D\xAAV[\x93\x16`@\x82\x01R``\x81\x84\x03\x91\x01Ra\x06\xB8V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x1B\x93WV[\x80T\x80\x15a%BW`\0\x19\x01\x90a$\xD9\x82\x82a\x05\0V[a KW\x80`\0`\x01\x92U\x01a$\xEF\x81Ta\x05\x1CV[\x90\x81a$\xFAWPPUV[`\x1F\x82\x11`\x01\x14a%\rW`\0\x91PUUV[a%/a%?\x92\x82`\0R`\x01`\x1F` `\0 \x92\x01`\x05\x1C\x82\x01\x91\x01a\x1EkV[`\0\x90\x80\x82R\x81` \x81 \x91UUV[UV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[\x92\x91a%|a%u\x85a\xFF\xFF\x16`\0R`\x0B` R`@`\0 \x90V[\x82\x84a\x1DEV[\x92[\x83T\x80\x15a&?Wa%\x9Ba%\x95a%\xA1\x92a\x1B\x98V[\x86a\x05\0V[Pa$&V[a%\xC4a%\xB8a%\xB8\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90a%\xD9` \x82\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x91`@\x80\x92\x01Q\x90\x80;\x15a\x02^Wa&\x0E\x93\x89\x88\x94Q\x80\x96\x81\x95\x82\x94b\x1D5g`\xE0\x1B\x84R\x8B`\0\x99\x8A\x96`\x04\x87\x01a$qV[\x03\x92Z\xF1\x80\x15a\x0E\xD3Wa&,W[Pa&'\x84a$\xC2V[a%~V[\x80a\x0E\xC7a&9\x92a\x03\xCAV[8a&\x1DV[P\x93PPPPV[\x90\x91\x90\x15a&WWPP`\x04T\x90V[\x81\x01\x80\x91\x11a\x1B\x93Wa&pa'\x10\x91`\x05T\x90a$\xAFV[\x04\x90V[\x15a&{WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FLayerZeroMock: dstNativeAmt too `D\x82\x01Re\x03c\x0B\x93;)`\xD5\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x91`\x01`\x01`\x80\x1B\x03\x80\x80\x94\x16\x91\x16\x02\x91\x82\x16\x91\x82\x03a\x1B\x93WV\xFE\xA2dipfsX\"\x12 \n\xC6\xEE\xC7\xD2W\xAE\xCBb\xEB\xB2B\xF5\x0F\xFFXz\xFF\x88\xD3\xCCN\xE6$\x14y\r\xC7\xE5\xB0\xBC\x8CdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static LZENDPOINTMOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LZEndpointMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LZEndpointMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LZEndpointMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LZEndpointMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LZEndpointMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LZEndpointMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LZEndpointMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LZENDPOINTMOCK_ABI.clone(),
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
                LZENDPOINTMOCK_ABI.clone(),
                LZENDPOINTMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `blockNextMsg` (0xd23104f1) function
        pub fn block_next_msg(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 49, 4, 241], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultAdapterParams` (0x272bd384) function
        pub fn default_adapter_params(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([39, 43, 211, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `estimateFees` (0x40a7bb10) function
        pub fn estimate_fees(
            &self,
            dst_chain_id: u16,
            user_application: ::ethers::core::types::Address,
            payload: ::ethers::core::types::Bytes,
            pay_in_zro: bool,
            adapter_params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [64, 167, 187, 16],
                    (dst_chain_id, user_application, payload, pay_in_zro, adapter_params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `forceResumeReceive` (0x42d65a8d) function
        pub fn force_resume_receive(
            &self,
            src_chain_id: u16,
            path: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 214, 90, 141], (src_chain_id, path))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainId` (0x3408e470) function
        pub fn get_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConfig` (0xf5ecbdbc) function
        pub fn get_config(
            &self,
            p0: u16,
            p1: u16,
            p2: ::ethers::core::types::Address,
            p3: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([245, 236, 189, 188], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInboundNonce` (0xfdc07c70) function
        pub fn get_inbound_nonce(
            &self,
            chain_id: u16,
            path: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([253, 192, 124, 112], (chain_id, path))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLengthOfQueue` (0x7f6df8e6) function
        pub fn get_length_of_queue(
            &self,
            src_chain_id: u16,
            src_address: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([127, 109, 248, 230], (src_chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOutboundNonce` (0x7a145748) function
        pub fn get_outbound_nonce(
            &self,
            chain_id: u16,
            src_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([122, 20, 87, 72], (chain_id, src_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReceiveLibraryAddress` (0x71ba2fd6) function
        pub fn get_receive_library_address(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([113, 186, 47, 214], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReceiveVersion` (0xda1a7c9a) function
        pub fn get_receive_version(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([218, 26, 124, 154], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSendLibraryAddress` (0x9c729da1) function
        pub fn get_send_library_address(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([156, 114, 157, 161], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSendVersion` (0x096568f6) function
        pub fn get_send_version(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([9, 101, 104, 246], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasStoredPayload` (0x0eaf6ea6) function
        pub fn has_stored_payload(
            &self,
            src_chain_id: u16,
            path: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([14, 175, 110, 166], (src_chain_id, path))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `inboundNonce` (0x9924d33b) function
        pub fn inbound_nonce(
            &self,
            p0: u16,
            p1: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([153, 36, 211, 59], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isReceivingPayload` (0xca066b35) function
        pub fn is_receiving_payload(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([202, 6, 107, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSendingPayload` (0xe97a448a) function
        pub fn is_sending_payload(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 122, 68, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lzEndpointLookup` (0xc81b383a) function
        pub fn lz_endpoint_lookup(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([200, 27, 56, 58], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockChainId` (0xdb14f305) function
        pub fn mock_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([219, 20, 243, 5], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `msgsToDeliver` (0x12a9ee6b) function
        pub fn msgs_to_deliver(
            &self,
            p0: u16,
            p1: ::ethers::core::types::Bytes,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u64, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([18, 169, 238, 107], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nextMsgBlocked` (0x3e0dd83e) function
        pub fn next_msg_blocked(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([62, 13, 216, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `oracleFee` (0xf9cd3ceb) function
        pub fn oracle_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([249, 205, 60, 235], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `outboundNonce` (0xb2086499) function
        pub fn outbound_nonce(
            &self,
            p0: u16,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([178, 8, 100, 153], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `protocolFeeConfig` (0x07d3277f) function
        pub fn protocol_fee_config(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([7, 211, 39, 127], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `receivePayload` (0xc2fa4813) function
        pub fn receive_payload(
            &self,
            src_chain_id: u16,
            path: ::ethers::core::types::Bytes,
            dst_address: ::ethers::core::types::Address,
            nonce: u64,
            gas_limit: ::ethers::core::types::U256,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [194, 250, 72, 19],
                    (src_chain_id, path, dst_address, nonce, gas_limit, payload),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayerFeeConfig` (0x907c5e7e) function
        pub fn relayer_fee_config(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u128, u128, u128, u64, u64),
        > {
            self.0
                .method_hash([144, 124, 94, 126], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `retryPayload` (0xaaff5f16) function
        pub fn retry_payload(
            &self,
            src_chain_id: u16,
            path: ::ethers::core::types::Bytes,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 255, 95, 22], (src_chain_id, path, payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `send` (0xc5803100) function
        pub fn send(
            &self,
            chain_id: u16,
            path: ::ethers::core::types::Bytes,
            payload: ::ethers::core::types::Bytes,
            refund_address: ::ethers::core::types::Address,
            zro_payment_address: ::ethers::core::types::Address,
            adapter_params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [197, 128, 49, 0],
                    (
                        chain_id,
                        path,
                        payload,
                        refund_address,
                        zro_payment_address,
                        adapter_params,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0xcbed8b9c) function
        pub fn set_config(
            &self,
            p0: u16,
            p1: u16,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 237, 139, 156], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDefaultAdapterParams` (0xfbba623b) function
        pub fn set_default_adapter_params(
            &self,
            adapter_params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 186, 98, 59], adapter_params)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDestLzEndpoint` (0xc08f15a1) function
        pub fn set_dest_lz_endpoint(
            &self,
            dest_addr: ::ethers::core::types::Address,
            lz_endpoint_addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 143, 21, 161], (dest_addr, lz_endpoint_addr))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOracleFee` (0xb6d9ef60) function
        pub fn set_oracle_fee(
            &self,
            oracle_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 217, 239, 96], oracle_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProtocolFee` (0x240de277) function
        pub fn set_protocol_fee(
            &self,
            zro_fee: ::ethers::core::types::U256,
            native_bp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 13, 226, 119], (zro_fee, native_bp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReceiveVersion` (0x10ddb137) function
        pub fn set_receive_version(
            &self,
            p0: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 221, 177, 55], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRelayerPrice` (0x2c365e25) function
        pub fn set_relayer_price(
            &self,
            dst_price_ratio: u128,
            dst_gas_price_in_wei: u128,
            dst_native_amt_cap: u128,
            base_gas: u64,
            gas_per_byte: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [44, 54, 94, 37],
                    (
                        dst_price_ratio,
                        dst_gas_price_in_wei,
                        dst_native_amt_cap,
                        base_gas,
                        gas_per_byte,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSendVersion` (0x07e0db17) function
        pub fn set_send_version(
            &self,
            p0: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 224, 219, 23], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `storedPayload` (0x76a386dc) function
        pub fn stored_payload(
            &self,
            p0: u16,
            p1: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u64, ::ethers::core::types::Address, [u8; 32]),
        > {
            self.0
                .method_hash([118, 163, 134, 220], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PayloadCleared` event
        pub fn payload_cleared_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PayloadClearedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PayloadStored` event
        pub fn payload_stored_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PayloadStoredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UaForceResumeReceive` event
        pub fn ua_force_resume_receive_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UaForceResumeReceiveFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ValueTransferFailed` event
        pub fn value_transfer_failed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ValueTransferFailedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LZEndpointMockEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LZEndpointMock<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `GasTooLow` with signature `GasTooLow()` and selector `0x858c8974`
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
    #[etherror(name = "GasTooLow", abi = "GasTooLow()")]
    pub struct GasTooLow;
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
    ///Custom Error type `UnsupportedTxType` with signature `UnsupportedTxType()` and selector `0xdf87b546`
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
    #[etherror(name = "UnsupportedTxType", abi = "UnsupportedTxType()")]
    pub struct UnsupportedTxType;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LZEndpointMockErrors {
        GasTooLow(GasTooLow),
        InvalidAdapterParams(InvalidAdapterParams),
        UnsupportedTxType(UnsupportedTxType),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LZEndpointMockErrors {
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
                = <GasTooLow as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GasTooLow(decoded));
            }
            if let Ok(decoded)
                = <InvalidAdapterParams as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidAdapterParams(decoded));
            }
            if let Ok(decoded)
                = <UnsupportedTxType as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnsupportedTxType(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LZEndpointMockErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::GasTooLow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAdapterParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsupportedTxType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LZEndpointMockErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <GasTooLow as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidAdapterParams as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnsupportedTxType as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LZEndpointMockErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GasTooLow(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAdapterParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnsupportedTxType(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LZEndpointMockErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<GasTooLow> for LZEndpointMockErrors {
        fn from(value: GasTooLow) -> Self {
            Self::GasTooLow(value)
        }
    }
    impl ::core::convert::From<InvalidAdapterParams> for LZEndpointMockErrors {
        fn from(value: InvalidAdapterParams) -> Self {
            Self::InvalidAdapterParams(value)
        }
    }
    impl ::core::convert::From<UnsupportedTxType> for LZEndpointMockErrors {
        fn from(value: UnsupportedTxType) -> Self {
            Self::UnsupportedTxType(value)
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
    #[ethevent(
        name = "PayloadCleared",
        abi = "PayloadCleared(uint16,bytes,uint64,address)"
    )]
    pub struct PayloadClearedFilter {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub nonce: u64,
        pub dst_address: ::ethers::core::types::Address,
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
        name = "PayloadStored",
        abi = "PayloadStored(uint16,bytes,address,uint64,bytes,bytes)"
    )]
    pub struct PayloadStoredFilter {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
        pub dst_address: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "UaForceResumeReceive",
        abi = "UaForceResumeReceive(uint16,bytes)"
    )]
    pub struct UaForceResumeReceiveFilter {
        pub chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
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
        name = "ValueTransferFailed",
        abi = "ValueTransferFailed(address,uint256)"
    )]
    pub struct ValueTransferFailedFilter {
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quantity: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LZEndpointMockEvents {
        PayloadClearedFilter(PayloadClearedFilter),
        PayloadStoredFilter(PayloadStoredFilter),
        UaForceResumeReceiveFilter(UaForceResumeReceiveFilter),
        ValueTransferFailedFilter(ValueTransferFailedFilter),
    }
    impl ::ethers::contract::EthLogDecode for LZEndpointMockEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PayloadClearedFilter::decode_log(log) {
                return Ok(LZEndpointMockEvents::PayloadClearedFilter(decoded));
            }
            if let Ok(decoded) = PayloadStoredFilter::decode_log(log) {
                return Ok(LZEndpointMockEvents::PayloadStoredFilter(decoded));
            }
            if let Ok(decoded) = UaForceResumeReceiveFilter::decode_log(log) {
                return Ok(LZEndpointMockEvents::UaForceResumeReceiveFilter(decoded));
            }
            if let Ok(decoded) = ValueTransferFailedFilter::decode_log(log) {
                return Ok(LZEndpointMockEvents::ValueTransferFailedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LZEndpointMockEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PayloadClearedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PayloadStoredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UaForceResumeReceiveFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValueTransferFailedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<PayloadClearedFilter> for LZEndpointMockEvents {
        fn from(value: PayloadClearedFilter) -> Self {
            Self::PayloadClearedFilter(value)
        }
    }
    impl ::core::convert::From<PayloadStoredFilter> for LZEndpointMockEvents {
        fn from(value: PayloadStoredFilter) -> Self {
            Self::PayloadStoredFilter(value)
        }
    }
    impl ::core::convert::From<UaForceResumeReceiveFilter> for LZEndpointMockEvents {
        fn from(value: UaForceResumeReceiveFilter) -> Self {
            Self::UaForceResumeReceiveFilter(value)
        }
    }
    impl ::core::convert::From<ValueTransferFailedFilter> for LZEndpointMockEvents {
        fn from(value: ValueTransferFailedFilter) -> Self {
            Self::ValueTransferFailedFilter(value)
        }
    }
    ///Container type for all input parameters for the `blockNextMsg` function with signature `blockNextMsg()` and selector `0xd23104f1`
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
    #[ethcall(name = "blockNextMsg", abi = "blockNextMsg()")]
    pub struct BlockNextMsgCall;
    ///Container type for all input parameters for the `defaultAdapterParams` function with signature `defaultAdapterParams()` and selector `0x272bd384`
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
    #[ethcall(name = "defaultAdapterParams", abi = "defaultAdapterParams()")]
    pub struct DefaultAdapterParamsCall;
    ///Container type for all input parameters for the `estimateFees` function with signature `estimateFees(uint16,address,bytes,bool,bytes)` and selector `0x40a7bb10`
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
        name = "estimateFees",
        abi = "estimateFees(uint16,address,bytes,bool,bytes)"
    )]
    pub struct EstimateFeesCall {
        pub dst_chain_id: u16,
        pub user_application: ::ethers::core::types::Address,
        pub payload: ::ethers::core::types::Bytes,
        pub pay_in_zro: bool,
        pub adapter_params: ::ethers::core::types::Bytes,
    }
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
        pub path: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
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
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
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
    pub struct GetConfigCall(
        pub u16,
        pub u16,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `getInboundNonce` function with signature `getInboundNonce(uint16,bytes)` and selector `0xfdc07c70`
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
    #[ethcall(name = "getInboundNonce", abi = "getInboundNonce(uint16,bytes)")]
    pub struct GetInboundNonceCall {
        pub chain_id: u16,
        pub path: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getLengthOfQueue` function with signature `getLengthOfQueue(uint16,bytes)` and selector `0x7f6df8e6`
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
    #[ethcall(name = "getLengthOfQueue", abi = "getLengthOfQueue(uint16,bytes)")]
    pub struct GetLengthOfQueueCall {
        pub src_chain_id: u16,
        pub src_address: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getOutboundNonce` function with signature `getOutboundNonce(uint16,address)` and selector `0x7a145748`
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
    #[ethcall(name = "getOutboundNonce", abi = "getOutboundNonce(uint16,address)")]
    pub struct GetOutboundNonceCall {
        pub chain_id: u16,
        pub src_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getReceiveLibraryAddress` function with signature `getReceiveLibraryAddress(address)` and selector `0x71ba2fd6`
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
        name = "getReceiveLibraryAddress",
        abi = "getReceiveLibraryAddress(address)"
    )]
    pub struct GetReceiveLibraryAddressCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getReceiveVersion` function with signature `getReceiveVersion(address)` and selector `0xda1a7c9a`
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
    #[ethcall(name = "getReceiveVersion", abi = "getReceiveVersion(address)")]
    pub struct GetReceiveVersionCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getSendLibraryAddress` function with signature `getSendLibraryAddress(address)` and selector `0x9c729da1`
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
    #[ethcall(name = "getSendLibraryAddress", abi = "getSendLibraryAddress(address)")]
    pub struct GetSendLibraryAddressCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getSendVersion` function with signature `getSendVersion(address)` and selector `0x096568f6`
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
    #[ethcall(name = "getSendVersion", abi = "getSendVersion(address)")]
    pub struct GetSendVersionCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `hasStoredPayload` function with signature `hasStoredPayload(uint16,bytes)` and selector `0x0eaf6ea6`
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
    #[ethcall(name = "hasStoredPayload", abi = "hasStoredPayload(uint16,bytes)")]
    pub struct HasStoredPayloadCall {
        pub src_chain_id: u16,
        pub path: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `inboundNonce` function with signature `inboundNonce(uint16,bytes)` and selector `0x9924d33b`
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
    #[ethcall(name = "inboundNonce", abi = "inboundNonce(uint16,bytes)")]
    pub struct InboundNonceCall(pub u16, pub ::ethers::core::types::Bytes);
    ///Container type for all input parameters for the `isReceivingPayload` function with signature `isReceivingPayload()` and selector `0xca066b35`
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
    #[ethcall(name = "isReceivingPayload", abi = "isReceivingPayload()")]
    pub struct IsReceivingPayloadCall;
    ///Container type for all input parameters for the `isSendingPayload` function with signature `isSendingPayload()` and selector `0xe97a448a`
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
    #[ethcall(name = "isSendingPayload", abi = "isSendingPayload()")]
    pub struct IsSendingPayloadCall;
    ///Container type for all input parameters for the `lzEndpointLookup` function with signature `lzEndpointLookup(address)` and selector `0xc81b383a`
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
    #[ethcall(name = "lzEndpointLookup", abi = "lzEndpointLookup(address)")]
    pub struct LzEndpointLookupCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `mockChainId` function with signature `mockChainId()` and selector `0xdb14f305`
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
    #[ethcall(name = "mockChainId", abi = "mockChainId()")]
    pub struct MockChainIdCall;
    ///Container type for all input parameters for the `msgsToDeliver` function with signature `msgsToDeliver(uint16,bytes,uint256)` and selector `0x12a9ee6b`
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
    #[ethcall(name = "msgsToDeliver", abi = "msgsToDeliver(uint16,bytes,uint256)")]
    pub struct MsgsToDeliverCall(
        pub u16,
        pub ::ethers::core::types::Bytes,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `nextMsgBlocked` function with signature `nextMsgBlocked()` and selector `0x3e0dd83e`
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
    #[ethcall(name = "nextMsgBlocked", abi = "nextMsgBlocked()")]
    pub struct NextMsgBlockedCall;
    ///Container type for all input parameters for the `oracleFee` function with signature `oracleFee()` and selector `0xf9cd3ceb`
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
    #[ethcall(name = "oracleFee", abi = "oracleFee()")]
    pub struct OracleFeeCall;
    ///Container type for all input parameters for the `outboundNonce` function with signature `outboundNonce(uint16,address)` and selector `0xb2086499`
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
    #[ethcall(name = "outboundNonce", abi = "outboundNonce(uint16,address)")]
    pub struct OutboundNonceCall(pub u16, pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `protocolFeeConfig` function with signature `protocolFeeConfig()` and selector `0x07d3277f`
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
    #[ethcall(name = "protocolFeeConfig", abi = "protocolFeeConfig()")]
    pub struct ProtocolFeeConfigCall;
    ///Container type for all input parameters for the `receivePayload` function with signature `receivePayload(uint16,bytes,address,uint64,uint256,bytes)` and selector `0xc2fa4813`
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
        name = "receivePayload",
        abi = "receivePayload(uint16,bytes,address,uint64,uint256,bytes)"
    )]
    pub struct ReceivePayloadCall {
        pub src_chain_id: u16,
        pub path: ::ethers::core::types::Bytes,
        pub dst_address: ::ethers::core::types::Address,
        pub nonce: u64,
        pub gas_limit: ::ethers::core::types::U256,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `relayerFeeConfig` function with signature `relayerFeeConfig()` and selector `0x907c5e7e`
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
    #[ethcall(name = "relayerFeeConfig", abi = "relayerFeeConfig()")]
    pub struct RelayerFeeConfigCall;
    ///Container type for all input parameters for the `retryPayload` function with signature `retryPayload(uint16,bytes,bytes)` and selector `0xaaff5f16`
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
    #[ethcall(name = "retryPayload", abi = "retryPayload(uint16,bytes,bytes)")]
    pub struct RetryPayloadCall {
        pub src_chain_id: u16,
        pub path: ::ethers::core::types::Bytes,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `send` function with signature `send(uint16,bytes,bytes,address,address,bytes)` and selector `0xc5803100`
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
    #[ethcall(name = "send", abi = "send(uint16,bytes,bytes,address,address,bytes)")]
    pub struct SendCall {
        pub chain_id: u16,
        pub path: ::ethers::core::types::Bytes,
        pub payload: ::ethers::core::types::Bytes,
        pub refund_address: ::ethers::core::types::Address,
        pub zro_payment_address: ::ethers::core::types::Address,
        pub adapter_params: ::ethers::core::types::Bytes,
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
    pub struct SetConfigCall(
        pub u16,
        pub u16,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `setDefaultAdapterParams` function with signature `setDefaultAdapterParams(bytes)` and selector `0xfbba623b`
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
    #[ethcall(name = "setDefaultAdapterParams", abi = "setDefaultAdapterParams(bytes)")]
    pub struct SetDefaultAdapterParamsCall {
        pub adapter_params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setDestLzEndpoint` function with signature `setDestLzEndpoint(address,address)` and selector `0xc08f15a1`
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
    #[ethcall(name = "setDestLzEndpoint", abi = "setDestLzEndpoint(address,address)")]
    pub struct SetDestLzEndpointCall {
        pub dest_addr: ::ethers::core::types::Address,
        pub lz_endpoint_addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setOracleFee` function with signature `setOracleFee(uint256)` and selector `0xb6d9ef60`
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
    #[ethcall(name = "setOracleFee", abi = "setOracleFee(uint256)")]
    pub struct SetOracleFeeCall {
        pub oracle_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setProtocolFee` function with signature `setProtocolFee(uint256,uint256)` and selector `0x240de277`
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
    #[ethcall(name = "setProtocolFee", abi = "setProtocolFee(uint256,uint256)")]
    pub struct SetProtocolFeeCall {
        pub zro_fee: ::ethers::core::types::U256,
        pub native_bp: ::ethers::core::types::U256,
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
    pub struct SetReceiveVersionCall(pub u16);
    ///Container type for all input parameters for the `setRelayerPrice` function with signature `setRelayerPrice(uint128,uint128,uint128,uint64,uint64)` and selector `0x2c365e25`
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
        name = "setRelayerPrice",
        abi = "setRelayerPrice(uint128,uint128,uint128,uint64,uint64)"
    )]
    pub struct SetRelayerPriceCall {
        pub dst_price_ratio: u128,
        pub dst_gas_price_in_wei: u128,
        pub dst_native_amt_cap: u128,
        pub base_gas: u64,
        pub gas_per_byte: u64,
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
    pub struct SetSendVersionCall(pub u16);
    ///Container type for all input parameters for the `storedPayload` function with signature `storedPayload(uint16,bytes)` and selector `0x76a386dc`
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
    #[ethcall(name = "storedPayload", abi = "storedPayload(uint16,bytes)")]
    pub struct StoredPayloadCall(pub u16, pub ::ethers::core::types::Bytes);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LZEndpointMockCalls {
        BlockNextMsg(BlockNextMsgCall),
        DefaultAdapterParams(DefaultAdapterParamsCall),
        EstimateFees(EstimateFeesCall),
        ForceResumeReceive(ForceResumeReceiveCall),
        GetChainId(GetChainIdCall),
        GetConfig(GetConfigCall),
        GetInboundNonce(GetInboundNonceCall),
        GetLengthOfQueue(GetLengthOfQueueCall),
        GetOutboundNonce(GetOutboundNonceCall),
        GetReceiveLibraryAddress(GetReceiveLibraryAddressCall),
        GetReceiveVersion(GetReceiveVersionCall),
        GetSendLibraryAddress(GetSendLibraryAddressCall),
        GetSendVersion(GetSendVersionCall),
        HasStoredPayload(HasStoredPayloadCall),
        InboundNonce(InboundNonceCall),
        IsReceivingPayload(IsReceivingPayloadCall),
        IsSendingPayload(IsSendingPayloadCall),
        LzEndpointLookup(LzEndpointLookupCall),
        MockChainId(MockChainIdCall),
        MsgsToDeliver(MsgsToDeliverCall),
        NextMsgBlocked(NextMsgBlockedCall),
        OracleFee(OracleFeeCall),
        OutboundNonce(OutboundNonceCall),
        ProtocolFeeConfig(ProtocolFeeConfigCall),
        ReceivePayload(ReceivePayloadCall),
        RelayerFeeConfig(RelayerFeeConfigCall),
        RetryPayload(RetryPayloadCall),
        Send(SendCall),
        SetConfig(SetConfigCall),
        SetDefaultAdapterParams(SetDefaultAdapterParamsCall),
        SetDestLzEndpoint(SetDestLzEndpointCall),
        SetOracleFee(SetOracleFeeCall),
        SetProtocolFee(SetProtocolFeeCall),
        SetReceiveVersion(SetReceiveVersionCall),
        SetRelayerPrice(SetRelayerPriceCall),
        SetSendVersion(SetSendVersionCall),
        StoredPayload(StoredPayloadCall),
    }
    impl ::ethers::core::abi::AbiDecode for LZEndpointMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BlockNextMsgCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BlockNextMsg(decoded));
            }
            if let Ok(decoded)
                = <DefaultAdapterParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultAdapterParams(decoded));
            }
            if let Ok(decoded)
                = <EstimateFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EstimateFees(decoded));
            }
            if let Ok(decoded)
                = <ForceResumeReceiveCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ForceResumeReceive(decoded));
            }
            if let Ok(decoded)
                = <GetChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetChainId(decoded));
            }
            if let Ok(decoded)
                = <GetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetConfig(decoded));
            }
            if let Ok(decoded)
                = <GetInboundNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetInboundNonce(decoded));
            }
            if let Ok(decoded)
                = <GetLengthOfQueueCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetLengthOfQueue(decoded));
            }
            if let Ok(decoded)
                = <GetOutboundNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetOutboundNonce(decoded));
            }
            if let Ok(decoded)
                = <GetReceiveLibraryAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetReceiveLibraryAddress(decoded));
            }
            if let Ok(decoded)
                = <GetReceiveVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetReceiveVersion(decoded));
            }
            if let Ok(decoded)
                = <GetSendLibraryAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetSendLibraryAddress(decoded));
            }
            if let Ok(decoded)
                = <GetSendVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSendVersion(decoded));
            }
            if let Ok(decoded)
                = <HasStoredPayloadCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::HasStoredPayload(decoded));
            }
            if let Ok(decoded)
                = <InboundNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InboundNonce(decoded));
            }
            if let Ok(decoded)
                = <IsReceivingPayloadCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsReceivingPayload(decoded));
            }
            if let Ok(decoded)
                = <IsSendingPayloadCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsSendingPayload(decoded));
            }
            if let Ok(decoded)
                = <LzEndpointLookupCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LzEndpointLookup(decoded));
            }
            if let Ok(decoded)
                = <MockChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MockChainId(decoded));
            }
            if let Ok(decoded)
                = <MsgsToDeliverCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MsgsToDeliver(decoded));
            }
            if let Ok(decoded)
                = <NextMsgBlockedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NextMsgBlocked(decoded));
            }
            if let Ok(decoded)
                = <OracleFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OracleFee(decoded));
            }
            if let Ok(decoded)
                = <OutboundNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutboundNonce(decoded));
            }
            if let Ok(decoded)
                = <ProtocolFeeConfigCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProtocolFeeConfig(decoded));
            }
            if let Ok(decoded)
                = <ReceivePayloadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReceivePayload(decoded));
            }
            if let Ok(decoded)
                = <RelayerFeeConfigCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RelayerFeeConfig(decoded));
            }
            if let Ok(decoded)
                = <RetryPayloadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RetryPayload(decoded));
            }
            if let Ok(decoded)
                = <SendCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Send(decoded));
            }
            if let Ok(decoded)
                = <SetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetConfig(decoded));
            }
            if let Ok(decoded)
                = <SetDefaultAdapterParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetDefaultAdapterParams(decoded));
            }
            if let Ok(decoded)
                = <SetDestLzEndpointCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetDestLzEndpoint(decoded));
            }
            if let Ok(decoded)
                = <SetOracleFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetOracleFee(decoded));
            }
            if let Ok(decoded)
                = <SetProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetProtocolFee(decoded));
            }
            if let Ok(decoded)
                = <SetReceiveVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReceiveVersion(decoded));
            }
            if let Ok(decoded)
                = <SetRelayerPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetRelayerPrice(decoded));
            }
            if let Ok(decoded)
                = <SetSendVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSendVersion(decoded));
            }
            if let Ok(decoded)
                = <StoredPayloadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StoredPayload(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LZEndpointMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BlockNextMsg(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdapterParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EstimateFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ForceResumeReceive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInboundNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLengthOfQueue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOutboundNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReceiveLibraryAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReceiveVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSendLibraryAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSendVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasStoredPayload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InboundNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsReceivingPayload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsSendingPayload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LzEndpointLookup(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MockChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MsgsToDeliver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NextMsgBlocked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OracleFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutboundNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProtocolFeeConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReceivePayload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayerFeeConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RetryPayload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Send(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDefaultAdapterParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDestLzEndpoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOracleFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetReceiveVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRelayerPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSendVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StoredPayload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LZEndpointMockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BlockNextMsg(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdapterParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EstimateFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForceResumeReceive(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInboundNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLengthOfQueue(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOutboundNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReceiveLibraryAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetReceiveVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSendLibraryAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSendVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasStoredPayload(element) => ::core::fmt::Display::fmt(element, f),
                Self::InboundNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsReceivingPayload(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsSendingPayload(element) => ::core::fmt::Display::fmt(element, f),
                Self::LzEndpointLookup(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::MsgsToDeliver(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextMsgBlocked(element) => ::core::fmt::Display::fmt(element, f),
                Self::OracleFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutboundNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFeeConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReceivePayload(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayerFeeConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::RetryPayload(element) => ::core::fmt::Display::fmt(element, f),
                Self::Send(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDefaultAdapterParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetDestLzEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOracleFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReceiveVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRelayerPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSendVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::StoredPayload(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BlockNextMsgCall> for LZEndpointMockCalls {
        fn from(value: BlockNextMsgCall) -> Self {
            Self::BlockNextMsg(value)
        }
    }
    impl ::core::convert::From<DefaultAdapterParamsCall> for LZEndpointMockCalls {
        fn from(value: DefaultAdapterParamsCall) -> Self {
            Self::DefaultAdapterParams(value)
        }
    }
    impl ::core::convert::From<EstimateFeesCall> for LZEndpointMockCalls {
        fn from(value: EstimateFeesCall) -> Self {
            Self::EstimateFees(value)
        }
    }
    impl ::core::convert::From<ForceResumeReceiveCall> for LZEndpointMockCalls {
        fn from(value: ForceResumeReceiveCall) -> Self {
            Self::ForceResumeReceive(value)
        }
    }
    impl ::core::convert::From<GetChainIdCall> for LZEndpointMockCalls {
        fn from(value: GetChainIdCall) -> Self {
            Self::GetChainId(value)
        }
    }
    impl ::core::convert::From<GetConfigCall> for LZEndpointMockCalls {
        fn from(value: GetConfigCall) -> Self {
            Self::GetConfig(value)
        }
    }
    impl ::core::convert::From<GetInboundNonceCall> for LZEndpointMockCalls {
        fn from(value: GetInboundNonceCall) -> Self {
            Self::GetInboundNonce(value)
        }
    }
    impl ::core::convert::From<GetLengthOfQueueCall> for LZEndpointMockCalls {
        fn from(value: GetLengthOfQueueCall) -> Self {
            Self::GetLengthOfQueue(value)
        }
    }
    impl ::core::convert::From<GetOutboundNonceCall> for LZEndpointMockCalls {
        fn from(value: GetOutboundNonceCall) -> Self {
            Self::GetOutboundNonce(value)
        }
    }
    impl ::core::convert::From<GetReceiveLibraryAddressCall> for LZEndpointMockCalls {
        fn from(value: GetReceiveLibraryAddressCall) -> Self {
            Self::GetReceiveLibraryAddress(value)
        }
    }
    impl ::core::convert::From<GetReceiveVersionCall> for LZEndpointMockCalls {
        fn from(value: GetReceiveVersionCall) -> Self {
            Self::GetReceiveVersion(value)
        }
    }
    impl ::core::convert::From<GetSendLibraryAddressCall> for LZEndpointMockCalls {
        fn from(value: GetSendLibraryAddressCall) -> Self {
            Self::GetSendLibraryAddress(value)
        }
    }
    impl ::core::convert::From<GetSendVersionCall> for LZEndpointMockCalls {
        fn from(value: GetSendVersionCall) -> Self {
            Self::GetSendVersion(value)
        }
    }
    impl ::core::convert::From<HasStoredPayloadCall> for LZEndpointMockCalls {
        fn from(value: HasStoredPayloadCall) -> Self {
            Self::HasStoredPayload(value)
        }
    }
    impl ::core::convert::From<InboundNonceCall> for LZEndpointMockCalls {
        fn from(value: InboundNonceCall) -> Self {
            Self::InboundNonce(value)
        }
    }
    impl ::core::convert::From<IsReceivingPayloadCall> for LZEndpointMockCalls {
        fn from(value: IsReceivingPayloadCall) -> Self {
            Self::IsReceivingPayload(value)
        }
    }
    impl ::core::convert::From<IsSendingPayloadCall> for LZEndpointMockCalls {
        fn from(value: IsSendingPayloadCall) -> Self {
            Self::IsSendingPayload(value)
        }
    }
    impl ::core::convert::From<LzEndpointLookupCall> for LZEndpointMockCalls {
        fn from(value: LzEndpointLookupCall) -> Self {
            Self::LzEndpointLookup(value)
        }
    }
    impl ::core::convert::From<MockChainIdCall> for LZEndpointMockCalls {
        fn from(value: MockChainIdCall) -> Self {
            Self::MockChainId(value)
        }
    }
    impl ::core::convert::From<MsgsToDeliverCall> for LZEndpointMockCalls {
        fn from(value: MsgsToDeliverCall) -> Self {
            Self::MsgsToDeliver(value)
        }
    }
    impl ::core::convert::From<NextMsgBlockedCall> for LZEndpointMockCalls {
        fn from(value: NextMsgBlockedCall) -> Self {
            Self::NextMsgBlocked(value)
        }
    }
    impl ::core::convert::From<OracleFeeCall> for LZEndpointMockCalls {
        fn from(value: OracleFeeCall) -> Self {
            Self::OracleFee(value)
        }
    }
    impl ::core::convert::From<OutboundNonceCall> for LZEndpointMockCalls {
        fn from(value: OutboundNonceCall) -> Self {
            Self::OutboundNonce(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeConfigCall> for LZEndpointMockCalls {
        fn from(value: ProtocolFeeConfigCall) -> Self {
            Self::ProtocolFeeConfig(value)
        }
    }
    impl ::core::convert::From<ReceivePayloadCall> for LZEndpointMockCalls {
        fn from(value: ReceivePayloadCall) -> Self {
            Self::ReceivePayload(value)
        }
    }
    impl ::core::convert::From<RelayerFeeConfigCall> for LZEndpointMockCalls {
        fn from(value: RelayerFeeConfigCall) -> Self {
            Self::RelayerFeeConfig(value)
        }
    }
    impl ::core::convert::From<RetryPayloadCall> for LZEndpointMockCalls {
        fn from(value: RetryPayloadCall) -> Self {
            Self::RetryPayload(value)
        }
    }
    impl ::core::convert::From<SendCall> for LZEndpointMockCalls {
        fn from(value: SendCall) -> Self {
            Self::Send(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for LZEndpointMockCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetDefaultAdapterParamsCall> for LZEndpointMockCalls {
        fn from(value: SetDefaultAdapterParamsCall) -> Self {
            Self::SetDefaultAdapterParams(value)
        }
    }
    impl ::core::convert::From<SetDestLzEndpointCall> for LZEndpointMockCalls {
        fn from(value: SetDestLzEndpointCall) -> Self {
            Self::SetDestLzEndpoint(value)
        }
    }
    impl ::core::convert::From<SetOracleFeeCall> for LZEndpointMockCalls {
        fn from(value: SetOracleFeeCall) -> Self {
            Self::SetOracleFee(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeeCall> for LZEndpointMockCalls {
        fn from(value: SetProtocolFeeCall) -> Self {
            Self::SetProtocolFee(value)
        }
    }
    impl ::core::convert::From<SetReceiveVersionCall> for LZEndpointMockCalls {
        fn from(value: SetReceiveVersionCall) -> Self {
            Self::SetReceiveVersion(value)
        }
    }
    impl ::core::convert::From<SetRelayerPriceCall> for LZEndpointMockCalls {
        fn from(value: SetRelayerPriceCall) -> Self {
            Self::SetRelayerPrice(value)
        }
    }
    impl ::core::convert::From<SetSendVersionCall> for LZEndpointMockCalls {
        fn from(value: SetSendVersionCall) -> Self {
            Self::SetSendVersion(value)
        }
    }
    impl ::core::convert::From<StoredPayloadCall> for LZEndpointMockCalls {
        fn from(value: StoredPayloadCall) -> Self {
            Self::StoredPayload(value)
        }
    }
    ///Container type for all return fields from the `defaultAdapterParams` function with signature `defaultAdapterParams()` and selector `0x272bd384`
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
    pub struct DefaultAdapterParamsReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `estimateFees` function with signature `estimateFees(uint16,address,bytes,bool,bytes)` and selector `0x40a7bb10`
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
    pub struct EstimateFeesReturn {
        pub native_fee: ::ethers::core::types::U256,
        pub zro_fee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
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
    pub struct GetChainIdReturn(pub u16);
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
    ///Container type for all return fields from the `getInboundNonce` function with signature `getInboundNonce(uint16,bytes)` and selector `0xfdc07c70`
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
    pub struct GetInboundNonceReturn(pub u64);
    ///Container type for all return fields from the `getLengthOfQueue` function with signature `getLengthOfQueue(uint16,bytes)` and selector `0x7f6df8e6`
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
    pub struct GetLengthOfQueueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getOutboundNonce` function with signature `getOutboundNonce(uint16,address)` and selector `0x7a145748`
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
    pub struct GetOutboundNonceReturn(pub u64);
    ///Container type for all return fields from the `getReceiveLibraryAddress` function with signature `getReceiveLibraryAddress(address)` and selector `0x71ba2fd6`
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
    pub struct GetReceiveLibraryAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getReceiveVersion` function with signature `getReceiveVersion(address)` and selector `0xda1a7c9a`
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
    pub struct GetReceiveVersionReturn(pub u16);
    ///Container type for all return fields from the `getSendLibraryAddress` function with signature `getSendLibraryAddress(address)` and selector `0x9c729da1`
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
    pub struct GetSendLibraryAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSendVersion` function with signature `getSendVersion(address)` and selector `0x096568f6`
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
    pub struct GetSendVersionReturn(pub u16);
    ///Container type for all return fields from the `hasStoredPayload` function with signature `hasStoredPayload(uint16,bytes)` and selector `0x0eaf6ea6`
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
    pub struct HasStoredPayloadReturn(pub bool);
    ///Container type for all return fields from the `inboundNonce` function with signature `inboundNonce(uint16,bytes)` and selector `0x9924d33b`
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
    pub struct InboundNonceReturn(pub u64);
    ///Container type for all return fields from the `isReceivingPayload` function with signature `isReceivingPayload()` and selector `0xca066b35`
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
    pub struct IsReceivingPayloadReturn(pub bool);
    ///Container type for all return fields from the `isSendingPayload` function with signature `isSendingPayload()` and selector `0xe97a448a`
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
    pub struct IsSendingPayloadReturn(pub bool);
    ///Container type for all return fields from the `lzEndpointLookup` function with signature `lzEndpointLookup(address)` and selector `0xc81b383a`
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
    pub struct LzEndpointLookupReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `mockChainId` function with signature `mockChainId()` and selector `0xdb14f305`
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
    pub struct MockChainIdReturn(pub u16);
    ///Container type for all return fields from the `msgsToDeliver` function with signature `msgsToDeliver(uint16,bytes,uint256)` and selector `0x12a9ee6b`
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
    pub struct MsgsToDeliverReturn {
        pub dst_address: ::ethers::core::types::Address,
        pub nonce: u64,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `nextMsgBlocked` function with signature `nextMsgBlocked()` and selector `0x3e0dd83e`
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
    pub struct NextMsgBlockedReturn(pub bool);
    ///Container type for all return fields from the `oracleFee` function with signature `oracleFee()` and selector `0xf9cd3ceb`
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
    pub struct OracleFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `outboundNonce` function with signature `outboundNonce(uint16,address)` and selector `0xb2086499`
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
    pub struct OutboundNonceReturn(pub u64);
    ///Container type for all return fields from the `protocolFeeConfig` function with signature `protocolFeeConfig()` and selector `0x07d3277f`
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
    pub struct ProtocolFeeConfigReturn {
        pub zro_fee: ::ethers::core::types::U256,
        pub native_bp: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `relayerFeeConfig` function with signature `relayerFeeConfig()` and selector `0x907c5e7e`
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
    pub struct RelayerFeeConfigReturn {
        pub dst_price_ratio: u128,
        pub dst_gas_price_in_wei: u128,
        pub dst_native_amt_cap: u128,
        pub base_gas: u64,
        pub gas_per_byte: u64,
    }
    ///Container type for all return fields from the `storedPayload` function with signature `storedPayload(uint16,bytes)` and selector `0x76a386dc`
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
    pub struct StoredPayloadReturn {
        pub payload_length: u64,
        pub dst_address: ::ethers::core::types::Address,
        pub payload_hash: [u8; 32],
    }
}
