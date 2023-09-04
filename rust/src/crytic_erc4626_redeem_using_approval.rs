pub use crytic_erc4626_redeem_using_approval::*;
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
pub mod crytic_erc4626_redeem_using_approval {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverId"),
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
                    ::std::borrow::ToOwned::to_owned("depositForSelfSimple"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "depositForSelfSimple",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
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
                                    name: ::std::borrow::ToOwned::to_owned("receiverId"),
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
                    ::std::borrow::ToOwned::to_owned("mintAsset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintAsset"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverId"),
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
                    ::std::borrow::ToOwned::to_owned("recognizeLossProxy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recognizeLossProxy"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("loss"),
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
                    ::std::borrow::ToOwned::to_owned("recognizeProfitProxy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "recognizeProfitProxy",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("profit"),
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
                    ::std::borrow::ToOwned::to_owned("redeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeem"),
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
                                    name: ::std::borrow::ToOwned::to_owned("ownerId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverId"),
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
                    ::std::borrow::ToOwned::to_owned("redeemForSelfSimple"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "redeemForSelfSimple",
                            ),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "verify_redeemRequiresTokenApproval",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_redeemRequiresTokenApproval",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("sharesApproved"),
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
                    ::std::borrow::ToOwned::to_owned("verify_redeemViaApprovalProxy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_redeemViaApprovalProxy",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "verify_withdrawRequiresTokenApproval",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_withdrawRequiresTokenApproval",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("sharesApproved"),
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
                    ::std::borrow::ToOwned::to_owned("verify_withdrawViaApprovalProxy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_withdrawViaApprovalProxy",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
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
                                    name: ::std::borrow::ToOwned::to_owned("sharesBurned"),
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ownerId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverId"),
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
            ]),
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
    pub static CRYTICERC4626REDEEMUSINGAPPROVAL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa \x13\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x1B.\xF1\xCA\x14a\x14\x13WP\x80c'#\xF9\xEE\x14a\x12\xE2W\x80cH\x0F\xEFj\x14a\x0F\xE8W\x80co\x89\xDD.\x14a\rOW\x83\x81c\x96\xEB'\xA1\x14a\x0C\x90WP\x80c\xA4\x1F\xE4\x9F\x14a\x0C5W\x80c\xA8\x15\xC1\x0F\x14a\x0B!W\x80c\xAA\xEBB\x03\x14a\x07\xE0W\x80c\xB8\x19\"\x05\x14a\x07tW\x83\x81c\xB8\x8D\xAB2\x14a\x06\xFAWP\x80c\xC2\x1E\xDAo\x14a\x02JW\x83\x81c\xCC\xC9J\xE9\x14a\x01eWPc\xE2\xBB\xB1X\x14a\0\xB9W`\0\x80\xFD[4a\x01aW` a\0\xD8a\x01\x17\x93a\0\xD06a\x14\xA2V[\x92\x90\x92a\x18wV[\x91\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93cnU?e`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x90\x81\x15a\x01XWPa\x01+WP\x80\xF3[` \x90\x81=\x81\x11a\x01QW[a\x01A\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x80\xF3[`\0\x80\xFD[P=a\x017V[Q=\x84\x82>=\x90\xFD[\x82\x80\xFD[\x80\x84\x844a\x02FW` 6`\x03\x19\x01\x12a\x02FW`\xFF\x82T`\xA0\x1C\x16\x15a\x02FW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x02<W\x85\x91a\x02\x08W[P\x15a\x02\x03W\x82;\x15a\x02\x03W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cU\xDFp\r`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01XWPa\x01\xF0WP\xF3[a\x01\xF9\x90a\x14\xD2V[a\x02\0W\x80\xF3[\x80\xFD[PPP\xFD[\x94PP` \x84=\x82\x11a\x024W[\x81a\x02#` \x93\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x84\x93Q\x86a\x01\xB5V[=\x91Pa\x02\x16V[\x83Q=\x87\x82>=\x90\xFD[PP\xFD[P\x82\x904a\x06\xF6Wa\x02i\x92a\x02pa\x02b6a\x14\xA2V[\x95\x90a\x18wV[\x940a\x1A\xDCV[\x93`\x01`\x01`\xA0\x1B\x03\x93\x84`\x01T\x16\x95\x84Q\x92c\n(\xA4w`\xE0\x1B\x84R\x81\x85\x85\x01R` \x97\x88\x85`$\x81\x84Z\xFA\x94\x85\x15a\x06\xB9W\x90\x89\x91\x85\x96a\x06\xC3W[P\x86T\x88Qc\t^\xA7\xB3`\xE0\x1B\x81R\x90\x8A\x16`\x01`\x01`\xA0\x1B\x03\x16\x81\x89\x01\x90\x81R` \x81\x01\x88\x90R\x90\x92\x91\x83\x91\x82\x90\x88\x90\x82\x90`@\x01[\x03\x92Z\xF1\x80\x15a\x06\xB9W\x89\x93a\x03\x9A\x93\x85\x93\x89\x93a\x06\x8CW[Pa\x03Y\x8AQa\x03\r\x81a\x15\x18V[`\x05\x81Rd\x1D\x98][\x1D`\xDA\x1B\x86\x82\x01R\x8BQ\x90a\x03*\x82a\x15\x18V[`\x0F\x82R\x7Fbefore withdraw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x83\x01R0a\x16;V[PP\x82T\x8AQc$\xFBV\x9F`\xE0\x1B\x81R\x93\x84\x01\x92\x83R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x83\x01R0`@\x83\x01R\x91\x93\x84\x92\x8B\x16\x91\x83\x91\x88\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x83\x91\x81a\x06[W[Pa\x04\nWP\x7F withdraw via approval\0\0\0\0\0\0\0\0\0\0\x85\x7Fvault.withdraw() reverted during\x81Q\x93a\x03\xFB\x85a\x154V[`6\x85R\x84\x01R\x82\x01Ra\x1DKV[\x91`\xA0\x91a\x04\xB8\x93\x97\x87Q\x88\x81R`#\x89\x82\x01R\x7Fwithdraw consumed this many shar``\x82\x01Rb2\xB9\x9D`\xE9\x1B`\x80\x82\x01R\x89\x83\x82\x01R\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x94\x85\x91\xA1\x80`\x01T\x16\x90\x87T\x16\x90\x88Q\x80\x80\x97\x81\x94cn\xB1v\x9F`\xE1\x1B\x83R0\x8C\x84\x01\x90` \x90\x93\x92\x93`@\x83\x01\x94`\x01`\x01`\xA0\x1B\x03\x80\x92\x16\x84R\x16\x91\x01RV[\x03\x91Z\xFA\x92\x83\x15a\x06OW\x81\x93a\x06 W[P\x86\x84\x03\x93\x84\x11a\x06\rWP`\x80\x85Q\x86\x81R`\x1E\x87\x82\x01R\x7FExpecting allowance to now be:\0\0``\x82\x01R\x84\x89\x82\x01R\xA1a\x05\x17a\x15\x8AV[\x91\x81\x81\x03a\x05)WPPPPQ\x90\x81R\xF3[a\x05\xF8`\x01\x95a\x05\xE9`5\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x96\x95a\x05ja\x05d\x8D\x97a\x1F\x9EV[\x91a\x1F\x9EV[\x96\x84Q\x97\x88\x92h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x89\x85\x01Ra\x05\x94\x81Q\x80\x92\x8B`)\x88\x01\x91\x01a\x15\xF3V[\x83\x01a!=`\xF0\x1B`)\x82\x01Ra\x05\xB4\x82Q\x80\x93\x8B`+\x85\x01\x91\x01a\x15\xF3V[\x01i\x01a\x03\x93+\x0B\x9B{q\xD1`\xB5\x1B`+\x82\x01Ra\x05\xDA\x82Q\x80\x93\x8A\x87\x85\x01\x91\x01a\x15\xF3V[\x01\x03`\x15\x81\x01\x87R\x01\x85a\x15PV[Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x16\x16V[\x03\x90\xA1cNH{q`\xE0\x1B`\0RR`$`\0\xFD[cNH{q`\xE0\x1B\x81R`\x11\x85R`$\x90\xFD[\x90\x92P\x87\x81\x81=\x83\x11a\x06HW[a\x068\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LWQ\x91\x88a\x04\xCAV[P=a\x06.V[P\x85Q\x90=\x90\x82>=\x90\xFD[\x83\x81\x94\x92\x93P=\x83\x11a\x06\x85W[a\x06s\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x87\x91Q\x90\x89a\x03\xA6V[P=a\x06iV[a\x06\xAB\x90\x85=\x87\x11a\x06\xB2W[a\x06\xA3\x81\x83a\x15PV[\x81\x01\x90a\x15rV[P\x8Ca\x02\xFEV[P=a\x06\x99V[\x87Q=\x86\x82>=\x90\xFD[\x82\x81\x93\x92\x97P=\x83\x11a\x06\xEFW[a\x06\xDB\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LWQ\x93\x88\x90a\x02\xE5a\x02\xAEV[P=a\x06\xD1V[P\x80\xFD[\x80\x84\x844a\x02FWa\x07\x17\x91a\x07\x0F6a\x14\xA2V[\x93\x90\x93a\x18wV[\x92`\x01`\x01`\xA0\x1B\x03\x85T\x16\x80;\x15a\x07pW\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x92\x85\x01\x92\x83R` \x83\x01\x91\x90\x91R\x84\x91\x84\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x01XWPa\x01\xF0WP\xF3[\x85\x80\xFD[P4a\x01aW` a\x07\x97a\x07\x9D\x93a\x07\x8C6a\x14\xB8V[\x96\x91\x96\x93\x90\x93a\x18wV[\x92a\x18wV[`\x01T\x85Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\x01\x17V[P\x82\x904a\x06\xF6Wa\x07\xF16a\x14\xA2V[\x93\x90a\x07\xFC\x90a\x18wV[\x93a\x08\x07\x900a\x18\xBDV[`\x01\x80T\x83T\x85Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81\x87\x01\x90\x81R` \x81\x81\x01\x87\x90R\x99\x92\x98\x94\x96\x95\x93\x8A\x91\x83\x91\x8B\x16\x90\x82\x90\x88\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x06\xB9W\x92a\x08\xFF\x92\x8A\x94\x92\x85\x93a\x0B\x04W[Pa\x08\xBD\x89Qa\x08q\x81a\x15\x18V[`\x05\x81Rd\x1D\x98][\x1D`\xDA\x1B\x85\x82\x01R\x8AQ\x90a\x08\x8E\x82a\x15\x18V[`\x12\x82R\x7Fbefore redeemption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x83\x01R0a\x16;V[PP\x86T\x89Qce\x18<\xAD`\xE1\x1B\x81R\x80\x89\x01\x93\x84R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x84\x01R0`@\x84\x01R\x90\x93\x84\x92\x91\x8B\x16\x91\x83\x91\x88\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x83\x91\x81a\n\xD3W[Pa\toWP\x7Fedeem via approval\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x7Fvault.redeem() reverted during r\x81Q\x93a\t`\x85a\x154V[`2\x85R\x84\x01R\x82\x01Ra\x1DKV[\x84T\x84T\x87Qcn\xB1v\x9F`\xE1\x1B\x81R0\x81\x88\x01\x90\x81R\x91\x8A\x16`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R\x92\x98\x92\x93\x92\x84\x92\x16\x90\x82\x90\x81\x90`@\x01\x03\x91Z\xFA\x91\x82\x15a\n\xC8W\x91a\n\x9BW[Pa\t\xC1a\x15\x8AV[\x90\x80a\t\xD1WPPPPQ\x90\x81R\xF3[\x86\x91a\t\xDD\x86\x92a\x1F\x9EV[\x90\x82Q`\xA0\x81\x01\x84R`\x80\x81\x01\x90`\0\x82R\x87`\0\x90[a\n}W[P`5\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x96\x94a\x05\xF8\x94\x84a\x05\xE9\x94`\x80a\x05\xB4\x9A\x97`\x1F\x19\x81\x01\x92\x03\x01\x81R\x86Q\x99\x8A\x94h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x8B\x87\x01Ra\nb\x81Q\x80\x92\x8D`)\x8A\x01\x91\x01a\x15\xF3V[\x85\x01\x91a!=`\xF0\x1B`)\x84\x01RQ\x80\x93`+\x84\x01\x90a\x15\xF3V[\x91`\0\x19\x01\x91`\n\x90\x81\x81\x06`0\x01\x84S\x04\x88\x81a\t\xF4WPa\t\xF9V[\x90P\x85\x81\x81=\x83\x11a\n\xC1W[a\n\xB2\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LWQ\x86a\t\xB8V[P=a\n\xA8V[\x85Q\x90=\x90\x82>=\x90\xFD[\x83\x81\x94\x92\x93P=\x83\x11a\n\xFDW[a\n\xEB\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x87\x91Q\x90\x89a\t\x0BV[P=a\n\xE1V[a\x0B\x1A\x90\x84=\x86\x11a\x06\xB2Wa\x06\xA3\x81\x83a\x15PV[P\x8Ba\x08bV[P\x904a\x01aW` \x91\x82`\x03\x196\x01\x12a\x0C1W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qcp\xA0\x821`\xE0\x1B\x81R0\x82\x82\x01R\x84\x81`$\x81\x87Z\xFA\x90\x81\x15a\x0C'W\x90\x85\x92\x91\x87\x91a\x0B\xF4W[P\x93a\x0B~a\x0B\xAF\x95\x835a\x1E\x8BV[\x84Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x90\x81R0` \x82\x01\x81\x90R`@\x82\x01R\x91\x94\x85\x92\x83\x91\x89\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x90\x81\x15a\x0B\xEBWPa\x0B\xC3W\x82\x80\xF3[\x81=\x83\x11a\x0B\xE4W[a\x0B\xD6\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LW8\x80\x82\x80\xF3[P=a\x0B\xCCV[Q=\x85\x82>=\x90\xFD[\x83\x81\x94\x92P=\x83\x11a\x0C W[a\x0C\x0B\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x90Q\x84\x91\x90a\x0B~a\x0BnV[P=a\x0C\x01V[\x83Q=\x88\x82>=\x90\xFD[\x83\x80\xFD[P4a\x01aW` a\x07\x97a\x0CM\x93a\x07\x8C6a\x14\xB8V[`\x01T\x85Qc-\x18+\xE5`\xE2\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\x01\x17V[\x80\x84\x844a\x02FW` 6`\x03\x19\x01\x12a\x02FW`\xFF\x82T`\xA0\x1C\x16\x15a\x02FW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x02<W\x85\x91a\r\x1BW[P\x15a\x02\x03W\x82;\x15a\x02\x03W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cb!\xE4\xF1`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01XWPa\x01\xF0WP\xF3[\x94PP` \x84=\x82\x11a\rGW[\x81a\r6` \x93\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x84\x93Q\x86a\x0C\xE0V[=\x91Pa\r)V[P\x904a\x01aWa\rx`\xA0a\rqa\rg6a\x14\xB8V[\x94\x91\x92\x90\x92a\x18wV[\x910a\x18\xBDV[\x93\x85Q\x94\x86\x86R`.\x87\x87\x01R\x7FWill attempt to proxy redeem thi``\x87\x01R\x7Fs many shares:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x87\x01R` \x95\x81\x87\x82\x01R\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x94\x85\x91\xA1\x80\x85\x10\x15a\x0F\xE4Wa\x0E\xA6\x93\x87Q\x80a\x0E]\x88\x82\x91\x90`@\x83R`$`@\x84\x01R\x7FApproving spend of this many sha``\x84\x01Rc92\xB9\x9D`\xE1\x1B`\x80\x84\x01R` `\xA0\x84\x01\x93\x01RV[\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x92\x88\x87\x87\x86`\x01T\x16\x87\x87T\x16\x84\x8DQ\x80\x9B\x81\x95\x82\x94c\t^\xA7\xB3`\xE0\x1B\x84R\x8C\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x94\x85\x15a\x0F\xD8Wa\x0E\xF9\x96\x89\x96a\x0F\xBBW[P\x84T\x8AQce\x18<\xAD`\xE1\x1B\x81R\x95\x86\x01\x94\x85R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x85\x01R0`@\x85\x01R\x93\x95\x86\x94\x90\x92\x16\x92\x84\x92\x83\x91``\x01\x90V[\x03\x92Z\xF1\x85\x91\x81a\x0F\x8CW[Pa\x0F\x0EW\x84\x80\xF3[\x7Fe to redeem more shares than it \x84\x7FRedemption proxy must not be abla\x0F\x84\x96Q\x95a\x0F`\x87a\x14\xFCV[`L\x87R\x86\x01R\x84\x01Rk\x1D\xD8\\\xC8\x18\\\x1C\x1C\x9B\xDD\x99Y`\xA2\x1B``\x84\x01Ra\x1D\x9DV[8\x80\x80\x80\x84\x80\xF3[\x90\x91P\x83\x81\x81=\x83\x11a\x0F\xB4W[a\x0F\xA4\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LWQ\x908a\x0F\x05V[P=a\x0F\x9AV[a\x0F\xD1\x90\x87=\x89\x11a\x06\xB2Wa\x06\xA3\x81\x83a\x15PV[P8a\x0E\xBCV[P\x88Q\x90=\x90\x82>=\x90\xFD[\x87\x80\xFD[P\x904a\x01aWa\x10\x10a\x10\t\x91a\x0F\xFF6a\x14\xB8V[\x93\x91\x94\x90\x94a\x18wV[\x930a\x1A\xDCV[\x90`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16\x92\x86Q\x95c\n(\xA4w`\xE0\x1B\x87R\x81\x83\x88\x01R\x88` \x97\x88\x81`$\x81\x8AZ\xFA\x90\x81\x15a\x12\xD8W\x82\x91a\x12\xA6W[P\x95`\xA0\x96\x8AQ\x8B\x81R`0\x8C\x82\x01R\x7FWill attempt to proxy withdraw t``\x82\x01R\x7Fhis many shares:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x82\x01R\x81\x8B\x82\x01R\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x98\x89\x91\xA1\x88\x10\x15a\x06\xF6W\x87\x89\x91a\x11k\x98\x8CQ\x80a\x113\x85\x82\x91\x90`@\x83R`$`@\x84\x01R\x7FApproving spend of this many sha``\x84\x01Rc92\xB9\x9D`\xE1\x1B`\x80\x84\x01R` `\xA0\x84\x01\x93\x01RV[\x03\x90\xA1\x87\x87T\x16\x84\x8DQ\x80\x9B\x81\x95\x82\x94c\t^\xA7\xB3`\xE0\x1B\x84R\x8C\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x94\x85\x15a\x0F\xD8Wa\x11\xBE\x96\x89\x96a\x12\x89W[P\x84T\x8AQc$\xFBV\x9F`\xE0\x1B\x81R\x95\x86\x01\x94\x85R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x85\x01R0`@\x85\x01R\x93\x95\x86\x94\x90\x92\x16\x92\x84\x92\x83\x91``\x01\x90V[\x03\x92Z\xF1\x85\x91\x81a\x12ZW[Pa\x11\xD3W\x84\x80\xF3[\x7Fe to withdraw more shares than i\x84\x7FRedemption proxy must not be abla\x0F\x84\x96Q\x95a\x12%\x87a\x14\xFCV[`N\x87R\x86\x01R\x84\x01R\x7Ft was approved\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x01Ra\x1D\x9DV[\x90\x91P\x83\x81\x81=\x83\x11a\x12\x82W[a\x12r\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LWQ\x908a\x11\xCAV[P=a\x12hV[a\x12\x9F\x90\x87=\x89\x11a\x06\xB2Wa\x06\xA3\x81\x83a\x15PV[P8a\x11\x81V[\x80\x92P\x89\x80\x92P=\x83\x11a\x12\xD1W[a\x12\xBF\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LWQ\x89\x90`\xA0a\x10LV[P=a\x12\xB5V[\x8AQ=\x84\x82>=\x90\xFD[P4a\x01aW` \x91\x82`\x03\x196\x01\x12a\x0C1W\x805`\x01`\x01`\xA0\x1B\x03\x80\x86T\x16\x80;\x15a\x14\x0FW\x84Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x85\x82\x01\x90\x81R` \x81\x01\x85\x90R\x90\x91\x88\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x13\xF2Wa\x13\xFCW[Pa\x13\x84\x85\x83\x83\x89T\x16\x84`\x01T\x16\x8A\x89Q\x80\x96\x81\x95\x82\x94c\t^\xA7\xB3`\xE0\x1B\x84R\x8C\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x13\xF2W\x91\x86\x93\x91a\x0B\xAF\x96\x95\x93a\x13\xD5W[P`\x01T\x16\x90\x87\x85Q\x80\x97\x81\x95\x82\x94cnU?e`\xE0\x1B\x84R0\x91\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[a\x13\xEB\x90\x85=\x87\x11a\x06\xB2Wa\x06\xA3\x81\x83a\x15PV[P8a\x13\x9DV[\x85Q=\x89\x82>=\x90\xFD[a\x14\x08\x90\x96\x91\x96a\x14\xD2V[\x948a\x13DV[\x86\x80\xFD[\x92\x90P4a\x0C1W\x82` \x91\x81\x86\x81a\x14fa\x149a\x1416a\x14\xA2V[\x91\x90\x91a\x18wV[`\x01Tc\x94\xBF\x80M`\xE0\x1B\x85R\x96\x84\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R\x90\x95\x16\x94`@\x01\x90V[\x03\x92Z\xF1\x90\x81\x15a\x01XWPa\x14zWP\x80\xF3[` \x90\x81=\x81\x11a\x14\x9BW[a\x14\x90\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0W\x80\xF3[P=a\x14\x86V[`@\x90`\x03\x19\x01\x12a\x01LW`\x045\x90`$5\x90V[``\x90`\x03\x19\x01\x12a\x01LW`\x045\x90`$5\x90`D5\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x14\xE6W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x14\xE6W`@RV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x14\xE6W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x14\xE6W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x14\xE6W`@RV[\x90\x81` \x91\x03\x12a\x01LWQ\x80\x15\x15\x81\x03a\x01LW\x90V[`@Q\x90a\x15\x97\x82a\x14\xFCV[`A\x82R`e`\xF8\x1B``\x83\x7FThe vault failed to update the r` \x82\x01R\x7Fedemption proxy's share allowanc`@\x82\x01R\x01RV[`\0[\x83\x81\x10a\x16\x06WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x15\xF6V[\x90` \x91a\x16/\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x15\xF3V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x92\x90\x91\x92`\x01`\x01`\xA0\x1B\x03\x93`\0\x85\x81T\x16`@\x91\x82Q\x94\x85\x92\x89cp\xA0\x821`\xE0\x1B\x92\x83\x86R\x16\x90\x81`\x04\x86\x01R\x84`$` \x96\x87\x93Z\xFA\x96\x87\x15a\x18mW\x83\x97a\x18>W[P\x83\x90`$\x88\x9B`\x01T\x16\x93\x87Q\x94\x85\x93\x84\x92\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a\x183W\x80\x92a\x18\x03W[PP\x82Q\x7Fasset.balanceOf(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x82\x01R\x87Q\x91\x97\x90\x95\x83\x82\x01\x95\x89\x93a\x16\xF0\x81`0\x8B\x01\x8Aa\x15\xF3V[\x88\x01\x90b\x05$\x05`\xEB\x1B\x90\x81`0\x84\x01R\x80Q\x87\x82\x01\x93\x81`3\x82\x01\x90a\x17\x17\x91\x87a\x15\xF3V[\x01\x93\x8A`)`\xF8\x1B\x95\x86`3\x82\x01R\x03`\x14\x81\x01\x8CR`4\x01a\x17:\x90\x8Ca\x15PV[\x88Q\x9A\x89\x8CR\x89\x8C\x01a\x17L\x91a\x16\x16V[\x90\x88\x8C\x01R\x8A\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x9B\x8C\x92\x03\x90\xA1\x87Q\x98\x89\x95\x88\x87\x01\x7Fvault.balanceOf(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90RQ\x90\x81`0\x88\x01a\x17\xB4\x92a\x15\xF3V[\x85\x01\x91`0\x83\x01RQ\x91\x82`3\x83\x01a\x17\xCC\x92a\x15\xF3V[\x01\x90`3\x82\x01R\x03`\x14\x81\x01\x85R`4\x01a\x17\xE7\x90\x85a\x15PV[\x82Q\x93\x83\x85\x94\x85R\x84\x01a\x17\xFA\x91a\x16\x16V[\x91\x83\x01R\x03\x90\xA1V[\x90\x91P\x82\x82\x81=\x83\x11a\x18,W[a\x18\x1B\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ8\x80a\x16\xADV[P=a\x18\x11V[\x84Q\x90=\x90\x82>=\x90\xFD[\x90\x96P\x83\x81\x81=\x83\x11a\x18fW[a\x18V\x81\x83a\x15PV[\x81\x01\x03\x12a\x01aWQ\x95\x83a\x16\x83V[P=a\x18LV[\x85Q=\x85\x82>=\x90\xFD[`\x03\x90\x06\x80\x15a\x18\xB8W`\x01\x14a\x18\xA0Ws\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\x90V[s\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\x90V[P0\x90V[\x90\x80\x15a\x01LW`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16\x92`@\x92\x83Q\x91cl\x82\xBB\xBF`\xE1\x1B\x83R\x16\x80`\x04\x83\x01R` \x92\x83\x83`$\x81\x89Z\xFA\x92\x83\x15a\x1A\xA2W`\0\x93a\x1A\xADW[P\x82\x15a\x01LW\x84Q\x91cp\xA0\x821`\xE0\x1B\x83R`\x04\x83\x01R\x83\x82`$\x81\x89Z\xFA\x91\x82\x15a\x1A\xA2W`\0\x92a\x1AsW[P\x81\x15a\x01LWa\x19M\x92a\x19H\x91a\x1E\x8BV[a\x1E\x8BV[\x82Qc\x03\xD1h\x9D`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x93\x82\x82`$\x81\x84Z\xFA\x91\x82\x15a\x1A5W\x90\x83\x91`\0\x93a\x1A@W[P\x90`$\x91\x85Q\x92\x83\x80\x92c&mj\x83`\xE1\x1B\x82R\x89`\x04\x83\x01RZ\xFA\x80\x15a\x1A5W\x90\x83\x91a\x1A\x0CW[PP\x15a\x01LW\x81\x83`\x80\x92`\x1C\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x95Q\x93\x80\x85R\x84\x01R\x7FShares to use in redemption:\0\0\0\0``\x84\x01R\x82\x01R\xA1\x90V[\x81=\x83\x11a\x1A.W[a\x1A\x1F\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x818a\x19\xA7V[P=a\x1A\x15V[\x84Q=`\0\x82>=\x90\xFD[\x91\x82\x81\x94\x92\x94=\x83\x11a\x1AlW[a\x1AX\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x90\x82\x90`$a\x19|V[P=a\x1ANV[\x90\x91\x84\x82\x81=\x83\x11a\x1A\x9BW[a\x1A\x8A\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x908a\x194V[P=a\x1A\x80V[\x85Q=`\0\x82>=\x90\xFD[\x90\x92\x84\x82\x81=\x83\x11a\x1A\xD5W[a\x1A\xC4\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x918a\x19\x04V[P=a\x1A\xBAV[\x90`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16\x92`@\x92\x83Q\x91c\xCE\x96\xCBw`\xE0\x1B\x83R\x16\x90\x81`\x04\x82\x01R` \x92\x83\x82`$\x81\x89Z\xFA\x91\x82\x15a\x1A\xA2W`\0\x92a\x1D\x1CW[P\x81\x15a\x01LW\x84Qcp\xA0\x821`\xE0\x1B\x80\x82R`\x04\x82\x01\x85\x90R\x90\x93\x85\x85`$\x81\x8BZ\xFA\x94\x85\x15a\x1C\xE2W`\0\x95a\x1C\xEDW[P\x84\x15a\x01LW\x86Q\x92c\n(\xA4w`\xE0\x1B\x84R`\x04\x84\x01R\x85\x83`$\x81\x8BZ\xFA\x92\x83\x15a\x1C\xE2W`\0\x93a\x1C\xB3W[P\x86Q\x91\x82R`\x04\x82\x01R\x84\x81`$\x81\x8AZ\xFA\x90\x81\x15a\x1C\xA8W`\0\x91a\x1CyW[Pa\x1B\xAE\x91a\x1E\x8BV[\x91\x82\x11a\x01LW\x82`$\x95\x85Q\x96\x87\x80\x92c&mj\x83`\xE1\x1B\x82R\x86`\x04\x83\x01RZ\xFA\x94\x85\x15a\x1A5W`\0\x95a\x1CJW[P\x84\x11a\x01LW\x15a\x01LW\x81\x83`\x80\x92`\x1A\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x95Q\x93\x80\x85R\x84\x01R\x7FTokens to use in withdraw:\0\0\0\0\0\0``\x84\x01R\x82\x01R\xA1\x90V[\x90\x94\x83\x82\x81=\x83\x11a\x1CrW[a\x1Ca\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x938a\x1B\xE0V[P=a\x1CWV[\x90\x85\x82\x81=\x83\x11a\x1C\xA1W[a\x1C\x8F\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQa\x1B\xAEa\x1B\xA4V[P=a\x1C\x85V[\x86Q=`\0\x82>=\x90\xFD[\x90\x92\x86\x82\x81=\x83\x11a\x1C\xDBW[a\x1C\xCA\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x918a\x1B\x82V[P=a\x1C\xC0V[\x87Q=`\0\x82>=\x90\xFD[\x90\x94\x86\x82\x81=\x83\x11a\x1D\x15W[a\x1D\x04\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x938a\x1BRV[P=a\x1C\xFAV[\x90\x91\x84\x82\x81=\x83\x11a\x1DDW[a\x1D3\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x908a\x1B\x1EV[P=a\x1D)V[a\x1D\x84\x7F\xEB\x03\xCA\x8C\x87\xC7\x84\x9B\xEF\x8FT\xCF\xDD,k\x96{'4\xFE\x87/u\x19x\xC3K\xB9\x1E\x13\xD3Q\x91`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x16\x16V[\x03\x90\xA1cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x81\x81\x11a\x1D\xA9WPPPV[\x7Fb\xBD\xDA\x9A\x05\xCD\xBC\xDB\xF9\x05\xCB\xAD\x99\xC6\xEB\xDC\t\x8Bo\t3\xD8\xF2\xEB<\xFA\xB7@\x0B`%\x14\x93P`;a\x1D\xE3a\x1D\xDDa\x1Ew\x93a\x1F\x9EV[\x93a\x1F\x9EV[`@Q\x94\x85\x91` \x95h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x87\x85\x01Ra\x1E\x10\x81Q\x80\x92\x89`)\x88\x01\x91\x01a\x15\xF3V[\x83\x01`\x1F`\xF9\x1B`)\x82\x01Ra\x1E/\x82Q\x80\x93\x89`*\x85\x01\x91\x01a\x15\xF3V[\x01\x7F failed, reason: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`*\x82\x01Ra\x1Eh\x82Q\x80\x93\x88\x87\x85\x01\x91\x01a\x15\xF3V[\x01\x03`\x1B\x81\x01\x85R\x01\x83a\x15PV[a\x1D\x84`@Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x16\x16V[\x90\x80\x82\x11a\x1E\x97WP\x90V[`\x01\x81\x01\x80\x91\x11a\x1F\x88W\x80\x15a\x1FrWa\x1Fla\x1E\xD8\x7F\xA9^n*\x18$\x11\xE7\xA6\xF9\xED\x11J\x85\xC3v\x1D\x87\xF9\xB8\xF4S\xD8B\xC7\x125\xAAd\xFF\xF9\x9F\x92\x84\x06\x93a\x1F\x9EV[a\x1FX`3a\x1E\xE6\x86a\x1F\x9EV[\x92`@Q\x93\x84\x91\x7FClamping value \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01Ra\x1F'\x81Q\x80\x92` `/\x87\x01\x91\x01a\x15\xF3V[\x82\x01c\x01\x03\xA3y`\xE5\x1B`/\x82\x01Ra\x1FI\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x15\xF3V[\x01\x03`\x13\x81\x01\x84R\x01\x82a\x15PV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x16\x16V[\x03\x90\xA1\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\x1F\xB4W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \x1Cm\xD4\xF74j\xED\x8E pH`\x9E\x92's/\xEC\xA7\xBDg\xEA*\x91)h\xFC\x06\xA4\x80\x9D\x16dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static CRYTICERC4626REDEEMUSINGAPPROVAL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x1B.\xF1\xCA\x14a\x14\x13WP\x80c'#\xF9\xEE\x14a\x12\xE2W\x80cH\x0F\xEFj\x14a\x0F\xE8W\x80co\x89\xDD.\x14a\rOW\x83\x81c\x96\xEB'\xA1\x14a\x0C\x90WP\x80c\xA4\x1F\xE4\x9F\x14a\x0C5W\x80c\xA8\x15\xC1\x0F\x14a\x0B!W\x80c\xAA\xEBB\x03\x14a\x07\xE0W\x80c\xB8\x19\"\x05\x14a\x07tW\x83\x81c\xB8\x8D\xAB2\x14a\x06\xFAWP\x80c\xC2\x1E\xDAo\x14a\x02JW\x83\x81c\xCC\xC9J\xE9\x14a\x01eWPc\xE2\xBB\xB1X\x14a\0\xB9W`\0\x80\xFD[4a\x01aW` a\0\xD8a\x01\x17\x93a\0\xD06a\x14\xA2V[\x92\x90\x92a\x18wV[\x91\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93cnU?e`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x90\x81\x15a\x01XWPa\x01+WP\x80\xF3[` \x90\x81=\x81\x11a\x01QW[a\x01A\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x80\xF3[`\0\x80\xFD[P=a\x017V[Q=\x84\x82>=\x90\xFD[\x82\x80\xFD[\x80\x84\x844a\x02FW` 6`\x03\x19\x01\x12a\x02FW`\xFF\x82T`\xA0\x1C\x16\x15a\x02FW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x02<W\x85\x91a\x02\x08W[P\x15a\x02\x03W\x82;\x15a\x02\x03W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cU\xDFp\r`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01XWPa\x01\xF0WP\xF3[a\x01\xF9\x90a\x14\xD2V[a\x02\0W\x80\xF3[\x80\xFD[PPP\xFD[\x94PP` \x84=\x82\x11a\x024W[\x81a\x02#` \x93\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x84\x93Q\x86a\x01\xB5V[=\x91Pa\x02\x16V[\x83Q=\x87\x82>=\x90\xFD[PP\xFD[P\x82\x904a\x06\xF6Wa\x02i\x92a\x02pa\x02b6a\x14\xA2V[\x95\x90a\x18wV[\x940a\x1A\xDCV[\x93`\x01`\x01`\xA0\x1B\x03\x93\x84`\x01T\x16\x95\x84Q\x92c\n(\xA4w`\xE0\x1B\x84R\x81\x85\x85\x01R` \x97\x88\x85`$\x81\x84Z\xFA\x94\x85\x15a\x06\xB9W\x90\x89\x91\x85\x96a\x06\xC3W[P\x86T\x88Qc\t^\xA7\xB3`\xE0\x1B\x81R\x90\x8A\x16`\x01`\x01`\xA0\x1B\x03\x16\x81\x89\x01\x90\x81R` \x81\x01\x88\x90R\x90\x92\x91\x83\x91\x82\x90\x88\x90\x82\x90`@\x01[\x03\x92Z\xF1\x80\x15a\x06\xB9W\x89\x93a\x03\x9A\x93\x85\x93\x89\x93a\x06\x8CW[Pa\x03Y\x8AQa\x03\r\x81a\x15\x18V[`\x05\x81Rd\x1D\x98][\x1D`\xDA\x1B\x86\x82\x01R\x8BQ\x90a\x03*\x82a\x15\x18V[`\x0F\x82R\x7Fbefore withdraw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x83\x01R0a\x16;V[PP\x82T\x8AQc$\xFBV\x9F`\xE0\x1B\x81R\x93\x84\x01\x92\x83R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x83\x01R0`@\x83\x01R\x91\x93\x84\x92\x8B\x16\x91\x83\x91\x88\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x83\x91\x81a\x06[W[Pa\x04\nWP\x7F withdraw via approval\0\0\0\0\0\0\0\0\0\0\x85\x7Fvault.withdraw() reverted during\x81Q\x93a\x03\xFB\x85a\x154V[`6\x85R\x84\x01R\x82\x01Ra\x1DKV[\x91`\xA0\x91a\x04\xB8\x93\x97\x87Q\x88\x81R`#\x89\x82\x01R\x7Fwithdraw consumed this many shar``\x82\x01Rb2\xB9\x9D`\xE9\x1B`\x80\x82\x01R\x89\x83\x82\x01R\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x94\x85\x91\xA1\x80`\x01T\x16\x90\x87T\x16\x90\x88Q\x80\x80\x97\x81\x94cn\xB1v\x9F`\xE1\x1B\x83R0\x8C\x84\x01\x90` \x90\x93\x92\x93`@\x83\x01\x94`\x01`\x01`\xA0\x1B\x03\x80\x92\x16\x84R\x16\x91\x01RV[\x03\x91Z\xFA\x92\x83\x15a\x06OW\x81\x93a\x06 W[P\x86\x84\x03\x93\x84\x11a\x06\rWP`\x80\x85Q\x86\x81R`\x1E\x87\x82\x01R\x7FExpecting allowance to now be:\0\0``\x82\x01R\x84\x89\x82\x01R\xA1a\x05\x17a\x15\x8AV[\x91\x81\x81\x03a\x05)WPPPPQ\x90\x81R\xF3[a\x05\xF8`\x01\x95a\x05\xE9`5\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x96\x95a\x05ja\x05d\x8D\x97a\x1F\x9EV[\x91a\x1F\x9EV[\x96\x84Q\x97\x88\x92h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x89\x85\x01Ra\x05\x94\x81Q\x80\x92\x8B`)\x88\x01\x91\x01a\x15\xF3V[\x83\x01a!=`\xF0\x1B`)\x82\x01Ra\x05\xB4\x82Q\x80\x93\x8B`+\x85\x01\x91\x01a\x15\xF3V[\x01i\x01a\x03\x93+\x0B\x9B{q\xD1`\xB5\x1B`+\x82\x01Ra\x05\xDA\x82Q\x80\x93\x8A\x87\x85\x01\x91\x01a\x15\xF3V[\x01\x03`\x15\x81\x01\x87R\x01\x85a\x15PV[Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x16\x16V[\x03\x90\xA1cNH{q`\xE0\x1B`\0RR`$`\0\xFD[cNH{q`\xE0\x1B\x81R`\x11\x85R`$\x90\xFD[\x90\x92P\x87\x81\x81=\x83\x11a\x06HW[a\x068\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LWQ\x91\x88a\x04\xCAV[P=a\x06.V[P\x85Q\x90=\x90\x82>=\x90\xFD[\x83\x81\x94\x92\x93P=\x83\x11a\x06\x85W[a\x06s\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x87\x91Q\x90\x89a\x03\xA6V[P=a\x06iV[a\x06\xAB\x90\x85=\x87\x11a\x06\xB2W[a\x06\xA3\x81\x83a\x15PV[\x81\x01\x90a\x15rV[P\x8Ca\x02\xFEV[P=a\x06\x99V[\x87Q=\x86\x82>=\x90\xFD[\x82\x81\x93\x92\x97P=\x83\x11a\x06\xEFW[a\x06\xDB\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LWQ\x93\x88\x90a\x02\xE5a\x02\xAEV[P=a\x06\xD1V[P\x80\xFD[\x80\x84\x844a\x02FWa\x07\x17\x91a\x07\x0F6a\x14\xA2V[\x93\x90\x93a\x18wV[\x92`\x01`\x01`\xA0\x1B\x03\x85T\x16\x80;\x15a\x07pW\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x92\x85\x01\x92\x83R` \x83\x01\x91\x90\x91R\x84\x91\x84\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x01XWPa\x01\xF0WP\xF3[\x85\x80\xFD[P4a\x01aW` a\x07\x97a\x07\x9D\x93a\x07\x8C6a\x14\xB8V[\x96\x91\x96\x93\x90\x93a\x18wV[\x92a\x18wV[`\x01T\x85Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\x01\x17V[P\x82\x904a\x06\xF6Wa\x07\xF16a\x14\xA2V[\x93\x90a\x07\xFC\x90a\x18wV[\x93a\x08\x07\x900a\x18\xBDV[`\x01\x80T\x83T\x85Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81\x87\x01\x90\x81R` \x81\x81\x01\x87\x90R\x99\x92\x98\x94\x96\x95\x93\x8A\x91\x83\x91\x8B\x16\x90\x82\x90\x88\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x06\xB9W\x92a\x08\xFF\x92\x8A\x94\x92\x85\x93a\x0B\x04W[Pa\x08\xBD\x89Qa\x08q\x81a\x15\x18V[`\x05\x81Rd\x1D\x98][\x1D`\xDA\x1B\x85\x82\x01R\x8AQ\x90a\x08\x8E\x82a\x15\x18V[`\x12\x82R\x7Fbefore redeemption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x83\x01R0a\x16;V[PP\x86T\x89Qce\x18<\xAD`\xE1\x1B\x81R\x80\x89\x01\x93\x84R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x84\x01R0`@\x84\x01R\x90\x93\x84\x92\x91\x8B\x16\x91\x83\x91\x88\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x83\x91\x81a\n\xD3W[Pa\toWP\x7Fedeem via approval\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x7Fvault.redeem() reverted during r\x81Q\x93a\t`\x85a\x154V[`2\x85R\x84\x01R\x82\x01Ra\x1DKV[\x84T\x84T\x87Qcn\xB1v\x9F`\xE1\x1B\x81R0\x81\x88\x01\x90\x81R\x91\x8A\x16`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R\x92\x98\x92\x93\x92\x84\x92\x16\x90\x82\x90\x81\x90`@\x01\x03\x91Z\xFA\x91\x82\x15a\n\xC8W\x91a\n\x9BW[Pa\t\xC1a\x15\x8AV[\x90\x80a\t\xD1WPPPPQ\x90\x81R\xF3[\x86\x91a\t\xDD\x86\x92a\x1F\x9EV[\x90\x82Q`\xA0\x81\x01\x84R`\x80\x81\x01\x90`\0\x82R\x87`\0\x90[a\n}W[P`5\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x96\x94a\x05\xF8\x94\x84a\x05\xE9\x94`\x80a\x05\xB4\x9A\x97`\x1F\x19\x81\x01\x92\x03\x01\x81R\x86Q\x99\x8A\x94h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x8B\x87\x01Ra\nb\x81Q\x80\x92\x8D`)\x8A\x01\x91\x01a\x15\xF3V[\x85\x01\x91a!=`\xF0\x1B`)\x84\x01RQ\x80\x93`+\x84\x01\x90a\x15\xF3V[\x91`\0\x19\x01\x91`\n\x90\x81\x81\x06`0\x01\x84S\x04\x88\x81a\t\xF4WPa\t\xF9V[\x90P\x85\x81\x81=\x83\x11a\n\xC1W[a\n\xB2\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LWQ\x86a\t\xB8V[P=a\n\xA8V[\x85Q\x90=\x90\x82>=\x90\xFD[\x83\x81\x94\x92\x93P=\x83\x11a\n\xFDW[a\n\xEB\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x87\x91Q\x90\x89a\t\x0BV[P=a\n\xE1V[a\x0B\x1A\x90\x84=\x86\x11a\x06\xB2Wa\x06\xA3\x81\x83a\x15PV[P\x8Ba\x08bV[P\x904a\x01aW` \x91\x82`\x03\x196\x01\x12a\x0C1W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qcp\xA0\x821`\xE0\x1B\x81R0\x82\x82\x01R\x84\x81`$\x81\x87Z\xFA\x90\x81\x15a\x0C'W\x90\x85\x92\x91\x87\x91a\x0B\xF4W[P\x93a\x0B~a\x0B\xAF\x95\x835a\x1E\x8BV[\x84Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x90\x81R0` \x82\x01\x81\x90R`@\x82\x01R\x91\x94\x85\x92\x83\x91\x89\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x90\x81\x15a\x0B\xEBWPa\x0B\xC3W\x82\x80\xF3[\x81=\x83\x11a\x0B\xE4W[a\x0B\xD6\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LW8\x80\x82\x80\xF3[P=a\x0B\xCCV[Q=\x85\x82>=\x90\xFD[\x83\x81\x94\x92P=\x83\x11a\x0C W[a\x0C\x0B\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x90Q\x84\x91\x90a\x0B~a\x0BnV[P=a\x0C\x01V[\x83Q=\x88\x82>=\x90\xFD[\x83\x80\xFD[P4a\x01aW` a\x07\x97a\x0CM\x93a\x07\x8C6a\x14\xB8V[`\x01T\x85Qc-\x18+\xE5`\xE2\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\x01\x17V[\x80\x84\x844a\x02FW` 6`\x03\x19\x01\x12a\x02FW`\xFF\x82T`\xA0\x1C\x16\x15a\x02FW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x02<W\x85\x91a\r\x1BW[P\x15a\x02\x03W\x82;\x15a\x02\x03W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cb!\xE4\xF1`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01XWPa\x01\xF0WP\xF3[\x94PP` \x84=\x82\x11a\rGW[\x81a\r6` \x93\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x84\x93Q\x86a\x0C\xE0V[=\x91Pa\r)V[P\x904a\x01aWa\rx`\xA0a\rqa\rg6a\x14\xB8V[\x94\x91\x92\x90\x92a\x18wV[\x910a\x18\xBDV[\x93\x85Q\x94\x86\x86R`.\x87\x87\x01R\x7FWill attempt to proxy redeem thi``\x87\x01R\x7Fs many shares:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x87\x01R` \x95\x81\x87\x82\x01R\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x94\x85\x91\xA1\x80\x85\x10\x15a\x0F\xE4Wa\x0E\xA6\x93\x87Q\x80a\x0E]\x88\x82\x91\x90`@\x83R`$`@\x84\x01R\x7FApproving spend of this many sha``\x84\x01Rc92\xB9\x9D`\xE1\x1B`\x80\x84\x01R` `\xA0\x84\x01\x93\x01RV[\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x92\x88\x87\x87\x86`\x01T\x16\x87\x87T\x16\x84\x8DQ\x80\x9B\x81\x95\x82\x94c\t^\xA7\xB3`\xE0\x1B\x84R\x8C\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x94\x85\x15a\x0F\xD8Wa\x0E\xF9\x96\x89\x96a\x0F\xBBW[P\x84T\x8AQce\x18<\xAD`\xE1\x1B\x81R\x95\x86\x01\x94\x85R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x85\x01R0`@\x85\x01R\x93\x95\x86\x94\x90\x92\x16\x92\x84\x92\x83\x91``\x01\x90V[\x03\x92Z\xF1\x85\x91\x81a\x0F\x8CW[Pa\x0F\x0EW\x84\x80\xF3[\x7Fe to redeem more shares than it \x84\x7FRedemption proxy must not be abla\x0F\x84\x96Q\x95a\x0F`\x87a\x14\xFCV[`L\x87R\x86\x01R\x84\x01Rk\x1D\xD8\\\xC8\x18\\\x1C\x1C\x9B\xDD\x99Y`\xA2\x1B``\x84\x01Ra\x1D\x9DV[8\x80\x80\x80\x84\x80\xF3[\x90\x91P\x83\x81\x81=\x83\x11a\x0F\xB4W[a\x0F\xA4\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LWQ\x908a\x0F\x05V[P=a\x0F\x9AV[a\x0F\xD1\x90\x87=\x89\x11a\x06\xB2Wa\x06\xA3\x81\x83a\x15PV[P8a\x0E\xBCV[P\x88Q\x90=\x90\x82>=\x90\xFD[\x87\x80\xFD[P\x904a\x01aWa\x10\x10a\x10\t\x91a\x0F\xFF6a\x14\xB8V[\x93\x91\x94\x90\x94a\x18wV[\x930a\x1A\xDCV[\x90`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16\x92\x86Q\x95c\n(\xA4w`\xE0\x1B\x87R\x81\x83\x88\x01R\x88` \x97\x88\x81`$\x81\x8AZ\xFA\x90\x81\x15a\x12\xD8W\x82\x91a\x12\xA6W[P\x95`\xA0\x96\x8AQ\x8B\x81R`0\x8C\x82\x01R\x7FWill attempt to proxy withdraw t``\x82\x01R\x7Fhis many shares:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x82\x01R\x81\x8B\x82\x01R\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x98\x89\x91\xA1\x88\x10\x15a\x06\xF6W\x87\x89\x91a\x11k\x98\x8CQ\x80a\x113\x85\x82\x91\x90`@\x83R`$`@\x84\x01R\x7FApproving spend of this many sha``\x84\x01Rc92\xB9\x9D`\xE1\x1B`\x80\x84\x01R` `\xA0\x84\x01\x93\x01RV[\x03\x90\xA1\x87\x87T\x16\x84\x8DQ\x80\x9B\x81\x95\x82\x94c\t^\xA7\xB3`\xE0\x1B\x84R\x8C\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x94\x85\x15a\x0F\xD8Wa\x11\xBE\x96\x89\x96a\x12\x89W[P\x84T\x8AQc$\xFBV\x9F`\xE0\x1B\x81R\x95\x86\x01\x94\x85R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x85\x01R0`@\x85\x01R\x93\x95\x86\x94\x90\x92\x16\x92\x84\x92\x83\x91``\x01\x90V[\x03\x92Z\xF1\x85\x91\x81a\x12ZW[Pa\x11\xD3W\x84\x80\xF3[\x7Fe to withdraw more shares than i\x84\x7FRedemption proxy must not be abla\x0F\x84\x96Q\x95a\x12%\x87a\x14\xFCV[`N\x87R\x86\x01R\x84\x01R\x7Ft was approved\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x01Ra\x1D\x9DV[\x90\x91P\x83\x81\x81=\x83\x11a\x12\x82W[a\x12r\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LWQ\x908a\x11\xCAV[P=a\x12hV[a\x12\x9F\x90\x87=\x89\x11a\x06\xB2Wa\x06\xA3\x81\x83a\x15PV[P8a\x11\x81V[\x80\x92P\x89\x80\x92P=\x83\x11a\x12\xD1W[a\x12\xBF\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LWQ\x89\x90`\xA0a\x10LV[P=a\x12\xB5V[\x8AQ=\x84\x82>=\x90\xFD[P4a\x01aW` \x91\x82`\x03\x196\x01\x12a\x0C1W\x805`\x01`\x01`\xA0\x1B\x03\x80\x86T\x16\x80;\x15a\x14\x0FW\x84Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x85\x82\x01\x90\x81R` \x81\x01\x85\x90R\x90\x91\x88\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x13\xF2Wa\x13\xFCW[Pa\x13\x84\x85\x83\x83\x89T\x16\x84`\x01T\x16\x8A\x89Q\x80\x96\x81\x95\x82\x94c\t^\xA7\xB3`\xE0\x1B\x84R\x8C\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x13\xF2W\x91\x86\x93\x91a\x0B\xAF\x96\x95\x93a\x13\xD5W[P`\x01T\x16\x90\x87\x85Q\x80\x97\x81\x95\x82\x94cnU?e`\xE0\x1B\x84R0\x91\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[a\x13\xEB\x90\x85=\x87\x11a\x06\xB2Wa\x06\xA3\x81\x83a\x15PV[P8a\x13\x9DV[\x85Q=\x89\x82>=\x90\xFD[a\x14\x08\x90\x96\x91\x96a\x14\xD2V[\x948a\x13DV[\x86\x80\xFD[\x92\x90P4a\x0C1W\x82` \x91\x81\x86\x81a\x14fa\x149a\x1416a\x14\xA2V[\x91\x90\x91a\x18wV[`\x01Tc\x94\xBF\x80M`\xE0\x1B\x85R\x96\x84\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R\x90\x95\x16\x94`@\x01\x90V[\x03\x92Z\xF1\x90\x81\x15a\x01XWPa\x14zWP\x80\xF3[` \x90\x81=\x81\x11a\x14\x9BW[a\x14\x90\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0W\x80\xF3[P=a\x14\x86V[`@\x90`\x03\x19\x01\x12a\x01LW`\x045\x90`$5\x90V[``\x90`\x03\x19\x01\x12a\x01LW`\x045\x90`$5\x90`D5\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x14\xE6W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x14\xE6W`@RV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x14\xE6W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x14\xE6W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x14\xE6W`@RV[\x90\x81` \x91\x03\x12a\x01LWQ\x80\x15\x15\x81\x03a\x01LW\x90V[`@Q\x90a\x15\x97\x82a\x14\xFCV[`A\x82R`e`\xF8\x1B``\x83\x7FThe vault failed to update the r` \x82\x01R\x7Fedemption proxy's share allowanc`@\x82\x01R\x01RV[`\0[\x83\x81\x10a\x16\x06WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x15\xF6V[\x90` \x91a\x16/\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x15\xF3V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x92\x90\x91\x92`\x01`\x01`\xA0\x1B\x03\x93`\0\x85\x81T\x16`@\x91\x82Q\x94\x85\x92\x89cp\xA0\x821`\xE0\x1B\x92\x83\x86R\x16\x90\x81`\x04\x86\x01R\x84`$` \x96\x87\x93Z\xFA\x96\x87\x15a\x18mW\x83\x97a\x18>W[P\x83\x90`$\x88\x9B`\x01T\x16\x93\x87Q\x94\x85\x93\x84\x92\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a\x183W\x80\x92a\x18\x03W[PP\x82Q\x7Fasset.balanceOf(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x82\x01R\x87Q\x91\x97\x90\x95\x83\x82\x01\x95\x89\x93a\x16\xF0\x81`0\x8B\x01\x8Aa\x15\xF3V[\x88\x01\x90b\x05$\x05`\xEB\x1B\x90\x81`0\x84\x01R\x80Q\x87\x82\x01\x93\x81`3\x82\x01\x90a\x17\x17\x91\x87a\x15\xF3V[\x01\x93\x8A`)`\xF8\x1B\x95\x86`3\x82\x01R\x03`\x14\x81\x01\x8CR`4\x01a\x17:\x90\x8Ca\x15PV[\x88Q\x9A\x89\x8CR\x89\x8C\x01a\x17L\x91a\x16\x16V[\x90\x88\x8C\x01R\x8A\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x9B\x8C\x92\x03\x90\xA1\x87Q\x98\x89\x95\x88\x87\x01\x7Fvault.balanceOf(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90RQ\x90\x81`0\x88\x01a\x17\xB4\x92a\x15\xF3V[\x85\x01\x91`0\x83\x01RQ\x91\x82`3\x83\x01a\x17\xCC\x92a\x15\xF3V[\x01\x90`3\x82\x01R\x03`\x14\x81\x01\x85R`4\x01a\x17\xE7\x90\x85a\x15PV[\x82Q\x93\x83\x85\x94\x85R\x84\x01a\x17\xFA\x91a\x16\x16V[\x91\x83\x01R\x03\x90\xA1V[\x90\x91P\x82\x82\x81=\x83\x11a\x18,W[a\x18\x1B\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ8\x80a\x16\xADV[P=a\x18\x11V[\x84Q\x90=\x90\x82>=\x90\xFD[\x90\x96P\x83\x81\x81=\x83\x11a\x18fW[a\x18V\x81\x83a\x15PV[\x81\x01\x03\x12a\x01aWQ\x95\x83a\x16\x83V[P=a\x18LV[\x85Q=\x85\x82>=\x90\xFD[`\x03\x90\x06\x80\x15a\x18\xB8W`\x01\x14a\x18\xA0Ws\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\x90V[s\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\x90V[P0\x90V[\x90\x80\x15a\x01LW`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16\x92`@\x92\x83Q\x91cl\x82\xBB\xBF`\xE1\x1B\x83R\x16\x80`\x04\x83\x01R` \x92\x83\x83`$\x81\x89Z\xFA\x92\x83\x15a\x1A\xA2W`\0\x93a\x1A\xADW[P\x82\x15a\x01LW\x84Q\x91cp\xA0\x821`\xE0\x1B\x83R`\x04\x83\x01R\x83\x82`$\x81\x89Z\xFA\x91\x82\x15a\x1A\xA2W`\0\x92a\x1AsW[P\x81\x15a\x01LWa\x19M\x92a\x19H\x91a\x1E\x8BV[a\x1E\x8BV[\x82Qc\x03\xD1h\x9D`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x93\x82\x82`$\x81\x84Z\xFA\x91\x82\x15a\x1A5W\x90\x83\x91`\0\x93a\x1A@W[P\x90`$\x91\x85Q\x92\x83\x80\x92c&mj\x83`\xE1\x1B\x82R\x89`\x04\x83\x01RZ\xFA\x80\x15a\x1A5W\x90\x83\x91a\x1A\x0CW[PP\x15a\x01LW\x81\x83`\x80\x92`\x1C\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x95Q\x93\x80\x85R\x84\x01R\x7FShares to use in redemption:\0\0\0\0``\x84\x01R\x82\x01R\xA1\x90V[\x81=\x83\x11a\x1A.W[a\x1A\x1F\x81\x83a\x15PV[\x81\x01\x03\x12a\x01LW\x818a\x19\xA7V[P=a\x1A\x15V[\x84Q=`\0\x82>=\x90\xFD[\x91\x82\x81\x94\x92\x94=\x83\x11a\x1AlW[a\x1AX\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x90\x82\x90`$a\x19|V[P=a\x1ANV[\x90\x91\x84\x82\x81=\x83\x11a\x1A\x9BW[a\x1A\x8A\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x908a\x194V[P=a\x1A\x80V[\x85Q=`\0\x82>=\x90\xFD[\x90\x92\x84\x82\x81=\x83\x11a\x1A\xD5W[a\x1A\xC4\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x918a\x19\x04V[P=a\x1A\xBAV[\x90`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16\x92`@\x92\x83Q\x91c\xCE\x96\xCBw`\xE0\x1B\x83R\x16\x90\x81`\x04\x82\x01R` \x92\x83\x82`$\x81\x89Z\xFA\x91\x82\x15a\x1A\xA2W`\0\x92a\x1D\x1CW[P\x81\x15a\x01LW\x84Qcp\xA0\x821`\xE0\x1B\x80\x82R`\x04\x82\x01\x85\x90R\x90\x93\x85\x85`$\x81\x8BZ\xFA\x94\x85\x15a\x1C\xE2W`\0\x95a\x1C\xEDW[P\x84\x15a\x01LW\x86Q\x92c\n(\xA4w`\xE0\x1B\x84R`\x04\x84\x01R\x85\x83`$\x81\x8BZ\xFA\x92\x83\x15a\x1C\xE2W`\0\x93a\x1C\xB3W[P\x86Q\x91\x82R`\x04\x82\x01R\x84\x81`$\x81\x8AZ\xFA\x90\x81\x15a\x1C\xA8W`\0\x91a\x1CyW[Pa\x1B\xAE\x91a\x1E\x8BV[\x91\x82\x11a\x01LW\x82`$\x95\x85Q\x96\x87\x80\x92c&mj\x83`\xE1\x1B\x82R\x86`\x04\x83\x01RZ\xFA\x94\x85\x15a\x1A5W`\0\x95a\x1CJW[P\x84\x11a\x01LW\x15a\x01LW\x81\x83`\x80\x92`\x1A\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x95Q\x93\x80\x85R\x84\x01R\x7FTokens to use in withdraw:\0\0\0\0\0\0``\x84\x01R\x82\x01R\xA1\x90V[\x90\x94\x83\x82\x81=\x83\x11a\x1CrW[a\x1Ca\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x938a\x1B\xE0V[P=a\x1CWV[\x90\x85\x82\x81=\x83\x11a\x1C\xA1W[a\x1C\x8F\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQa\x1B\xAEa\x1B\xA4V[P=a\x1C\x85V[\x86Q=`\0\x82>=\x90\xFD[\x90\x92\x86\x82\x81=\x83\x11a\x1C\xDBW[a\x1C\xCA\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x918a\x1B\x82V[P=a\x1C\xC0V[\x87Q=`\0\x82>=\x90\xFD[\x90\x94\x86\x82\x81=\x83\x11a\x1D\x15W[a\x1D\x04\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x938a\x1BRV[P=a\x1C\xFAV[\x90\x91\x84\x82\x81=\x83\x11a\x1DDW[a\x1D3\x81\x83a\x15PV[\x81\x01\x03\x12a\x02\0WPQ\x908a\x1B\x1EV[P=a\x1D)V[a\x1D\x84\x7F\xEB\x03\xCA\x8C\x87\xC7\x84\x9B\xEF\x8FT\xCF\xDD,k\x96{'4\xFE\x87/u\x19x\xC3K\xB9\x1E\x13\xD3Q\x91`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x16\x16V[\x03\x90\xA1cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x81\x81\x11a\x1D\xA9WPPPV[\x7Fb\xBD\xDA\x9A\x05\xCD\xBC\xDB\xF9\x05\xCB\xAD\x99\xC6\xEB\xDC\t\x8Bo\t3\xD8\xF2\xEB<\xFA\xB7@\x0B`%\x14\x93P`;a\x1D\xE3a\x1D\xDDa\x1Ew\x93a\x1F\x9EV[\x93a\x1F\x9EV[`@Q\x94\x85\x91` \x95h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x87\x85\x01Ra\x1E\x10\x81Q\x80\x92\x89`)\x88\x01\x91\x01a\x15\xF3V[\x83\x01`\x1F`\xF9\x1B`)\x82\x01Ra\x1E/\x82Q\x80\x93\x89`*\x85\x01\x91\x01a\x15\xF3V[\x01\x7F failed, reason: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`*\x82\x01Ra\x1Eh\x82Q\x80\x93\x88\x87\x85\x01\x91\x01a\x15\xF3V[\x01\x03`\x1B\x81\x01\x85R\x01\x83a\x15PV[a\x1D\x84`@Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x16\x16V[\x90\x80\x82\x11a\x1E\x97WP\x90V[`\x01\x81\x01\x80\x91\x11a\x1F\x88W\x80\x15a\x1FrWa\x1Fla\x1E\xD8\x7F\xA9^n*\x18$\x11\xE7\xA6\xF9\xED\x11J\x85\xC3v\x1D\x87\xF9\xB8\xF4S\xD8B\xC7\x125\xAAd\xFF\xF9\x9F\x92\x84\x06\x93a\x1F\x9EV[a\x1FX`3a\x1E\xE6\x86a\x1F\x9EV[\x92`@Q\x93\x84\x91\x7FClamping value \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01Ra\x1F'\x81Q\x80\x92` `/\x87\x01\x91\x01a\x15\xF3V[\x82\x01c\x01\x03\xA3y`\xE5\x1B`/\x82\x01Ra\x1FI\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x15\xF3V[\x01\x03`\x13\x81\x01\x84R\x01\x82a\x15PV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x16\x16V[\x03\x90\xA1\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\x1F\xB4W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \x1Cm\xD4\xF74j\xED\x8E pH`\x9E\x92's/\xEC\xA7\xBDg\xEA*\x91)h\xFC\x06\xA4\x80\x9D\x16dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static CRYTICERC4626REDEEMUSINGAPPROVAL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CryticERC4626RedeemUsingApproval<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CryticERC4626RedeemUsingApproval<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CryticERC4626RedeemUsingApproval<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CryticERC4626RedeemUsingApproval<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CryticERC4626RedeemUsingApproval<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CryticERC4626RedeemUsingApproval))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CryticERC4626RedeemUsingApproval<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CRYTICERC4626REDEEMUSINGAPPROVAL_ABI.clone(),
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
                CRYTICERC4626REDEEMUSINGAPPROVAL_ABI.clone(),
                CRYTICERC4626REDEEMUSINGAPPROVAL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `deposit` (0xe2bbb158) function
        pub fn deposit(
            &self,
            assets: ::ethers::core::types::U256,
            receiver_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 187, 177, 88], (assets, receiver_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositForSelfSimple` (0x2723f9ee) function
        pub fn deposit_for_self_simple(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 35, 249, 238], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x1b2ef1ca) function
        pub fn mint(
            &self,
            shares: ::ethers::core::types::U256,
            receiver_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 46, 241, 202], (shares, receiver_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintAsset` (0xb88dab32) function
        pub fn mint_asset(
            &self,
            assets: ::ethers::core::types::U256,
            receiver_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 171, 50], (assets, receiver_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recognizeLossProxy` (0x96eb27a1) function
        pub fn recognize_loss_proxy(
            &self,
            loss: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 235, 39, 161], loss)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recognizeProfitProxy` (0xccc94ae9) function
        pub fn recognize_profit_proxy(
            &self,
            profit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 201, 74, 233], profit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeem` (0xb8192205) function
        pub fn redeem(
            &self,
            shares: ::ethers::core::types::U256,
            owner_id: ::ethers::core::types::U256,
            receiver_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 25, 34, 5], (shares, owner_id, receiver_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeemForSelfSimple` (0xa815c10f) function
        pub fn redeem_for_self_simple(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([168, 21, 193, 15], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_redeemRequiresTokenApproval` (0x6f89dd2e) function
        pub fn verify_redeem_requires_token_approval(
            &self,
            receiver_id: ::ethers::core::types::U256,
            shares: ::ethers::core::types::U256,
            shares_approved: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 137, 221, 46], (receiver_id, shares, shares_approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_redeemViaApprovalProxy` (0xaaeb4203) function
        pub fn verify_redeem_via_approval_proxy(
            &self,
            receiver_id: ::ethers::core::types::U256,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([170, 235, 66, 3], (receiver_id, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_withdrawRequiresTokenApproval` (0x480fef6a) function
        pub fn verify_withdraw_requires_token_approval(
            &self,
            receiver_id: ::ethers::core::types::U256,
            tokens: ::ethers::core::types::U256,
            shares_approved: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 15, 239, 106], (receiver_id, tokens, shares_approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_withdrawViaApprovalProxy` (0xc21eda6f) function
        pub fn verify_withdraw_via_approval_proxy(
            &self,
            receiver_id: ::ethers::core::types::U256,
            tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([194, 30, 218, 111], (receiver_id, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0xa41fe49f) function
        pub fn withdraw(
            &self,
            assets: ::ethers::core::types::U256,
            owner_id: ::ethers::core::types::U256,
            receiver_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 31, 228, 159], (assets, owner_id, receiver_id))
                .expect("method not found (this should never happen)")
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
            CryticERC4626RedeemUsingApprovalEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CryticERC4626RedeemUsingApproval<M> {
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
    pub enum CryticERC4626RedeemUsingApprovalEvents {
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
    impl ::ethers::contract::EthLogDecode for CryticERC4626RedeemUsingApprovalEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssertEqFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626RedeemUsingApprovalEvents::AssertEqFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626RedeemUsingApprovalEvents::AssertFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertGtFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626RedeemUsingApprovalEvents::AssertGtFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertGteFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626RedeemUsingApprovalEvents::AssertGteFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertLtFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626RedeemUsingApprovalEvents::AssertLtFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertLteFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626RedeemUsingApprovalEvents::AssertLteFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertNeqFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626RedeemUsingApprovalEvents::AssertNeqFailFilter(decoded),
                );
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(
                    CryticERC4626RedeemUsingApprovalEvents::LogAddressFilter(decoded),
                );
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(
                    CryticERC4626RedeemUsingApprovalEvents::LogStringFilter(decoded),
                );
            }
            if let Ok(decoded) = LogUint256Filter::decode_log(log) {
                return Ok(
                    CryticERC4626RedeemUsingApprovalEvents::LogUint256Filter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CryticERC4626RedeemUsingApprovalEvents {
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
    impl ::core::convert::From<AssertEqFailFilter>
    for CryticERC4626RedeemUsingApprovalEvents {
        fn from(value: AssertEqFailFilter) -> Self {
            Self::AssertEqFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertFailFilter>
    for CryticERC4626RedeemUsingApprovalEvents {
        fn from(value: AssertFailFilter) -> Self {
            Self::AssertFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGtFailFilter>
    for CryticERC4626RedeemUsingApprovalEvents {
        fn from(value: AssertGtFailFilter) -> Self {
            Self::AssertGtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGteFailFilter>
    for CryticERC4626RedeemUsingApprovalEvents {
        fn from(value: AssertGteFailFilter) -> Self {
            Self::AssertGteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLtFailFilter>
    for CryticERC4626RedeemUsingApprovalEvents {
        fn from(value: AssertLtFailFilter) -> Self {
            Self::AssertLtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLteFailFilter>
    for CryticERC4626RedeemUsingApprovalEvents {
        fn from(value: AssertLteFailFilter) -> Self {
            Self::AssertLteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertNeqFailFilter>
    for CryticERC4626RedeemUsingApprovalEvents {
        fn from(value: AssertNeqFailFilter) -> Self {
            Self::AssertNeqFailFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter>
    for CryticERC4626RedeemUsingApprovalEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter>
    for CryticERC4626RedeemUsingApprovalEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUint256Filter>
    for CryticERC4626RedeemUsingApprovalEvents {
        fn from(value: LogUint256Filter) -> Self {
            Self::LogUint256Filter(value)
        }
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(uint256,uint256)` and selector `0xe2bbb158`
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
    #[ethcall(name = "deposit", abi = "deposit(uint256,uint256)")]
    pub struct DepositCall {
        pub assets: ::ethers::core::types::U256,
        pub receiver_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `depositForSelfSimple` function with signature `depositForSelfSimple(uint256)` and selector `0x2723f9ee`
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
    #[ethcall(name = "depositForSelfSimple", abi = "depositForSelfSimple(uint256)")]
    pub struct DepositForSelfSimpleCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint(uint256,uint256)` and selector `0x1b2ef1ca`
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
    #[ethcall(name = "mint", abi = "mint(uint256,uint256)")]
    pub struct MintCall {
        pub shares: ::ethers::core::types::U256,
        pub receiver_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mintAsset` function with signature `mintAsset(uint256,uint256)` and selector `0xb88dab32`
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
    #[ethcall(name = "mintAsset", abi = "mintAsset(uint256,uint256)")]
    pub struct MintAssetCall {
        pub assets: ::ethers::core::types::U256,
        pub receiver_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `recognizeLossProxy` function with signature `recognizeLossProxy(uint256)` and selector `0x96eb27a1`
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
    #[ethcall(name = "recognizeLossProxy", abi = "recognizeLossProxy(uint256)")]
    pub struct RecognizeLossProxyCall {
        pub loss: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `recognizeProfitProxy` function with signature `recognizeProfitProxy(uint256)` and selector `0xccc94ae9`
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
    #[ethcall(name = "recognizeProfitProxy", abi = "recognizeProfitProxy(uint256)")]
    pub struct RecognizeProfitProxyCall {
        pub profit: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `redeem` function with signature `redeem(uint256,uint256,uint256)` and selector `0xb8192205`
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
    #[ethcall(name = "redeem", abi = "redeem(uint256,uint256,uint256)")]
    pub struct RedeemCall {
        pub shares: ::ethers::core::types::U256,
        pub owner_id: ::ethers::core::types::U256,
        pub receiver_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `redeemForSelfSimple` function with signature `redeemForSelfSimple(uint256)` and selector `0xa815c10f`
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
    #[ethcall(name = "redeemForSelfSimple", abi = "redeemForSelfSimple(uint256)")]
    pub struct RedeemForSelfSimpleCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_redeemRequiresTokenApproval` function with signature `verify_redeemRequiresTokenApproval(uint256,uint256,uint256)` and selector `0x6f89dd2e`
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
        name = "verify_redeemRequiresTokenApproval",
        abi = "verify_redeemRequiresTokenApproval(uint256,uint256,uint256)"
    )]
    pub struct VerifyRedeemRequiresTokenApprovalCall {
        pub receiver_id: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
        pub shares_approved: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_redeemViaApprovalProxy` function with signature `verify_redeemViaApprovalProxy(uint256,uint256)` and selector `0xaaeb4203`
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
        name = "verify_redeemViaApprovalProxy",
        abi = "verify_redeemViaApprovalProxy(uint256,uint256)"
    )]
    pub struct VerifyRedeemViaApprovalProxyCall {
        pub receiver_id: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_withdrawRequiresTokenApproval` function with signature `verify_withdrawRequiresTokenApproval(uint256,uint256,uint256)` and selector `0x480fef6a`
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
        name = "verify_withdrawRequiresTokenApproval",
        abi = "verify_withdrawRequiresTokenApproval(uint256,uint256,uint256)"
    )]
    pub struct VerifyWithdrawRequiresTokenApprovalCall {
        pub receiver_id: ::ethers::core::types::U256,
        pub tokens: ::ethers::core::types::U256,
        pub shares_approved: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_withdrawViaApprovalProxy` function with signature `verify_withdrawViaApprovalProxy(uint256,uint256)` and selector `0xc21eda6f`
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
        name = "verify_withdrawViaApprovalProxy",
        abi = "verify_withdrawViaApprovalProxy(uint256,uint256)"
    )]
    pub struct VerifyWithdrawViaApprovalProxyCall {
        pub receiver_id: ::ethers::core::types::U256,
        pub tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256,uint256,uint256)` and selector `0xa41fe49f`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,uint256,uint256)")]
    pub struct WithdrawCall {
        pub assets: ::ethers::core::types::U256,
        pub owner_id: ::ethers::core::types::U256,
        pub receiver_id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CryticERC4626RedeemUsingApprovalCalls {
        Deposit(DepositCall),
        DepositForSelfSimple(DepositForSelfSimpleCall),
        Mint(MintCall),
        MintAsset(MintAssetCall),
        RecognizeLossProxy(RecognizeLossProxyCall),
        RecognizeProfitProxy(RecognizeProfitProxyCall),
        Redeem(RedeemCall),
        RedeemForSelfSimple(RedeemForSelfSimpleCall),
        VerifyRedeemRequiresTokenApproval(VerifyRedeemRequiresTokenApprovalCall),
        VerifyRedeemViaApprovalProxy(VerifyRedeemViaApprovalProxyCall),
        VerifyWithdrawRequiresTokenApproval(VerifyWithdrawRequiresTokenApprovalCall),
        VerifyWithdrawViaApprovalProxy(VerifyWithdrawViaApprovalProxyCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for CryticERC4626RedeemUsingApprovalCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <DepositForSelfSimpleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DepositForSelfSimple(decoded));
            }
            if let Ok(decoded)
                = <MintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded)
                = <MintAssetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MintAsset(decoded));
            }
            if let Ok(decoded)
                = <RecognizeLossProxyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RecognizeLossProxy(decoded));
            }
            if let Ok(decoded)
                = <RecognizeProfitProxyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RecognizeProfitProxy(decoded));
            }
            if let Ok(decoded)
                = <RedeemCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Redeem(decoded));
            }
            if let Ok(decoded)
                = <RedeemForSelfSimpleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RedeemForSelfSimple(decoded));
            }
            if let Ok(decoded)
                = <VerifyRedeemRequiresTokenApprovalCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyRedeemRequiresTokenApproval(decoded));
            }
            if let Ok(decoded)
                = <VerifyRedeemViaApprovalProxyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyRedeemViaApprovalProxy(decoded));
            }
            if let Ok(decoded)
                = <VerifyWithdrawRequiresTokenApprovalCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyWithdrawRequiresTokenApproval(decoded));
            }
            if let Ok(decoded)
                = <VerifyWithdrawViaApprovalProxyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyWithdrawViaApprovalProxy(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CryticERC4626RedeemUsingApprovalCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositForSelfSimple(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MintAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecognizeLossProxy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecognizeProfitProxy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Redeem(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RedeemForSelfSimple(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyRedeemRequiresTokenApproval(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyRedeemViaApprovalProxy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyWithdrawRequiresTokenApproval(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyWithdrawViaApprovalProxy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CryticERC4626RedeemUsingApprovalCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositForSelfSimple(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecognizeLossProxy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RecognizeProfitProxy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Redeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedeemForSelfSimple(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyRedeemRequiresTokenApproval(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyRedeemViaApprovalProxy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyWithdrawRequiresTokenApproval(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyWithdrawViaApprovalProxy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositCall> for CryticERC4626RedeemUsingApprovalCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositForSelfSimpleCall>
    for CryticERC4626RedeemUsingApprovalCalls {
        fn from(value: DepositForSelfSimpleCall) -> Self {
            Self::DepositForSelfSimple(value)
        }
    }
    impl ::core::convert::From<MintCall> for CryticERC4626RedeemUsingApprovalCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<MintAssetCall> for CryticERC4626RedeemUsingApprovalCalls {
        fn from(value: MintAssetCall) -> Self {
            Self::MintAsset(value)
        }
    }
    impl ::core::convert::From<RecognizeLossProxyCall>
    for CryticERC4626RedeemUsingApprovalCalls {
        fn from(value: RecognizeLossProxyCall) -> Self {
            Self::RecognizeLossProxy(value)
        }
    }
    impl ::core::convert::From<RecognizeProfitProxyCall>
    for CryticERC4626RedeemUsingApprovalCalls {
        fn from(value: RecognizeProfitProxyCall) -> Self {
            Self::RecognizeProfitProxy(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for CryticERC4626RedeemUsingApprovalCalls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<RedeemForSelfSimpleCall>
    for CryticERC4626RedeemUsingApprovalCalls {
        fn from(value: RedeemForSelfSimpleCall) -> Self {
            Self::RedeemForSelfSimple(value)
        }
    }
    impl ::core::convert::From<VerifyRedeemRequiresTokenApprovalCall>
    for CryticERC4626RedeemUsingApprovalCalls {
        fn from(value: VerifyRedeemRequiresTokenApprovalCall) -> Self {
            Self::VerifyRedeemRequiresTokenApproval(value)
        }
    }
    impl ::core::convert::From<VerifyRedeemViaApprovalProxyCall>
    for CryticERC4626RedeemUsingApprovalCalls {
        fn from(value: VerifyRedeemViaApprovalProxyCall) -> Self {
            Self::VerifyRedeemViaApprovalProxy(value)
        }
    }
    impl ::core::convert::From<VerifyWithdrawRequiresTokenApprovalCall>
    for CryticERC4626RedeemUsingApprovalCalls {
        fn from(value: VerifyWithdrawRequiresTokenApprovalCall) -> Self {
            Self::VerifyWithdrawRequiresTokenApproval(value)
        }
    }
    impl ::core::convert::From<VerifyWithdrawViaApprovalProxyCall>
    for CryticERC4626RedeemUsingApprovalCalls {
        fn from(value: VerifyWithdrawViaApprovalProxyCall) -> Self {
            Self::VerifyWithdrawViaApprovalProxy(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for CryticERC4626RedeemUsingApprovalCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `verify_redeemViaApprovalProxy` function with signature `verify_redeemViaApprovalProxy(uint256,uint256)` and selector `0xaaeb4203`
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
    pub struct VerifyRedeemViaApprovalProxyReturn {
        pub tokens_withdrawn: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `verify_withdrawViaApprovalProxy` function with signature `verify_withdrawViaApprovalProxy(uint256,uint256)` and selector `0xc21eda6f`
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
    pub struct VerifyWithdrawViaApprovalProxyReturn {
        pub shares_burned: ::ethers::core::types::U256,
    }
}
