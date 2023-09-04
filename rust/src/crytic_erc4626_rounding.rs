pub use crytic_erc4626_rounding::*;
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
pub mod crytic_erc4626_rounding {
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
                    ::std::borrow::ToOwned::to_owned("verify_convertRoundTrip"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_convertRoundTrip",
                            ),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("verify_convertRoundTrip2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_convertRoundTrip2",
                            ),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned(
                        "verify_convertToAssetsRoundingDirection",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_convertToAssetsRoundingDirection",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "verify_convertToSharesRoundingDirection",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_convertToSharesRoundingDirection",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verify_depositRoundingDirection"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_depositRoundingDirection",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verify_mintRoundingDirection"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_mintRoundingDirection",
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
                        "verify_previewDepositRoundingDirection",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_previewDepositRoundingDirection",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "verify_previewMintRoundingDirection",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_previewMintRoundingDirection",
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
                        "verify_previewRedeemRoundingDirection",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_previewRedeemRoundingDirection",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "verify_previewWithdrawRoundingDirection",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_previewWithdrawRoundingDirection",
                            ),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verify_redeemRoundingDirection"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_redeemRoundingDirection",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verify_withdrawRoundingDirection"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_withdrawRoundingDirection",
                            ),
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
                            ],
                            outputs: ::std::vec![],
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
    pub static CRYTICERC4626ROUNDING_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x19\xBE\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x04\xEF\x1FC\x14a\x12\xD3WP\x80c\x1B.\xF1\xCA\x14a\x12@W\x80c'#\xF9\xEE\x14a\x10\xB3W\x80c-\xFAF\n\x14a\x0F\xC9W\x80c7\x1Bx\xCD\x14a\x0E\xE0W\x80cM.\xDE\xC8\x14a\r\xECW\x80cr\x84Ji\x14a\r7W\x80c\x7F\xBB\xB3~\x14a\x0CXW\x80c\x81&\x05\\\x14a\n\xDAW\x80c\x94,\xF0\xFF\x14a\t\xDAW\x83\x81c\x96\xEB'\xA1\x14a\t\x1BWP\x80c\x97\x98\xBA\x90\x14a\x081W\x80c\x9BQ\xDB\xC4\x14a\x06\x9BW\x80c\xA4\x1F\xE4\x9F\x14a\x06@W\x80c\xA8\x15\xC1\x0F\x14a\x05,W\x80c\xB8\x19\"\x05\x14a\x04\xC0W\x83\x81c\xB8\x8D\xAB2\x14a\x04FWP\x80c\xBC\x1B\x9Dh\x14a\x03GW\x83\x81c\xCC\xC9J\xE9\x14a\x02bWP\x80c\xE2\xBB\xB1X\x14a\x01\xC7Wc\xF2\x84O\x1F\x14a\x01\x11W`\0\x80\xFD[4a\x01\xC3W\x82`\x03\x196\x01\x12a\x01\xC3W`\xFF\x82T`\xA0\x1C\x16\x15a\x01\xC3W` \x83`D`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x82\x85Q\x96\x87\x94\x85\x93cnU?e`\xE0\x1B\x85R\x84\x01R0`$\x84\x01RZ\xF1\x90\x81\x15a\x01\xBAWP\x82\x90a\x01\x82W[a\x01\x7F\x91Pa\x01ya\x14\xA1V[\x90a\x15zV[\x80\xF3[P` \x81=\x82\x11a\x01\xB2W[\x81a\x01\x9B` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x90Qa\x01lV[`\0\x80\xFD[=\x91Pa\x01\x8EV[Q=\x84\x82>=\x90\xFD[\x82\x80\xFD[P4a\x01\xC3W` a\x01\xE7a\x02&\x93a\x01\xDF6a\x13\xEDV[\x92\x90\x92a\x14\xE5V[\x91\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93cnU?e`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x90\x81\x15a\x01\xBAWPa\x02:WP\x80\xF3[` \x90\x81=\x81\x11a\x02[W[a\x02P\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x80\xF3[P=a\x02FV[\x80\x84\x844a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\xFF\x82T`\xA0\x1C\x16\x15a\x03CW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x039W\x85\x91a\x03\x05W[P\x15a\x03\0W\x82;\x15a\x03\0W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cU\xDFp\r`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01\xBAWPa\x02\xEDWP\xF3[a\x02\xF6\x90a\x14\x1DV[a\x02\xFDW\x80\xF3[\x80\xFD[PPP\xFD[\x94PP` \x84=\x82\x11a\x031W[\x81a\x03 ` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x84\x93Q\x86a\x02\xB2V[=\x91Pa\x03\x13V[\x83Q=\x87\x82>=\x90\xFD[PP\xFD[P4a\x01\xC3W\x82`\x03\x196\x01\x12a\x01\xC3W`\xFF\x82T`\xA0\x1C\x16\x15a\x01\xC3W` \x83`$`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x84Q\x95\x86\x93\x84\x92c\x03\xD1h\x9D`\xE1\x1B\x84R\x83\x01RZ\xFA\x80\x15a\x04<W\x83\x90a\x04\tW[a\x01\x7F\x92P\x7F assets to be withdrawn at no co\x82Q\x92a\x03\xCC\x84a\x14GV[`B\x84R\x7FconvertToAssets() must not allow` \x85\x01R\x83\x01Ra\x1C\xDD`\xF2\x1B``\x83\x01Ra\x15zV[P` \x82=\x82\x11a\x044W[\x81a\x04\"` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x91Qa\x03\x9AV[=\x91Pa\x04\x15V[\x81Q=\x85\x82>=\x90\xFD[\x80\x84\x844a\x03CWa\x04c\x91a\x04[6a\x13\xEDV[\x93\x90\x93a\x14\xE5V[\x92`\x01`\x01`\xA0\x1B\x03\x85T\x16\x80;\x15a\x04\xBCW\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x92\x85\x01\x92\x83R` \x83\x01\x91\x90\x91R\x84\x91\x84\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x01\xBAWPa\x02\xEDWP\xF3[\x85\x80\xFD[P4a\x01\xC3W` a\x04\xE3a\x04\xE9\x93a\x04\xD86a\x14\x03V[\x96\x91\x96\x93\x90\x93a\x14\xE5V[\x92a\x14\xE5V[`\x01T\x85Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\x02&V[P\x904a\x01\xC3W` \x91\x82`\x03\x196\x01\x12a\x06<W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qcp\xA0\x821`\xE0\x1B\x81R0\x82\x82\x01R\x84\x81`$\x81\x87Z\xFA\x90\x81\x15a\x062W\x90\x85\x92\x91\x87\x91a\x05\xFFW[P\x93a\x05\x89a\x05\xBA\x95\x835a\x18\nV[\x84Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x90\x81R0` \x82\x01\x81\x90R`@\x82\x01R\x91\x94\x85\x92\x83\x91\x89\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x90\x81\x15a\x05\xF6WPa\x05\xCEW\x82\x80\xF3[\x81=\x83\x11a\x05\xEFW[a\x05\xE1\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW8\x80\x82\x80\xF3[P=a\x05\xD7V[Q=\x85\x82>=\x90\xFD[\x83\x81\x94\x92P=\x83\x11a\x06+W[a\x06\x16\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x90Q\x84\x91\x90a\x05\x89a\x05yV[P=a\x06\x0CV[\x83Q=\x88\x82>=\x90\xFD[\x83\x80\xFD[P4a\x01\xC3W` a\x04\xE3a\x06X\x93a\x04\xD86a\x14\x03V[`\x01T\x85Qc-\x18+\xE5`\xE2\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\x02&V[P\x904a\x01\xC3W` \x90\x81`\x03\x196\x01\x12a\x06<W\x805`\xFF\x82T`\xA0\x1C\x16\x15a\x08-W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x84Q\x90c\x03\xD1h\x9D`\xE1\x1B\x82R\x82\x81\x83\x01R\x84\x82`$\x81\x87Z\xFA\x90\x81\x15a\x08#W\x85\x92\x88\x92a\x07\xF1W[P`$\x90\x87Q\x95\x86\x93\x84\x92ccsz\xC9`\xE1\x1B\x84R\x83\x01RZ\xFA\x91\x82\x15a\x07\xE7W\x85\x92a\x07\x96W[P\x7FconvertTo round trip (withdraw, \x84\x7FA profit was extractable from a a\x01\x7F\x96Q\x95a\x07q\x87a\x14GV[`M\x87R\x86\x01R\x84\x01Rlthen deposit)`\x98\x1B``\x84\x01Ra\x16gV[\x93\x91P\x82\x84\x81=\x83\x11a\x07\xE0W[a\x07\xAE\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x92Q\x90\x92\x7FconvertTo round trip (withdraw, a\x07\x1EV[P=a\x07\xA4V[\x84Q=\x87\x82>=\x90\xFD[\x83\x81\x94\x92\x93P=\x83\x11a\x08\x1CW[a\x08\t\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x90Q\x84\x91`$a\x06\xF6V[P=a\x07\xFFV[\x86Q=\x89\x82>=\x90\xFD[\x84\x80\xFD[P4a\x01\xC3W\x82`\x03\x196\x01\x12a\x01\xC3W`\xFF\x82T`\xA0\x1C\x16\x15a\x01\xC3W` \x83`$`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x84Q\x95\x86\x93\x84\x92c\xEF\x8B0\xF7`\xE0\x1B\x84R\x83\x01RZ\xFA\x80\x15a\x04<W\x83\x90a\x08\xE8W[a\x01\x7F\x92P\x7Fhares at no cost\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82Q\x92a\x08\xB6\x84a\x14cV[`0\x84R\x7FpreviewDeposit() must not mint s` \x85\x01R\x83\x01Ra\x15zV[P` \x82=\x82\x11a\t\x13W[\x81a\t\x01` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x91Qa\x08\x84V[=\x91Pa\x08\xF4V[\x80\x84\x844a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\xFF\x82T`\xA0\x1C\x16\x15a\x03CW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x039W\x85\x91a\t\xA6W[P\x15a\x03\0W\x82;\x15a\x03\0W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cb!\xE4\xF1`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01\xBAWPa\x02\xEDWP\xF3[\x94PP` \x84=\x82\x11a\t\xD2W[\x81a\t\xC1` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x84\x93Q\x86a\tkV[=\x91Pa\t\xB4V[P4a\x01\xC3W` \x90\x81`\x03\x196\x01\x12a\x06<W\x825\x92`\xFF\x81T`\xA0\x1C\x16\x15a\x08-W\x83\x15a\x08-W`\x01T\x82Qc-\x18+\xE5`\xE2\x1B\x81R\x91\x82\x01\x94\x85R0` \x86\x01\x81\x90R`@\x86\x01R\x90\x93\x83\x91\x85\x91\x82\x90\x03``\x01\x90\x82\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x92\x83\x15a\n\xD0W\x84\x93a\n\x9AW[P\x91cfree`\xE0\x1B\x83\x7FToken must not be withdrawn for a\x01\x7F\x95Q\x94a\n\x8B\x86a\x14cV[`$\x86R\x85\x01R\x83\x01Ra\x17:V[\x92P\x81\x83\x81=\x83\x11a\n\xC9W[a\n\xB1\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x91Q\x91cfree`\xE0\x1Ba\nPV[P=a\n\xA7V[\x81Q=\x86\x82>=\x90\xFD[P\x904a\x01\xC3W` \x90\x81`\x03\x196\x01\x12a\x06<W\x805`\xFF\x82T`\xA0\x1C\x16\x15a\x08-W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x84Q\x90ccsz\xC9`\xE1\x1B\x82R\x82\x81\x83\x01R\x84\x82`$\x81\x87Z\xFA\x90\x81\x15a\x08#W\x85\x92\x88\x92a\x0C&W[P`$\x90\x87Q\x95\x86\x93\x84\x92c\x03\xD1h\x9D`\xE1\x1B\x84R\x83\x01RZ\xFA\x91\x82\x15a\x07\xE7W\x85\x92a\x0B\xD5W[P\x7FconvertTo round trip (deposit, t\x84\x7FA profit was extractable from a a\x01\x7F\x96Q\x95a\x0B\xB0\x87a\x14GV[`M\x87R\x86\x01R\x84\x01Rlhen withdraw)`\x98\x1B``\x84\x01Ra\x16gV[\x93\x91P\x82\x84\x81=\x83\x11a\x0C\x1FW[a\x0B\xED\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x92Q\x90\x92\x7FconvertTo round trip (deposit, ta\x0B]V[P=a\x0B\xE3V[\x83\x81\x94\x92\x93P=\x83\x11a\x0CQW[a\x0C>\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x90Q\x84\x91`$a\x0B5V[P=a\x0C4V[P4a\x01\xC3W\x82`\x03\x196\x01\x12a\x01\xC3W`\xFF\x82T`\xA0\x1C\x16\x15a\x01\xC3W` \x83`d`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x82\x85Q\x96\x87\x94\x85\x93c]\x04;)`\xE1\x1B\x85R\x84\x01R0`$\x84\x01R0`D\x84\x01RZ\xF1\x80\x15a\x04<W\x83\x90a\r\x04W[a\x01\x7F\x92Pd free`\xD8\x1B\x82Q\x92a\x0C\xD2\x84a\x14cV[`%\x84R\x7FTokens must not be withdrawn for` \x85\x01R\x83\x01Ra\x15zV[P` \x82=\x82\x11a\r/W[\x81a\r\x1D` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x91Qa\x0C\xB8V[=\x91Pa\r\x10V[P4a\x01\xC3W` 6`\x03\x19\x01\x12a\x01\xC3W\x815`\xFF\x83T`\xA0\x1C\x16\x15a\x06<W\x80\x15a\x06<W`\x01T\x82Qc\x94\xBF\x80M`\xE0\x1B\x81R\x93\x84\x01\x91\x82R0` \x83\x81\x01\x91\x90\x91R\x91\x84\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x90\x87\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x01\xBAWP\x82\x90a\r\xB9W[a\x01\x7F\x91Pa\r\xB3a\x14\xA1V[\x90a\x17:V[P` \x81=\x82\x11a\r\xE4W[\x81a\r\xD2` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x90Qa\r\xA6V[=\x91Pa\r\xC5V[P4a\x01\xC3W` \x90\x81`\x03\x196\x01\x12a\x06<W\x825`\xFF\x84T`\xA0\x1C\x16\x15a\x08-W\x80\x15a\x08-W\x82\x90`$`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x95\x84Q\x96\x87\x93\x84\x92c\xB3\xD7\xF6\xB9`\xE0\x1B\x84R\x83\x01RZ\xFA\x92\x83\x15a\n\xD0W\x84\x93a\x0E\x9FW[P\x91n\x18\\\x99\\\xC8\x18]\x08\x1B\x9B\xC8\x18\xDB\xDC\xDD`\x8A\x1B\x83\x7FpreviewMint() must never mint sha\x01\x7F\x95Q\x94a\x0E\x90\x86a\x14cV[`/\x86R\x85\x01R\x83\x01Ra\x17:V[\x92P\x81\x83\x81=\x83\x11a\x0E\xD9W[a\x0E\xB6\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x91Q\x91n\x18\\\x99\\\xC8\x18]\x08\x1B\x9B\xC8\x18\xDB\xDC\xDD`\x8A\x1Ba\x0EJV[P=a\x0E\xACV[P4a\x01\xC3W\x82`\x03\x196\x01\x12a\x01\xC3W`\xFF\x82T`\xA0\x1C\x16\x15a\x01\xC3W` \x83`$`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x84Q\x95\x86\x93\x84\x92c&mj\x83`\xE1\x1B\x84R\x83\x01RZ\xFA\x80\x15a\x04<W\x83\x90a\x0F\x96W[a\x01\x7F\x92P\x7Fssets to be withdrawn at no cost\x82Q\x92a\x0Fe\x84a\x14cV[\x80\x84R\x7FpreviewRedeem() must not allow a` \x85\x01R\x83\x01Ra\x15zV[P` \x82=\x82\x11a\x0F\xC1W[\x81a\x0F\xAF` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x91Qa\x0F3V[=\x91Pa\x0F\xA2V[P4a\x01\xC3W\x82`\x03\x196\x01\x12a\x01\xC3W`\xFF\x82T`\xA0\x1C\x16\x15a\x01\xC3W` \x83`$`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x84Q\x95\x86\x93\x84\x92ccsz\xC9`\xE1\x1B\x84R\x83\x01RZ\xFA\x80\x15a\x04<W\x83\x90a\x10\x80W[a\x01\x7F\x92P\x7F shares to be minted at no cost\0\x82Q\x92a\x10N\x84a\x14cV[`?\x84R\x7FconvertToShares() must not allow` \x85\x01R\x83\x01Ra\x15zV[P` \x82=\x82\x11a\x10\xABW[\x81a\x10\x99` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x91Qa\x10\x1CV[=\x91Pa\x10\x8CV[P\x82\x904a\x12<W` \x90\x81`\x03\x196\x01\x12a\x01\xC3W\x835\x93`\x01`\x01`\xA0\x1B\x03\x94\x85\x85T\x16\x80;\x15a\x04\xBCW\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x84\x82\x01\x90\x81R` \x81\x01\x84\x90R\x90\x91\x87\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x122Wa\x12\x1DW[P\x85\x84\x82\x87\x98a\x11\\\x97\x98T\x16\x83`\x01T\x16\x8A\x88Q\x80\x9A\x81\x95\x82\x94c\t^\xA7\xB3`\xE0\x1B\x84R\x8B\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x12\x13W\x90\x86\x93\x92\x91a\x11\xD7W[`\x01T\x85QcnU?e`\xE0\x1B\x81R\x93\x84\x01\x92\x83R0` \x84\x01R\x92\x95P\x85\x92\x16\x90\x82\x90\x88\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x05\xF6WPa\x11\xAFW\x82\x80\xF3[\x81=\x83\x11a\x11\xD0W[a\x11\xC2\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x81\x80\x82\x80\xF3[P=a\x11\xB8V[\x92\x85\x81\x96\x92\x96=\x83\x11a\x12\x0CW[a\x11\xEF\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x12\x08WQ\x80\x15\x15\x03a\x04\xBCW\x83\x85\x92a\x11pV[\x86\x80\xFD[P=a\x11\xE5V[\x84Q=\x89\x82>=\x90\xFD[\x94a\x12+a\x11\\\x95\x96a\x14\x1DV[\x94\x93a\x11\x19V[\x84Q=\x88\x82>=\x90\xFD[P\x80\xFD[P4a\x01\xC3W` a\x12Xa\x12\x97\x93a\x01\xDF6a\x13\xEDV[\x91\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93c\x94\xBF\x80M`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x90\x81\x15a\x01\xBAWPa\x12\xABWP\x80\xF3[` \x90\x81=\x81\x11a\x12\xCCW[a\x12\xC1\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x02\xFDW\x80\xF3[P=a\x12\xB7V[\x92\x91\x90P4a\x06<W` \x91\x82`\x03\x196\x01\x12a\x08-W\x805`\xFF\x82T`\xA0\x1C\x16\x15a\x04\xBCW\x80\x15a\x04\xBCW`$\x85\x85\x93\x81\x93`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91c\n(\xA4w`\xE0\x1B\x84R\x83\x01RZ\xFA\x92\x83\x15a\n\xD0W\x84\x93a\x13\x9EW[P\x91\x7F assets to be withdrawn at no co\x83\x7FpreviewWithdraw() must not allowa\x01\x7F\x95Q\x94a\x13\x84\x86a\x14GV[`B\x86R\x85\x01R\x83\x01Ra\x1C\xDD`\xF2\x1B``\x83\x01Ra\x17:V[\x92P\x81\x83\x81=\x83\x11a\x13\xE6W[a\x13\xB5\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x06<W\x91Q\x91\x7F assets to be withdrawn at no coa\x130V[P=a\x13\xABV[`@\x90`\x03\x19\x01\x12a\x01\xADW`\x045\x90`$5\x90V[``\x90`\x03\x19\x01\x12a\x01\xADW`\x045\x90`$5\x90`D5\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x141W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x141W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x141W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x141W`@RV[`@Q\x90a\x14\xAE\x82a\x14cV[`\"\x82Raee`\xF0\x1B`@\x83\x7FShares must not be minted for fr` \x82\x01R\x01RV[`\x03\x90\x06\x80\x15a\x15&W`\x01\x14a\x15\x0EWs\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\x90V[s\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\x90V[P0\x90V[`\0[\x83\x81\x10a\x15>WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x15.V[`@\x91` \x82Ra\x15n\x81Q\x80\x92\x81` \x86\x01R` \x86\x86\x01\x91\x01a\x15+V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x80a\x15\x83WPPV[\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x91a\x16B`5a\x15\xB6a\x16N\x94a\x19JV[a\x15\xBEa\x19\tV[\x93`@Q\x94\x85\x92h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B` \x85\x01Ra\x15\xEB\x81Q\x80\x92` `)\x88\x01\x91\x01a\x15+V[\x83\x01a!=`\xF0\x1B`)\x82\x01Ra\x16\x0C\x82Q\x80\x93` `+\x85\x01\x91\x01a\x15+V[\x01i\x01a\x03\x93+\x0B\x9B{q\xD1`\xB5\x1B`+\x82\x01Ra\x163\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x15+V[\x01\x03`\x15\x81\x01\x84R\x01\x82a\x14\x7FV[`@Q\x91\x82\x91\x82a\x15NV[\x03\x90\xA1cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x81\x81\x10a\x16sWPPPV[\x91a\x16B`;a\x16N\x93a\x16\xB0a\x16\xAA\x7F\x94BN\xD2O\xB3\x968\xB6H\x17\xC77\xDDD?8z\xAA\x14\x86aM\xA4I\xB6hjd-ml\x97a\x19JV[\x91a\x19JV[\x93`@Q\x94\x85\x92h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B` \x85\x01Ra\x16\xDD\x81Q\x80\x92` `)\x88\x01\x91\x01a\x15+V[\x83\x01`\x0F`\xFA\x1B`)\x82\x01Ra\x16\xFD\x82Q\x80\x93` `*\x85\x01\x91\x01a\x15+V[\x01p\x01\x033\x0BKc+!a\x03\x93+\x0B\x9B{q\xD1`}\x1B`*\x82\x01Ra\x17+\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x15+V[\x01\x03`\x1B\x81\x01\x84R\x01\x82a\x14\x7FV[\x80\x15a\x17DWPPV[\x7Fp{\x8CV\xE4\xC2\x11\xCF\x13!\xFA\xEBAH#pb\"\x8D\xB2\xFC\xEC\xC9\xBEH~\x83\xA2h\x0E~P\x91a\x16B`<a\x17wa\x16N\x94a\x19JV[a\x17\x7Fa\x19\tV[\x93`@Q\x94\x85\x92h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B` \x85\x01Ra\x17\xAC\x81Q\x80\x92` `)\x88\x01\x91\x01a\x15+V[\x83\x01a<=`\xF0\x1B`)\x82\x01Ra\x17\xCD\x82Q\x80\x93` `+\x85\x01\x91\x01a\x15+V[\x01p\x01\x033\x0BKc+!a\x03\x93+\x0B\x9B{q\xD1`}\x1B`+\x82\x01Ra\x17\xFB\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x15+V[\x01\x03`\x1C\x81\x01\x84R\x01\x82a\x14\x7FV[\x90\x80\x82\x11a\x18\x16WP\x90V[`\x01\x81\x01\x80\x91\x11a\x18\xF3W\x80\x15a\x18\xDDWa\x18\xD7a\x18W\x7F\xA9^n*\x18$\x11\xE7\xA6\xF9\xED\x11J\x85\xC3v\x1D\x87\xF9\xB8\xF4S\xD8B\xC7\x125\xAAd\xFF\xF9\x9F\x92\x84\x06\x93a\x19JV[a\x16B`3a\x18e\x86a\x19JV[\x92`@Q\x93\x84\x91\x7FClamping value \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01Ra\x18\xA6\x81Q\x80\x92` `/\x87\x01\x91\x01a\x15+V[\x82\x01c\x01\x03\xA3y`\xE5\x1B`/\x82\x01Ra\x18\xC8\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x15+V[\x01\x03`\x13\x81\x01\x84R\x01\x82a\x14\x7FV[\x03\x90\xA1\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`@Q\x90`\xA0\x82\x01`@R`\x80\x82\x01\x91`\0\x83R`\0\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\x19!W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\x19!W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \x07H5\x8E\xFF\t2\xED\xC0?\xC1\xC9#(\x90Ou\x90\x9CCH\t\x8A\xE2\x1E\x90\xFAQS\xE5?\xE7dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static CRYTICERC4626ROUNDING_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x04\xEF\x1FC\x14a\x12\xD3WP\x80c\x1B.\xF1\xCA\x14a\x12@W\x80c'#\xF9\xEE\x14a\x10\xB3W\x80c-\xFAF\n\x14a\x0F\xC9W\x80c7\x1Bx\xCD\x14a\x0E\xE0W\x80cM.\xDE\xC8\x14a\r\xECW\x80cr\x84Ji\x14a\r7W\x80c\x7F\xBB\xB3~\x14a\x0CXW\x80c\x81&\x05\\\x14a\n\xDAW\x80c\x94,\xF0\xFF\x14a\t\xDAW\x83\x81c\x96\xEB'\xA1\x14a\t\x1BWP\x80c\x97\x98\xBA\x90\x14a\x081W\x80c\x9BQ\xDB\xC4\x14a\x06\x9BW\x80c\xA4\x1F\xE4\x9F\x14a\x06@W\x80c\xA8\x15\xC1\x0F\x14a\x05,W\x80c\xB8\x19\"\x05\x14a\x04\xC0W\x83\x81c\xB8\x8D\xAB2\x14a\x04FWP\x80c\xBC\x1B\x9Dh\x14a\x03GW\x83\x81c\xCC\xC9J\xE9\x14a\x02bWP\x80c\xE2\xBB\xB1X\x14a\x01\xC7Wc\xF2\x84O\x1F\x14a\x01\x11W`\0\x80\xFD[4a\x01\xC3W\x82`\x03\x196\x01\x12a\x01\xC3W`\xFF\x82T`\xA0\x1C\x16\x15a\x01\xC3W` \x83`D`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x82\x85Q\x96\x87\x94\x85\x93cnU?e`\xE0\x1B\x85R\x84\x01R0`$\x84\x01RZ\xF1\x90\x81\x15a\x01\xBAWP\x82\x90a\x01\x82W[a\x01\x7F\x91Pa\x01ya\x14\xA1V[\x90a\x15zV[\x80\xF3[P` \x81=\x82\x11a\x01\xB2W[\x81a\x01\x9B` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x90Qa\x01lV[`\0\x80\xFD[=\x91Pa\x01\x8EV[Q=\x84\x82>=\x90\xFD[\x82\x80\xFD[P4a\x01\xC3W` a\x01\xE7a\x02&\x93a\x01\xDF6a\x13\xEDV[\x92\x90\x92a\x14\xE5V[\x91\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93cnU?e`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x90\x81\x15a\x01\xBAWPa\x02:WP\x80\xF3[` \x90\x81=\x81\x11a\x02[W[a\x02P\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x80\xF3[P=a\x02FV[\x80\x84\x844a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\xFF\x82T`\xA0\x1C\x16\x15a\x03CW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x039W\x85\x91a\x03\x05W[P\x15a\x03\0W\x82;\x15a\x03\0W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cU\xDFp\r`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01\xBAWPa\x02\xEDWP\xF3[a\x02\xF6\x90a\x14\x1DV[a\x02\xFDW\x80\xF3[\x80\xFD[PPP\xFD[\x94PP` \x84=\x82\x11a\x031W[\x81a\x03 ` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x84\x93Q\x86a\x02\xB2V[=\x91Pa\x03\x13V[\x83Q=\x87\x82>=\x90\xFD[PP\xFD[P4a\x01\xC3W\x82`\x03\x196\x01\x12a\x01\xC3W`\xFF\x82T`\xA0\x1C\x16\x15a\x01\xC3W` \x83`$`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x84Q\x95\x86\x93\x84\x92c\x03\xD1h\x9D`\xE1\x1B\x84R\x83\x01RZ\xFA\x80\x15a\x04<W\x83\x90a\x04\tW[a\x01\x7F\x92P\x7F assets to be withdrawn at no co\x82Q\x92a\x03\xCC\x84a\x14GV[`B\x84R\x7FconvertToAssets() must not allow` \x85\x01R\x83\x01Ra\x1C\xDD`\xF2\x1B``\x83\x01Ra\x15zV[P` \x82=\x82\x11a\x044W[\x81a\x04\"` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x91Qa\x03\x9AV[=\x91Pa\x04\x15V[\x81Q=\x85\x82>=\x90\xFD[\x80\x84\x844a\x03CWa\x04c\x91a\x04[6a\x13\xEDV[\x93\x90\x93a\x14\xE5V[\x92`\x01`\x01`\xA0\x1B\x03\x85T\x16\x80;\x15a\x04\xBCW\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x92\x85\x01\x92\x83R` \x83\x01\x91\x90\x91R\x84\x91\x84\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x01\xBAWPa\x02\xEDWP\xF3[\x85\x80\xFD[P4a\x01\xC3W` a\x04\xE3a\x04\xE9\x93a\x04\xD86a\x14\x03V[\x96\x91\x96\x93\x90\x93a\x14\xE5V[\x92a\x14\xE5V[`\x01T\x85Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\x02&V[P\x904a\x01\xC3W` \x91\x82`\x03\x196\x01\x12a\x06<W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qcp\xA0\x821`\xE0\x1B\x81R0\x82\x82\x01R\x84\x81`$\x81\x87Z\xFA\x90\x81\x15a\x062W\x90\x85\x92\x91\x87\x91a\x05\xFFW[P\x93a\x05\x89a\x05\xBA\x95\x835a\x18\nV[\x84Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x90\x81R0` \x82\x01\x81\x90R`@\x82\x01R\x91\x94\x85\x92\x83\x91\x89\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x90\x81\x15a\x05\xF6WPa\x05\xCEW\x82\x80\xF3[\x81=\x83\x11a\x05\xEFW[a\x05\xE1\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW8\x80\x82\x80\xF3[P=a\x05\xD7V[Q=\x85\x82>=\x90\xFD[\x83\x81\x94\x92P=\x83\x11a\x06+W[a\x06\x16\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x90Q\x84\x91\x90a\x05\x89a\x05yV[P=a\x06\x0CV[\x83Q=\x88\x82>=\x90\xFD[\x83\x80\xFD[P4a\x01\xC3W` a\x04\xE3a\x06X\x93a\x04\xD86a\x14\x03V[`\x01T\x85Qc-\x18+\xE5`\xE2\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\x02&V[P\x904a\x01\xC3W` \x90\x81`\x03\x196\x01\x12a\x06<W\x805`\xFF\x82T`\xA0\x1C\x16\x15a\x08-W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x84Q\x90c\x03\xD1h\x9D`\xE1\x1B\x82R\x82\x81\x83\x01R\x84\x82`$\x81\x87Z\xFA\x90\x81\x15a\x08#W\x85\x92\x88\x92a\x07\xF1W[P`$\x90\x87Q\x95\x86\x93\x84\x92ccsz\xC9`\xE1\x1B\x84R\x83\x01RZ\xFA\x91\x82\x15a\x07\xE7W\x85\x92a\x07\x96W[P\x7FconvertTo round trip (withdraw, \x84\x7FA profit was extractable from a a\x01\x7F\x96Q\x95a\x07q\x87a\x14GV[`M\x87R\x86\x01R\x84\x01Rlthen deposit)`\x98\x1B``\x84\x01Ra\x16gV[\x93\x91P\x82\x84\x81=\x83\x11a\x07\xE0W[a\x07\xAE\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x92Q\x90\x92\x7FconvertTo round trip (withdraw, a\x07\x1EV[P=a\x07\xA4V[\x84Q=\x87\x82>=\x90\xFD[\x83\x81\x94\x92\x93P=\x83\x11a\x08\x1CW[a\x08\t\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x90Q\x84\x91`$a\x06\xF6V[P=a\x07\xFFV[\x86Q=\x89\x82>=\x90\xFD[\x84\x80\xFD[P4a\x01\xC3W\x82`\x03\x196\x01\x12a\x01\xC3W`\xFF\x82T`\xA0\x1C\x16\x15a\x01\xC3W` \x83`$`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x84Q\x95\x86\x93\x84\x92c\xEF\x8B0\xF7`\xE0\x1B\x84R\x83\x01RZ\xFA\x80\x15a\x04<W\x83\x90a\x08\xE8W[a\x01\x7F\x92P\x7Fhares at no cost\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82Q\x92a\x08\xB6\x84a\x14cV[`0\x84R\x7FpreviewDeposit() must not mint s` \x85\x01R\x83\x01Ra\x15zV[P` \x82=\x82\x11a\t\x13W[\x81a\t\x01` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x91Qa\x08\x84V[=\x91Pa\x08\xF4V[\x80\x84\x844a\x03CW` 6`\x03\x19\x01\x12a\x03CW`\xFF\x82T`\xA0\x1C\x16\x15a\x03CW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x039W\x85\x91a\t\xA6W[P\x15a\x03\0W\x82;\x15a\x03\0W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cb!\xE4\xF1`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01\xBAWPa\x02\xEDWP\xF3[\x94PP` \x84=\x82\x11a\t\xD2W[\x81a\t\xC1` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x84\x93Q\x86a\tkV[=\x91Pa\t\xB4V[P4a\x01\xC3W` \x90\x81`\x03\x196\x01\x12a\x06<W\x825\x92`\xFF\x81T`\xA0\x1C\x16\x15a\x08-W\x83\x15a\x08-W`\x01T\x82Qc-\x18+\xE5`\xE2\x1B\x81R\x91\x82\x01\x94\x85R0` \x86\x01\x81\x90R`@\x86\x01R\x90\x93\x83\x91\x85\x91\x82\x90\x03``\x01\x90\x82\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x92\x83\x15a\n\xD0W\x84\x93a\n\x9AW[P\x91cfree`\xE0\x1B\x83\x7FToken must not be withdrawn for a\x01\x7F\x95Q\x94a\n\x8B\x86a\x14cV[`$\x86R\x85\x01R\x83\x01Ra\x17:V[\x92P\x81\x83\x81=\x83\x11a\n\xC9W[a\n\xB1\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x91Q\x91cfree`\xE0\x1Ba\nPV[P=a\n\xA7V[\x81Q=\x86\x82>=\x90\xFD[P\x904a\x01\xC3W` \x90\x81`\x03\x196\x01\x12a\x06<W\x805`\xFF\x82T`\xA0\x1C\x16\x15a\x08-W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x84Q\x90ccsz\xC9`\xE1\x1B\x82R\x82\x81\x83\x01R\x84\x82`$\x81\x87Z\xFA\x90\x81\x15a\x08#W\x85\x92\x88\x92a\x0C&W[P`$\x90\x87Q\x95\x86\x93\x84\x92c\x03\xD1h\x9D`\xE1\x1B\x84R\x83\x01RZ\xFA\x91\x82\x15a\x07\xE7W\x85\x92a\x0B\xD5W[P\x7FconvertTo round trip (deposit, t\x84\x7FA profit was extractable from a a\x01\x7F\x96Q\x95a\x0B\xB0\x87a\x14GV[`M\x87R\x86\x01R\x84\x01Rlhen withdraw)`\x98\x1B``\x84\x01Ra\x16gV[\x93\x91P\x82\x84\x81=\x83\x11a\x0C\x1FW[a\x0B\xED\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x92Q\x90\x92\x7FconvertTo round trip (deposit, ta\x0B]V[P=a\x0B\xE3V[\x83\x81\x94\x92\x93P=\x83\x11a\x0CQW[a\x0C>\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x90Q\x84\x91`$a\x0B5V[P=a\x0C4V[P4a\x01\xC3W\x82`\x03\x196\x01\x12a\x01\xC3W`\xFF\x82T`\xA0\x1C\x16\x15a\x01\xC3W` \x83`d`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x82\x85Q\x96\x87\x94\x85\x93c]\x04;)`\xE1\x1B\x85R\x84\x01R0`$\x84\x01R0`D\x84\x01RZ\xF1\x80\x15a\x04<W\x83\x90a\r\x04W[a\x01\x7F\x92Pd free`\xD8\x1B\x82Q\x92a\x0C\xD2\x84a\x14cV[`%\x84R\x7FTokens must not be withdrawn for` \x85\x01R\x83\x01Ra\x15zV[P` \x82=\x82\x11a\r/W[\x81a\r\x1D` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x91Qa\x0C\xB8V[=\x91Pa\r\x10V[P4a\x01\xC3W` 6`\x03\x19\x01\x12a\x01\xC3W\x815`\xFF\x83T`\xA0\x1C\x16\x15a\x06<W\x80\x15a\x06<W`\x01T\x82Qc\x94\xBF\x80M`\xE0\x1B\x81R\x93\x84\x01\x91\x82R0` \x83\x81\x01\x91\x90\x91R\x91\x84\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x90\x87\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x01\xBAWP\x82\x90a\r\xB9W[a\x01\x7F\x91Pa\r\xB3a\x14\xA1V[\x90a\x17:V[P` \x81=\x82\x11a\r\xE4W[\x81a\r\xD2` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x90Qa\r\xA6V[=\x91Pa\r\xC5V[P4a\x01\xC3W` \x90\x81`\x03\x196\x01\x12a\x06<W\x825`\xFF\x84T`\xA0\x1C\x16\x15a\x08-W\x80\x15a\x08-W\x82\x90`$`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x95\x84Q\x96\x87\x93\x84\x92c\xB3\xD7\xF6\xB9`\xE0\x1B\x84R\x83\x01RZ\xFA\x92\x83\x15a\n\xD0W\x84\x93a\x0E\x9FW[P\x91n\x18\\\x99\\\xC8\x18]\x08\x1B\x9B\xC8\x18\xDB\xDC\xDD`\x8A\x1B\x83\x7FpreviewMint() must never mint sha\x01\x7F\x95Q\x94a\x0E\x90\x86a\x14cV[`/\x86R\x85\x01R\x83\x01Ra\x17:V[\x92P\x81\x83\x81=\x83\x11a\x0E\xD9W[a\x0E\xB6\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x91Q\x91n\x18\\\x99\\\xC8\x18]\x08\x1B\x9B\xC8\x18\xDB\xDC\xDD`\x8A\x1Ba\x0EJV[P=a\x0E\xACV[P4a\x01\xC3W\x82`\x03\x196\x01\x12a\x01\xC3W`\xFF\x82T`\xA0\x1C\x16\x15a\x01\xC3W` \x83`$`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x84Q\x95\x86\x93\x84\x92c&mj\x83`\xE1\x1B\x84R\x83\x01RZ\xFA\x80\x15a\x04<W\x83\x90a\x0F\x96W[a\x01\x7F\x92P\x7Fssets to be withdrawn at no cost\x82Q\x92a\x0Fe\x84a\x14cV[\x80\x84R\x7FpreviewRedeem() must not allow a` \x85\x01R\x83\x01Ra\x15zV[P` \x82=\x82\x11a\x0F\xC1W[\x81a\x0F\xAF` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x91Qa\x0F3V[=\x91Pa\x0F\xA2V[P4a\x01\xC3W\x82`\x03\x196\x01\x12a\x01\xC3W`\xFF\x82T`\xA0\x1C\x16\x15a\x01\xC3W` \x83`$`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x84Q\x95\x86\x93\x84\x92ccsz\xC9`\xE1\x1B\x84R\x83\x01RZ\xFA\x80\x15a\x04<W\x83\x90a\x10\x80W[a\x01\x7F\x92P\x7F shares to be minted at no cost\0\x82Q\x92a\x10N\x84a\x14cV[`?\x84R\x7FconvertToShares() must not allow` \x85\x01R\x83\x01Ra\x15zV[P` \x82=\x82\x11a\x10\xABW[\x81a\x10\x99` \x93\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADWa\x01\x7F\x91Qa\x10\x1CV[=\x91Pa\x10\x8CV[P\x82\x904a\x12<W` \x90\x81`\x03\x196\x01\x12a\x01\xC3W\x835\x93`\x01`\x01`\xA0\x1B\x03\x94\x85\x85T\x16\x80;\x15a\x04\xBCW\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x84\x82\x01\x90\x81R` \x81\x01\x84\x90R\x90\x91\x87\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x122Wa\x12\x1DW[P\x85\x84\x82\x87\x98a\x11\\\x97\x98T\x16\x83`\x01T\x16\x8A\x88Q\x80\x9A\x81\x95\x82\x94c\t^\xA7\xB3`\xE0\x1B\x84R\x8B\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x12\x13W\x90\x86\x93\x92\x91a\x11\xD7W[`\x01T\x85QcnU?e`\xE0\x1B\x81R\x93\x84\x01\x92\x83R0` \x84\x01R\x92\x95P\x85\x92\x16\x90\x82\x90\x88\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x05\xF6WPa\x11\xAFW\x82\x80\xF3[\x81=\x83\x11a\x11\xD0W[a\x11\xC2\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x01\xADW\x81\x80\x82\x80\xF3[P=a\x11\xB8V[\x92\x85\x81\x96\x92\x96=\x83\x11a\x12\x0CW[a\x11\xEF\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x12\x08WQ\x80\x15\x15\x03a\x04\xBCW\x83\x85\x92a\x11pV[\x86\x80\xFD[P=a\x11\xE5V[\x84Q=\x89\x82>=\x90\xFD[\x94a\x12+a\x11\\\x95\x96a\x14\x1DV[\x94\x93a\x11\x19V[\x84Q=\x88\x82>=\x90\xFD[P\x80\xFD[P4a\x01\xC3W` a\x12Xa\x12\x97\x93a\x01\xDF6a\x13\xEDV[\x91\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93c\x94\xBF\x80M`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x90\x81\x15a\x01\xBAWPa\x12\xABWP\x80\xF3[` \x90\x81=\x81\x11a\x12\xCCW[a\x12\xC1\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x02\xFDW\x80\xF3[P=a\x12\xB7V[\x92\x91\x90P4a\x06<W` \x91\x82`\x03\x196\x01\x12a\x08-W\x805`\xFF\x82T`\xA0\x1C\x16\x15a\x04\xBCW\x80\x15a\x04\xBCW`$\x85\x85\x93\x81\x93`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91c\n(\xA4w`\xE0\x1B\x84R\x83\x01RZ\xFA\x92\x83\x15a\n\xD0W\x84\x93a\x13\x9EW[P\x91\x7F assets to be withdrawn at no co\x83\x7FpreviewWithdraw() must not allowa\x01\x7F\x95Q\x94a\x13\x84\x86a\x14GV[`B\x86R\x85\x01R\x83\x01Ra\x1C\xDD`\xF2\x1B``\x83\x01Ra\x17:V[\x92P\x81\x83\x81=\x83\x11a\x13\xE6W[a\x13\xB5\x81\x83a\x14\x7FV[\x81\x01\x03\x12a\x06<W\x91Q\x91\x7F assets to be withdrawn at no coa\x130V[P=a\x13\xABV[`@\x90`\x03\x19\x01\x12a\x01\xADW`\x045\x90`$5\x90V[``\x90`\x03\x19\x01\x12a\x01\xADW`\x045\x90`$5\x90`D5\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x141W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x141W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x141W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x141W`@RV[`@Q\x90a\x14\xAE\x82a\x14cV[`\"\x82Raee`\xF0\x1B`@\x83\x7FShares must not be minted for fr` \x82\x01R\x01RV[`\x03\x90\x06\x80\x15a\x15&W`\x01\x14a\x15\x0EWs\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\x90V[s\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\x90V[P0\x90V[`\0[\x83\x81\x10a\x15>WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x15.V[`@\x91` \x82Ra\x15n\x81Q\x80\x92\x81` \x86\x01R` \x86\x86\x01\x91\x01a\x15+V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x80a\x15\x83WPPV[\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x91a\x16B`5a\x15\xB6a\x16N\x94a\x19JV[a\x15\xBEa\x19\tV[\x93`@Q\x94\x85\x92h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B` \x85\x01Ra\x15\xEB\x81Q\x80\x92` `)\x88\x01\x91\x01a\x15+V[\x83\x01a!=`\xF0\x1B`)\x82\x01Ra\x16\x0C\x82Q\x80\x93` `+\x85\x01\x91\x01a\x15+V[\x01i\x01a\x03\x93+\x0B\x9B{q\xD1`\xB5\x1B`+\x82\x01Ra\x163\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x15+V[\x01\x03`\x15\x81\x01\x84R\x01\x82a\x14\x7FV[`@Q\x91\x82\x91\x82a\x15NV[\x03\x90\xA1cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x81\x81\x10a\x16sWPPPV[\x91a\x16B`;a\x16N\x93a\x16\xB0a\x16\xAA\x7F\x94BN\xD2O\xB3\x968\xB6H\x17\xC77\xDDD?8z\xAA\x14\x86aM\xA4I\xB6hjd-ml\x97a\x19JV[\x91a\x19JV[\x93`@Q\x94\x85\x92h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B` \x85\x01Ra\x16\xDD\x81Q\x80\x92` `)\x88\x01\x91\x01a\x15+V[\x83\x01`\x0F`\xFA\x1B`)\x82\x01Ra\x16\xFD\x82Q\x80\x93` `*\x85\x01\x91\x01a\x15+V[\x01p\x01\x033\x0BKc+!a\x03\x93+\x0B\x9B{q\xD1`}\x1B`*\x82\x01Ra\x17+\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x15+V[\x01\x03`\x1B\x81\x01\x84R\x01\x82a\x14\x7FV[\x80\x15a\x17DWPPV[\x7Fp{\x8CV\xE4\xC2\x11\xCF\x13!\xFA\xEBAH#pb\"\x8D\xB2\xFC\xEC\xC9\xBEH~\x83\xA2h\x0E~P\x91a\x16B`<a\x17wa\x16N\x94a\x19JV[a\x17\x7Fa\x19\tV[\x93`@Q\x94\x85\x92h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B` \x85\x01Ra\x17\xAC\x81Q\x80\x92` `)\x88\x01\x91\x01a\x15+V[\x83\x01a<=`\xF0\x1B`)\x82\x01Ra\x17\xCD\x82Q\x80\x93` `+\x85\x01\x91\x01a\x15+V[\x01p\x01\x033\x0BKc+!a\x03\x93+\x0B\x9B{q\xD1`}\x1B`+\x82\x01Ra\x17\xFB\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x15+V[\x01\x03`\x1C\x81\x01\x84R\x01\x82a\x14\x7FV[\x90\x80\x82\x11a\x18\x16WP\x90V[`\x01\x81\x01\x80\x91\x11a\x18\xF3W\x80\x15a\x18\xDDWa\x18\xD7a\x18W\x7F\xA9^n*\x18$\x11\xE7\xA6\xF9\xED\x11J\x85\xC3v\x1D\x87\xF9\xB8\xF4S\xD8B\xC7\x125\xAAd\xFF\xF9\x9F\x92\x84\x06\x93a\x19JV[a\x16B`3a\x18e\x86a\x19JV[\x92`@Q\x93\x84\x91\x7FClamping value \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01Ra\x18\xA6\x81Q\x80\x92` `/\x87\x01\x91\x01a\x15+V[\x82\x01c\x01\x03\xA3y`\xE5\x1B`/\x82\x01Ra\x18\xC8\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x15+V[\x01\x03`\x13\x81\x01\x84R\x01\x82a\x14\x7FV[\x03\x90\xA1\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`@Q\x90`\xA0\x82\x01`@R`\x80\x82\x01\x91`\0\x83R`\0\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\x19!W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\x19!W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \x07H5\x8E\xFF\t2\xED\xC0?\xC1\xC9#(\x90Ou\x90\x9CCH\t\x8A\xE2\x1E\x90\xFAQS\xE5?\xE7dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static CRYTICERC4626ROUNDING_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CryticERC4626Rounding<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CryticERC4626Rounding<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CryticERC4626Rounding<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CryticERC4626Rounding<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CryticERC4626Rounding<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CryticERC4626Rounding))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CryticERC4626Rounding<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CRYTICERC4626ROUNDING_ABI.clone(),
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
                CRYTICERC4626ROUNDING_ABI.clone(),
                CRYTICERC4626ROUNDING_BYTECODE.clone().into(),
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
        ///Calls the contract's `verify_convertRoundTrip` (0x8126055c) function
        pub fn verify_convert_round_trip(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 38, 5, 92], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_convertRoundTrip2` (0x9b51dbc4) function
        pub fn verify_convert_round_trip_2(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 81, 219, 196], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_convertToAssetsRoundingDirection` (0xbc1b9d68) function
        pub fn verify_convert_to_assets_rounding_direction(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 27, 157, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_convertToSharesRoundingDirection` (0x2dfa460a) function
        pub fn verify_convert_to_shares_rounding_direction(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 250, 70, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_depositRoundingDirection` (0xf2844f1f) function
        pub fn verify_deposit_rounding_direction(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 132, 79, 31], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_mintRoundingDirection` (0x72844a69) function
        pub fn verify_mint_rounding_direction(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 132, 74, 105], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_previewDepositRoundingDirection` (0x9798ba90) function
        pub fn verify_preview_deposit_rounding_direction(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 152, 186, 144], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_previewMintRoundingDirection` (0x4d2edec8) function
        pub fn verify_preview_mint_rounding_direction(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 46, 222, 200], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_previewRedeemRoundingDirection` (0x371b78cd) function
        pub fn verify_preview_redeem_rounding_direction(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([55, 27, 120, 205], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_previewWithdrawRoundingDirection` (0x04ef1f43) function
        pub fn verify_preview_withdraw_rounding_direction(
            &self,
            tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 239, 31, 67], tokens)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_redeemRoundingDirection` (0x7fbbb37e) function
        pub fn verify_redeem_rounding_direction(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 187, 179, 126], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_withdrawRoundingDirection` (0x942cf0ff) function
        pub fn verify_withdraw_rounding_direction(
            &self,
            tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 44, 240, 255], tokens)
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
            CryticERC4626RoundingEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CryticERC4626Rounding<M> {
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
    pub enum CryticERC4626RoundingEvents {
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
    impl ::ethers::contract::EthLogDecode for CryticERC4626RoundingEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssertEqFailFilter::decode_log(log) {
                return Ok(CryticERC4626RoundingEvents::AssertEqFailFilter(decoded));
            }
            if let Ok(decoded) = AssertFailFilter::decode_log(log) {
                return Ok(CryticERC4626RoundingEvents::AssertFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGtFailFilter::decode_log(log) {
                return Ok(CryticERC4626RoundingEvents::AssertGtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGteFailFilter::decode_log(log) {
                return Ok(CryticERC4626RoundingEvents::AssertGteFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLtFailFilter::decode_log(log) {
                return Ok(CryticERC4626RoundingEvents::AssertLtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLteFailFilter::decode_log(log) {
                return Ok(CryticERC4626RoundingEvents::AssertLteFailFilter(decoded));
            }
            if let Ok(decoded) = AssertNeqFailFilter::decode_log(log) {
                return Ok(CryticERC4626RoundingEvents::AssertNeqFailFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(CryticERC4626RoundingEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(CryticERC4626RoundingEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUint256Filter::decode_log(log) {
                return Ok(CryticERC4626RoundingEvents::LogUint256Filter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CryticERC4626RoundingEvents {
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
    impl ::core::convert::From<AssertEqFailFilter> for CryticERC4626RoundingEvents {
        fn from(value: AssertEqFailFilter) -> Self {
            Self::AssertEqFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertFailFilter> for CryticERC4626RoundingEvents {
        fn from(value: AssertFailFilter) -> Self {
            Self::AssertFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGtFailFilter> for CryticERC4626RoundingEvents {
        fn from(value: AssertGtFailFilter) -> Self {
            Self::AssertGtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGteFailFilter> for CryticERC4626RoundingEvents {
        fn from(value: AssertGteFailFilter) -> Self {
            Self::AssertGteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLtFailFilter> for CryticERC4626RoundingEvents {
        fn from(value: AssertLtFailFilter) -> Self {
            Self::AssertLtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLteFailFilter> for CryticERC4626RoundingEvents {
        fn from(value: AssertLteFailFilter) -> Self {
            Self::AssertLteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertNeqFailFilter> for CryticERC4626RoundingEvents {
        fn from(value: AssertNeqFailFilter) -> Self {
            Self::AssertNeqFailFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for CryticERC4626RoundingEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for CryticERC4626RoundingEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUint256Filter> for CryticERC4626RoundingEvents {
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
    ///Container type for all input parameters for the `verify_convertRoundTrip` function with signature `verify_convertRoundTrip(uint256)` and selector `0x8126055c`
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
        name = "verify_convertRoundTrip",
        abi = "verify_convertRoundTrip(uint256)"
    )]
    pub struct VerifyConvertRoundTripCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_convertRoundTrip2` function with signature `verify_convertRoundTrip2(uint256)` and selector `0x9b51dbc4`
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
        name = "verify_convertRoundTrip2",
        abi = "verify_convertRoundTrip2(uint256)"
    )]
    pub struct VerifyConvertRoundTrip2Call {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_convertToAssetsRoundingDirection` function with signature `verify_convertToAssetsRoundingDirection()` and selector `0xbc1b9d68`
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
        name = "verify_convertToAssetsRoundingDirection",
        abi = "verify_convertToAssetsRoundingDirection()"
    )]
    pub struct VerifyConvertToAssetsRoundingDirectionCall;
    ///Container type for all input parameters for the `verify_convertToSharesRoundingDirection` function with signature `verify_convertToSharesRoundingDirection()` and selector `0x2dfa460a`
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
        name = "verify_convertToSharesRoundingDirection",
        abi = "verify_convertToSharesRoundingDirection()"
    )]
    pub struct VerifyConvertToSharesRoundingDirectionCall;
    ///Container type for all input parameters for the `verify_depositRoundingDirection` function with signature `verify_depositRoundingDirection()` and selector `0xf2844f1f`
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
        name = "verify_depositRoundingDirection",
        abi = "verify_depositRoundingDirection()"
    )]
    pub struct VerifyDepositRoundingDirectionCall;
    ///Container type for all input parameters for the `verify_mintRoundingDirection` function with signature `verify_mintRoundingDirection(uint256)` and selector `0x72844a69`
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
        name = "verify_mintRoundingDirection",
        abi = "verify_mintRoundingDirection(uint256)"
    )]
    pub struct VerifyMintRoundingDirectionCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_previewDepositRoundingDirection` function with signature `verify_previewDepositRoundingDirection()` and selector `0x9798ba90`
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
        name = "verify_previewDepositRoundingDirection",
        abi = "verify_previewDepositRoundingDirection()"
    )]
    pub struct VerifyPreviewDepositRoundingDirectionCall;
    ///Container type for all input parameters for the `verify_previewMintRoundingDirection` function with signature `verify_previewMintRoundingDirection(uint256)` and selector `0x4d2edec8`
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
        name = "verify_previewMintRoundingDirection",
        abi = "verify_previewMintRoundingDirection(uint256)"
    )]
    pub struct VerifyPreviewMintRoundingDirectionCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_previewRedeemRoundingDirection` function with signature `verify_previewRedeemRoundingDirection()` and selector `0x371b78cd`
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
        name = "verify_previewRedeemRoundingDirection",
        abi = "verify_previewRedeemRoundingDirection()"
    )]
    pub struct VerifyPreviewRedeemRoundingDirectionCall;
    ///Container type for all input parameters for the `verify_previewWithdrawRoundingDirection` function with signature `verify_previewWithdrawRoundingDirection(uint256)` and selector `0x04ef1f43`
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
        name = "verify_previewWithdrawRoundingDirection",
        abi = "verify_previewWithdrawRoundingDirection(uint256)"
    )]
    pub struct VerifyPreviewWithdrawRoundingDirectionCall {
        pub tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_redeemRoundingDirection` function with signature `verify_redeemRoundingDirection()` and selector `0x7fbbb37e`
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
        name = "verify_redeemRoundingDirection",
        abi = "verify_redeemRoundingDirection()"
    )]
    pub struct VerifyRedeemRoundingDirectionCall;
    ///Container type for all input parameters for the `verify_withdrawRoundingDirection` function with signature `verify_withdrawRoundingDirection(uint256)` and selector `0x942cf0ff`
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
        name = "verify_withdrawRoundingDirection",
        abi = "verify_withdrawRoundingDirection(uint256)"
    )]
    pub struct VerifyWithdrawRoundingDirectionCall {
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
    pub enum CryticERC4626RoundingCalls {
        Deposit(DepositCall),
        DepositForSelfSimple(DepositForSelfSimpleCall),
        Mint(MintCall),
        MintAsset(MintAssetCall),
        RecognizeLossProxy(RecognizeLossProxyCall),
        RecognizeProfitProxy(RecognizeProfitProxyCall),
        Redeem(RedeemCall),
        RedeemForSelfSimple(RedeemForSelfSimpleCall),
        VerifyConvertRoundTrip(VerifyConvertRoundTripCall),
        VerifyConvertRoundTrip2(VerifyConvertRoundTrip2Call),
        VerifyConvertToAssetsRoundingDirection(
            VerifyConvertToAssetsRoundingDirectionCall,
        ),
        VerifyConvertToSharesRoundingDirection(
            VerifyConvertToSharesRoundingDirectionCall,
        ),
        VerifyDepositRoundingDirection(VerifyDepositRoundingDirectionCall),
        VerifyMintRoundingDirection(VerifyMintRoundingDirectionCall),
        VerifyPreviewDepositRoundingDirection(VerifyPreviewDepositRoundingDirectionCall),
        VerifyPreviewMintRoundingDirection(VerifyPreviewMintRoundingDirectionCall),
        VerifyPreviewRedeemRoundingDirection(VerifyPreviewRedeemRoundingDirectionCall),
        VerifyPreviewWithdrawRoundingDirection(
            VerifyPreviewWithdrawRoundingDirectionCall,
        ),
        VerifyRedeemRoundingDirection(VerifyRedeemRoundingDirectionCall),
        VerifyWithdrawRoundingDirection(VerifyWithdrawRoundingDirectionCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for CryticERC4626RoundingCalls {
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
                = <VerifyConvertRoundTripCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyConvertRoundTrip(decoded));
            }
            if let Ok(decoded)
                = <VerifyConvertRoundTrip2Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyConvertRoundTrip2(decoded));
            }
            if let Ok(decoded)
                = <VerifyConvertToAssetsRoundingDirectionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyConvertToAssetsRoundingDirection(decoded));
            }
            if let Ok(decoded)
                = <VerifyConvertToSharesRoundingDirectionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyConvertToSharesRoundingDirection(decoded));
            }
            if let Ok(decoded)
                = <VerifyDepositRoundingDirectionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyDepositRoundingDirection(decoded));
            }
            if let Ok(decoded)
                = <VerifyMintRoundingDirectionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyMintRoundingDirection(decoded));
            }
            if let Ok(decoded)
                = <VerifyPreviewDepositRoundingDirectionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyPreviewDepositRoundingDirection(decoded));
            }
            if let Ok(decoded)
                = <VerifyPreviewMintRoundingDirectionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyPreviewMintRoundingDirection(decoded));
            }
            if let Ok(decoded)
                = <VerifyPreviewRedeemRoundingDirectionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyPreviewRedeemRoundingDirection(decoded));
            }
            if let Ok(decoded)
                = <VerifyPreviewWithdrawRoundingDirectionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyPreviewWithdrawRoundingDirection(decoded));
            }
            if let Ok(decoded)
                = <VerifyRedeemRoundingDirectionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyRedeemRoundingDirection(decoded));
            }
            if let Ok(decoded)
                = <VerifyWithdrawRoundingDirectionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyWithdrawRoundingDirection(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CryticERC4626RoundingCalls {
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
                Self::VerifyConvertRoundTrip(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyConvertRoundTrip2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyConvertToAssetsRoundingDirection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyConvertToSharesRoundingDirection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyDepositRoundingDirection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyMintRoundingDirection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyPreviewDepositRoundingDirection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyPreviewMintRoundingDirection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyPreviewRedeemRoundingDirection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyPreviewWithdrawRoundingDirection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyRedeemRoundingDirection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyWithdrawRoundingDirection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CryticERC4626RoundingCalls {
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
                Self::VerifyConvertRoundTrip(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyConvertRoundTrip2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyConvertToAssetsRoundingDirection(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyConvertToSharesRoundingDirection(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyDepositRoundingDirection(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyMintRoundingDirection(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyPreviewDepositRoundingDirection(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyPreviewMintRoundingDirection(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyPreviewRedeemRoundingDirection(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyPreviewWithdrawRoundingDirection(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyRedeemRoundingDirection(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyWithdrawRoundingDirection(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositCall> for CryticERC4626RoundingCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositForSelfSimpleCall> for CryticERC4626RoundingCalls {
        fn from(value: DepositForSelfSimpleCall) -> Self {
            Self::DepositForSelfSimple(value)
        }
    }
    impl ::core::convert::From<MintCall> for CryticERC4626RoundingCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<MintAssetCall> for CryticERC4626RoundingCalls {
        fn from(value: MintAssetCall) -> Self {
            Self::MintAsset(value)
        }
    }
    impl ::core::convert::From<RecognizeLossProxyCall> for CryticERC4626RoundingCalls {
        fn from(value: RecognizeLossProxyCall) -> Self {
            Self::RecognizeLossProxy(value)
        }
    }
    impl ::core::convert::From<RecognizeProfitProxyCall> for CryticERC4626RoundingCalls {
        fn from(value: RecognizeProfitProxyCall) -> Self {
            Self::RecognizeProfitProxy(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for CryticERC4626RoundingCalls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<RedeemForSelfSimpleCall> for CryticERC4626RoundingCalls {
        fn from(value: RedeemForSelfSimpleCall) -> Self {
            Self::RedeemForSelfSimple(value)
        }
    }
    impl ::core::convert::From<VerifyConvertRoundTripCall>
    for CryticERC4626RoundingCalls {
        fn from(value: VerifyConvertRoundTripCall) -> Self {
            Self::VerifyConvertRoundTrip(value)
        }
    }
    impl ::core::convert::From<VerifyConvertRoundTrip2Call>
    for CryticERC4626RoundingCalls {
        fn from(value: VerifyConvertRoundTrip2Call) -> Self {
            Self::VerifyConvertRoundTrip2(value)
        }
    }
    impl ::core::convert::From<VerifyConvertToAssetsRoundingDirectionCall>
    for CryticERC4626RoundingCalls {
        fn from(value: VerifyConvertToAssetsRoundingDirectionCall) -> Self {
            Self::VerifyConvertToAssetsRoundingDirection(value)
        }
    }
    impl ::core::convert::From<VerifyConvertToSharesRoundingDirectionCall>
    for CryticERC4626RoundingCalls {
        fn from(value: VerifyConvertToSharesRoundingDirectionCall) -> Self {
            Self::VerifyConvertToSharesRoundingDirection(value)
        }
    }
    impl ::core::convert::From<VerifyDepositRoundingDirectionCall>
    for CryticERC4626RoundingCalls {
        fn from(value: VerifyDepositRoundingDirectionCall) -> Self {
            Self::VerifyDepositRoundingDirection(value)
        }
    }
    impl ::core::convert::From<VerifyMintRoundingDirectionCall>
    for CryticERC4626RoundingCalls {
        fn from(value: VerifyMintRoundingDirectionCall) -> Self {
            Self::VerifyMintRoundingDirection(value)
        }
    }
    impl ::core::convert::From<VerifyPreviewDepositRoundingDirectionCall>
    for CryticERC4626RoundingCalls {
        fn from(value: VerifyPreviewDepositRoundingDirectionCall) -> Self {
            Self::VerifyPreviewDepositRoundingDirection(value)
        }
    }
    impl ::core::convert::From<VerifyPreviewMintRoundingDirectionCall>
    for CryticERC4626RoundingCalls {
        fn from(value: VerifyPreviewMintRoundingDirectionCall) -> Self {
            Self::VerifyPreviewMintRoundingDirection(value)
        }
    }
    impl ::core::convert::From<VerifyPreviewRedeemRoundingDirectionCall>
    for CryticERC4626RoundingCalls {
        fn from(value: VerifyPreviewRedeemRoundingDirectionCall) -> Self {
            Self::VerifyPreviewRedeemRoundingDirection(value)
        }
    }
    impl ::core::convert::From<VerifyPreviewWithdrawRoundingDirectionCall>
    for CryticERC4626RoundingCalls {
        fn from(value: VerifyPreviewWithdrawRoundingDirectionCall) -> Self {
            Self::VerifyPreviewWithdrawRoundingDirection(value)
        }
    }
    impl ::core::convert::From<VerifyRedeemRoundingDirectionCall>
    for CryticERC4626RoundingCalls {
        fn from(value: VerifyRedeemRoundingDirectionCall) -> Self {
            Self::VerifyRedeemRoundingDirection(value)
        }
    }
    impl ::core::convert::From<VerifyWithdrawRoundingDirectionCall>
    for CryticERC4626RoundingCalls {
        fn from(value: VerifyWithdrawRoundingDirectionCall) -> Self {
            Self::VerifyWithdrawRoundingDirection(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for CryticERC4626RoundingCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
}
