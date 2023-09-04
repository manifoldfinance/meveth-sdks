pub use gnosis_safe::*;
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
pub mod gnosis_safe {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("VERSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("VERSION"),
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
                    ::std::borrow::ToOwned::to_owned("addOwnerWithThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addOwnerWithThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_threshold"),
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
                    ::std::borrow::ToOwned::to_owned("approveHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approveHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hashToApprove"),
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
                    ::std::borrow::ToOwned::to_owned("approvedHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approvedHashes"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("changeThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeThreshold"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_threshold"),
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
                    ::std::borrow::ToOwned::to_owned("checkNSignatures"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkNSignatures"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "requiredSignatures",
                                    ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkSignatures"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkSignatures"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dataHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("disableModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("disableModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevModule"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
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
                    ::std::borrow::ToOwned::to_owned("domainSeparator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("domainSeparator"),
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
                    ::std::borrow::ToOwned::to_owned("enableModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("enableModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
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
                    ::std::borrow::ToOwned::to_owned("encodeTransactionData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "encodeTransactionData",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("safeTxGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundReceiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
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
                    ::std::borrow::ToOwned::to_owned("execTransaction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execTransaction"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("safeTxGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundReceiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("execTransactionFromModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "execTransactionFromModule",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "execTransactionFromModuleReturnData",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "execTransactionFromModuleReturnData",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("getModulesPaginated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getModulesPaginated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pageSize"),
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
                                    name: ::std::borrow::ToOwned::to_owned("array"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("next"),
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
                    ::std::borrow::ToOwned::to_owned("getOwners"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOwners"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStorageAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStorageAt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("offset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
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
                    ::std::borrow::ToOwned::to_owned("getThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getThreshold"),
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
                    ::std::borrow::ToOwned::to_owned("getTransactionHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTransactionHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("safeTxGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("baseGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundReceiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_nonce"),
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
                    ::std::borrow::ToOwned::to_owned("isModuleEnabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isModuleEnabled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
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
                    ::std::borrow::ToOwned::to_owned("isOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isOwner"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("nonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonce"),
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
                    ::std::borrow::ToOwned::to_owned("removeOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevOwner"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_threshold"),
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
                    ::std::borrow::ToOwned::to_owned("requiredTxGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("requiredTxGas"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Enum.Operation"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFallbackHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFallbackHandler"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("handler"),
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
                    ::std::borrow::ToOwned::to_owned("setGuard"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setGuard"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("guard"),
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
                    ::std::borrow::ToOwned::to_owned("setup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setup"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_owners"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_threshold"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fallbackHandler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentReceiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable"),
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
                    ::std::borrow::ToOwned::to_owned("signedMessages"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("signedMessages"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("simulateAndRevert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateAndRevert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("calldataPayload"),
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
                    ::std::borrow::ToOwned::to_owned("swapOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prevOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oldOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("AddedOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AddedOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApproveHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApproveHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approvedHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangedFallbackHandler"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ChangedFallbackHandler",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("handler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangedGuard"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangedGuard"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("guard"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangedThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChangedThreshold"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("threshold"),
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
                    ::std::borrow::ToOwned::to_owned("DisabledModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DisabledModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnabledModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EnabledModule"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionFailure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ExecutionFailure"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
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
                    ::std::borrow::ToOwned::to_owned("ExecutionFromModuleFailure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExecutionFromModuleFailure",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionFromModuleSuccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExecutionFromModuleSuccess",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("module"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionSuccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ExecutionSuccess"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("txHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
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
                    ::std::borrow::ToOwned::to_owned("RemovedOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RemovedOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeReceived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SafeReceived"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("SafeSetup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SafeSetup"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initiator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owners"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("threshold"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initializer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fallbackHandler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SignMsg"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SignMsg"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GNOSISSAFE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x1BW`\x01`\x04Ua*\xB4\x90\x81a\0!\x829\xF3[`\0\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0*W[6\x15a\0 W4\x15a#\xE5W[`\0\x80\xFD[a\0(a)\\V[\0[`\x005`\xE0\x1C\x80c\rX/\x13\x14a\x02\x1AW\x80c\x12\xFBh\xE0\x14a\x02\x15W\x80c-\x9A\xD5=\x14a\x02\x10W\x80c/T\xBFn\x14a\x02\x0BW\x80c4\x08\xE4p\x14a\x02\x06W\x80cF\x87!\xA7\x14a\x02\x01W\x80cR)\x07?\x14a\x01\xFCW\x80cV$\xB2[\x14a\x01\xF7W\x80cZ\xE6\xBD7\x14a\x01\xF2W\x80ca\x0BY%\x14a\x01\xEDW\x80ciN\x80\xC3\x14a\x01\xE8W\x80cjv\x12\x02\x14a\x01\xE3W\x80c}\x83)t\x14a\x01\xDEW\x80c\x93O:\x11\x14a\x01\xD9W\x80c\xA0\xE6~+\x14a\x01\xD4W\x80c\xAF\xFE\xD0\xE0\x14a\x01\xCFW\x80c\xB4\xFA\xBA\t\x14a\x01\xCAW\x80c\xB6>\x80\r\x14a\x01\xC5W\x80c\xC4\xCA:\x9C\x14a\x01\xC0W\x80c\xCC/\x84R\x14a\x01\xBBW\x80c\xD4\xD9\xBD\xCD\x14a\x01\xB6W\x80c\xD8\xD1\x1Fx\x14a\x01\xB1W\x80c\xE0\t\xCF\xDE\x14a\x01\xACW\x80c\xE1\x9A\x9D\xD9\x14a\x01\xA7W\x80c\xE3\x18\xB5+\x14a\x01\xA2W\x80c\xE7R5\xB8\x14a\x01\x9DW\x80c\xE8f7\xDB\x14a\x01\x98W\x80c\xF0\x8A\x03#\x14a\x01\x93W\x80c\xF6\x98\xDA%\x14a\x01\x8EW\x80c\xF8\xDC]\xD9\x14a\x01\x89Wc\xFF\xA1\xADt\x03a\0\x0EWa\x15\x92V[a\x14\x97V[a\x14tV[a\x13\xF7V[a\x13\xCFV[a\x13\xB1V[a\x11\xF9V[a\x11|V[a\x10:V[a\x10\x07V[a\x0E\xCCV[a\r\xF1V[a\r\"V[a\x0C\x8AV[a\x0C*V[a\x0C\x0CV[a\x0ByV[a\n\xE6V[a\n\x9AV[a\t\xF8V[a\tUV[a\x08@V[a\x08\x14V[a\x07\x84V[a\x07(V[a\x06\xC2V[a\x06LV[a\x05\xE5V[a\x05\x88V[a\x05/V[a\x02fV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\0\x1BWV[`\xE45\x90a\x02=\x82a\x02\x1FV[V[`D5\x90a\x02=\x82a\x02\x1FV[`\x845\x90a\x02=\x82a\x02\x1FV[`\xA45\x90a\x02=\x82a\x02\x1FV[4a\0\x1BW`@6`\x03\x19\x01\x12a\0\x1BW`\x045a\x02\x83\x81a\x02\x1FV[\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&a\x03\xEA`$5\x92a\x02\xB3a)\x8AV[a\x02\xD6`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x15\x15\x90\x81a\x04\x0CW[\x81a\x04\x01W[Pa'@V[a\x03\x1Ba\x03\x15a\x03\ta\x02\xFC\x84`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x15a'tV[`\x01`\0R`\x02` Ra\x03\x89a\x03Q\x7F\xE9\x0B{\xCE\xB6\xE7\xDFT\x18\xFBx\xD8\xEETn\x97\xC8:\x08\xBB\xCC\xC0\x1A\x06D\xD5\x99\xCC\xD2\xA7\xC2\xE0a\x02\xFCV[a\x03n\x83`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x90`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x82T\x16\x17\x90UV[`\x01`\0R`\x02` Ra\x03\xBD\x81\x7F\xE9\x0B{\xCE\xB6\xE7\xDFT\x18\xFBx\xD8\xEETn\x97\xC8:\x08\xBB\xCC\xC0\x1A\x06D\xD5\x99\xCC\xD2\xA7\xC2\xE0a\x03nV[a\x03\xD0a\x03\xCB`\x03Ta\x17\x84V[`\x03UV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA1\x80`\x04T\x03a\x03\xF8W\0[a\0(\x90a)\x08V[\x90P0\x14\x158a\x02\xD0V[`\x01\x81\x14\x15\x91Pa\x02\xCAV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04BW`@RV[a\x04\x18V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04BW`@RV[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04BW`@RV[a\x01\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04BW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04BW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04BW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x04\xE6\x82a\x04\xBEV[\x91a\x04\xF4`@Q\x93\x84a\x04\x9CV[\x82\x94\x81\x84R\x81\x83\x01\x11a\0\x1BW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\0\x1BW\x81` a\x05,\x935\x91\x01a\x04\xDAV[\x90V[4a\0\x1BW`\x806`\x03\x19\x01\x12a\0\x1BWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x81\x81\x11a\0\x1BWa\x05a\x906\x90`\x04\x01a\x05\x11V[`D5\x91\x82\x11a\0\x1BWa\x05|a\0(\x926\x90`\x04\x01a\x05\x11V[`d5\x91`\x045a\x1F\x95V[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW` `\x045a\x05\xA7\x81a\x02\x1FV[`\x01`\x01`\xA0\x1B\x03\x80\x91\x16\x90\x81`\x01\x14\x15\x91\x82a\x05\xCBW[PP`@Q\x90\x15\x15\x81R\xF3[\x90\x91P`\0R`\x01\x82R`@`\0 T\x16\x15\x158\x80a\x05\xBFV[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW` `\x045a\x06\x04\x81a\x02\x1FV[`\x01`\x01`\xA0\x1B\x03\x80\x91\x16\x90`\x01\x82\x14\x15\x91\x82a\x06'WPP`@Q\x90\x15\x15\x81R\xF3[\x90\x91P`\0R`\x02\x82R`@`\0 T\x16\x15\x158\x80a\x05\xBFV[`\0\x91\x03\x12a\0\x1BWV[4a\0\x1BW`\x006`\x03\x19\x01\x12a\0\x1BW` `@QF\x81R\xF3[`d5\x90`\x02\x82\x10\x15a\0\x1BWV[`\x80`\x03\x19\x82\x01\x12a\0\x1BW`\x045a\x06\x8E\x81a\x02\x1FV[\x91`$5\x91`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\0\x1BWa\x06\xB2\x91`\x04\x01a\x05\x11V[\x90`d5`\x02\x81\x10\x15a\0\x1BW\x90V[4a\0\x1BW` a\x06\xDEa\x06\xD56a\x06vV[\x92\x91\x90\x91a%\x85V[`@Q\x90\x15\x15\x81R\xF3[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x07\x14WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x06\xF3V[4a\0\x1BWa\x079a\x06\xD56a\x06vV[`@Q\x90` =\x83\x01\x01`@R=\x82R=`\0` \x84\x01>a\x07o`@Q\x92\x83\x92\x15\x15\x83R`@` \x84\x01R`@\x83\x01\x90a\x06\xE8V[\x03\x90\xF3[\x90` a\x05,\x92\x81\x81R\x01\x90a\x06\xE8V[4a\0\x1BW`@6`\x03\x19\x01\x12a\0\x1BW`\x045`$5`\x05\x81\x81\x1B\x92` \x90\x83\x85\x04\x82\x14\x84\x15\x17\x15a\x08\x0FWa\x07\xBA\x85a\x04\xBEV[\x94a\x07\xC8`@Q\x96\x87a\x04\x9CV[\x80\x86Ra\x07\xD7`\x1F\x19\x91a\x04\xBEV[\x016\x83\x87\x017`\0[\x84\x81\x10a\x07\xF5W`@Q\x80a\x07o\x88\x82a\x07sV[\x80a\x08\n\x91\x83\x01T\x84\x82\x87\x1B\x89\x01\x01Ra\x17\x84V[a\x07\xE0V[a\x17nV[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW`\x045`\0R`\x07` R` `@`\0 T`@Q\x90\x81R\xF3[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW\x7F\xEC\xDF:>\xFF\xEAW\x83\xA3\xC4\xC2\x14\x0Eguwfd(\xD4N\xD9\xD4t\xA0\xB3\xA4\xC9\x94?\x84@a\tE`\x045a\x08\x81\x81a\x02\x1FV[a\x08\x89a)\x8AV[a\x08\xBE`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x80\x15\x15\x80a\tJW[a\x08\xA9\x90a%\x1DV[`\0R`\x01` R`@`\0 T\x16\x15a%QV[`\x01`\0\x81\x90R` Ra\t\x11a\x08\xF4\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/a\x02\xFCV[a\x03n\x83`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[`\x01`\0\x81\x90R` Ra\x03\xD0\x81\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/a\x03nV[\x03\x90\xA1\0[P`\x01\x81\x14\x15a\x08\xA0V[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW\x7Fa\x0F\x7F\xF2\xB3\x04\xAE\x89\x03\xC3\xDEt\xC6\x0Cj\xB1\xF7\xD6\"k?R\xC5\x16\x19\x05\xBBZ\xD4\x03\x9C\x93` `\x045a\t\x94a)\x8AV[a\t\xA2`\x03T\x82\x11\x15a&\xD8V[a\t\xAF`\x01\x82\x10\x15a'\x0CV[\x80`\x04U`@Q\x90\x81R\xA1\0[\x91\x81`\x1F\x84\x01\x12\x15a\0\x1BW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\x1BW` \x83\x81\x86\x01\x95\x01\x01\x11a\0\x1BWV[a\x01\x045\x90a\x02=\x82a\x02\x1FV[a\x01@6`\x03\x19\x01\x12a\0\x1BW`\x045a\n\x11\x81a\x02\x1FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`D5\x81\x81\x11a\0\x1BWa\n2\x906\x90`\x04\x01a\t\xBCV[\x90a\n;a\x06gV[\x91`\xE45\x90a\nI\x82a\x02\x1FV[a\nQa\t\xEAV[\x92a\x01$5\x95\x86\x11a\0\x1BWa\x07o\x96a\nra\n\x88\x976\x90`\x04\x01a\x05\x11V[\x95`\xC45\x93`\xA45\x93`\x845\x93`$5\x90a\x19QV[`@Q\x90\x15\x15\x81R\x90\x81\x90` \x82\x01\x90V[4a\0\x1BW`@6`\x03\x19\x01\x12a\0\x1BW`\x01`\x01`\xA0\x1B\x03`\x045a\n\xBF\x81a\x02\x1FV[\x16`\0R`\x08` R`@`\0 `$5`\0R` R` `@`\0 T`@Q\x90\x81R\xF3[4a\0\x1BW``6`\x03\x19\x01\x12a\0\x1BWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x81\x81\x11a\0\x1BWa\x0B\x18\x906\x90`\x04\x01a\x05\x11V[`D5\x91\x82\x11a\0\x1BWa\x0B3a\0(\x926\x90`\x04\x01a\x05\x11V[\x90`\x045a\x1D\x90V[\x90\x81Q\x80\x82R` \x80\x80\x93\x01\x93\x01\x91`\0[\x82\x81\x10a\x0B\\WPPPP\x90V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x0BNV[4a\0\x1BW`\0\x80`\x03\x196\x01\x12a\x0C\tWa\x0B\x96`\x03Ta&HV[\x90\x80`\x01\x91\x82\x82R`\x02` \x92\x81\x84R`\x01`\x01`\xA0\x1B\x03\x91`@\x93\x86\x84\x86\x85 T\x16\x90[a\x0B\xD2W[\x85Q\x87\x81R\x80a\x07o\x81\x8A\x01\x8Ca\x0B<V[\x84\x16\x87\x81\x14a\x0C\x04W\x90\x81\x88\x92a\x0B\xE9\x83\x8Ca&zV[R\x84R\x82\x87Ra\x0B\xFE\x85\x87\x86 T\x16\x91a\x17\x84V[\x91a\x0B\xBBV[a\x0B\xC0V[\x80\xFD[4a\0\x1BW`\x006`\x03\x19\x01\x12a\0\x1BW` `\x05T`@Q\x90\x81R\xF3[4a\0\x1BW`@6`\x03\x19\x01\x12a\0\x1BW`\x045a\x0CG\x81a\x02\x1FV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\x1BW`\0\x91a\x0Ck\x83\x926\x90`\x04\x01a\x05\x11V[\x90` \x82Q\x92\x01\x90Z\xF4`\0R=` R=`\0`@>`@=\x01`\0\xFD[4a\0\x1BWa\x01\x006`\x03\x19\x01\x12a\0\x1BWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x81\x81\x11a\0\x1BW6`#\x82\x01\x12\x15a\0\x1BW\x80`\x04\x015\x82\x81\x11a\0\x1BW6`$\x82`\x05\x1B\x84\x01\x01\x11a\0\x1BWa\x0C\xDDa\x02?V[`d5\x93\x84\x11a\0\x1BWa\x0C\xF8a\0(\x946\x90`\x04\x01a\t\xBCV[a\r\0a\x02LV[\x91a\r\ta\x02YV[\x93a\r\x12a\x020V[\x96`\xC45\x96`$\x805\x92\x01a\x16^V[4a\0\x1BW`\x806`\x03\x19\x01\x12a\0\x1BW`\x045a\r?\x81a\x02\x1FV[`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\x1BWa\rba\r\x85\x916\x90`\x04\x01a\t\xBCV[\x90\x92a\rla\x06gV[\x90a\r{Z\x95Z\x946\x91a\x04\xDAV[\x90`$5\x90a#\xB5V[\x15a\0\x1BWZ\x81\x03\x90\x81\x11a\x08\x0FWa\r\xC6\x90`@Q\x90` \x82\x01R` \x81Ra\r\xAE\x81a\x04GV[`@QbF\x1B\xCD`\xE5\x1B\x81R\x91\x82\x91`\x04\x83\x01a\x07sV[\x03\x90\xFD[\x90`\x01`\x01`\xA0\x1B\x03a\r\xEA` \x92\x95\x94\x95`@\x85R`@\x85\x01\x90a\x0B<V[\x94\x16\x91\x01RV[4a\0\x1BW`@6`\x03\x19\x01\x12a\0\x1BW`\x045a\x0E\x0E\x81a\x02\x1FV[`$5\x90a\x0E\x1B\x82a&HV[\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x80\x91\x16\x82R`\x01\x93\x84\x91\x82` R\x80`@\x85 T\x16\x95[a\x0EWW[PPP\x81Ra\x07o`@Q\x92\x83\x92\x83a\r\xCAV[\x90\x91\x92\x94\x81\x81\x16\x80\x15\x15\x80a\x0E\xC2W[\x80a\x0E\xB9W[\x15a\x0E\xB1Wa\x0E\xA3a\x02\xFC\x86\x94\x93a\x0E\xA9\x93a\x0E\x89\x8B\x8Ba&zV[R`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x96a\x17\x84V[\x93\x92\x91a\x0E>V[P\x94\x92a\x0ECV[P\x83\x87\x10a\x0EmV[P\x84\x81\x14\x15a\x0EgV[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW`\x045`\0\x903\x82R`\x02` R`\x01`\x01`\xA0\x1B\x03`@\x83 T\x16\x15a\x0F`Wa\x0F7a\x0F1\x82a\x0F\"3`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x08` R`@`\0 \x90V[\x90`\0R` R`@`\0 \x90V[`\x01\x90UV[3\x90\x7F\xF2\xA0\xEB\x15dr\xD1D\x02U\xB0\xD7\xC1\xE1\x9C\xC0q\x15\xD1\x05\x1F\xE6\x05\xB0\xDC\xE6\x9A\xCF\xEC\x88M\x9C\x83\x80\xA3\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x033`\xDC\x1B`D\x82\x01R`d\x90\xFD[a\x01@`\x03\x19\x82\x01\x12a\0\x1BW`\x045a\x0F\xA6\x81a\x02\x1FV[\x91`$5\x91`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\0\x1BWa\x0F\xCA\x91`\x04\x01a\t\xBCV[\x90\x91`d5`\x02\x81\x10\x15a\0\x1BW\x90`\x845\x90`\xA45\x90`\xC45\x90`\xE45a\x0F\xF1\x81a\x02\x1FV[\x90a\x01\x045a\x0F\xFF\x81a\x02\x1FV[\x90a\x01$5\x90V[4a\0\x1BW` a\x10,a\x10\x1A6a\x0F\x8DV[\x99\x98\x90\x98\x97\x91\x97\x96\x92\x96\x95\x93\x95a\"\xD4V[\x81\x81Q\x91\x01 `@Q\x90\x81R\xF3[4a\0\x1BW`@6`\x03\x19\x01\x12a\0\x1BW`\x045a\x10W\x81a\x02\x1FV[`$5\x90a\x10d\x82a\x02\x1FV[a\x10la)\x8AV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x90\x81\x15\x15\x80a\x11qW[a\x10\x8A\x90a%\x1DV[\x80\x83\x16`\0R`\x01` R`@`\0 T\x16\x03a\x11DW\x81a\x11\x07a\tE\x92a\x03na\x10\xEDa\x02\xFC\x7F\xAA\xB4\xFA+F?X\x1B+2\xCB;~;pK\x9C\xE3|\xC2\t\xB5\xFBMw\xE5\x93\xAC\xE4\x05Bv\x97`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x91`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[a\x03\xD0a\x11'\x82`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS103`\xD8\x1B`D\x82\x01R`d\x90\xFD[P`\x01\x82\x14\x15a\x10\x81V[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW\x7F\x11Q\x11i\x14Q[\xC0\x89\x1F\xF9\x04zl\xB3,\xF9\x02To\x83\x06d\x99\xBC\xF8\xBA3\xD25?\xA2` `\x045a\x11\xBC\x81a\x02\x1FV[a\x11\xC4a)\x8AV[\x80\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8U`\x01`\x01`\xA0\x1B\x03`@Q\x91\x16\x81R\xA1\0[4a\0\x1BW``6`\x03\x19\x01\x12a\0\x1BW\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&a\tEa\x03n\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAFa\x13\x89`\x045a\x12a\x81a\x02\x1FV[a\x13i`$5\x91a\x12q\x83a\x02\x1FV[`D5\x95\x86\x91a\x12\x80\x83a\x02\x1FV[a\x12\x88a)\x8AV[a\x13\x10`\x01`\x01`\xA0\x1B\x03a\x12\xAD\x81\x86\x16\x80\x15\x15\x90\x81a\x04\x0CW\x81a\x04\x01WPa'@V[a\x12\xD3a\x03\x15a\x03\ta\x02\xFC\x88`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x86\x16\x80\x15\x15\x80a\x13\xA6W[a\x12\xE7\x90a'@V[a\x13\na\x03\ta\x02\xFC\x85`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x14a(\xC7V[a\x13Pa\x133a\x02\xFC\x87`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[a\x03n\x85`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[a\x03\xD0a\x11'\x82`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x03\x90\xA1`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[P`\x01\x81\x14\x15a\x12\xDEV[4a\0\x1BW`\x006`\x03\x19\x01\x12a\0\x1BW` `\x04T`@Q\x90\x81R\xF3[4a\0\x1BWa\x07oa\x13\xE3a\x10\x1A6a\x0F\x8DV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\xE8V[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW\x7FZ\xC6\xC4l\x93\xC8\xD0\xE57\x14\xBA;S\xDB>|\x04m\xA9\x941=~\xD0\xD1\x92\x02\x8B\xC7\xC2(\xB0` `\x045a\x147\x81a\x02\x1FV[a\x14?a)\x8AV[\x80\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5U`\x01`\x01`\xA0\x1B\x03`@Q\x91\x16\x81R\xA1\0[4a\0\x1BW`\x006`\x03\x19\x01\x12a\0\x1BW` a\x14\x8Fa\"\x8AV[`@Q\x90\x81R\xF3[4a\0\x1BW``6`\x03\x19\x01\x12a\0\x1BW`\x045a\x14\xB4\x81a\x02\x1FV[\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAFa\x03\xEA`$5a\x14\xE4\x81a\x02\x1FV[a\x15d`D5\x94a\x14\xF3a)\x8AV[a\x15\t\x86a\x15\x02`\x03Ta\x18\xFEV[\x10\x15a&\xD8V[a\x15'`\x01`\x01`\xA0\x1B\x03\x84\x16\x80\x15\x15\x80a\x13\xA6Wa\x12\xE7\x90a'@V[a\x03na\x15Ja\x02\xFC\x85`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x91`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[a\x15\x84a\x11'\x82`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[a\x03\xD0a\x03\xCB`\x03Ta(\xFBV[4a\0\x1BW`\x006`\x03\x19\x01\x12a\0\x1BWa\x07o`@Qa\x15\xB2\x81a\x04GV[`\x05\x81Rd\x03\x12\xE32\xE3`\xDC\x1B` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\xE8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04BW`\x05\x1B` \x01\x90V[\x93\x92\x95\x94\x91\x80`\x80\x86\x01`\x80\x87RR`\xA0\x85\x01\x91\x90`\0[\x81\x81\x10a\x163WPPP\x90``\x92\x91\x95` \x85\x01R`\x01`\x01`\xA0\x1B\x03\x80\x92\x16`@\x85\x01R\x16\x91\x01RV[\x90\x91\x92`\x01\x90`\x01`\x01`\xA0\x1B\x03\x855a\x16L\x81a\x02\x1FV[\x16\x81R` \x90\x81\x01\x94\x01\x92\x91\x01a\x16\x08V[\x98\x92\x94\x97\x93\x90\x96\x91\x97a\x16p\x88a\x15\xD8V[a\x16}`@Q\x91\x82a\x04\x9CV[\x88\x81R` \x80\x82\x01\x8A`\x05\x1B\x8D\x01\x906\x82\x11a\0\x1BW\x8D\x90[\x82\x82\x10a\x17UWPPPPa\x16\xF4a\x17\x11\x97\x95\x93\x8B\x9A\x99\x97\x95\x93a\x16\xDE\x7F\x14\x1D\xF8h\xA63\x1A\xF5(\xE3\x8C\x83\xB7\xAA\x03\xED\xC1\x9B\xE6n7\xAEg\xF9([\xF4\xF8\xE3\xC6\xA1\xA8\x9Da\x16\xFA\x95a'\xA8V[`\x01`\x01`\xA0\x1B\x03\x89\x16a\x17(W[6\x91a\x04\xDAV[\x85a$6V[\x82a\x17\x16W[PPP`@Q\x94\x85\x943\x98\x86a\x15\xF0V[\x03\x90\xA2V[a\x17\x1F\x92a\x1CCV[P8\x80\x80a\x17\0V[a\x17P\x89\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5UV[a\x16\xEDV[\x83\x80\x91\x835a\x17c\x81a\x02\x1FV[\x81R\x01\x91\x01\x90a\x16\x96V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x19\x81\x14a\x08\x0FW`\x01\x01\x90V[\x90`\x02\x82\x10\x15a\x17\xA0WRV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x9C\x9B\x99\x92\x95\x97`\xC0\x8Ea\x02=\x9D\x99a\x18Z\x9Ba\x18\x1Ba\x18H\x99a\x01@\x9F\x97\x98`\x01`\x01`\xA0\x1B\x03a\x187\x9A\x16\x86R` \x86\x01R\x80a\x01`\x80`@\x88\x01R\x86\x01R\x80a\x01\x80\x9D\x8E\x87\x017`\0\x85\x82\x01\x8E\x01R`\x1F\x01`\x1F\x19\x16\x84\x01\x9A``\x85\x01\x90a\x17\x93V[`\x80\x83\x01R`\xA0\x82\x01R\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x8D\x01RV[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x8B\x01RV[\x81\x89\x82\x03\x01a\x01 \x8A\x01R\x01\x90a\x06\xE8V[\x94\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`@Q=`\0\x82>=\x90\xFD[\x90\x81`\x06\x1B\x91\x80\x83\x04`@\x14\x90\x15\x17\x15a\x08\x0FWV[\x81\x15a\x18\x96W\x04\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x90a\t\xC4\x82\x01\x80\x92\x11a\x08\x0FWV[\x90a\x01\xF4\x82\x01\x80\x92\x11a\x08\x0FWV[\x15a\x18\xD1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03\x13`\xDC\x1B`D\x82\x01R`d\x90\xFD[`\0\x19\x81\x01\x91\x90\x82\x11a\x08\x0FWV[a\t\xC3\x19\x81\x01\x91\x90\x82\x11a\x08\x0FWV[\x15a\x19$WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS013`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x97\x93\x83\x8A\x8C\x9B\x97\x93\x9D\x9C\x99\x95\x8B\x8F\x96\x80\x9B\x97\x8A\x8A\x8A\x8A\x8A`\x05T\x98\x89\x97`\x01`\x01`\xA0\x1B\x03\x80\x9C\x16\x97a\x19\x83\x9Aa\"\xD4V[\x90a\x19\x8D\x90a\x17\x84V[`\x05U\x80Q` \x82\x01 \x9Ca\x19\xA2\x91\x8Ea\x1D\x90V[\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8T\x16\x99\x8A\x15\x15\x99\x8Aa\x1BhW[P\x90a\x1AA\x92a\x1A9\x92a\x1A\x12Za\x1A\x0Ba\x1A\x06a\x19\xF7a\x19\xF0\x8Ba\x18vV[`?\x90\x04\x90V[a\x1A\0\x8Ba\x18\xACV[\x90a*mV[a\x18\xBBV[\x11\x15a\x18\xCAV[Z\x9Fa\x1A3\x89\x15\x9C\x8D`\0\x14a\x1BaWa\x1A+Za\x19\rV[\x956\x91a\x04\xDAV[\x91a#\xB5V[\x9BZ\x90a*0V[\x90\x8B\x90\x8C\x15a\x1BWW[P\x80\x15a\x1BOW[a\x1A\\\x90a\x19\x1DV[`\0\x95\x15a\x1B;W[PPPPP\x85`\0\x14a\x1B\x04W`@\x80Q\x85\x81R` \x81\x01\x92\x90\x92R\x7FD.q_bcF\xE8\xC5C\x81\0-\xA6\x14\xF6+\xEE\x8D'8e5\xB2R\x1E\xC8T\x08\x98Un\x91\xA1[a\x1A\xADWPPV[\x80;\x15a\0\x1BW`@Qc\x12d\xE2m`\xE3\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x83\x15\x15`$\x83\x01R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x1A\xFFWa\x1A\xECWPV[\x80a\x1A\xF9a\x02=\x92a\x04.V[\x80a\x06AV[a\x18jV[`@\x80Q\x85\x81R` \x81\x01\x92\x90\x92R\x7F#B\x8B\x18\xAC\xFB>\xA6K\x08\xDC\x0C\x1D)n\xA9\xC0\x97\x02\xC0\x90\x83\xCARr\xE6M\x11[h}#\x91\xA1a\x1A\xA5V[a\x1BE\x95Pa\x1D$V[8\x80\x80\x80\x80a\x1AeV[P\x85\x15a\x1ASV[\x90P\x15\x158a\x1AKV[\x88\x95a\x16\xEDV[\x8B\x92\x91\x92;\x15a\0\x1BW\x8B\x8F\x8B\x85\x85\x92\x88\x95\x8E\x8E\x8E\x8E\x8E`@Q\x9E\x8F\x9A\x8B\x9Ac:\xF8]\xA9`\xE1\x1B\x8CR3\x99`\x04\x8D\x01\x9Ba\x1B\xA1\x9Ca\x17\xB6V[\x03\x81Z`\0\x94\x85\x91\xF1\x92\x83\x15a\x1A\xFFWa\x1AA\x94a\x1A9\x94a\x1B\xC8W[P\x91\x92P\x92a\x19\xD0V[\x80a\x1A\xF9a\x1B\xD5\x92a\x04.V[8a\x1B\xBEV[\x15a\x1B\xE2WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x18\x99`\xD9\x1B`D\x82\x01R`d\x90\xFD[\x15a\x1C\x16WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS011`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x81\x16a\x1D\x1EWP2\x93[\x80\x82\x16a\x1C\xA6WP`\0\x80\x80\x80\x93a\x1C\x81a\x02=\x96:`\x01\x10\x84\x14a\x1C\x9FW`\x01\x90a*\x0BV[\x97\x88\x91\x83\x91\x83\x15a\x1C\x95W[\x16\x90\xF1a\x1C\x0FV[a\x08\xFC\x92Pa\x1C\x8DV[:\x90a*\x0BV[a\x1C\xB1` \x93a)\xEFV[\x94`\0\x80\x93`@Q\x90\x86\x82\x01\x93c\xA9\x05\x9C\xBB`\xE0\x1B\x85R\x16`$\x82\x01R\x87`D\x82\x01R`D\x81Ra\x1C\xE1\x81a\x04cV[Q\x92a'\x0F\x19Z\x01\xF1=\x80\x15a\x1D\x14W` \x14a\x1D\x03WPa\x02=`\0a\x1B\xDBV[a\x02=\x90`\0Q\x15\x90\x15\x17\x15a\x1B\xDBV[Pa\x02=\x90a\x1B\xDBV[\x93a\x1CZV[\x92\x94\x93\x90\x92`\x01`\x01`\xA0\x1B\x03\x92\x91\x83\x81\x16a\x1D\x8AWP2\x95[\x80\x84\x16a\x1DuWP`\0\x80\x93a\x1C\x81\x82\x94a\x1D]a\x02=\x98\x85\x96a*YV[\x90:\x81\x10\x85\x14a\x1DmW\x90a*\x0BV[P:\x90a*\x0BV[\x91a\x1D\x85` \x95a\x1C\xB1\x93a*YV[a*\x0BV[\x95a\x1D>V[\x91`\x04T\x91\x82\x15a\x1D\xA4Wa\x02=\x93a\x1F\x95V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS001`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x15a\x1D\xD8WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03#`\xDC\x1B`D\x82\x01R`d\x90\xFD[`\xFF`\x03\x19\x91\x16\x01\x90`\xFF\x82\x11a\x08\x0FWV[\x15a\x1E\x1FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS025`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x15a\x1ESWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS021`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x15a\x1E\x87WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x19\x19`\xD9\x1B`D\x82\x01R`d\x90\xFD[\x15a\x1E\xBBWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS023`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x90\x81` \x91\x03\x12a\0\x1BWQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\0\x1BW\x90V[\x90\x91a\x1F\x1Fa\x05,\x93`@\x84R`@\x84\x01\x90a\x06\xE8V[\x91` \x81\x84\x03\x91\x01Ra\x06\xE8V[\x15a\x1F4WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCC\x0C\x8D`\xDA\x1B`D\x82\x01R`d\x90\xFD[\x15a\x1FhWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x19\x1B`\xD9\x1B`D\x82\x01R`d\x90\xFD[\x91\x92\x90\x92a\x1F\xAE\x81Qa\x1F\xA7\x84a)\xC0V[\x11\x15a\x1D\xD1V[`\0\x90\x81[\x83\x83\x10a\x1F\xC2WPPPPPPV[\x85\x90a\x1F\xE4\x84\x84\x90`A\x02\x01` \x81\x01Q\x90`\xFF`A`@\x83\x01Q\x92\x01Q\x16\x92V[\x92\x91`\xFF\x81\x16\x80a!7WPP\x90`\x01`\x01`\xA0\x1B\x03a n\x92\x16\x92a \x14a \x0C\x89a)\xC0V[\x82\x10\x15a\x1ELV[a )a  \x82a*DV[\x87Q\x10\x15a\x1E\x80V[` \x81a Ra I\x83\x80\x95\x8B\x01\x01\x92a D\x84Q\x91a*DV[a*YV[\x89Q\x10\x15a\x1E\xB4V[`@Q\x80\x95\x81\x92c \xC1;\x0B`\xE0\x1B\x99\x8A\x84R`\x04\x84\x01a\x1F\x08V[\x03\x81\x87Z\xFA\x94\x85\x15a\x1A\xFFWa \xCB\x95a \xC5\x94a \xA0\x93`\0\x92a!\nW[PP`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x1F-V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x80\x85\x16\x91\x16\x81\x11\x91\x82a \xDDW[P\x81a \xD1W[Pa\x1FaV[\x92a\x17\x84V[\x91a\x1F\xB3V[`\x01\x91P\x14\x158a \xBFV[\x90\x91Pa!\0a\x02\xFC\x85`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x16\x15\x15\x908a \xB8V[a!)\x92P\x80=\x10a!0W[a!!\x81\x83a\x04\x9CV[\x81\x01\x90a\x1E\xE8V[8\x80a \x8EV[P=a!\x17V[\x91\x93\x90\x92\x94P`\x01\x91\x82\x81\x14`\0\x14a!\xA7WPPPP\x90a \xC5`\x01`\x01`\xA0\x1B\x03a \xCB\x93\x16\x91\x823\x14\x80\x15a!xW[a!s\x90a\x1E\x18V[a \xA0V[Pa!sa!\x9D\x89a\x0F\"\x86`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x08` R`@`\0 \x90V[T\x15\x15\x90Pa!jV[\x92\x93\x92`\x1E\x10\x15a\"YW`\0\x92\x93a\"=`@\x92\x83Q\x96a\"\x18\x8D` \x99a\"\x0F\x81a\"\x01\x8D\x82\x01\x94\x85`<\x91\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x82R`\x1C\x82\x01R\x01\x90V[\x03`\x1F\x19\x81\x01\x83R\x82a\x04\x9CV[Q\x90 \x94a\x1E\x05V[\x94Q\x94\x85\x94\x85\x90\x94\x93\x92`\xFF``\x93`\x80\x84\x01\x97\x84R\x16` \x83\x01R`@\x82\x01R\x01RV[\x84\x80R\x03\x91Z\xFA\x15a\x1A\xFFWa \xCB\x90a \xC5`\0Q\x91a \xA0V[\x91` \x93a\"=`\0\x94`@Q\x93\x84\x93\x8D\x85\x90\x94\x93\x92`\xFF``\x93`\x80\x84\x01\x97\x84R\x16` \x83\x01R`@\x82\x01R\x01RV[`@Q` \x81\x01\x90\x7FG\xE7\x954\xA2E\x95.\x8B\x16\x89:3k\x85\xA3\xD9\xEA\x9F\xA8\xC5s\xF3\xD8\x03\xAF\xB9*yF\x92\x18\x82RF`@\x82\x01R0``\x82\x01R``\x81Ra\"\xCE\x81a\x04cV[Q\x90 \x90V[\x94\x90\x98\x95\x91\x7F\xBB\x83\x10\xD4\x866\x8D\xB6\xBDo\x84\x94\x02\xFD\xD7:\xD5=1kZK&D\xADn\xFE\x0F\x94\x12\x86\xD8\x9A\x98\x94a#\x0Fa#E\x92`@\x9A\x966\x91a\x04\xDAV[` \x81Q\x91\x01 \x89Q\x9B` \x8D\x01\x9D\x8ER\x8C`\x01`\x01`\xA0\x1B\x03\x9B\x8C\x80\x9B\x16\x91\x01R``\x8D\x01R`\x80\x8C\x01R`\xA0\x8B\x01\x90a\x17\x93V[`\xC0\x89\x01R`\xE0\x88\x01Ra\x01\0\x87\x01R\x16a\x01 \x85\x01R\x16a\x01@\x83\x01Ra\x01`\x90\x81\x83\x01R\x81Ra#v\x81a\x04\x7FV[Q\x90 a\x05,a#\x84a\"\x8AV[`@Q`\x19`\xF8\x1B` \x82\x01R`\x01`\xF8\x1B`!\x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x92\x90\x92R\x81`b\x81\x01a\"\x01V[\x93\x90\x93`\x02\x84\x10\x15a\x17\xA0W`\0\x94\x85\x94`\x01\x03a#\xD9WP` \x83Q\x93\x01\x91\xF4\x90V[\x90` \x84Q\x94\x01\x92\xF1\x90V[\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5T\x80\x15a\0(W`\0\x80\x80\x926\x82\x8073``\x1B6R\x81\x80`\x146\x01\x92Z\xF1=\x82\x80>\x15a$2W=\x90\xF3[=\x90\xFD[`\x01`\0R`\x01` R`\x01`\x01`\xA0\x1B\x03\x80`@`\0 T\x16a$\xF0W`\x01`\0\x81\x90R` Ra$\xA3\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/[\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x90UV[\x81\x16a$\xADWPPV[`\0\x91\x82\x91Z\x90` \x83Q\x93\x01\x91\xF4\x15a$\xC3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03\x03`\xDC\x1B`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x13\x03`\xDC\x1B`D\x82\x01R`d\x90\xFD[\x15a%$WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS101`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x15a%XWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x98\x19`\xD9\x1B`D\x82\x01R`d\x90\xFD[\x91\x92\x90\x92`\x013\x14\x15\x80a&(W[\x15a%\xFBWa%\xA4\x93Z\x93a#\xB5V[\x90\x81\x15a%\xD3W3\x7Fh\x95\xC16d\xAAOg(\x8B%\xD7\xA2\x1Dz\xAA4\x91n5_\xB9\xB6\xFA\xE0\xA19\xA9\x08[\xEC\xB8`\0\x80\xA2V[3\x7F\xAC\xD2\xC8p(\x04\x12\x8F\xDB\r\xB2\xBBI\xF6\xD1'\xDD\x01\x81\xC1?\xD4]\xBF\xE1m\xE0\x93\x0E+\xD3u`\0\x80\xA2V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCCL\r`\xDA\x1B`D\x82\x01R`d\x90\xFD[P3`\0R`\x01` R`\x01`\x01`\xA0\x1B\x03`@`\0 T\x16\x15\x15a%\x94V[\x90a&R\x82a\x15\xD8V[a&_`@Q\x91\x82a\x04\x9CV[\x82\x81R\x80\x92a&p`\x1F\x19\x91a\x15\xD8V[\x01\x90` 6\x91\x017V[\x80Q\x82\x10\x15a&\x8EW` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x15a&\xABWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3#\x03`\xDC\x1B`D\x82\x01R`d\x90\xFD[\x15a&\xDFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS201`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x15a'\x13WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x99\x18\x19`\xD9\x1B`D\x82\x01R`d\x90\xFD[\x15a'GWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS203`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x15a'{WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCC\x8C\r`\xDA\x1B`D\x82\x01R`d\x90\xFD[a'\xB4`\x04T\x15a&\xA4V[a'\xC1\x81Q\x83\x11\x15a&\xD8V[`\x01a'\xCF\x81\x84\x10\x15a'\x0CV[`\0\x81\x80[a(\x0EW[PPa\x02=\x92\x91a(\x03a$\x83a(\t\x93`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[Q`\x03UV[`\x04UV[\x90\x91\x83Q\x83\x10\x15a(\xC1Wa(\x98a\x03na(\x92\x84\x93a(>a(1\x88\x8Aa&zV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92\x83\x91a(l`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x80\x15\x15\x91\x82a(\xB6W[\x82a(\xABW[\x82a(\x9EW[PPa'@V[a\x13Pa\x03\x15a\x03\ta\x02\xFC\x86`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x93a\x17\x84V[\x91a'\xD4V[\x84\x16\x14\x15\x90P8\x80a(eV[0\x82\x14\x15\x92Pa(_V[\x81\x8B\x14\x15\x92Pa(YV[\x91a'\xD9V[\x15a(\xCEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x80\x15a\x08\x0FW`\0\x19\x01\x90V[` \x7Fa\x0F\x7F\xF2\xB3\x04\xAE\x89\x03\xC3\xDEt\xC6\x0Cj\xB1\xF7\xD6\"k?R\xC5\x16\x19\x05\xBBZ\xD4\x03\x9C\x93\x91a)4a)\x8AV[a)B`\x03T\x82\x11\x15a&\xD8V[a)O`\x01\x82\x10\x15a'\x0CV[\x80`\x04U`@Q\x90\x81R\xA1V[`@Q4\x81R\x7F=\x0C\xE9\xBF\xC3\xED}hb\xDB\xB2\x8B-\xEA\x94V\x1F\xE7\x14\xA1\xB4\xD0\x19\xAA\x8A\xF3\x970\xD1\xAD|=` 3\x92\xA2V[03\x03a)\x93WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS031`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x80\x15a)\xE9W`A\x81\x02\x90\x80\x82\x04`A\x03a\x08\x0FWa)\xE1`A\x91\x83a\x18\x8CV[\x03a\0\x1BW\x90V[P`\0\x90V[\x80\x15a)\xE9W\x80\x80\x04`\x01\x03a\x08\x0FW`\x01a)\xE1\x82\x80a\x18\x8CV[\x90\x81\x15a*)W\x80\x82\x02\x91\x80\x83\x04\x82\x03a\x08\x0FWa)\xE1\x90\x83a\x18\x8CV[PP`\0\x90V[\x90\x81\x81\x11a\0\x1BW\x81\x03\x90\x81\x11a\x08\x0FW\x90V[` \x81\x01\x90\x81\x81\x11a\x08\x0FW\x81\x10a\0\x1BW\x90V[\x90\x81\x01\x90\x81\x81\x11a\x08\x0FW\x81\x10a\0\x1BW\x90V[\x90\x80\x82\x10a*yWP\x90V[\x90P\x90V\xFE\xA2dipfsX\"\x12 :\xC1\xB7<\x9A\x1C$\xBD\xD8D\xA9x\xE2Qtz'A\xC5LA\x87\x10\x96\xBF\xA9\x800\xAAuwJdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static GNOSISSAFE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0*W[6\x15a\0 W4\x15a#\xE5W[`\0\x80\xFD[a\0(a)\\V[\0[`\x005`\xE0\x1C\x80c\rX/\x13\x14a\x02\x1AW\x80c\x12\xFBh\xE0\x14a\x02\x15W\x80c-\x9A\xD5=\x14a\x02\x10W\x80c/T\xBFn\x14a\x02\x0BW\x80c4\x08\xE4p\x14a\x02\x06W\x80cF\x87!\xA7\x14a\x02\x01W\x80cR)\x07?\x14a\x01\xFCW\x80cV$\xB2[\x14a\x01\xF7W\x80cZ\xE6\xBD7\x14a\x01\xF2W\x80ca\x0BY%\x14a\x01\xEDW\x80ciN\x80\xC3\x14a\x01\xE8W\x80cjv\x12\x02\x14a\x01\xE3W\x80c}\x83)t\x14a\x01\xDEW\x80c\x93O:\x11\x14a\x01\xD9W\x80c\xA0\xE6~+\x14a\x01\xD4W\x80c\xAF\xFE\xD0\xE0\x14a\x01\xCFW\x80c\xB4\xFA\xBA\t\x14a\x01\xCAW\x80c\xB6>\x80\r\x14a\x01\xC5W\x80c\xC4\xCA:\x9C\x14a\x01\xC0W\x80c\xCC/\x84R\x14a\x01\xBBW\x80c\xD4\xD9\xBD\xCD\x14a\x01\xB6W\x80c\xD8\xD1\x1Fx\x14a\x01\xB1W\x80c\xE0\t\xCF\xDE\x14a\x01\xACW\x80c\xE1\x9A\x9D\xD9\x14a\x01\xA7W\x80c\xE3\x18\xB5+\x14a\x01\xA2W\x80c\xE7R5\xB8\x14a\x01\x9DW\x80c\xE8f7\xDB\x14a\x01\x98W\x80c\xF0\x8A\x03#\x14a\x01\x93W\x80c\xF6\x98\xDA%\x14a\x01\x8EW\x80c\xF8\xDC]\xD9\x14a\x01\x89Wc\xFF\xA1\xADt\x03a\0\x0EWa\x15\x92V[a\x14\x97V[a\x14tV[a\x13\xF7V[a\x13\xCFV[a\x13\xB1V[a\x11\xF9V[a\x11|V[a\x10:V[a\x10\x07V[a\x0E\xCCV[a\r\xF1V[a\r\"V[a\x0C\x8AV[a\x0C*V[a\x0C\x0CV[a\x0ByV[a\n\xE6V[a\n\x9AV[a\t\xF8V[a\tUV[a\x08@V[a\x08\x14V[a\x07\x84V[a\x07(V[a\x06\xC2V[a\x06LV[a\x05\xE5V[a\x05\x88V[a\x05/V[a\x02fV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\0\x1BWV[`\xE45\x90a\x02=\x82a\x02\x1FV[V[`D5\x90a\x02=\x82a\x02\x1FV[`\x845\x90a\x02=\x82a\x02\x1FV[`\xA45\x90a\x02=\x82a\x02\x1FV[4a\0\x1BW`@6`\x03\x19\x01\x12a\0\x1BW`\x045a\x02\x83\x81a\x02\x1FV[\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&a\x03\xEA`$5\x92a\x02\xB3a)\x8AV[a\x02\xD6`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x15\x15\x90\x81a\x04\x0CW[\x81a\x04\x01W[Pa'@V[a\x03\x1Ba\x03\x15a\x03\ta\x02\xFC\x84`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x15a'tV[`\x01`\0R`\x02` Ra\x03\x89a\x03Q\x7F\xE9\x0B{\xCE\xB6\xE7\xDFT\x18\xFBx\xD8\xEETn\x97\xC8:\x08\xBB\xCC\xC0\x1A\x06D\xD5\x99\xCC\xD2\xA7\xC2\xE0a\x02\xFCV[a\x03n\x83`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x90`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x82T\x16\x17\x90UV[`\x01`\0R`\x02` Ra\x03\xBD\x81\x7F\xE9\x0B{\xCE\xB6\xE7\xDFT\x18\xFBx\xD8\xEETn\x97\xC8:\x08\xBB\xCC\xC0\x1A\x06D\xD5\x99\xCC\xD2\xA7\xC2\xE0a\x03nV[a\x03\xD0a\x03\xCB`\x03Ta\x17\x84V[`\x03UV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA1\x80`\x04T\x03a\x03\xF8W\0[a\0(\x90a)\x08V[\x90P0\x14\x158a\x02\xD0V[`\x01\x81\x14\x15\x91Pa\x02\xCAV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04BW`@RV[a\x04\x18V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04BW`@RV[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04BW`@RV[a\x01\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04BW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04BW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04BW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x04\xE6\x82a\x04\xBEV[\x91a\x04\xF4`@Q\x93\x84a\x04\x9CV[\x82\x94\x81\x84R\x81\x83\x01\x11a\0\x1BW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\0\x1BW\x81` a\x05,\x935\x91\x01a\x04\xDAV[\x90V[4a\0\x1BW`\x806`\x03\x19\x01\x12a\0\x1BWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x81\x81\x11a\0\x1BWa\x05a\x906\x90`\x04\x01a\x05\x11V[`D5\x91\x82\x11a\0\x1BWa\x05|a\0(\x926\x90`\x04\x01a\x05\x11V[`d5\x91`\x045a\x1F\x95V[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW` `\x045a\x05\xA7\x81a\x02\x1FV[`\x01`\x01`\xA0\x1B\x03\x80\x91\x16\x90\x81`\x01\x14\x15\x91\x82a\x05\xCBW[PP`@Q\x90\x15\x15\x81R\xF3[\x90\x91P`\0R`\x01\x82R`@`\0 T\x16\x15\x158\x80a\x05\xBFV[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW` `\x045a\x06\x04\x81a\x02\x1FV[`\x01`\x01`\xA0\x1B\x03\x80\x91\x16\x90`\x01\x82\x14\x15\x91\x82a\x06'WPP`@Q\x90\x15\x15\x81R\xF3[\x90\x91P`\0R`\x02\x82R`@`\0 T\x16\x15\x158\x80a\x05\xBFV[`\0\x91\x03\x12a\0\x1BWV[4a\0\x1BW`\x006`\x03\x19\x01\x12a\0\x1BW` `@QF\x81R\xF3[`d5\x90`\x02\x82\x10\x15a\0\x1BWV[`\x80`\x03\x19\x82\x01\x12a\0\x1BW`\x045a\x06\x8E\x81a\x02\x1FV[\x91`$5\x91`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\0\x1BWa\x06\xB2\x91`\x04\x01a\x05\x11V[\x90`d5`\x02\x81\x10\x15a\0\x1BW\x90V[4a\0\x1BW` a\x06\xDEa\x06\xD56a\x06vV[\x92\x91\x90\x91a%\x85V[`@Q\x90\x15\x15\x81R\xF3[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x07\x14WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x06\xF3V[4a\0\x1BWa\x079a\x06\xD56a\x06vV[`@Q\x90` =\x83\x01\x01`@R=\x82R=`\0` \x84\x01>a\x07o`@Q\x92\x83\x92\x15\x15\x83R`@` \x84\x01R`@\x83\x01\x90a\x06\xE8V[\x03\x90\xF3[\x90` a\x05,\x92\x81\x81R\x01\x90a\x06\xE8V[4a\0\x1BW`@6`\x03\x19\x01\x12a\0\x1BW`\x045`$5`\x05\x81\x81\x1B\x92` \x90\x83\x85\x04\x82\x14\x84\x15\x17\x15a\x08\x0FWa\x07\xBA\x85a\x04\xBEV[\x94a\x07\xC8`@Q\x96\x87a\x04\x9CV[\x80\x86Ra\x07\xD7`\x1F\x19\x91a\x04\xBEV[\x016\x83\x87\x017`\0[\x84\x81\x10a\x07\xF5W`@Q\x80a\x07o\x88\x82a\x07sV[\x80a\x08\n\x91\x83\x01T\x84\x82\x87\x1B\x89\x01\x01Ra\x17\x84V[a\x07\xE0V[a\x17nV[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW`\x045`\0R`\x07` R` `@`\0 T`@Q\x90\x81R\xF3[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW\x7F\xEC\xDF:>\xFF\xEAW\x83\xA3\xC4\xC2\x14\x0Eguwfd(\xD4N\xD9\xD4t\xA0\xB3\xA4\xC9\x94?\x84@a\tE`\x045a\x08\x81\x81a\x02\x1FV[a\x08\x89a)\x8AV[a\x08\xBE`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x80\x15\x15\x80a\tJW[a\x08\xA9\x90a%\x1DV[`\0R`\x01` R`@`\0 T\x16\x15a%QV[`\x01`\0\x81\x90R` Ra\t\x11a\x08\xF4\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/a\x02\xFCV[a\x03n\x83`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[`\x01`\0\x81\x90R` Ra\x03\xD0\x81\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/a\x03nV[\x03\x90\xA1\0[P`\x01\x81\x14\x15a\x08\xA0V[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW\x7Fa\x0F\x7F\xF2\xB3\x04\xAE\x89\x03\xC3\xDEt\xC6\x0Cj\xB1\xF7\xD6\"k?R\xC5\x16\x19\x05\xBBZ\xD4\x03\x9C\x93` `\x045a\t\x94a)\x8AV[a\t\xA2`\x03T\x82\x11\x15a&\xD8V[a\t\xAF`\x01\x82\x10\x15a'\x0CV[\x80`\x04U`@Q\x90\x81R\xA1\0[\x91\x81`\x1F\x84\x01\x12\x15a\0\x1BW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\x1BW` \x83\x81\x86\x01\x95\x01\x01\x11a\0\x1BWV[a\x01\x045\x90a\x02=\x82a\x02\x1FV[a\x01@6`\x03\x19\x01\x12a\0\x1BW`\x045a\n\x11\x81a\x02\x1FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`D5\x81\x81\x11a\0\x1BWa\n2\x906\x90`\x04\x01a\t\xBCV[\x90a\n;a\x06gV[\x91`\xE45\x90a\nI\x82a\x02\x1FV[a\nQa\t\xEAV[\x92a\x01$5\x95\x86\x11a\0\x1BWa\x07o\x96a\nra\n\x88\x976\x90`\x04\x01a\x05\x11V[\x95`\xC45\x93`\xA45\x93`\x845\x93`$5\x90a\x19QV[`@Q\x90\x15\x15\x81R\x90\x81\x90` \x82\x01\x90V[4a\0\x1BW`@6`\x03\x19\x01\x12a\0\x1BW`\x01`\x01`\xA0\x1B\x03`\x045a\n\xBF\x81a\x02\x1FV[\x16`\0R`\x08` R`@`\0 `$5`\0R` R` `@`\0 T`@Q\x90\x81R\xF3[4a\0\x1BW``6`\x03\x19\x01\x12a\0\x1BWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x81\x81\x11a\0\x1BWa\x0B\x18\x906\x90`\x04\x01a\x05\x11V[`D5\x91\x82\x11a\0\x1BWa\x0B3a\0(\x926\x90`\x04\x01a\x05\x11V[\x90`\x045a\x1D\x90V[\x90\x81Q\x80\x82R` \x80\x80\x93\x01\x93\x01\x91`\0[\x82\x81\x10a\x0B\\WPPPP\x90V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x0BNV[4a\0\x1BW`\0\x80`\x03\x196\x01\x12a\x0C\tWa\x0B\x96`\x03Ta&HV[\x90\x80`\x01\x91\x82\x82R`\x02` \x92\x81\x84R`\x01`\x01`\xA0\x1B\x03\x91`@\x93\x86\x84\x86\x85 T\x16\x90[a\x0B\xD2W[\x85Q\x87\x81R\x80a\x07o\x81\x8A\x01\x8Ca\x0B<V[\x84\x16\x87\x81\x14a\x0C\x04W\x90\x81\x88\x92a\x0B\xE9\x83\x8Ca&zV[R\x84R\x82\x87Ra\x0B\xFE\x85\x87\x86 T\x16\x91a\x17\x84V[\x91a\x0B\xBBV[a\x0B\xC0V[\x80\xFD[4a\0\x1BW`\x006`\x03\x19\x01\x12a\0\x1BW` `\x05T`@Q\x90\x81R\xF3[4a\0\x1BW`@6`\x03\x19\x01\x12a\0\x1BW`\x045a\x0CG\x81a\x02\x1FV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\x1BW`\0\x91a\x0Ck\x83\x926\x90`\x04\x01a\x05\x11V[\x90` \x82Q\x92\x01\x90Z\xF4`\0R=` R=`\0`@>`@=\x01`\0\xFD[4a\0\x1BWa\x01\x006`\x03\x19\x01\x12a\0\x1BWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x81\x81\x11a\0\x1BW6`#\x82\x01\x12\x15a\0\x1BW\x80`\x04\x015\x82\x81\x11a\0\x1BW6`$\x82`\x05\x1B\x84\x01\x01\x11a\0\x1BWa\x0C\xDDa\x02?V[`d5\x93\x84\x11a\0\x1BWa\x0C\xF8a\0(\x946\x90`\x04\x01a\t\xBCV[a\r\0a\x02LV[\x91a\r\ta\x02YV[\x93a\r\x12a\x020V[\x96`\xC45\x96`$\x805\x92\x01a\x16^V[4a\0\x1BW`\x806`\x03\x19\x01\x12a\0\x1BW`\x045a\r?\x81a\x02\x1FV[`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\x1BWa\rba\r\x85\x916\x90`\x04\x01a\t\xBCV[\x90\x92a\rla\x06gV[\x90a\r{Z\x95Z\x946\x91a\x04\xDAV[\x90`$5\x90a#\xB5V[\x15a\0\x1BWZ\x81\x03\x90\x81\x11a\x08\x0FWa\r\xC6\x90`@Q\x90` \x82\x01R` \x81Ra\r\xAE\x81a\x04GV[`@QbF\x1B\xCD`\xE5\x1B\x81R\x91\x82\x91`\x04\x83\x01a\x07sV[\x03\x90\xFD[\x90`\x01`\x01`\xA0\x1B\x03a\r\xEA` \x92\x95\x94\x95`@\x85R`@\x85\x01\x90a\x0B<V[\x94\x16\x91\x01RV[4a\0\x1BW`@6`\x03\x19\x01\x12a\0\x1BW`\x045a\x0E\x0E\x81a\x02\x1FV[`$5\x90a\x0E\x1B\x82a&HV[\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x80\x91\x16\x82R`\x01\x93\x84\x91\x82` R\x80`@\x85 T\x16\x95[a\x0EWW[PPP\x81Ra\x07o`@Q\x92\x83\x92\x83a\r\xCAV[\x90\x91\x92\x94\x81\x81\x16\x80\x15\x15\x80a\x0E\xC2W[\x80a\x0E\xB9W[\x15a\x0E\xB1Wa\x0E\xA3a\x02\xFC\x86\x94\x93a\x0E\xA9\x93a\x0E\x89\x8B\x8Ba&zV[R`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x96a\x17\x84V[\x93\x92\x91a\x0E>V[P\x94\x92a\x0ECV[P\x83\x87\x10a\x0EmV[P\x84\x81\x14\x15a\x0EgV[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW`\x045`\0\x903\x82R`\x02` R`\x01`\x01`\xA0\x1B\x03`@\x83 T\x16\x15a\x0F`Wa\x0F7a\x0F1\x82a\x0F\"3`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x08` R`@`\0 \x90V[\x90`\0R` R`@`\0 \x90V[`\x01\x90UV[3\x90\x7F\xF2\xA0\xEB\x15dr\xD1D\x02U\xB0\xD7\xC1\xE1\x9C\xC0q\x15\xD1\x05\x1F\xE6\x05\xB0\xDC\xE6\x9A\xCF\xEC\x88M\x9C\x83\x80\xA3\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x033`\xDC\x1B`D\x82\x01R`d\x90\xFD[a\x01@`\x03\x19\x82\x01\x12a\0\x1BW`\x045a\x0F\xA6\x81a\x02\x1FV[\x91`$5\x91`D5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\0\x1BWa\x0F\xCA\x91`\x04\x01a\t\xBCV[\x90\x91`d5`\x02\x81\x10\x15a\0\x1BW\x90`\x845\x90`\xA45\x90`\xC45\x90`\xE45a\x0F\xF1\x81a\x02\x1FV[\x90a\x01\x045a\x0F\xFF\x81a\x02\x1FV[\x90a\x01$5\x90V[4a\0\x1BW` a\x10,a\x10\x1A6a\x0F\x8DV[\x99\x98\x90\x98\x97\x91\x97\x96\x92\x96\x95\x93\x95a\"\xD4V[\x81\x81Q\x91\x01 `@Q\x90\x81R\xF3[4a\0\x1BW`@6`\x03\x19\x01\x12a\0\x1BW`\x045a\x10W\x81a\x02\x1FV[`$5\x90a\x10d\x82a\x02\x1FV[a\x10la)\x8AV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x90\x81\x15\x15\x80a\x11qW[a\x10\x8A\x90a%\x1DV[\x80\x83\x16`\0R`\x01` R`@`\0 T\x16\x03a\x11DW\x81a\x11\x07a\tE\x92a\x03na\x10\xEDa\x02\xFC\x7F\xAA\xB4\xFA+F?X\x1B+2\xCB;~;pK\x9C\xE3|\xC2\t\xB5\xFBMw\xE5\x93\xAC\xE4\x05Bv\x97`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x91`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[a\x03\xD0a\x11'\x82`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS103`\xD8\x1B`D\x82\x01R`d\x90\xFD[P`\x01\x82\x14\x15a\x10\x81V[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW\x7F\x11Q\x11i\x14Q[\xC0\x89\x1F\xF9\x04zl\xB3,\xF9\x02To\x83\x06d\x99\xBC\xF8\xBA3\xD25?\xA2` `\x045a\x11\xBC\x81a\x02\x1FV[a\x11\xC4a)\x8AV[\x80\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8U`\x01`\x01`\xA0\x1B\x03`@Q\x91\x16\x81R\xA1\0[4a\0\x1BW``6`\x03\x19\x01\x12a\0\x1BW\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&a\tEa\x03n\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAFa\x13\x89`\x045a\x12a\x81a\x02\x1FV[a\x13i`$5\x91a\x12q\x83a\x02\x1FV[`D5\x95\x86\x91a\x12\x80\x83a\x02\x1FV[a\x12\x88a)\x8AV[a\x13\x10`\x01`\x01`\xA0\x1B\x03a\x12\xAD\x81\x86\x16\x80\x15\x15\x90\x81a\x04\x0CW\x81a\x04\x01WPa'@V[a\x12\xD3a\x03\x15a\x03\ta\x02\xFC\x88`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x86\x16\x80\x15\x15\x80a\x13\xA6W[a\x12\xE7\x90a'@V[a\x13\na\x03\ta\x02\xFC\x85`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x14a(\xC7V[a\x13Pa\x133a\x02\xFC\x87`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[a\x03n\x85`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[a\x03\xD0a\x11'\x82`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x03\x90\xA1`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[P`\x01\x81\x14\x15a\x12\xDEV[4a\0\x1BW`\x006`\x03\x19\x01\x12a\0\x1BW` `\x04T`@Q\x90\x81R\xF3[4a\0\x1BWa\x07oa\x13\xE3a\x10\x1A6a\x0F\x8DV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\xE8V[4a\0\x1BW` 6`\x03\x19\x01\x12a\0\x1BW\x7FZ\xC6\xC4l\x93\xC8\xD0\xE57\x14\xBA;S\xDB>|\x04m\xA9\x941=~\xD0\xD1\x92\x02\x8B\xC7\xC2(\xB0` `\x045a\x147\x81a\x02\x1FV[a\x14?a)\x8AV[\x80\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5U`\x01`\x01`\xA0\x1B\x03`@Q\x91\x16\x81R\xA1\0[4a\0\x1BW`\x006`\x03\x19\x01\x12a\0\x1BW` a\x14\x8Fa\"\x8AV[`@Q\x90\x81R\xF3[4a\0\x1BW``6`\x03\x19\x01\x12a\0\x1BW`\x045a\x14\xB4\x81a\x02\x1FV[\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAFa\x03\xEA`$5a\x14\xE4\x81a\x02\x1FV[a\x15d`D5\x94a\x14\xF3a)\x8AV[a\x15\t\x86a\x15\x02`\x03Ta\x18\xFEV[\x10\x15a&\xD8V[a\x15'`\x01`\x01`\xA0\x1B\x03\x84\x16\x80\x15\x15\x80a\x13\xA6Wa\x12\xE7\x90a'@V[a\x03na\x15Ja\x02\xFC\x85`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x91`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[a\x15\x84a\x11'\x82`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[a\x03\xD0a\x03\xCB`\x03Ta(\xFBV[4a\0\x1BW`\x006`\x03\x19\x01\x12a\0\x1BWa\x07o`@Qa\x15\xB2\x81a\x04GV[`\x05\x81Rd\x03\x12\xE32\xE3`\xDC\x1B` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x06\xE8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04BW`\x05\x1B` \x01\x90V[\x93\x92\x95\x94\x91\x80`\x80\x86\x01`\x80\x87RR`\xA0\x85\x01\x91\x90`\0[\x81\x81\x10a\x163WPPP\x90``\x92\x91\x95` \x85\x01R`\x01`\x01`\xA0\x1B\x03\x80\x92\x16`@\x85\x01R\x16\x91\x01RV[\x90\x91\x92`\x01\x90`\x01`\x01`\xA0\x1B\x03\x855a\x16L\x81a\x02\x1FV[\x16\x81R` \x90\x81\x01\x94\x01\x92\x91\x01a\x16\x08V[\x98\x92\x94\x97\x93\x90\x96\x91\x97a\x16p\x88a\x15\xD8V[a\x16}`@Q\x91\x82a\x04\x9CV[\x88\x81R` \x80\x82\x01\x8A`\x05\x1B\x8D\x01\x906\x82\x11a\0\x1BW\x8D\x90[\x82\x82\x10a\x17UWPPPPa\x16\xF4a\x17\x11\x97\x95\x93\x8B\x9A\x99\x97\x95\x93a\x16\xDE\x7F\x14\x1D\xF8h\xA63\x1A\xF5(\xE3\x8C\x83\xB7\xAA\x03\xED\xC1\x9B\xE6n7\xAEg\xF9([\xF4\xF8\xE3\xC6\xA1\xA8\x9Da\x16\xFA\x95a'\xA8V[`\x01`\x01`\xA0\x1B\x03\x89\x16a\x17(W[6\x91a\x04\xDAV[\x85a$6V[\x82a\x17\x16W[PPP`@Q\x94\x85\x943\x98\x86a\x15\xF0V[\x03\x90\xA2V[a\x17\x1F\x92a\x1CCV[P8\x80\x80a\x17\0V[a\x17P\x89\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5UV[a\x16\xEDV[\x83\x80\x91\x835a\x17c\x81a\x02\x1FV[\x81R\x01\x91\x01\x90a\x16\x96V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x19\x81\x14a\x08\x0FW`\x01\x01\x90V[\x90`\x02\x82\x10\x15a\x17\xA0WRV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x9C\x9B\x99\x92\x95\x97`\xC0\x8Ea\x02=\x9D\x99a\x18Z\x9Ba\x18\x1Ba\x18H\x99a\x01@\x9F\x97\x98`\x01`\x01`\xA0\x1B\x03a\x187\x9A\x16\x86R` \x86\x01R\x80a\x01`\x80`@\x88\x01R\x86\x01R\x80a\x01\x80\x9D\x8E\x87\x017`\0\x85\x82\x01\x8E\x01R`\x1F\x01`\x1F\x19\x16\x84\x01\x9A``\x85\x01\x90a\x17\x93V[`\x80\x83\x01R`\xA0\x82\x01R\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x8D\x01RV[`\x01`\x01`\xA0\x1B\x03\x16a\x01\0\x8B\x01RV[\x81\x89\x82\x03\x01a\x01 \x8A\x01R\x01\x90a\x06\xE8V[\x94\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`@Q=`\0\x82>=\x90\xFD[\x90\x81`\x06\x1B\x91\x80\x83\x04`@\x14\x90\x15\x17\x15a\x08\x0FWV[\x81\x15a\x18\x96W\x04\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x90a\t\xC4\x82\x01\x80\x92\x11a\x08\x0FWV[\x90a\x01\xF4\x82\x01\x80\x92\x11a\x08\x0FWV[\x15a\x18\xD1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03\x13`\xDC\x1B`D\x82\x01R`d\x90\xFD[`\0\x19\x81\x01\x91\x90\x82\x11a\x08\x0FWV[a\t\xC3\x19\x81\x01\x91\x90\x82\x11a\x08\x0FWV[\x15a\x19$WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS013`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x97\x93\x83\x8A\x8C\x9B\x97\x93\x9D\x9C\x99\x95\x8B\x8F\x96\x80\x9B\x97\x8A\x8A\x8A\x8A\x8A`\x05T\x98\x89\x97`\x01`\x01`\xA0\x1B\x03\x80\x9C\x16\x97a\x19\x83\x9Aa\"\xD4V[\x90a\x19\x8D\x90a\x17\x84V[`\x05U\x80Q` \x82\x01 \x9Ca\x19\xA2\x91\x8Ea\x1D\x90V[\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8T\x16\x99\x8A\x15\x15\x99\x8Aa\x1BhW[P\x90a\x1AA\x92a\x1A9\x92a\x1A\x12Za\x1A\x0Ba\x1A\x06a\x19\xF7a\x19\xF0\x8Ba\x18vV[`?\x90\x04\x90V[a\x1A\0\x8Ba\x18\xACV[\x90a*mV[a\x18\xBBV[\x11\x15a\x18\xCAV[Z\x9Fa\x1A3\x89\x15\x9C\x8D`\0\x14a\x1BaWa\x1A+Za\x19\rV[\x956\x91a\x04\xDAV[\x91a#\xB5V[\x9BZ\x90a*0V[\x90\x8B\x90\x8C\x15a\x1BWW[P\x80\x15a\x1BOW[a\x1A\\\x90a\x19\x1DV[`\0\x95\x15a\x1B;W[PPPPP\x85`\0\x14a\x1B\x04W`@\x80Q\x85\x81R` \x81\x01\x92\x90\x92R\x7FD.q_bcF\xE8\xC5C\x81\0-\xA6\x14\xF6+\xEE\x8D'8e5\xB2R\x1E\xC8T\x08\x98Un\x91\xA1[a\x1A\xADWPPV[\x80;\x15a\0\x1BW`@Qc\x12d\xE2m`\xE3\x1B\x81R`\x04\x81\x01\x92\x90\x92R\x83\x15\x15`$\x83\x01R`\0\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x1A\xFFWa\x1A\xECWPV[\x80a\x1A\xF9a\x02=\x92a\x04.V[\x80a\x06AV[a\x18jV[`@\x80Q\x85\x81R` \x81\x01\x92\x90\x92R\x7F#B\x8B\x18\xAC\xFB>\xA6K\x08\xDC\x0C\x1D)n\xA9\xC0\x97\x02\xC0\x90\x83\xCARr\xE6M\x11[h}#\x91\xA1a\x1A\xA5V[a\x1BE\x95Pa\x1D$V[8\x80\x80\x80\x80a\x1AeV[P\x85\x15a\x1ASV[\x90P\x15\x158a\x1AKV[\x88\x95a\x16\xEDV[\x8B\x92\x91\x92;\x15a\0\x1BW\x8B\x8F\x8B\x85\x85\x92\x88\x95\x8E\x8E\x8E\x8E\x8E`@Q\x9E\x8F\x9A\x8B\x9Ac:\xF8]\xA9`\xE1\x1B\x8CR3\x99`\x04\x8D\x01\x9Ba\x1B\xA1\x9Ca\x17\xB6V[\x03\x81Z`\0\x94\x85\x91\xF1\x92\x83\x15a\x1A\xFFWa\x1AA\x94a\x1A9\x94a\x1B\xC8W[P\x91\x92P\x92a\x19\xD0V[\x80a\x1A\xF9a\x1B\xD5\x92a\x04.V[8a\x1B\xBEV[\x15a\x1B\xE2WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x18\x99`\xD9\x1B`D\x82\x01R`d\x90\xFD[\x15a\x1C\x16WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS011`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x81\x16a\x1D\x1EWP2\x93[\x80\x82\x16a\x1C\xA6WP`\0\x80\x80\x80\x93a\x1C\x81a\x02=\x96:`\x01\x10\x84\x14a\x1C\x9FW`\x01\x90a*\x0BV[\x97\x88\x91\x83\x91\x83\x15a\x1C\x95W[\x16\x90\xF1a\x1C\x0FV[a\x08\xFC\x92Pa\x1C\x8DV[:\x90a*\x0BV[a\x1C\xB1` \x93a)\xEFV[\x94`\0\x80\x93`@Q\x90\x86\x82\x01\x93c\xA9\x05\x9C\xBB`\xE0\x1B\x85R\x16`$\x82\x01R\x87`D\x82\x01R`D\x81Ra\x1C\xE1\x81a\x04cV[Q\x92a'\x0F\x19Z\x01\xF1=\x80\x15a\x1D\x14W` \x14a\x1D\x03WPa\x02=`\0a\x1B\xDBV[a\x02=\x90`\0Q\x15\x90\x15\x17\x15a\x1B\xDBV[Pa\x02=\x90a\x1B\xDBV[\x93a\x1CZV[\x92\x94\x93\x90\x92`\x01`\x01`\xA0\x1B\x03\x92\x91\x83\x81\x16a\x1D\x8AWP2\x95[\x80\x84\x16a\x1DuWP`\0\x80\x93a\x1C\x81\x82\x94a\x1D]a\x02=\x98\x85\x96a*YV[\x90:\x81\x10\x85\x14a\x1DmW\x90a*\x0BV[P:\x90a*\x0BV[\x91a\x1D\x85` \x95a\x1C\xB1\x93a*YV[a*\x0BV[\x95a\x1D>V[\x91`\x04T\x91\x82\x15a\x1D\xA4Wa\x02=\x93a\x1F\x95V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS001`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x15a\x1D\xD8WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03#`\xDC\x1B`D\x82\x01R`d\x90\xFD[`\xFF`\x03\x19\x91\x16\x01\x90`\xFF\x82\x11a\x08\x0FWV[\x15a\x1E\x1FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS025`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x15a\x1ESWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS021`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x15a\x1E\x87WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x19\x19`\xD9\x1B`D\x82\x01R`d\x90\xFD[\x15a\x1E\xBBWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS023`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x90\x81` \x91\x03\x12a\0\x1BWQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\0\x1BW\x90V[\x90\x91a\x1F\x1Fa\x05,\x93`@\x84R`@\x84\x01\x90a\x06\xE8V[\x91` \x81\x84\x03\x91\x01Ra\x06\xE8V[\x15a\x1F4WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCC\x0C\x8D`\xDA\x1B`D\x82\x01R`d\x90\xFD[\x15a\x1FhWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x19\x1B`\xD9\x1B`D\x82\x01R`d\x90\xFD[\x91\x92\x90\x92a\x1F\xAE\x81Qa\x1F\xA7\x84a)\xC0V[\x11\x15a\x1D\xD1V[`\0\x90\x81[\x83\x83\x10a\x1F\xC2WPPPPPPV[\x85\x90a\x1F\xE4\x84\x84\x90`A\x02\x01` \x81\x01Q\x90`\xFF`A`@\x83\x01Q\x92\x01Q\x16\x92V[\x92\x91`\xFF\x81\x16\x80a!7WPP\x90`\x01`\x01`\xA0\x1B\x03a n\x92\x16\x92a \x14a \x0C\x89a)\xC0V[\x82\x10\x15a\x1ELV[a )a  \x82a*DV[\x87Q\x10\x15a\x1E\x80V[` \x81a Ra I\x83\x80\x95\x8B\x01\x01\x92a D\x84Q\x91a*DV[a*YV[\x89Q\x10\x15a\x1E\xB4V[`@Q\x80\x95\x81\x92c \xC1;\x0B`\xE0\x1B\x99\x8A\x84R`\x04\x84\x01a\x1F\x08V[\x03\x81\x87Z\xFA\x94\x85\x15a\x1A\xFFWa \xCB\x95a \xC5\x94a \xA0\x93`\0\x92a!\nW[PP`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x1F-V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x80\x85\x16\x91\x16\x81\x11\x91\x82a \xDDW[P\x81a \xD1W[Pa\x1FaV[\x92a\x17\x84V[\x91a\x1F\xB3V[`\x01\x91P\x14\x158a \xBFV[\x90\x91Pa!\0a\x02\xFC\x85`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x16\x15\x15\x908a \xB8V[a!)\x92P\x80=\x10a!0W[a!!\x81\x83a\x04\x9CV[\x81\x01\x90a\x1E\xE8V[8\x80a \x8EV[P=a!\x17V[\x91\x93\x90\x92\x94P`\x01\x91\x82\x81\x14`\0\x14a!\xA7WPPPP\x90a \xC5`\x01`\x01`\xA0\x1B\x03a \xCB\x93\x16\x91\x823\x14\x80\x15a!xW[a!s\x90a\x1E\x18V[a \xA0V[Pa!sa!\x9D\x89a\x0F\"\x86`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x08` R`@`\0 \x90V[T\x15\x15\x90Pa!jV[\x92\x93\x92`\x1E\x10\x15a\"YW`\0\x92\x93a\"=`@\x92\x83Q\x96a\"\x18\x8D` \x99a\"\x0F\x81a\"\x01\x8D\x82\x01\x94\x85`<\x91\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x82R`\x1C\x82\x01R\x01\x90V[\x03`\x1F\x19\x81\x01\x83R\x82a\x04\x9CV[Q\x90 \x94a\x1E\x05V[\x94Q\x94\x85\x94\x85\x90\x94\x93\x92`\xFF``\x93`\x80\x84\x01\x97\x84R\x16` \x83\x01R`@\x82\x01R\x01RV[\x84\x80R\x03\x91Z\xFA\x15a\x1A\xFFWa \xCB\x90a \xC5`\0Q\x91a \xA0V[\x91` \x93a\"=`\0\x94`@Q\x93\x84\x93\x8D\x85\x90\x94\x93\x92`\xFF``\x93`\x80\x84\x01\x97\x84R\x16` \x83\x01R`@\x82\x01R\x01RV[`@Q` \x81\x01\x90\x7FG\xE7\x954\xA2E\x95.\x8B\x16\x89:3k\x85\xA3\xD9\xEA\x9F\xA8\xC5s\xF3\xD8\x03\xAF\xB9*yF\x92\x18\x82RF`@\x82\x01R0``\x82\x01R``\x81Ra\"\xCE\x81a\x04cV[Q\x90 \x90V[\x94\x90\x98\x95\x91\x7F\xBB\x83\x10\xD4\x866\x8D\xB6\xBDo\x84\x94\x02\xFD\xD7:\xD5=1kZK&D\xADn\xFE\x0F\x94\x12\x86\xD8\x9A\x98\x94a#\x0Fa#E\x92`@\x9A\x966\x91a\x04\xDAV[` \x81Q\x91\x01 \x89Q\x9B` \x8D\x01\x9D\x8ER\x8C`\x01`\x01`\xA0\x1B\x03\x9B\x8C\x80\x9B\x16\x91\x01R``\x8D\x01R`\x80\x8C\x01R`\xA0\x8B\x01\x90a\x17\x93V[`\xC0\x89\x01R`\xE0\x88\x01Ra\x01\0\x87\x01R\x16a\x01 \x85\x01R\x16a\x01@\x83\x01Ra\x01`\x90\x81\x83\x01R\x81Ra#v\x81a\x04\x7FV[Q\x90 a\x05,a#\x84a\"\x8AV[`@Q`\x19`\xF8\x1B` \x82\x01R`\x01`\xF8\x1B`!\x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x92\x90\x92R\x81`b\x81\x01a\"\x01V[\x93\x90\x93`\x02\x84\x10\x15a\x17\xA0W`\0\x94\x85\x94`\x01\x03a#\xD9WP` \x83Q\x93\x01\x91\xF4\x90V[\x90` \x84Q\x94\x01\x92\xF1\x90V[\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5T\x80\x15a\0(W`\0\x80\x80\x926\x82\x8073``\x1B6R\x81\x80`\x146\x01\x92Z\xF1=\x82\x80>\x15a$2W=\x90\xF3[=\x90\xFD[`\x01`\0R`\x01` R`\x01`\x01`\xA0\x1B\x03\x80`@`\0 T\x16a$\xF0W`\x01`\0\x81\x90R` Ra$\xA3\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/[\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x90UV[\x81\x16a$\xADWPPV[`\0\x91\x82\x91Z\x90` \x83Q\x93\x01\x91\xF4\x15a$\xC3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03\x03`\xDC\x1B`D\x82\x01R`d\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x13\x03`\xDC\x1B`D\x82\x01R`d\x90\xFD[\x15a%$WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS101`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x15a%XWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x98\x19`\xD9\x1B`D\x82\x01R`d\x90\xFD[\x91\x92\x90\x92`\x013\x14\x15\x80a&(W[\x15a%\xFBWa%\xA4\x93Z\x93a#\xB5V[\x90\x81\x15a%\xD3W3\x7Fh\x95\xC16d\xAAOg(\x8B%\xD7\xA2\x1Dz\xAA4\x91n5_\xB9\xB6\xFA\xE0\xA19\xA9\x08[\xEC\xB8`\0\x80\xA2V[3\x7F\xAC\xD2\xC8p(\x04\x12\x8F\xDB\r\xB2\xBBI\xF6\xD1'\xDD\x01\x81\xC1?\xD4]\xBF\xE1m\xE0\x93\x0E+\xD3u`\0\x80\xA2V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCCL\r`\xDA\x1B`D\x82\x01R`d\x90\xFD[P3`\0R`\x01` R`\x01`\x01`\xA0\x1B\x03`@`\0 T\x16\x15\x15a%\x94V[\x90a&R\x82a\x15\xD8V[a&_`@Q\x91\x82a\x04\x9CV[\x82\x81R\x80\x92a&p`\x1F\x19\x91a\x15\xD8V[\x01\x90` 6\x91\x017V[\x80Q\x82\x10\x15a&\x8EW` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x15a&\xABWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3#\x03`\xDC\x1B`D\x82\x01R`d\x90\xFD[\x15a&\xDFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS201`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x15a'\x13WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x99\x18\x19`\xD9\x1B`D\x82\x01R`d\x90\xFD[\x15a'GWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS203`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x15a'{WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCC\x8C\r`\xDA\x1B`D\x82\x01R`d\x90\xFD[a'\xB4`\x04T\x15a&\xA4V[a'\xC1\x81Q\x83\x11\x15a&\xD8V[`\x01a'\xCF\x81\x84\x10\x15a'\x0CV[`\0\x81\x80[a(\x0EW[PPa\x02=\x92\x91a(\x03a$\x83a(\t\x93`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[Q`\x03UV[`\x04UV[\x90\x91\x83Q\x83\x10\x15a(\xC1Wa(\x98a\x03na(\x92\x84\x93a(>a(1\x88\x8Aa&zV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92\x83\x91a(l`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x80\x15\x15\x91\x82a(\xB6W[\x82a(\xABW[\x82a(\x9EW[PPa'@V[a\x13Pa\x03\x15a\x03\ta\x02\xFC\x86`\x01`\x01`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[\x93a\x17\x84V[\x91a'\xD4V[\x84\x16\x14\x15\x90P8\x80a(eV[0\x82\x14\x15\x92Pa(_V[\x81\x8B\x14\x15\x92Pa(YV[\x91a'\xD9V[\x15a(\xCEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x80\x15a\x08\x0FW`\0\x19\x01\x90V[` \x7Fa\x0F\x7F\xF2\xB3\x04\xAE\x89\x03\xC3\xDEt\xC6\x0Cj\xB1\xF7\xD6\"k?R\xC5\x16\x19\x05\xBBZ\xD4\x03\x9C\x93\x91a)4a)\x8AV[a)B`\x03T\x82\x11\x15a&\xD8V[a)O`\x01\x82\x10\x15a'\x0CV[\x80`\x04U`@Q\x90\x81R\xA1V[`@Q4\x81R\x7F=\x0C\xE9\xBF\xC3\xED}hb\xDB\xB2\x8B-\xEA\x94V\x1F\xE7\x14\xA1\xB4\xD0\x19\xAA\x8A\xF3\x970\xD1\xAD|=` 3\x92\xA2V[03\x03a)\x93WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS031`\xD8\x1B`D\x82\x01R`d\x90\xFD[\x80\x15a)\xE9W`A\x81\x02\x90\x80\x82\x04`A\x03a\x08\x0FWa)\xE1`A\x91\x83a\x18\x8CV[\x03a\0\x1BW\x90V[P`\0\x90V[\x80\x15a)\xE9W\x80\x80\x04`\x01\x03a\x08\x0FW`\x01a)\xE1\x82\x80a\x18\x8CV[\x90\x81\x15a*)W\x80\x82\x02\x91\x80\x83\x04\x82\x03a\x08\x0FWa)\xE1\x90\x83a\x18\x8CV[PP`\0\x90V[\x90\x81\x81\x11a\0\x1BW\x81\x03\x90\x81\x11a\x08\x0FW\x90V[` \x81\x01\x90\x81\x81\x11a\x08\x0FW\x81\x10a\0\x1BW\x90V[\x90\x81\x01\x90\x81\x81\x11a\x08\x0FW\x81\x10a\0\x1BW\x90V[\x90\x80\x82\x10a*yWP\x90V[\x90P\x90V\xFE\xA2dipfsX\"\x12 :\xC1\xB7<\x9A\x1C$\xBD\xD8D\xA9x\xE2Qtz'A\xC5LA\x87\x10\x96\xBF\xA9\x800\xAAuwJdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static GNOSISSAFE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GnosisSafe<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GnosisSafe<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GnosisSafe<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GnosisSafe<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GnosisSafe<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GnosisSafe)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GnosisSafe<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GNOSISSAFE_ABI.clone(),
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
                GNOSISSAFE_ABI.clone(),
                GNOSISSAFE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `VERSION` (0xffa1ad74) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addOwnerWithThreshold` (0x0d582f13) function
        pub fn add_owner_with_threshold(
            &self,
            owner: ::ethers::core::types::Address,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 88, 47, 19], (owner, threshold))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveHash` (0xd4d9bdcd) function
        pub fn approve_hash(
            &self,
            hash_to_approve: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 217, 189, 205], hash_to_approve)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approvedHashes` (0x7d832974) function
        pub fn approved_hashes(
            &self,
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([125, 131, 41, 116], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeThreshold` (0x694e80c3) function
        pub fn change_threshold(
            &self,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 78, 128, 195], threshold)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkNSignatures` (0x12fb68e0) function
        pub fn check_n_signatures(
            &self,
            data_hash: [u8; 32],
            data: ::ethers::core::types::Bytes,
            signatures: ::ethers::core::types::Bytes,
            required_signatures: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [18, 251, 104, 224],
                    (data_hash, data, signatures, required_signatures),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSignatures` (0x934f3a11) function
        pub fn check_signatures(
            &self,
            data_hash: [u8; 32],
            data: ::ethers::core::types::Bytes,
            signatures: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 79, 58, 17], (data_hash, data, signatures))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disableModule` (0xe009cfde) function
        pub fn disable_module(
            &self,
            prev_module: ::ethers::core::types::Address,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 9, 207, 222], (prev_module, module))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `domainSeparator` (0xf698da25) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([246, 152, 218, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableModule` (0x610b5925) function
        pub fn enable_module(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 11, 89, 37], module)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `encodeTransactionData` (0xe86637db) function
        pub fn encode_transaction_data(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ::ethers::core::types::U256,
            base_gas: ::ethers::core::types::U256,
            gas_price: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            refund_receiver: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [232, 102, 55, 219],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        nonce,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execTransaction` (0x6a761202) function
        pub fn exec_transaction(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ::ethers::core::types::U256,
            base_gas: ::ethers::core::types::U256,
            gas_price: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            refund_receiver: ::ethers::core::types::Address,
            signatures: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [106, 118, 18, 2],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        signatures,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execTransactionFromModule` (0x468721a7) function
        pub fn exec_transaction_from_module(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([70, 135, 33, 167], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execTransactionFromModuleReturnData` (0x5229073f) function
        pub fn exec_transaction_from_module_return_data(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([82, 41, 7, 63], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainId` (0x3408e470) function
        pub fn get_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getModulesPaginated` (0xcc2f8452) function
        pub fn get_modules_paginated(
            &self,
            start: ::ethers::core::types::Address,
            page_size: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([204, 47, 132, 82], (start, page_size))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOwners` (0xa0e67e2b) function
        pub fn get_owners(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([160, 230, 126, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStorageAt` (0x5624b25b) function
        pub fn get_storage_at(
            &self,
            offset: ::ethers::core::types::U256,
            length: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([86, 36, 178, 91], (offset, length))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getThreshold` (0xe75235b8) function
        pub fn get_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([231, 82, 53, 184], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTransactionHash` (0xd8d11f78) function
        pub fn get_transaction_hash(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ::ethers::core::types::U256,
            base_gas: ::ethers::core::types::U256,
            gas_price: ::ethers::core::types::U256,
            gas_token: ::ethers::core::types::Address,
            refund_receiver: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [216, 209, 31, 120],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        nonce,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isModuleEnabled` (0x2d9ad53d) function
        pub fn is_module_enabled(
            &self,
            module: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([45, 154, 213, 61], module)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOwner` (0x2f54bf6e) function
        pub fn is_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 84, 191, 110], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonce` (0xaffed0e0) function
        pub fn nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeOwner` (0xf8dc5dd9) function
        pub fn remove_owner(
            &self,
            prev_owner: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 220, 93, 217], (prev_owner, owner, threshold))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requiredTxGas` (0xc4ca3a9c) function
        pub fn required_tx_gas(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operation: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([196, 202, 58, 156], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFallbackHandler` (0xf08a0323) function
        pub fn set_fallback_handler(
            &self,
            handler: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 138, 3, 35], handler)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGuard` (0xe19a9dd9) function
        pub fn set_guard(
            &self,
            guard: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 154, 157, 217], guard)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setup` (0xb63e800d) function
        pub fn setup(
            &self,
            owners: ::std::vec::Vec<::ethers::core::types::Address>,
            threshold: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
            fallback_handler: ::ethers::core::types::Address,
            payment_token: ::ethers::core::types::Address,
            payment: ::ethers::core::types::U256,
            payment_receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [182, 62, 128, 13],
                    (
                        owners,
                        threshold,
                        to,
                        data,
                        fallback_handler,
                        payment_token,
                        payment,
                        payment_receiver,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedMessages` (0x5ae6bd37) function
        pub fn signed_messages(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([90, 230, 189, 55], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateAndRevert` (0xb4faba09) function
        pub fn simulate_and_revert(
            &self,
            target_contract: ::ethers::core::types::Address,
            calldata_payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 250, 186, 9], (target_contract, calldata_payload))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapOwner` (0xe318b52b) function
        pub fn swap_owner(
            &self,
            prev_owner: ::ethers::core::types::Address,
            old_owner: ::ethers::core::types::Address,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 24, 181, 43], (prev_owner, old_owner, new_owner))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddedOwner` event
        pub fn added_owner_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddedOwnerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApproveHash` event
        pub fn approve_hash_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApproveHashFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangedFallbackHandler` event
        pub fn changed_fallback_handler_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedFallbackHandlerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangedGuard` event
        pub fn changed_guard_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedGuardFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangedThreshold` event
        pub fn changed_threshold_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedThresholdFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DisabledModule` event
        pub fn disabled_module_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DisabledModuleFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EnabledModule` event
        pub fn enabled_module_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnabledModuleFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFailure` event
        pub fn execution_failure_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFailureFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFromModuleFailure` event
        pub fn execution_from_module_failure_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFromModuleFailureFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionFromModuleSuccess` event
        pub fn execution_from_module_success_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionFromModuleSuccessFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutionSuccess` event
        pub fn execution_success_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutionSuccessFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RemovedOwner` event
        pub fn removed_owner_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemovedOwnerFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SafeReceived` event
        pub fn safe_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SafeReceivedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SafeSetup` event
        pub fn safe_setup_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SafeSetupFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SignMsg` event
        pub fn sign_msg_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SignMsgFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GnosisSafeEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GnosisSafe<M> {
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
    #[ethevent(name = "AddedOwner", abi = "AddedOwner(address)")]
    pub struct AddedOwnerFilter {
        pub owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "ApproveHash", abi = "ApproveHash(bytes32,address)")]
    pub struct ApproveHashFilter {
        #[ethevent(indexed)]
        pub approved_hash: [u8; 32],
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "ChangedFallbackHandler", abi = "ChangedFallbackHandler(address)")]
    pub struct ChangedFallbackHandlerFilter {
        pub handler: ::ethers::core::types::Address,
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
    #[ethevent(name = "ChangedGuard", abi = "ChangedGuard(address)")]
    pub struct ChangedGuardFilter {
        pub guard: ::ethers::core::types::Address,
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
    #[ethevent(name = "ChangedThreshold", abi = "ChangedThreshold(uint256)")]
    pub struct ChangedThresholdFilter {
        pub threshold: ::ethers::core::types::U256,
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
    #[ethevent(name = "DisabledModule", abi = "DisabledModule(address)")]
    pub struct DisabledModuleFilter {
        pub module: ::ethers::core::types::Address,
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
    #[ethevent(name = "EnabledModule", abi = "EnabledModule(address)")]
    pub struct EnabledModuleFilter {
        pub module: ::ethers::core::types::Address,
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
    #[ethevent(name = "ExecutionFailure", abi = "ExecutionFailure(bytes32,uint256)")]
    pub struct ExecutionFailureFilter {
        pub tx_hash: [u8; 32],
        pub payment: ::ethers::core::types::U256,
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
        name = "ExecutionFromModuleFailure",
        abi = "ExecutionFromModuleFailure(address)"
    )]
    pub struct ExecutionFromModuleFailureFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
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
        name = "ExecutionFromModuleSuccess",
        abi = "ExecutionFromModuleSuccess(address)"
    )]
    pub struct ExecutionFromModuleSuccessFilter {
        #[ethevent(indexed)]
        pub module: ::ethers::core::types::Address,
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
    #[ethevent(name = "ExecutionSuccess", abi = "ExecutionSuccess(bytes32,uint256)")]
    pub struct ExecutionSuccessFilter {
        pub tx_hash: [u8; 32],
        pub payment: ::ethers::core::types::U256,
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
    #[ethevent(name = "RemovedOwner", abi = "RemovedOwner(address)")]
    pub struct RemovedOwnerFilter {
        pub owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "SafeReceived", abi = "SafeReceived(address,uint256)")]
    pub struct SafeReceivedFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
        name = "SafeSetup",
        abi = "SafeSetup(address,address[],uint256,address,address)"
    )]
    pub struct SafeSetupFilter {
        #[ethevent(indexed)]
        pub initiator: ::ethers::core::types::Address,
        pub owners: ::std::vec::Vec<::ethers::core::types::Address>,
        pub threshold: ::ethers::core::types::U256,
        pub initializer: ::ethers::core::types::Address,
        pub fallback_handler: ::ethers::core::types::Address,
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
    #[ethevent(name = "SignMsg", abi = "SignMsg(bytes32)")]
    pub struct SignMsgFilter {
        #[ethevent(indexed)]
        pub msg_hash: [u8; 32],
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GnosisSafeEvents {
        AddedOwnerFilter(AddedOwnerFilter),
        ApproveHashFilter(ApproveHashFilter),
        ChangedFallbackHandlerFilter(ChangedFallbackHandlerFilter),
        ChangedGuardFilter(ChangedGuardFilter),
        ChangedThresholdFilter(ChangedThresholdFilter),
        DisabledModuleFilter(DisabledModuleFilter),
        EnabledModuleFilter(EnabledModuleFilter),
        ExecutionFailureFilter(ExecutionFailureFilter),
        ExecutionFromModuleFailureFilter(ExecutionFromModuleFailureFilter),
        ExecutionFromModuleSuccessFilter(ExecutionFromModuleSuccessFilter),
        ExecutionSuccessFilter(ExecutionSuccessFilter),
        RemovedOwnerFilter(RemovedOwnerFilter),
        SafeReceivedFilter(SafeReceivedFilter),
        SafeSetupFilter(SafeSetupFilter),
        SignMsgFilter(SignMsgFilter),
    }
    impl ::ethers::contract::EthLogDecode for GnosisSafeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddedOwnerFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::AddedOwnerFilter(decoded));
            }
            if let Ok(decoded) = ApproveHashFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ApproveHashFilter(decoded));
            }
            if let Ok(decoded) = ChangedFallbackHandlerFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ChangedFallbackHandlerFilter(decoded));
            }
            if let Ok(decoded) = ChangedGuardFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ChangedGuardFilter(decoded));
            }
            if let Ok(decoded) = ChangedThresholdFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ChangedThresholdFilter(decoded));
            }
            if let Ok(decoded) = DisabledModuleFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::DisabledModuleFilter(decoded));
            }
            if let Ok(decoded) = EnabledModuleFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::EnabledModuleFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFailureFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ExecutionFailureFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFromModuleFailureFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ExecutionFromModuleFailureFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFromModuleSuccessFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ExecutionFromModuleSuccessFilter(decoded));
            }
            if let Ok(decoded) = ExecutionSuccessFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ExecutionSuccessFilter(decoded));
            }
            if let Ok(decoded) = RemovedOwnerFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::RemovedOwnerFilter(decoded));
            }
            if let Ok(decoded) = SafeReceivedFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::SafeReceivedFilter(decoded));
            }
            if let Ok(decoded) = SafeSetupFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::SafeSetupFilter(decoded));
            }
            if let Ok(decoded) = SignMsgFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::SignMsgFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GnosisSafeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddedOwnerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveHashFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangedFallbackHandlerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangedGuardFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangedThresholdFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DisabledModuleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnabledModuleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFailureFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFromModuleFailureFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionFromModuleSuccessFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutionSuccessFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemovedOwnerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeReceivedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeSetupFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignMsgFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddedOwnerFilter> for GnosisSafeEvents {
        fn from(value: AddedOwnerFilter) -> Self {
            Self::AddedOwnerFilter(value)
        }
    }
    impl ::core::convert::From<ApproveHashFilter> for GnosisSafeEvents {
        fn from(value: ApproveHashFilter) -> Self {
            Self::ApproveHashFilter(value)
        }
    }
    impl ::core::convert::From<ChangedFallbackHandlerFilter> for GnosisSafeEvents {
        fn from(value: ChangedFallbackHandlerFilter) -> Self {
            Self::ChangedFallbackHandlerFilter(value)
        }
    }
    impl ::core::convert::From<ChangedGuardFilter> for GnosisSafeEvents {
        fn from(value: ChangedGuardFilter) -> Self {
            Self::ChangedGuardFilter(value)
        }
    }
    impl ::core::convert::From<ChangedThresholdFilter> for GnosisSafeEvents {
        fn from(value: ChangedThresholdFilter) -> Self {
            Self::ChangedThresholdFilter(value)
        }
    }
    impl ::core::convert::From<DisabledModuleFilter> for GnosisSafeEvents {
        fn from(value: DisabledModuleFilter) -> Self {
            Self::DisabledModuleFilter(value)
        }
    }
    impl ::core::convert::From<EnabledModuleFilter> for GnosisSafeEvents {
        fn from(value: EnabledModuleFilter) -> Self {
            Self::EnabledModuleFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFailureFilter> for GnosisSafeEvents {
        fn from(value: ExecutionFailureFilter) -> Self {
            Self::ExecutionFailureFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFromModuleFailureFilter> for GnosisSafeEvents {
        fn from(value: ExecutionFromModuleFailureFilter) -> Self {
            Self::ExecutionFromModuleFailureFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionFromModuleSuccessFilter> for GnosisSafeEvents {
        fn from(value: ExecutionFromModuleSuccessFilter) -> Self {
            Self::ExecutionFromModuleSuccessFilter(value)
        }
    }
    impl ::core::convert::From<ExecutionSuccessFilter> for GnosisSafeEvents {
        fn from(value: ExecutionSuccessFilter) -> Self {
            Self::ExecutionSuccessFilter(value)
        }
    }
    impl ::core::convert::From<RemovedOwnerFilter> for GnosisSafeEvents {
        fn from(value: RemovedOwnerFilter) -> Self {
            Self::RemovedOwnerFilter(value)
        }
    }
    impl ::core::convert::From<SafeReceivedFilter> for GnosisSafeEvents {
        fn from(value: SafeReceivedFilter) -> Self {
            Self::SafeReceivedFilter(value)
        }
    }
    impl ::core::convert::From<SafeSetupFilter> for GnosisSafeEvents {
        fn from(value: SafeSetupFilter) -> Self {
            Self::SafeSetupFilter(value)
        }
    }
    impl ::core::convert::From<SignMsgFilter> for GnosisSafeEvents {
        fn from(value: SignMsgFilter) -> Self {
            Self::SignMsgFilter(value)
        }
    }
    ///Container type for all input parameters for the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
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
    #[ethcall(name = "VERSION", abi = "VERSION()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `addOwnerWithThreshold` function with signature `addOwnerWithThreshold(address,uint256)` and selector `0x0d582f13`
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
        name = "addOwnerWithThreshold",
        abi = "addOwnerWithThreshold(address,uint256)"
    )]
    pub struct AddOwnerWithThresholdCall {
        pub owner: ::ethers::core::types::Address,
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `approveHash` function with signature `approveHash(bytes32)` and selector `0xd4d9bdcd`
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
    #[ethcall(name = "approveHash", abi = "approveHash(bytes32)")]
    pub struct ApproveHashCall {
        pub hash_to_approve: [u8; 32],
    }
    ///Container type for all input parameters for the `approvedHashes` function with signature `approvedHashes(address,bytes32)` and selector `0x7d832974`
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
    #[ethcall(name = "approvedHashes", abi = "approvedHashes(address,bytes32)")]
    pub struct ApprovedHashesCall(pub ::ethers::core::types::Address, pub [u8; 32]);
    ///Container type for all input parameters for the `changeThreshold` function with signature `changeThreshold(uint256)` and selector `0x694e80c3`
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
    #[ethcall(name = "changeThreshold", abi = "changeThreshold(uint256)")]
    pub struct ChangeThresholdCall {
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkNSignatures` function with signature `checkNSignatures(bytes32,bytes,bytes,uint256)` and selector `0x12fb68e0`
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
        name = "checkNSignatures",
        abi = "checkNSignatures(bytes32,bytes,bytes,uint256)"
    )]
    pub struct CheckNSignaturesCall {
        pub data_hash: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
        pub signatures: ::ethers::core::types::Bytes,
        pub required_signatures: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkSignatures` function with signature `checkSignatures(bytes32,bytes,bytes)` and selector `0x934f3a11`
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
    #[ethcall(name = "checkSignatures", abi = "checkSignatures(bytes32,bytes,bytes)")]
    pub struct CheckSignaturesCall {
        pub data_hash: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
        pub signatures: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `disableModule` function with signature `disableModule(address,address)` and selector `0xe009cfde`
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
    #[ethcall(name = "disableModule", abi = "disableModule(address,address)")]
    pub struct DisableModuleCall {
        pub prev_module: ::ethers::core::types::Address,
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
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
    #[ethcall(name = "domainSeparator", abi = "domainSeparator()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `enableModule` function with signature `enableModule(address)` and selector `0x610b5925`
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
    #[ethcall(name = "enableModule", abi = "enableModule(address)")]
    pub struct EnableModuleCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `encodeTransactionData` function with signature `encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xe86637db`
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
        name = "encodeTransactionData",
        abi = "encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)"
    )]
    pub struct EncodeTransactionDataCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ::ethers::core::types::U256,
        pub base_gas: ::ethers::core::types::U256,
        pub gas_price: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub refund_receiver: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `execTransaction` function with signature `execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)` and selector `0x6a761202`
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
        name = "execTransaction",
        abi = "execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)"
    )]
    pub struct ExecTransactionCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ::ethers::core::types::U256,
        pub base_gas: ::ethers::core::types::U256,
        pub gas_price: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub refund_receiver: ::ethers::core::types::Address,
        pub signatures: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `execTransactionFromModule` function with signature `execTransactionFromModule(address,uint256,bytes,uint8)` and selector `0x468721a7`
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
        name = "execTransactionFromModule",
        abi = "execTransactionFromModule(address,uint256,bytes,uint8)"
    )]
    pub struct ExecTransactionFromModuleCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the `execTransactionFromModuleReturnData` function with signature `execTransactionFromModuleReturnData(address,uint256,bytes,uint8)` and selector `0x5229073f`
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
        name = "execTransactionFromModuleReturnData",
        abi = "execTransactionFromModuleReturnData(address,uint256,bytes,uint8)"
    )]
    pub struct ExecTransactionFromModuleReturnDataCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
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
    ///Container type for all input parameters for the `getModulesPaginated` function with signature `getModulesPaginated(address,uint256)` and selector `0xcc2f8452`
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
        name = "getModulesPaginated",
        abi = "getModulesPaginated(address,uint256)"
    )]
    pub struct GetModulesPaginatedCall {
        pub start: ::ethers::core::types::Address,
        pub page_size: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getOwners` function with signature `getOwners()` and selector `0xa0e67e2b`
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
    #[ethcall(name = "getOwners", abi = "getOwners()")]
    pub struct GetOwnersCall;
    ///Container type for all input parameters for the `getStorageAt` function with signature `getStorageAt(uint256,uint256)` and selector `0x5624b25b`
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
    #[ethcall(name = "getStorageAt", abi = "getStorageAt(uint256,uint256)")]
    pub struct GetStorageAtCall {
        pub offset: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getThreshold` function with signature `getThreshold()` and selector `0xe75235b8`
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
    #[ethcall(name = "getThreshold", abi = "getThreshold()")]
    pub struct GetThresholdCall;
    ///Container type for all input parameters for the `getTransactionHash` function with signature `getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xd8d11f78`
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
        name = "getTransactionHash",
        abi = "getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)"
    )]
    pub struct GetTransactionHashCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ::ethers::core::types::U256,
        pub base_gas: ::ethers::core::types::U256,
        pub gas_price: ::ethers::core::types::U256,
        pub gas_token: ::ethers::core::types::Address,
        pub refund_receiver: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isModuleEnabled` function with signature `isModuleEnabled(address)` and selector `0x2d9ad53d`
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
    #[ethcall(name = "isModuleEnabled", abi = "isModuleEnabled(address)")]
    pub struct IsModuleEnabledCall {
        pub module: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isOwner` function with signature `isOwner(address)` and selector `0x2f54bf6e`
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
    #[ethcall(name = "isOwner", abi = "isOwner(address)")]
    pub struct IsOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
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
    #[ethcall(name = "nonce", abi = "nonce()")]
    pub struct NonceCall;
    ///Container type for all input parameters for the `removeOwner` function with signature `removeOwner(address,address,uint256)` and selector `0xf8dc5dd9`
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
    #[ethcall(name = "removeOwner", abi = "removeOwner(address,address,uint256)")]
    pub struct RemoveOwnerCall {
        pub prev_owner: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `requiredTxGas` function with signature `requiredTxGas(address,uint256,bytes,uint8)` and selector `0xc4ca3a9c`
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
        name = "requiredTxGas",
        abi = "requiredTxGas(address,uint256,bytes,uint8)"
    )]
    pub struct RequiredTxGasCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operation: u8,
    }
    ///Container type for all input parameters for the `setFallbackHandler` function with signature `setFallbackHandler(address)` and selector `0xf08a0323`
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
    #[ethcall(name = "setFallbackHandler", abi = "setFallbackHandler(address)")]
    pub struct SetFallbackHandlerCall {
        pub handler: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setGuard` function with signature `setGuard(address)` and selector `0xe19a9dd9`
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
    #[ethcall(name = "setGuard", abi = "setGuard(address)")]
    pub struct SetGuardCall {
        pub guard: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setup` function with signature `setup(address[],uint256,address,bytes,address,address,uint256,address)` and selector `0xb63e800d`
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
        name = "setup",
        abi = "setup(address[],uint256,address,bytes,address,address,uint256,address)"
    )]
    pub struct SetupCall {
        pub owners: ::std::vec::Vec<::ethers::core::types::Address>,
        pub threshold: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub fallback_handler: ::ethers::core::types::Address,
        pub payment_token: ::ethers::core::types::Address,
        pub payment: ::ethers::core::types::U256,
        pub payment_receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `signedMessages` function with signature `signedMessages(bytes32)` and selector `0x5ae6bd37`
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
    #[ethcall(name = "signedMessages", abi = "signedMessages(bytes32)")]
    pub struct SignedMessagesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `simulateAndRevert` function with signature `simulateAndRevert(address,bytes)` and selector `0xb4faba09`
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
    #[ethcall(name = "simulateAndRevert", abi = "simulateAndRevert(address,bytes)")]
    pub struct SimulateAndRevertCall {
        pub target_contract: ::ethers::core::types::Address,
        pub calldata_payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `swapOwner` function with signature `swapOwner(address,address,address)` and selector `0xe318b52b`
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
    #[ethcall(name = "swapOwner", abi = "swapOwner(address,address,address)")]
    pub struct SwapOwnerCall {
        pub prev_owner: ::ethers::core::types::Address,
        pub old_owner: ::ethers::core::types::Address,
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GnosisSafeCalls {
        Version(VersionCall),
        AddOwnerWithThreshold(AddOwnerWithThresholdCall),
        ApproveHash(ApproveHashCall),
        ApprovedHashes(ApprovedHashesCall),
        ChangeThreshold(ChangeThresholdCall),
        CheckNSignatures(CheckNSignaturesCall),
        CheckSignatures(CheckSignaturesCall),
        DisableModule(DisableModuleCall),
        DomainSeparator(DomainSeparatorCall),
        EnableModule(EnableModuleCall),
        EncodeTransactionData(EncodeTransactionDataCall),
        ExecTransaction(ExecTransactionCall),
        ExecTransactionFromModule(ExecTransactionFromModuleCall),
        ExecTransactionFromModuleReturnData(ExecTransactionFromModuleReturnDataCall),
        GetChainId(GetChainIdCall),
        GetModulesPaginated(GetModulesPaginatedCall),
        GetOwners(GetOwnersCall),
        GetStorageAt(GetStorageAtCall),
        GetThreshold(GetThresholdCall),
        GetTransactionHash(GetTransactionHashCall),
        IsModuleEnabled(IsModuleEnabledCall),
        IsOwner(IsOwnerCall),
        Nonce(NonceCall),
        RemoveOwner(RemoveOwnerCall),
        RequiredTxGas(RequiredTxGasCall),
        SetFallbackHandler(SetFallbackHandlerCall),
        SetGuard(SetGuardCall),
        Setup(SetupCall),
        SignedMessages(SignedMessagesCall),
        SimulateAndRevert(SimulateAndRevertCall),
        SwapOwner(SwapOwnerCall),
    }
    impl ::ethers::core::abi::AbiDecode for GnosisSafeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded)
                = <AddOwnerWithThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddOwnerWithThreshold(decoded));
            }
            if let Ok(decoded)
                = <ApproveHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApproveHash(decoded));
            }
            if let Ok(decoded)
                = <ApprovedHashesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApprovedHashes(decoded));
            }
            if let Ok(decoded)
                = <ChangeThresholdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChangeThreshold(decoded));
            }
            if let Ok(decoded)
                = <CheckNSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CheckNSignatures(decoded));
            }
            if let Ok(decoded)
                = <CheckSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckSignatures(decoded));
            }
            if let Ok(decoded)
                = <DisableModuleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisableModule(decoded));
            }
            if let Ok(decoded)
                = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded)
                = <EnableModuleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EnableModule(decoded));
            }
            if let Ok(decoded)
                = <EncodeTransactionDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EncodeTransactionData(decoded));
            }
            if let Ok(decoded)
                = <ExecTransactionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecTransaction(decoded));
            }
            if let Ok(decoded)
                = <ExecTransactionFromModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecTransactionFromModule(decoded));
            }
            if let Ok(decoded)
                = <ExecTransactionFromModuleReturnDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecTransactionFromModuleReturnData(decoded));
            }
            if let Ok(decoded)
                = <GetChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetChainId(decoded));
            }
            if let Ok(decoded)
                = <GetModulesPaginatedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetModulesPaginated(decoded));
            }
            if let Ok(decoded)
                = <GetOwnersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetOwners(decoded));
            }
            if let Ok(decoded)
                = <GetStorageAtCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetStorageAt(decoded));
            }
            if let Ok(decoded)
                = <GetThresholdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetThreshold(decoded));
            }
            if let Ok(decoded)
                = <GetTransactionHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetTransactionHash(decoded));
            }
            if let Ok(decoded)
                = <IsModuleEnabledCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsModuleEnabled(decoded));
            }
            if let Ok(decoded)
                = <IsOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsOwner(decoded));
            }
            if let Ok(decoded)
                = <NonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonce(decoded));
            }
            if let Ok(decoded)
                = <RemoveOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveOwner(decoded));
            }
            if let Ok(decoded)
                = <RequiredTxGasCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RequiredTxGas(decoded));
            }
            if let Ok(decoded)
                = <SetFallbackHandlerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetFallbackHandler(decoded));
            }
            if let Ok(decoded)
                = <SetGuardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetGuard(decoded));
            }
            if let Ok(decoded)
                = <SetupCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Setup(decoded));
            }
            if let Ok(decoded)
                = <SignedMessagesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SignedMessages(decoded));
            }
            if let Ok(decoded)
                = <SimulateAndRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SimulateAndRevert(decoded));
            }
            if let Ok(decoded)
                = <SwapOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SwapOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GnosisSafeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddOwnerWithThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproveHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApprovedHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckNSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DisableModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnableModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EncodeTransactionData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransactionFromModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecTransactionFromModuleReturnData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetModulesPaginated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOwners(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStorageAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTransactionHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsModuleEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequiredTxGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFallbackHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGuard(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Setup(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignedMessages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateAndRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for GnosisSafeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOwnerWithThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApproveHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovedHashes(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckNSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisableModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::EncodeTransactionData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecTransaction(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecTransactionFromModule(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecTransactionFromModuleReturnData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetModulesPaginated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOwners(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStorageAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTransactionHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsModuleEnabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequiredTxGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFallbackHandler(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetGuard(element) => ::core::fmt::Display::fmt(element, f),
                Self::Setup(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedMessages(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateAndRevert(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapOwner(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<VersionCall> for GnosisSafeCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<AddOwnerWithThresholdCall> for GnosisSafeCalls {
        fn from(value: AddOwnerWithThresholdCall) -> Self {
            Self::AddOwnerWithThreshold(value)
        }
    }
    impl ::core::convert::From<ApproveHashCall> for GnosisSafeCalls {
        fn from(value: ApproveHashCall) -> Self {
            Self::ApproveHash(value)
        }
    }
    impl ::core::convert::From<ApprovedHashesCall> for GnosisSafeCalls {
        fn from(value: ApprovedHashesCall) -> Self {
            Self::ApprovedHashes(value)
        }
    }
    impl ::core::convert::From<ChangeThresholdCall> for GnosisSafeCalls {
        fn from(value: ChangeThresholdCall) -> Self {
            Self::ChangeThreshold(value)
        }
    }
    impl ::core::convert::From<CheckNSignaturesCall> for GnosisSafeCalls {
        fn from(value: CheckNSignaturesCall) -> Self {
            Self::CheckNSignatures(value)
        }
    }
    impl ::core::convert::From<CheckSignaturesCall> for GnosisSafeCalls {
        fn from(value: CheckSignaturesCall) -> Self {
            Self::CheckSignatures(value)
        }
    }
    impl ::core::convert::From<DisableModuleCall> for GnosisSafeCalls {
        fn from(value: DisableModuleCall) -> Self {
            Self::DisableModule(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for GnosisSafeCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<EnableModuleCall> for GnosisSafeCalls {
        fn from(value: EnableModuleCall) -> Self {
            Self::EnableModule(value)
        }
    }
    impl ::core::convert::From<EncodeTransactionDataCall> for GnosisSafeCalls {
        fn from(value: EncodeTransactionDataCall) -> Self {
            Self::EncodeTransactionData(value)
        }
    }
    impl ::core::convert::From<ExecTransactionCall> for GnosisSafeCalls {
        fn from(value: ExecTransactionCall) -> Self {
            Self::ExecTransaction(value)
        }
    }
    impl ::core::convert::From<ExecTransactionFromModuleCall> for GnosisSafeCalls {
        fn from(value: ExecTransactionFromModuleCall) -> Self {
            Self::ExecTransactionFromModule(value)
        }
    }
    impl ::core::convert::From<ExecTransactionFromModuleReturnDataCall>
    for GnosisSafeCalls {
        fn from(value: ExecTransactionFromModuleReturnDataCall) -> Self {
            Self::ExecTransactionFromModuleReturnData(value)
        }
    }
    impl ::core::convert::From<GetChainIdCall> for GnosisSafeCalls {
        fn from(value: GetChainIdCall) -> Self {
            Self::GetChainId(value)
        }
    }
    impl ::core::convert::From<GetModulesPaginatedCall> for GnosisSafeCalls {
        fn from(value: GetModulesPaginatedCall) -> Self {
            Self::GetModulesPaginated(value)
        }
    }
    impl ::core::convert::From<GetOwnersCall> for GnosisSafeCalls {
        fn from(value: GetOwnersCall) -> Self {
            Self::GetOwners(value)
        }
    }
    impl ::core::convert::From<GetStorageAtCall> for GnosisSafeCalls {
        fn from(value: GetStorageAtCall) -> Self {
            Self::GetStorageAt(value)
        }
    }
    impl ::core::convert::From<GetThresholdCall> for GnosisSafeCalls {
        fn from(value: GetThresholdCall) -> Self {
            Self::GetThreshold(value)
        }
    }
    impl ::core::convert::From<GetTransactionHashCall> for GnosisSafeCalls {
        fn from(value: GetTransactionHashCall) -> Self {
            Self::GetTransactionHash(value)
        }
    }
    impl ::core::convert::From<IsModuleEnabledCall> for GnosisSafeCalls {
        fn from(value: IsModuleEnabledCall) -> Self {
            Self::IsModuleEnabled(value)
        }
    }
    impl ::core::convert::From<IsOwnerCall> for GnosisSafeCalls {
        fn from(value: IsOwnerCall) -> Self {
            Self::IsOwner(value)
        }
    }
    impl ::core::convert::From<NonceCall> for GnosisSafeCalls {
        fn from(value: NonceCall) -> Self {
            Self::Nonce(value)
        }
    }
    impl ::core::convert::From<RemoveOwnerCall> for GnosisSafeCalls {
        fn from(value: RemoveOwnerCall) -> Self {
            Self::RemoveOwner(value)
        }
    }
    impl ::core::convert::From<RequiredTxGasCall> for GnosisSafeCalls {
        fn from(value: RequiredTxGasCall) -> Self {
            Self::RequiredTxGas(value)
        }
    }
    impl ::core::convert::From<SetFallbackHandlerCall> for GnosisSafeCalls {
        fn from(value: SetFallbackHandlerCall) -> Self {
            Self::SetFallbackHandler(value)
        }
    }
    impl ::core::convert::From<SetGuardCall> for GnosisSafeCalls {
        fn from(value: SetGuardCall) -> Self {
            Self::SetGuard(value)
        }
    }
    impl ::core::convert::From<SetupCall> for GnosisSafeCalls {
        fn from(value: SetupCall) -> Self {
            Self::Setup(value)
        }
    }
    impl ::core::convert::From<SignedMessagesCall> for GnosisSafeCalls {
        fn from(value: SignedMessagesCall) -> Self {
            Self::SignedMessages(value)
        }
    }
    impl ::core::convert::From<SimulateAndRevertCall> for GnosisSafeCalls {
        fn from(value: SimulateAndRevertCall) -> Self {
            Self::SimulateAndRevert(value)
        }
    }
    impl ::core::convert::From<SwapOwnerCall> for GnosisSafeCalls {
        fn from(value: SwapOwnerCall) -> Self {
            Self::SwapOwner(value)
        }
    }
    ///Container type for all return fields from the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
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
    pub struct VersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `approvedHashes` function with signature `approvedHashes(address,bytes32)` and selector `0x7d832974`
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
    pub struct ApprovedHashesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
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
    ///Container type for all return fields from the `encodeTransactionData` function with signature `encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xe86637db`
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
    pub struct EncodeTransactionDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `execTransaction` function with signature `execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)` and selector `0x6a761202`
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
    pub struct ExecTransactionReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `execTransactionFromModule` function with signature `execTransactionFromModule(address,uint256,bytes,uint8)` and selector `0x468721a7`
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
    pub struct ExecTransactionFromModuleReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `execTransactionFromModuleReturnData` function with signature `execTransactionFromModuleReturnData(address,uint256,bytes,uint8)` and selector `0x5229073f`
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
    pub struct ExecTransactionFromModuleReturnDataReturn {
        pub success: bool,
        pub return_data: ::ethers::core::types::Bytes,
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
    pub struct GetChainIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getModulesPaginated` function with signature `getModulesPaginated(address,uint256)` and selector `0xcc2f8452`
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
    pub struct GetModulesPaginatedReturn {
        pub array: ::std::vec::Vec<::ethers::core::types::Address>,
        pub next: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getOwners` function with signature `getOwners()` and selector `0xa0e67e2b`
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
    pub struct GetOwnersReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getStorageAt` function with signature `getStorageAt(uint256,uint256)` and selector `0x5624b25b`
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
    pub struct GetStorageAtReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getThreshold` function with signature `getThreshold()` and selector `0xe75235b8`
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
    pub struct GetThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTransactionHash` function with signature `getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xd8d11f78`
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
    pub struct GetTransactionHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `isModuleEnabled` function with signature `isModuleEnabled(address)` and selector `0x2d9ad53d`
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
    pub struct IsModuleEnabledReturn(pub bool);
    ///Container type for all return fields from the `isOwner` function with signature `isOwner(address)` and selector `0x2f54bf6e`
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
    pub struct IsOwnerReturn(pub bool);
    ///Container type for all return fields from the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
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
    pub struct NonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `requiredTxGas` function with signature `requiredTxGas(address,uint256,bytes,uint8)` and selector `0xc4ca3a9c`
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
    pub struct RequiredTxGasReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `signedMessages` function with signature `signedMessages(bytes32)` and selector `0x5ae6bd37`
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
    pub struct SignedMessagesReturn(pub ::ethers::core::types::U256);
}
