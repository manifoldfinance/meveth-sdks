pub use crytic_erc4626_must_not_revert::*;
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
pub mod crytic_erc4626_must_not_revert {
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
                    ::std::borrow::ToOwned::to_owned("verify_assetMustNotRevert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_assetMustNotRevert",
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
                        "verify_convertToAssetsMustNotRevert",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_convertToAssetsMustNotRevert",
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
                        "verify_convertToSharesMustNotRevert",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_convertToSharesMustNotRevert",
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
                    ::std::borrow::ToOwned::to_owned("verify_maxDepositMustNotRevert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_maxDepositMustNotRevert",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("verify_maxMintMustNotRevert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_maxMintMustNotRevert",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("verify_maxRedeemMustNotRevert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_maxRedeemMustNotRevert",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("verify_maxWithdrawMustNotRevert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_maxWithdrawMustNotRevert",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("verify_totalAssetsMustNotRevert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_totalAssetsMustNotRevert",
                            ),
                            inputs: ::std::vec![],
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
    pub static CRYTICERC4626MUSTNOTREVERT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x16C\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`@`\x80\x81R`\x04\x806\x10\x15a\0\x14W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x80c\x143\x08$\x14a\x08\xB0W\x80c\x1B.\xF1\xCA\x14a\x08\x1DW\x80c'#\xF9\xEE\x14a\x06\x94W\x80c/\xFD]F\x14a\x06wW\x80cI\xB6\x92\xDC\x14a\x06ZW\x80c]#\x88\xE0\x14a\x06-W\x80c\x8B\x96\x99\xED\x14a\x06\0W\x83\x81c\x96\xEB'\xA1\x14a\x05AWP\x80c\xA2y\xC0\xEE\x14a\x05\x14W\x80c\xA4\x1F\xE4\x9F\x14a\x04\xB9W\x80c\xA6\xC8;\x03\x14a\x04\xA0W\x80c\xA8\x15\xC1\x0F\x14a\x03\x8CW\x80c\xB8\x19\"\x05\x14a\x03 W\x83\x81c\xB8\x8D\xAB2\x14a\x02\xA6WP\x80c\xC01\r\x7F\x14a\x02rW\x83\x81c\xCC\xC9J\xE9\x14a\x01\x8DWPc\xE2\xBB\xB1X\x14a\0\xE1W`\0\x80\xFD[4a\x01\x89W` a\x01\0a\x01?\x93a\0\xF86a\x08\xC9V[\x92\x90\x92a\x13\xE4V[\x91\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93cnU?e`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x90\x81\x15a\x01\x80WPa\x01SWP\x80\xF3[` \x90\x81=\x81\x11a\x01yW[a\x01i\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x80\xF3[`\0\x80\xFD[P=a\x01_V[Q=\x84\x82>=\x90\xFD[\x82\x80\xFD[\x80\x84\x844a\x02nW` 6`\x03\x19\x01\x12a\x02nW`\xFF\x82T`\xA0\x1C\x16\x15a\x02nW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x02dW\x85\x91a\x020W[P\x15a\x02+W\x82;\x15a\x02+W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cU\xDFp\r`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01\x80WPa\x02\x18WP\xF3[a\x02!\x90a\x08\xF9V[a\x02(W\x80\xF3[\x80\xFD[PPP\xFD[\x94PP` \x84=\x82\x11a\x02\\W[\x81a\x02K` \x93\x83a\t[V[\x81\x01\x03\x12a\x01tW\x84\x93Q\x86a\x01\xDDV[=\x91Pa\x02>V[\x83Q=\x87\x82>=\x90\xFD[PP\xFD[PP4a\x02\xA2W` 6`\x03\x19\x01\x12a\x02\xA2W5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xA2Wa\x02\x9F\x90a\x12\x8EV[\x80\xF3[P\x80\xFD[\x80\x84\x844a\x02nWa\x02\xC3\x91a\x02\xBB6a\x08\xC9V[\x93\x90\x93a\x13\xE4V[\x92`\x01`\x01`\xA0\x1B\x03\x85T\x16\x80;\x15a\x03\x1CW\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x92\x85\x01\x92\x83R` \x83\x01\x91\x90\x91R\x84\x91\x84\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x01\x80WPa\x02\x18WP\xF3[\x85\x80\xFD[P4a\x01\x89W` a\x03Ca\x03I\x93a\x0386a\x08\xDFV[\x96\x91\x96\x93\x90\x93a\x13\xE4V[\x92a\x13\xE4V[`\x01T\x85Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\x01?V[P\x904a\x01\x89W` \x91\x82`\x03\x196\x01\x12a\x04\x9CW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qcp\xA0\x821`\xE0\x1B\x81R0\x82\x82\x01R\x84\x81`$\x81\x87Z\xFA\x90\x81\x15a\x04\x92W\x90\x85\x92\x91\x87\x91a\x04_W[P\x93a\x03\xE9a\x04\x1A\x95\x835a\x14\xC3V[\x84Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x90\x81R0` \x82\x01\x81\x90R`@\x82\x01R\x91\x94\x85\x92\x83\x91\x89\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x90\x81\x15a\x04VWPa\x04.W\x82\x80\xF3[\x81=\x83\x11a\x04OW[a\x04A\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW8\x80\x82\x80\xF3[P=a\x047V[Q=\x85\x82>=\x90\xFD[\x83\x81\x94\x92P=\x83\x11a\x04\x8BW[a\x04v\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x90Q\x84\x91\x90a\x03\xE9a\x03\xD9V[P=a\x04lV[\x83Q=\x88\x82>=\x90\xFD[\x83\x80\xFD[\x834a\x02(W\x80`\x03\x196\x01\x12a\x02(Wa\x02\x9Fa\t}V[P4a\x01\x89W` a\x03Ca\x04\xD1\x93a\x0386a\x08\xDFV[`\x01T\x85Qc-\x18+\xE5`\xE2\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\x01?V[PP4a\x02\xA2W` 6`\x03\x19\x01\x12a\x02\xA2W5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xA2Wa\x02\x9F\x90a\x10\x91V[\x80\x84\x844a\x02nW` 6`\x03\x19\x01\x12a\x02nW`\xFF\x82T`\xA0\x1C\x16\x15a\x02nW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x02dW\x85\x91a\x05\xCCW[P\x15a\x02+W\x82;\x15a\x02+W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cb!\xE4\xF1`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01\x80WPa\x02\x18WP\xF3[\x94PP` \x84=\x82\x11a\x05\xF8W[\x81a\x05\xE7` \x93\x83a\t[V[\x81\x01\x03\x12a\x01tW\x84\x93Q\x86a\x05\x91V[=\x91Pa\x05\xDAV[PP4a\x02\xA2W` 6`\x03\x19\x01\x12a\x02\xA2W5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xA2Wa\x02\x9F\x90a\x0F\xE8V[PP4a\x02\xA2W` 6`\x03\x19\x01\x12a\x02\xA2W5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xA2Wa\x02\x9F\x90a\x11/V[PP4a\x02\xA2W` 6`\x03\x19\x01\x12a\x02\xA2Wa\x02\x9F\x905a\n\xC3V[PP4a\x02\xA2W` 6`\x03\x19\x01\x12a\x02\xA2Wa\x02\x9F\x905a\x0E\x05V[P\x82\x904a\x02\xA2W` \x90\x81`\x03\x196\x01\x12a\x01\x89W\x835\x93`\x01`\x01`\xA0\x1B\x03\x94\x85\x85T\x16\x80;\x15a\x03\x1CW\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x84\x82\x01\x90\x81R` \x81\x01\x84\x90R\x90\x91\x87\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x08\x13Wa\x07\xFEW[P\x85\x84\x82\x87\x98a\x07=\x97\x98T\x16\x83`\x01T\x16\x8A\x88Q\x80\x9A\x81\x95\x82\x94c\t^\xA7\xB3`\xE0\x1B\x84R\x8B\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x07\xF4W\x90\x86\x93\x92\x91a\x07\xB8W[`\x01T\x85QcnU?e`\xE0\x1B\x81R\x93\x84\x01\x92\x83R0` \x84\x01R\x92\x95P\x85\x92\x16\x90\x82\x90\x88\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x04VWPa\x07\x90W\x82\x80\xF3[\x81=\x83\x11a\x07\xB1W[a\x07\xA3\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x81\x80\x82\x80\xF3[P=a\x07\x99V[\x92\x85\x81\x96\x92\x96=\x83\x11a\x07\xEDW[a\x07\xD0\x81\x83a\t[V[\x81\x01\x03\x12a\x07\xE9WQ\x80\x15\x15\x03a\x03\x1CW\x83\x85\x92a\x07QV[\x86\x80\xFD[P=a\x07\xC6V[\x84Q=\x89\x82>=\x90\xFD[\x94a\x08\x0Ca\x07=\x95\x96a\x08\xF9V[\x94\x93a\x06\xFAV[\x84Q=\x88\x82>=\x90\xFD[P4a\x01\x89W` a\x085a\x08t\x93a\0\xF86a\x08\xC9V[\x91\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93c\x94\xBF\x80M`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x90\x81\x15a\x01\x80WPa\x08\x88WP\x80\xF3[` \x90\x81=\x81\x11a\x08\xA9W[a\x08\x9E\x81\x83a\t[V[\x81\x01\x03\x12a\x02(W\x80\xF3[P=a\x08\x94V[\x834a\x02(W\x80`\x03\x196\x01\x12a\x02(Wa\x02\x9Fa\n#V[`@\x90`\x03\x19\x01\x12a\x01tW`\x045\x90`$5\x90V[``\x90`\x03\x19\x01\x12a\x01tW`\x045\x90`$5\x90`D5\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\t\rW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\rW`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\rW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\rW`@RV[`\x04`\x01`\x01`\xA0\x1B\x03` \x81`\x01T\x16`@Q\x93\x84\x80\x92c8\xD5.\x0F`\xE0\x1B\x82RZ\xFA\x91\x82a\t\xEBW[PPa\t\xE9W`@Qa\t\xBA\x81a\t#V[`\x1D\x81R\x7Fvault.asset() must not revert\0\0\0` \x82\x01Ra\x14yV[V[` \x81=\x82\x11a\n\x1BW[\x81a\n\x03` \x93\x83a\t[V[\x81\x01\x03\x12a\x01tWQ\x90\x81\x16\x03a\x01tW8\x80a\t\xA8V[=\x91Pa\t\xF6V[`\x04` `\x01`\x01`\xA0\x1B\x03`\x01T\x16`@Q\x92\x83\x80\x92bxtE`\xE2\x1B\x82RZ\xFA\x90\x81a\n\x98W[Pa\t\xE9W`@Qa\n]\x81a\t?V[`#\x81R\x7Fvault.totalAssets() must not rev` \x82\x01Rb\x19\\\x9D`\xEA\x1B`@\x82\x01Ra\x14yV[` \x90\x81=\x81\x11a\n\xBCW[a\n\xAE\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW8a\nLV[P=a\n\xA4V[\x90`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16\x91`@\x92\x83Q\x94c1<\xE5g`\xE0\x1B\x86R` \x95`\x04\x90\x87\x81\x83\x81`\0\x80\x98Z\xF1\x90\x81\x15a\rFW\x84\x91a\r\xCDW[P`\xFF`\x14\x91\x16\x01`\xFF\x81\x11a\r\xBAW`\xFF\x16`M\x81\x11a\r\xBAW`\n\n\x93`\x01T\x16\x93\x86Q\x92c\x18\x16\r\xDD`\xE0\x1B\x93\x84\x81R\x89\x81\x85\x81\x8AZ\xFA\x90\x81\x15a\r\xB0W\x90\x83\x91\x87\x91a\r\x7FW[P\x11a\r\x03W\x90a\x0B\\\x91a\x14\xC3V[\x86Q\x83\x81R\x88\x81\x84\x81\x89Z\xFA\x90\x81\x15a\r\x0EW\x85\x91a\rPW[Pa\x0B\x80\x91a\x14\xC3V[\x91\x86Q\x90\x81R\x87\x81\x83\x81\x88Z\xFA\x80\x15a\rFW\x84\x90a\r\x18W[`\x80\x91P\x87Q\x90\x88\x82R`\x0B\x89\x83\x01RjtotalSupply`\xA8\x1B``\x83\x01R\x89\x82\x01R\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x91\x82\x91\xA1\x86QbxtE`\xE2\x1B\x81R\x88\x81\x84\x81\x89Z\xFA\x90\x81\x15a\r\x0EW\x90\x89\x93\x92\x91\x86\x91a\x0C\xD6W[P\x91`$\x91`\x80\x85\x96\x94\x8BQ\x90\x8C\x82R`\x0B\x8D\x83\x01RjtotalAssets`\xA8\x1B``\x83\x01R\x87\x82\x01R\xA1\x88Q\x96\x87\x93\x84\x92c\x03\xD1h\x9D`\xE1\x1B\x84R\x83\x01RZ\xFA\x92\x83a\x0C\xADW[PPPa\x0C\xA8WP\x80\x7Fvault.convertToAssets() must notf\x08\x1C\x99]\x99\\\x9D`\xCA\x1B\x92Q\x93a\x0C\x99\x85a\t?V[`'\x85R\x84\x01R\x82\x01Ra\x14yV[\x91PPV[\x81=\x83\x11a\x0C\xCFW[a\x0C\xC0\x81\x83a\t[V[\x81\x01\x03\x12a\x02(W\x84\x81a\x0CXV[P=a\x0C\xB6V[\x84\x81\x93\x94\x95\x92P=\x83\x11a\r\x07W[a\x0C\xEF\x81\x83a\t[V[\x81\x01\x03\x12a\r\x03WQ\x88\x92\x91\x90`$a\x0C\x0CV[\x84\x80\xFD[P=a\x0C\xE5V[\x88Q=\x87\x82>=\x90\xFD[P\x87\x81\x81=\x83\x11a\r?W[a\r.\x81\x83a\t[V[\x81\x01\x03\x12a\x04\x9CW`\x80\x90Qa\x0B\x9AV[P=a\r$V[\x87Q=\x86\x82>=\x90\xFD[\x90P\x88\x81\x81=\x83\x11a\rxW[a\rg\x81\x83a\t[V[\x81\x01\x03\x12a\r\x03WQa\x0B\x80a\x0BvV[P=a\r]V[\x80\x92P\x8B\x80\x92P=\x83\x11a\r\xA9W[a\r\x98\x81\x83a\t[V[\x81\x01\x03\x12a\x03\x1CW\x82\x90Q8a\x0BLV[P=a\r\x8EV[\x89Q=\x88\x82>=\x90\xFD[cNH{q`\xE0\x1B\x84R`\x11\x82R`$\x84\xFD[\x90P\x87\x81\x81=\x83\x11a\r\xFEW[a\r\xE4\x81\x83a\t[V[\x81\x01\x03\x12a\x04\x9CWQ`\xFF\x81\x16\x81\x03a\x04\x9CW`\xFFa\x0B\x01V[P=a\r\xDAV[\x90`\x01`\x01`\xA0\x1B\x03\x90`\0\x82\x81T\x16\x90`@\x93\x84Q\x92c1<\xE5g`\xE0\x1B\x84R` \x96`\x04\x94\x88\x81\x87\x81\x86Z\xFA\x90\x81\x15a\r\x0EW\x85\x91a\x0F\xBBW[P`\x14\x81\x01\x80\x91\x11a\x0F\xA8W`M\x81\x11a\x0F\xA8W\x87Qc\x18\x16\r\xDD`\xE0\x1B\x80\x82R\x91\x8A\x82\x89\x81\x88Z\xFA\x91\x82\x15a\x0F\x9EW\x87\x92a\x0FoW[P`\n\n\x10a\r\x03W\x88\x90\x86\x89Q\x80\x95\x81\x93\x82RZ\xFA\x91\x82\x15a\rFW\x90\x88\x92\x91\x85\x92a\x0F=W[Pa\x0E\xAF\x83\x94\x92`$\x92a\x14\xC3V[`\x01T\x89Qccsz\xC9`\xE1\x1B\x81R\x97\x88\x01\x91\x90\x91R\x86\x92\x83\x91\x16Z\xFA\x92\x83a\x0F\x14W[PPPa\x0C\xA8WP\x80\x7Fvault.convertToShares() must notf\x08\x1C\x99]\x99\\\x9D`\xCA\x1B\x92Q\x93a\x0C\x99\x85a\t?V[\x81=\x83\x11a\x0F6W[a\x0F'\x81\x83a\t[V[\x81\x01\x03\x12a\x02(W\x84\x81a\x0E\xD3V[P=a\x0F\x1DV[\x80\x92P\x83\x91\x93=\x83\x11a\x0FhW[a\x0FU\x81\x83a\t[V[\x81\x01\x03\x12a\x04\x9CWQ\x87\x91a\x0E\xAFa\x0E\xA0V[P=a\x0FKV[\x90\x91P\x8A\x81\x81=\x83\x11a\x0F\x97W[a\x0F\x87\x81\x83a\t[V[\x81\x01\x03\x12a\x07\xE9WQ\x908a\x0ExV[P=a\x0F}V[\x8AQ=\x89\x82>=\x90\xFD[cNH{q`\xE0\x1B\x85R`\x11\x86R`$\x85\xFD[\x90P\x88\x81\x81=\x83\x11a\x0F\xE1W[a\x0F\xD2\x81\x83a\t[V[\x81\x01\x03\x12a\r\x03WQ8a\x0EAV[P=a\x0F\xC8V[` `\x01`\x01`\xA0\x1B\x03`$\x81`\x01T\x16\x93`@Q\x94\x85\x93\x84\x92c@-&}`\xE0\x1B\x84R\x16`\x04\x83\x01RZ\xFA\x90\x81a\x10fW[Pa\t\xE9W`@Qa\x10,\x81a\t?V[`\"\x81R\x7Fvault.maxDeposit() must not reve` \x82\x01Ra\x1C\x9D`\xF2\x1B`@\x82\x01Ra\x14yV[` \x90\x81=\x81\x11a\x10\x8AW[a\x10|\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW8a\x10\x1BV[P=a\x10rV[` `\x01`\x01`\xA0\x1B\x03`$\x81`\x01T\x16\x93`@Q\x94\x85\x93\x84\x92cc\x1E\xBA\xDB`\xE1\x1B\x84R\x16`\x04\x83\x01RZ\xFA\x90\x81a\x11\x04W[Pa\t\xE9W`@Qa\x10\xD5\x81a\t#V[`\x1F\x81R\x7Fvault.maxMint() must not revert\0` \x82\x01Ra\x14yV[` \x90\x81=\x81\x11a\x11(W[a\x11\x1A\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW8a\x10\xC4V[P=a\x11\x10V[\x90`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16`@\x92\x83Q\x94cp\xA0\x821`\xE0\x1B\x86R\x16\x80`\x04\x86\x01R` \x94\x85\x81`$\x81\x86Z\xFA\x90\x81\x15a\x12VW`\0\x91a\x12aW[P\x84Q\x90c\x03\xD1h\x9D`\xE1\x1B\x82R`\x04\x82\x01R\x85\x81`$\x81\x86Z\xFA\x80\x15a\x12VW\x90\x86\x91a\x12-W[P\x80\x91`$\x86Q\x80\x95\x81\x93cl\x82\xBB\xBF`\xE1\x1B\x83R`\x04\x83\x01RZ\xFA\x91\x82a\x12\x04W[PPa\x0C\xA8WP\x80\x7Fvault.maxRedeem() must not rever`\x1D`\xFA\x1B\x92Q\x93a\x11\xF5\x85a\t?V[`!\x85R\x84\x01R\x82\x01Ra\x14yV[\x81=\x83\x11a\x12&W[a\x12\x17\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x838a\x11\xBBV[P=a\x12\rV[\x81=\x83\x11a\x12OW[a\x12@\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x848a\x11\x98V[P=a\x126V[\x85Q=`\0\x82>=\x90\xFD[\x90\x86\x82\x81=\x83\x11a\x12\x87W[a\x12w\x81\x83a\t[V[\x81\x01\x03\x12a\x02(WPQ8a\x11oV[P=a\x12mV[\x90`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16`@\x92\x83Q\x94cp\xA0\x821`\xE0\x1B\x86R\x16\x80`\x04\x86\x01R` \x94\x85\x81`$\x81\x86Z\xFA\x90\x81\x15a\x12VW`\0\x91a\x13\xB7W[P\x84Q\x90c\x03\xD1h\x9D`\xE1\x1B\x82R`\x04\x82\x01R\x85\x81`$\x81\x86Z\xFA\x80\x15a\x12VW\x90\x86\x91a\x13\x8EW[P\x80\x91`$\x86Q\x80\x95\x81\x93c\xCE\x96\xCBw`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x91\x82a\x13eW[PPa\x0C\xA8WP\x80\x7Fvault.maxWithdraw() must not revb\x19\\\x9D`\xEA\x1B\x92Q\x93a\x13V\x85a\t?V[`#\x85R\x84\x01R\x82\x01Ra\x14yV[\x81=\x83\x11a\x13\x87W[a\x13x\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x838a\x13\x1AV[P=a\x13nV[\x81=\x83\x11a\x13\xB0W[a\x13\xA1\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x848a\x12\xF7V[P=a\x13\x97V[\x90\x86\x82\x81=\x83\x11a\x13\xDDW[a\x13\xCD\x81\x83a\t[V[\x81\x01\x03\x12a\x02(WPQ8a\x12\xCEV[P=a\x13\xC3V[`\x03\x90\x06\x80\x15a\x14%W`\x01\x14a\x14\rWs\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\x90V[s\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\x90V[P0\x90V[`\0[\x83\x81\x10a\x14=WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x14-V[`@\x91` \x82Ra\x14m\x81Q\x80\x92\x81` \x86\x01R` \x86\x86\x01\x91\x01a\x14*V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[a\x14\xAA\x7F\xEB\x03\xCA\x8C\x87\xC7\x84\x9B\xEF\x8FT\xCF\xDD,k\x96{'4\xFE\x87/u\x19x\xC3K\xB9\x1E\x13\xD3Q\x91`@Q\x91\x82\x91\x82a\x14MV[\x03\x90\xA1cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x90\x80\x82\x11a\x14\xCFWP\x90V[`\x01\x81\x01\x80\x91\x11a\x15\xB8W\x80\x15a\x15\xA2Wa\x15\x9Ca\x15\x10\x7F\xA9^n*\x18$\x11\xE7\xA6\xF9\xED\x11J\x85\xC3v\x1D\x87\xF9\xB8\xF4S\xD8B\xC7\x125\xAAd\xFF\xF9\x9F\x92\x84\x06\x93a\x15\xCEV[a\x15\x90`3a\x15\x1E\x86a\x15\xCEV[\x92`@Q\x93\x84\x91\x7FClamping value \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01Ra\x15_\x81Q\x80\x92` `/\x87\x01\x91\x01a\x14*V[\x82\x01c\x01\x03\xA3y`\xE5\x1B`/\x82\x01Ra\x15\x81\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x14*V[\x01\x03`\x13\x81\x01\x84R\x01\x82a\t[V[`@Q\x91\x82\x91\x82a\x14MV[\x03\x90\xA1\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\x15\xE4W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \xB5\x9A\xD6[\xA1\xBFcB\x96B\x19\xF6?!\x13\xEF\xDE\x06\xDC\xA6/\x05Oh\xEC\x14\xAEv\x16\x14h\x85dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static CRYTICERC4626MUSTNOTREVERT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x04\x806\x10\x15a\0\x14W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x80c\x143\x08$\x14a\x08\xB0W\x80c\x1B.\xF1\xCA\x14a\x08\x1DW\x80c'#\xF9\xEE\x14a\x06\x94W\x80c/\xFD]F\x14a\x06wW\x80cI\xB6\x92\xDC\x14a\x06ZW\x80c]#\x88\xE0\x14a\x06-W\x80c\x8B\x96\x99\xED\x14a\x06\0W\x83\x81c\x96\xEB'\xA1\x14a\x05AWP\x80c\xA2y\xC0\xEE\x14a\x05\x14W\x80c\xA4\x1F\xE4\x9F\x14a\x04\xB9W\x80c\xA6\xC8;\x03\x14a\x04\xA0W\x80c\xA8\x15\xC1\x0F\x14a\x03\x8CW\x80c\xB8\x19\"\x05\x14a\x03 W\x83\x81c\xB8\x8D\xAB2\x14a\x02\xA6WP\x80c\xC01\r\x7F\x14a\x02rW\x83\x81c\xCC\xC9J\xE9\x14a\x01\x8DWPc\xE2\xBB\xB1X\x14a\0\xE1W`\0\x80\xFD[4a\x01\x89W` a\x01\0a\x01?\x93a\0\xF86a\x08\xC9V[\x92\x90\x92a\x13\xE4V[\x91\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93cnU?e`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x90\x81\x15a\x01\x80WPa\x01SWP\x80\xF3[` \x90\x81=\x81\x11a\x01yW[a\x01i\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x80\xF3[`\0\x80\xFD[P=a\x01_V[Q=\x84\x82>=\x90\xFD[\x82\x80\xFD[\x80\x84\x844a\x02nW` 6`\x03\x19\x01\x12a\x02nW`\xFF\x82T`\xA0\x1C\x16\x15a\x02nW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x02dW\x85\x91a\x020W[P\x15a\x02+W\x82;\x15a\x02+W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cU\xDFp\r`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01\x80WPa\x02\x18WP\xF3[a\x02!\x90a\x08\xF9V[a\x02(W\x80\xF3[\x80\xFD[PPP\xFD[\x94PP` \x84=\x82\x11a\x02\\W[\x81a\x02K` \x93\x83a\t[V[\x81\x01\x03\x12a\x01tW\x84\x93Q\x86a\x01\xDDV[=\x91Pa\x02>V[\x83Q=\x87\x82>=\x90\xFD[PP\xFD[PP4a\x02\xA2W` 6`\x03\x19\x01\x12a\x02\xA2W5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xA2Wa\x02\x9F\x90a\x12\x8EV[\x80\xF3[P\x80\xFD[\x80\x84\x844a\x02nWa\x02\xC3\x91a\x02\xBB6a\x08\xC9V[\x93\x90\x93a\x13\xE4V[\x92`\x01`\x01`\xA0\x1B\x03\x85T\x16\x80;\x15a\x03\x1CW\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x92\x85\x01\x92\x83R` \x83\x01\x91\x90\x91R\x84\x91\x84\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x01\x80WPa\x02\x18WP\xF3[\x85\x80\xFD[P4a\x01\x89W` a\x03Ca\x03I\x93a\x0386a\x08\xDFV[\x96\x91\x96\x93\x90\x93a\x13\xE4V[\x92a\x13\xE4V[`\x01T\x85Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\x01?V[P\x904a\x01\x89W` \x91\x82`\x03\x196\x01\x12a\x04\x9CW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qcp\xA0\x821`\xE0\x1B\x81R0\x82\x82\x01R\x84\x81`$\x81\x87Z\xFA\x90\x81\x15a\x04\x92W\x90\x85\x92\x91\x87\x91a\x04_W[P\x93a\x03\xE9a\x04\x1A\x95\x835a\x14\xC3V[\x84Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x90\x81R0` \x82\x01\x81\x90R`@\x82\x01R\x91\x94\x85\x92\x83\x91\x89\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x90\x81\x15a\x04VWPa\x04.W\x82\x80\xF3[\x81=\x83\x11a\x04OW[a\x04A\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW8\x80\x82\x80\xF3[P=a\x047V[Q=\x85\x82>=\x90\xFD[\x83\x81\x94\x92P=\x83\x11a\x04\x8BW[a\x04v\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x90Q\x84\x91\x90a\x03\xE9a\x03\xD9V[P=a\x04lV[\x83Q=\x88\x82>=\x90\xFD[\x83\x80\xFD[\x834a\x02(W\x80`\x03\x196\x01\x12a\x02(Wa\x02\x9Fa\t}V[P4a\x01\x89W` a\x03Ca\x04\xD1\x93a\x0386a\x08\xDFV[`\x01T\x85Qc-\x18+\xE5`\xE2\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\x01?V[PP4a\x02\xA2W` 6`\x03\x19\x01\x12a\x02\xA2W5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xA2Wa\x02\x9F\x90a\x10\x91V[\x80\x84\x844a\x02nW` 6`\x03\x19\x01\x12a\x02nW`\xFF\x82T`\xA0\x1C\x16\x15a\x02nW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x02dW\x85\x91a\x05\xCCW[P\x15a\x02+W\x82;\x15a\x02+W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cb!\xE4\xF1`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01\x80WPa\x02\x18WP\xF3[\x94PP` \x84=\x82\x11a\x05\xF8W[\x81a\x05\xE7` \x93\x83a\t[V[\x81\x01\x03\x12a\x01tW\x84\x93Q\x86a\x05\x91V[=\x91Pa\x05\xDAV[PP4a\x02\xA2W` 6`\x03\x19\x01\x12a\x02\xA2W5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xA2Wa\x02\x9F\x90a\x0F\xE8V[PP4a\x02\xA2W` 6`\x03\x19\x01\x12a\x02\xA2W5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\xA2Wa\x02\x9F\x90a\x11/V[PP4a\x02\xA2W` 6`\x03\x19\x01\x12a\x02\xA2Wa\x02\x9F\x905a\n\xC3V[PP4a\x02\xA2W` 6`\x03\x19\x01\x12a\x02\xA2Wa\x02\x9F\x905a\x0E\x05V[P\x82\x904a\x02\xA2W` \x90\x81`\x03\x196\x01\x12a\x01\x89W\x835\x93`\x01`\x01`\xA0\x1B\x03\x94\x85\x85T\x16\x80;\x15a\x03\x1CW\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x84\x82\x01\x90\x81R` \x81\x01\x84\x90R\x90\x91\x87\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x08\x13Wa\x07\xFEW[P\x85\x84\x82\x87\x98a\x07=\x97\x98T\x16\x83`\x01T\x16\x8A\x88Q\x80\x9A\x81\x95\x82\x94c\t^\xA7\xB3`\xE0\x1B\x84R\x8B\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x07\xF4W\x90\x86\x93\x92\x91a\x07\xB8W[`\x01T\x85QcnU?e`\xE0\x1B\x81R\x93\x84\x01\x92\x83R0` \x84\x01R\x92\x95P\x85\x92\x16\x90\x82\x90\x88\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x04VWPa\x07\x90W\x82\x80\xF3[\x81=\x83\x11a\x07\xB1W[a\x07\xA3\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x81\x80\x82\x80\xF3[P=a\x07\x99V[\x92\x85\x81\x96\x92\x96=\x83\x11a\x07\xEDW[a\x07\xD0\x81\x83a\t[V[\x81\x01\x03\x12a\x07\xE9WQ\x80\x15\x15\x03a\x03\x1CW\x83\x85\x92a\x07QV[\x86\x80\xFD[P=a\x07\xC6V[\x84Q=\x89\x82>=\x90\xFD[\x94a\x08\x0Ca\x07=\x95\x96a\x08\xF9V[\x94\x93a\x06\xFAV[\x84Q=\x88\x82>=\x90\xFD[P4a\x01\x89W` a\x085a\x08t\x93a\0\xF86a\x08\xC9V[\x91\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93c\x94\xBF\x80M`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x90\x81\x15a\x01\x80WPa\x08\x88WP\x80\xF3[` \x90\x81=\x81\x11a\x08\xA9W[a\x08\x9E\x81\x83a\t[V[\x81\x01\x03\x12a\x02(W\x80\xF3[P=a\x08\x94V[\x834a\x02(W\x80`\x03\x196\x01\x12a\x02(Wa\x02\x9Fa\n#V[`@\x90`\x03\x19\x01\x12a\x01tW`\x045\x90`$5\x90V[``\x90`\x03\x19\x01\x12a\x01tW`\x045\x90`$5\x90`D5\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\t\rW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\rW`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\rW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\t\rW`@RV[`\x04`\x01`\x01`\xA0\x1B\x03` \x81`\x01T\x16`@Q\x93\x84\x80\x92c8\xD5.\x0F`\xE0\x1B\x82RZ\xFA\x91\x82a\t\xEBW[PPa\t\xE9W`@Qa\t\xBA\x81a\t#V[`\x1D\x81R\x7Fvault.asset() must not revert\0\0\0` \x82\x01Ra\x14yV[V[` \x81=\x82\x11a\n\x1BW[\x81a\n\x03` \x93\x83a\t[V[\x81\x01\x03\x12a\x01tWQ\x90\x81\x16\x03a\x01tW8\x80a\t\xA8V[=\x91Pa\t\xF6V[`\x04` `\x01`\x01`\xA0\x1B\x03`\x01T\x16`@Q\x92\x83\x80\x92bxtE`\xE2\x1B\x82RZ\xFA\x90\x81a\n\x98W[Pa\t\xE9W`@Qa\n]\x81a\t?V[`#\x81R\x7Fvault.totalAssets() must not rev` \x82\x01Rb\x19\\\x9D`\xEA\x1B`@\x82\x01Ra\x14yV[` \x90\x81=\x81\x11a\n\xBCW[a\n\xAE\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW8a\nLV[P=a\n\xA4V[\x90`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16\x91`@\x92\x83Q\x94c1<\xE5g`\xE0\x1B\x86R` \x95`\x04\x90\x87\x81\x83\x81`\0\x80\x98Z\xF1\x90\x81\x15a\rFW\x84\x91a\r\xCDW[P`\xFF`\x14\x91\x16\x01`\xFF\x81\x11a\r\xBAW`\xFF\x16`M\x81\x11a\r\xBAW`\n\n\x93`\x01T\x16\x93\x86Q\x92c\x18\x16\r\xDD`\xE0\x1B\x93\x84\x81R\x89\x81\x85\x81\x8AZ\xFA\x90\x81\x15a\r\xB0W\x90\x83\x91\x87\x91a\r\x7FW[P\x11a\r\x03W\x90a\x0B\\\x91a\x14\xC3V[\x86Q\x83\x81R\x88\x81\x84\x81\x89Z\xFA\x90\x81\x15a\r\x0EW\x85\x91a\rPW[Pa\x0B\x80\x91a\x14\xC3V[\x91\x86Q\x90\x81R\x87\x81\x83\x81\x88Z\xFA\x80\x15a\rFW\x84\x90a\r\x18W[`\x80\x91P\x87Q\x90\x88\x82R`\x0B\x89\x83\x01RjtotalSupply`\xA8\x1B``\x83\x01R\x89\x82\x01R\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x91\x82\x91\xA1\x86QbxtE`\xE2\x1B\x81R\x88\x81\x84\x81\x89Z\xFA\x90\x81\x15a\r\x0EW\x90\x89\x93\x92\x91\x86\x91a\x0C\xD6W[P\x91`$\x91`\x80\x85\x96\x94\x8BQ\x90\x8C\x82R`\x0B\x8D\x83\x01RjtotalAssets`\xA8\x1B``\x83\x01R\x87\x82\x01R\xA1\x88Q\x96\x87\x93\x84\x92c\x03\xD1h\x9D`\xE1\x1B\x84R\x83\x01RZ\xFA\x92\x83a\x0C\xADW[PPPa\x0C\xA8WP\x80\x7Fvault.convertToAssets() must notf\x08\x1C\x99]\x99\\\x9D`\xCA\x1B\x92Q\x93a\x0C\x99\x85a\t?V[`'\x85R\x84\x01R\x82\x01Ra\x14yV[\x91PPV[\x81=\x83\x11a\x0C\xCFW[a\x0C\xC0\x81\x83a\t[V[\x81\x01\x03\x12a\x02(W\x84\x81a\x0CXV[P=a\x0C\xB6V[\x84\x81\x93\x94\x95\x92P=\x83\x11a\r\x07W[a\x0C\xEF\x81\x83a\t[V[\x81\x01\x03\x12a\r\x03WQ\x88\x92\x91\x90`$a\x0C\x0CV[\x84\x80\xFD[P=a\x0C\xE5V[\x88Q=\x87\x82>=\x90\xFD[P\x87\x81\x81=\x83\x11a\r?W[a\r.\x81\x83a\t[V[\x81\x01\x03\x12a\x04\x9CW`\x80\x90Qa\x0B\x9AV[P=a\r$V[\x87Q=\x86\x82>=\x90\xFD[\x90P\x88\x81\x81=\x83\x11a\rxW[a\rg\x81\x83a\t[V[\x81\x01\x03\x12a\r\x03WQa\x0B\x80a\x0BvV[P=a\r]V[\x80\x92P\x8B\x80\x92P=\x83\x11a\r\xA9W[a\r\x98\x81\x83a\t[V[\x81\x01\x03\x12a\x03\x1CW\x82\x90Q8a\x0BLV[P=a\r\x8EV[\x89Q=\x88\x82>=\x90\xFD[cNH{q`\xE0\x1B\x84R`\x11\x82R`$\x84\xFD[\x90P\x87\x81\x81=\x83\x11a\r\xFEW[a\r\xE4\x81\x83a\t[V[\x81\x01\x03\x12a\x04\x9CWQ`\xFF\x81\x16\x81\x03a\x04\x9CW`\xFFa\x0B\x01V[P=a\r\xDAV[\x90`\x01`\x01`\xA0\x1B\x03\x90`\0\x82\x81T\x16\x90`@\x93\x84Q\x92c1<\xE5g`\xE0\x1B\x84R` \x96`\x04\x94\x88\x81\x87\x81\x86Z\xFA\x90\x81\x15a\r\x0EW\x85\x91a\x0F\xBBW[P`\x14\x81\x01\x80\x91\x11a\x0F\xA8W`M\x81\x11a\x0F\xA8W\x87Qc\x18\x16\r\xDD`\xE0\x1B\x80\x82R\x91\x8A\x82\x89\x81\x88Z\xFA\x91\x82\x15a\x0F\x9EW\x87\x92a\x0FoW[P`\n\n\x10a\r\x03W\x88\x90\x86\x89Q\x80\x95\x81\x93\x82RZ\xFA\x91\x82\x15a\rFW\x90\x88\x92\x91\x85\x92a\x0F=W[Pa\x0E\xAF\x83\x94\x92`$\x92a\x14\xC3V[`\x01T\x89Qccsz\xC9`\xE1\x1B\x81R\x97\x88\x01\x91\x90\x91R\x86\x92\x83\x91\x16Z\xFA\x92\x83a\x0F\x14W[PPPa\x0C\xA8WP\x80\x7Fvault.convertToShares() must notf\x08\x1C\x99]\x99\\\x9D`\xCA\x1B\x92Q\x93a\x0C\x99\x85a\t?V[\x81=\x83\x11a\x0F6W[a\x0F'\x81\x83a\t[V[\x81\x01\x03\x12a\x02(W\x84\x81a\x0E\xD3V[P=a\x0F\x1DV[\x80\x92P\x83\x91\x93=\x83\x11a\x0FhW[a\x0FU\x81\x83a\t[V[\x81\x01\x03\x12a\x04\x9CWQ\x87\x91a\x0E\xAFa\x0E\xA0V[P=a\x0FKV[\x90\x91P\x8A\x81\x81=\x83\x11a\x0F\x97W[a\x0F\x87\x81\x83a\t[V[\x81\x01\x03\x12a\x07\xE9WQ\x908a\x0ExV[P=a\x0F}V[\x8AQ=\x89\x82>=\x90\xFD[cNH{q`\xE0\x1B\x85R`\x11\x86R`$\x85\xFD[\x90P\x88\x81\x81=\x83\x11a\x0F\xE1W[a\x0F\xD2\x81\x83a\t[V[\x81\x01\x03\x12a\r\x03WQ8a\x0EAV[P=a\x0F\xC8V[` `\x01`\x01`\xA0\x1B\x03`$\x81`\x01T\x16\x93`@Q\x94\x85\x93\x84\x92c@-&}`\xE0\x1B\x84R\x16`\x04\x83\x01RZ\xFA\x90\x81a\x10fW[Pa\t\xE9W`@Qa\x10,\x81a\t?V[`\"\x81R\x7Fvault.maxDeposit() must not reve` \x82\x01Ra\x1C\x9D`\xF2\x1B`@\x82\x01Ra\x14yV[` \x90\x81=\x81\x11a\x10\x8AW[a\x10|\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW8a\x10\x1BV[P=a\x10rV[` `\x01`\x01`\xA0\x1B\x03`$\x81`\x01T\x16\x93`@Q\x94\x85\x93\x84\x92cc\x1E\xBA\xDB`\xE1\x1B\x84R\x16`\x04\x83\x01RZ\xFA\x90\x81a\x11\x04W[Pa\t\xE9W`@Qa\x10\xD5\x81a\t#V[`\x1F\x81R\x7Fvault.maxMint() must not revert\0` \x82\x01Ra\x14yV[` \x90\x81=\x81\x11a\x11(W[a\x11\x1A\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW8a\x10\xC4V[P=a\x11\x10V[\x90`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16`@\x92\x83Q\x94cp\xA0\x821`\xE0\x1B\x86R\x16\x80`\x04\x86\x01R` \x94\x85\x81`$\x81\x86Z\xFA\x90\x81\x15a\x12VW`\0\x91a\x12aW[P\x84Q\x90c\x03\xD1h\x9D`\xE1\x1B\x82R`\x04\x82\x01R\x85\x81`$\x81\x86Z\xFA\x80\x15a\x12VW\x90\x86\x91a\x12-W[P\x80\x91`$\x86Q\x80\x95\x81\x93cl\x82\xBB\xBF`\xE1\x1B\x83R`\x04\x83\x01RZ\xFA\x91\x82a\x12\x04W[PPa\x0C\xA8WP\x80\x7Fvault.maxRedeem() must not rever`\x1D`\xFA\x1B\x92Q\x93a\x11\xF5\x85a\t?V[`!\x85R\x84\x01R\x82\x01Ra\x14yV[\x81=\x83\x11a\x12&W[a\x12\x17\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x838a\x11\xBBV[P=a\x12\rV[\x81=\x83\x11a\x12OW[a\x12@\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x848a\x11\x98V[P=a\x126V[\x85Q=`\0\x82>=\x90\xFD[\x90\x86\x82\x81=\x83\x11a\x12\x87W[a\x12w\x81\x83a\t[V[\x81\x01\x03\x12a\x02(WPQ8a\x11oV[P=a\x12mV[\x90`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16`@\x92\x83Q\x94cp\xA0\x821`\xE0\x1B\x86R\x16\x80`\x04\x86\x01R` \x94\x85\x81`$\x81\x86Z\xFA\x90\x81\x15a\x12VW`\0\x91a\x13\xB7W[P\x84Q\x90c\x03\xD1h\x9D`\xE1\x1B\x82R`\x04\x82\x01R\x85\x81`$\x81\x86Z\xFA\x80\x15a\x12VW\x90\x86\x91a\x13\x8EW[P\x80\x91`$\x86Q\x80\x95\x81\x93c\xCE\x96\xCBw`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x91\x82a\x13eW[PPa\x0C\xA8WP\x80\x7Fvault.maxWithdraw() must not revb\x19\\\x9D`\xEA\x1B\x92Q\x93a\x13V\x85a\t?V[`#\x85R\x84\x01R\x82\x01Ra\x14yV[\x81=\x83\x11a\x13\x87W[a\x13x\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x838a\x13\x1AV[P=a\x13nV[\x81=\x83\x11a\x13\xB0W[a\x13\xA1\x81\x83a\t[V[\x81\x01\x03\x12a\x01tW\x848a\x12\xF7V[P=a\x13\x97V[\x90\x86\x82\x81=\x83\x11a\x13\xDDW[a\x13\xCD\x81\x83a\t[V[\x81\x01\x03\x12a\x02(WPQ8a\x12\xCEV[P=a\x13\xC3V[`\x03\x90\x06\x80\x15a\x14%W`\x01\x14a\x14\rWs\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\x90V[s\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\x90V[P0\x90V[`\0[\x83\x81\x10a\x14=WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x14-V[`@\x91` \x82Ra\x14m\x81Q\x80\x92\x81` \x86\x01R` \x86\x86\x01\x91\x01a\x14*V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[a\x14\xAA\x7F\xEB\x03\xCA\x8C\x87\xC7\x84\x9B\xEF\x8FT\xCF\xDD,k\x96{'4\xFE\x87/u\x19x\xC3K\xB9\x1E\x13\xD3Q\x91`@Q\x91\x82\x91\x82a\x14MV[\x03\x90\xA1cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x90\x80\x82\x11a\x14\xCFWP\x90V[`\x01\x81\x01\x80\x91\x11a\x15\xB8W\x80\x15a\x15\xA2Wa\x15\x9Ca\x15\x10\x7F\xA9^n*\x18$\x11\xE7\xA6\xF9\xED\x11J\x85\xC3v\x1D\x87\xF9\xB8\xF4S\xD8B\xC7\x125\xAAd\xFF\xF9\x9F\x92\x84\x06\x93a\x15\xCEV[a\x15\x90`3a\x15\x1E\x86a\x15\xCEV[\x92`@Q\x93\x84\x91\x7FClamping value \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x84\x01Ra\x15_\x81Q\x80\x92` `/\x87\x01\x91\x01a\x14*V[\x82\x01c\x01\x03\xA3y`\xE5\x1B`/\x82\x01Ra\x15\x81\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x14*V[\x01\x03`\x13\x81\x01\x84R\x01\x82a\t[V[`@Q\x91\x82\x91\x82a\x14MV[\x03\x90\xA1\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\x15\xE4W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \xB5\x9A\xD6[\xA1\xBFcB\x96B\x19\xF6?!\x13\xEF\xDE\x06\xDC\xA6/\x05Oh\xEC\x14\xAEv\x16\x14h\x85dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static CRYTICERC4626MUSTNOTREVERT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CryticERC4626MustNotRevert<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CryticERC4626MustNotRevert<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CryticERC4626MustNotRevert<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CryticERC4626MustNotRevert<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CryticERC4626MustNotRevert<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CryticERC4626MustNotRevert))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CryticERC4626MustNotRevert<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CRYTICERC4626MUSTNOTREVERT_ABI.clone(),
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
                CRYTICERC4626MUSTNOTREVERT_ABI.clone(),
                CRYTICERC4626MUSTNOTREVERT_BYTECODE.clone().into(),
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
        ///Calls the contract's `verify_assetMustNotRevert` (0xa6c83b03) function
        pub fn verify_asset_must_not_revert(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 200, 59, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_convertToAssetsMustNotRevert` (0x49b692dc) function
        pub fn verify_convert_to_assets_must_not_revert(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 182, 146, 220], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_convertToSharesMustNotRevert` (0x2ffd5d46) function
        pub fn verify_convert_to_shares_must_not_revert(
            &self,
            tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 253, 93, 70], tokens)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_maxDepositMustNotRevert` (0x8b9699ed) function
        pub fn verify_max_deposit_must_not_revert(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 150, 153, 237], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_maxMintMustNotRevert` (0xa279c0ee) function
        pub fn verify_max_mint_must_not_revert(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 121, 192, 238], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_maxRedeemMustNotRevert` (0x5d2388e0) function
        pub fn verify_max_redeem_must_not_revert(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 35, 136, 224], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_maxWithdrawMustNotRevert` (0xc0310d7f) function
        pub fn verify_max_withdraw_must_not_revert(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 49, 13, 127], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_totalAssetsMustNotRevert` (0x14330824) function
        pub fn verify_total_assets_must_not_revert(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 51, 8, 36], ())
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
            CryticERC4626MustNotRevertEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CryticERC4626MustNotRevert<M> {
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
    pub enum CryticERC4626MustNotRevertEvents {
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
    impl ::ethers::contract::EthLogDecode for CryticERC4626MustNotRevertEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssertEqFailFilter::decode_log(log) {
                return Ok(CryticERC4626MustNotRevertEvents::AssertEqFailFilter(decoded));
            }
            if let Ok(decoded) = AssertFailFilter::decode_log(log) {
                return Ok(CryticERC4626MustNotRevertEvents::AssertFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGtFailFilter::decode_log(log) {
                return Ok(CryticERC4626MustNotRevertEvents::AssertGtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGteFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626MustNotRevertEvents::AssertGteFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertLtFailFilter::decode_log(log) {
                return Ok(CryticERC4626MustNotRevertEvents::AssertLtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLteFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626MustNotRevertEvents::AssertLteFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertNeqFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626MustNotRevertEvents::AssertNeqFailFilter(decoded),
                );
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(CryticERC4626MustNotRevertEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(CryticERC4626MustNotRevertEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUint256Filter::decode_log(log) {
                return Ok(CryticERC4626MustNotRevertEvents::LogUint256Filter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CryticERC4626MustNotRevertEvents {
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
    impl ::core::convert::From<AssertEqFailFilter> for CryticERC4626MustNotRevertEvents {
        fn from(value: AssertEqFailFilter) -> Self {
            Self::AssertEqFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertFailFilter> for CryticERC4626MustNotRevertEvents {
        fn from(value: AssertFailFilter) -> Self {
            Self::AssertFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGtFailFilter> for CryticERC4626MustNotRevertEvents {
        fn from(value: AssertGtFailFilter) -> Self {
            Self::AssertGtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGteFailFilter>
    for CryticERC4626MustNotRevertEvents {
        fn from(value: AssertGteFailFilter) -> Self {
            Self::AssertGteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLtFailFilter> for CryticERC4626MustNotRevertEvents {
        fn from(value: AssertLtFailFilter) -> Self {
            Self::AssertLtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLteFailFilter>
    for CryticERC4626MustNotRevertEvents {
        fn from(value: AssertLteFailFilter) -> Self {
            Self::AssertLteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertNeqFailFilter>
    for CryticERC4626MustNotRevertEvents {
        fn from(value: AssertNeqFailFilter) -> Self {
            Self::AssertNeqFailFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for CryticERC4626MustNotRevertEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for CryticERC4626MustNotRevertEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUint256Filter> for CryticERC4626MustNotRevertEvents {
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
    ///Container type for all input parameters for the `verify_assetMustNotRevert` function with signature `verify_assetMustNotRevert()` and selector `0xa6c83b03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "verify_assetMustNotRevert", abi = "verify_assetMustNotRevert()")]
    pub struct VerifyAssetMustNotRevertCall;
    ///Container type for all input parameters for the `verify_convertToAssetsMustNotRevert` function with signature `verify_convertToAssetsMustNotRevert(uint256)` and selector `0x49b692dc`
    #[derive(
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
        name = "verify_convertToAssetsMustNotRevert",
        abi = "verify_convertToAssetsMustNotRevert(uint256)"
    )]
    pub struct VerifyConvertToAssetsMustNotRevertCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_convertToSharesMustNotRevert` function with signature `verify_convertToSharesMustNotRevert(uint256)` and selector `0x2ffd5d46`
    #[derive(
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
        name = "verify_convertToSharesMustNotRevert",
        abi = "verify_convertToSharesMustNotRevert(uint256)"
    )]
    pub struct VerifyConvertToSharesMustNotRevertCall {
        pub tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_maxDepositMustNotRevert` function with signature `verify_maxDepositMustNotRevert(address)` and selector `0x8b9699ed`
    #[derive(
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
        name = "verify_maxDepositMustNotRevert",
        abi = "verify_maxDepositMustNotRevert(address)"
    )]
    pub struct VerifyMaxDepositMustNotRevertCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `verify_maxMintMustNotRevert` function with signature `verify_maxMintMustNotRevert(address)` and selector `0xa279c0ee`
    #[derive(
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
        name = "verify_maxMintMustNotRevert",
        abi = "verify_maxMintMustNotRevert(address)"
    )]
    pub struct VerifyMaxMintMustNotRevertCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `verify_maxRedeemMustNotRevert` function with signature `verify_maxRedeemMustNotRevert(address)` and selector `0x5d2388e0`
    #[derive(
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
        name = "verify_maxRedeemMustNotRevert",
        abi = "verify_maxRedeemMustNotRevert(address)"
    )]
    pub struct VerifyMaxRedeemMustNotRevertCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `verify_maxWithdrawMustNotRevert` function with signature `verify_maxWithdrawMustNotRevert(address)` and selector `0xc0310d7f`
    #[derive(
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
        name = "verify_maxWithdrawMustNotRevert",
        abi = "verify_maxWithdrawMustNotRevert(address)"
    )]
    pub struct VerifyMaxWithdrawMustNotRevertCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `verify_totalAssetsMustNotRevert` function with signature `verify_totalAssetsMustNotRevert()` and selector `0x14330824`
    #[derive(
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
        name = "verify_totalAssetsMustNotRevert",
        abi = "verify_totalAssetsMustNotRevert()"
    )]
    pub struct VerifyTotalAssetsMustNotRevertCall;
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
    pub enum CryticERC4626MustNotRevertCalls {
        Deposit(DepositCall),
        DepositForSelfSimple(DepositForSelfSimpleCall),
        Mint(MintCall),
        MintAsset(MintAssetCall),
        RecognizeLossProxy(RecognizeLossProxyCall),
        RecognizeProfitProxy(RecognizeProfitProxyCall),
        Redeem(RedeemCall),
        RedeemForSelfSimple(RedeemForSelfSimpleCall),
        VerifyAssetMustNotRevert(VerifyAssetMustNotRevertCall),
        VerifyConvertToAssetsMustNotRevert(VerifyConvertToAssetsMustNotRevertCall),
        VerifyConvertToSharesMustNotRevert(VerifyConvertToSharesMustNotRevertCall),
        VerifyMaxDepositMustNotRevert(VerifyMaxDepositMustNotRevertCall),
        VerifyMaxMintMustNotRevert(VerifyMaxMintMustNotRevertCall),
        VerifyMaxRedeemMustNotRevert(VerifyMaxRedeemMustNotRevertCall),
        VerifyMaxWithdrawMustNotRevert(VerifyMaxWithdrawMustNotRevertCall),
        VerifyTotalAssetsMustNotRevert(VerifyTotalAssetsMustNotRevertCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for CryticERC4626MustNotRevertCalls {
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
                = <VerifyAssetMustNotRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyAssetMustNotRevert(decoded));
            }
            if let Ok(decoded)
                = <VerifyConvertToAssetsMustNotRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyConvertToAssetsMustNotRevert(decoded));
            }
            if let Ok(decoded)
                = <VerifyConvertToSharesMustNotRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyConvertToSharesMustNotRevert(decoded));
            }
            if let Ok(decoded)
                = <VerifyMaxDepositMustNotRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyMaxDepositMustNotRevert(decoded));
            }
            if let Ok(decoded)
                = <VerifyMaxMintMustNotRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyMaxMintMustNotRevert(decoded));
            }
            if let Ok(decoded)
                = <VerifyMaxRedeemMustNotRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyMaxRedeemMustNotRevert(decoded));
            }
            if let Ok(decoded)
                = <VerifyMaxWithdrawMustNotRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyMaxWithdrawMustNotRevert(decoded));
            }
            if let Ok(decoded)
                = <VerifyTotalAssetsMustNotRevertCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyTotalAssetsMustNotRevert(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CryticERC4626MustNotRevertCalls {
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
                Self::VerifyAssetMustNotRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyConvertToAssetsMustNotRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyConvertToSharesMustNotRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyMaxDepositMustNotRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyMaxMintMustNotRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyMaxRedeemMustNotRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyMaxWithdrawMustNotRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyTotalAssetsMustNotRevert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CryticERC4626MustNotRevertCalls {
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
                Self::VerifyAssetMustNotRevert(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyConvertToAssetsMustNotRevert(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyConvertToSharesMustNotRevert(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyMaxDepositMustNotRevert(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyMaxMintMustNotRevert(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyMaxRedeemMustNotRevert(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyMaxWithdrawMustNotRevert(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyTotalAssetsMustNotRevert(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositCall> for CryticERC4626MustNotRevertCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositForSelfSimpleCall>
    for CryticERC4626MustNotRevertCalls {
        fn from(value: DepositForSelfSimpleCall) -> Self {
            Self::DepositForSelfSimple(value)
        }
    }
    impl ::core::convert::From<MintCall> for CryticERC4626MustNotRevertCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<MintAssetCall> for CryticERC4626MustNotRevertCalls {
        fn from(value: MintAssetCall) -> Self {
            Self::MintAsset(value)
        }
    }
    impl ::core::convert::From<RecognizeLossProxyCall>
    for CryticERC4626MustNotRevertCalls {
        fn from(value: RecognizeLossProxyCall) -> Self {
            Self::RecognizeLossProxy(value)
        }
    }
    impl ::core::convert::From<RecognizeProfitProxyCall>
    for CryticERC4626MustNotRevertCalls {
        fn from(value: RecognizeProfitProxyCall) -> Self {
            Self::RecognizeProfitProxy(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for CryticERC4626MustNotRevertCalls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<RedeemForSelfSimpleCall>
    for CryticERC4626MustNotRevertCalls {
        fn from(value: RedeemForSelfSimpleCall) -> Self {
            Self::RedeemForSelfSimple(value)
        }
    }
    impl ::core::convert::From<VerifyAssetMustNotRevertCall>
    for CryticERC4626MustNotRevertCalls {
        fn from(value: VerifyAssetMustNotRevertCall) -> Self {
            Self::VerifyAssetMustNotRevert(value)
        }
    }
    impl ::core::convert::From<VerifyConvertToAssetsMustNotRevertCall>
    for CryticERC4626MustNotRevertCalls {
        fn from(value: VerifyConvertToAssetsMustNotRevertCall) -> Self {
            Self::VerifyConvertToAssetsMustNotRevert(value)
        }
    }
    impl ::core::convert::From<VerifyConvertToSharesMustNotRevertCall>
    for CryticERC4626MustNotRevertCalls {
        fn from(value: VerifyConvertToSharesMustNotRevertCall) -> Self {
            Self::VerifyConvertToSharesMustNotRevert(value)
        }
    }
    impl ::core::convert::From<VerifyMaxDepositMustNotRevertCall>
    for CryticERC4626MustNotRevertCalls {
        fn from(value: VerifyMaxDepositMustNotRevertCall) -> Self {
            Self::VerifyMaxDepositMustNotRevert(value)
        }
    }
    impl ::core::convert::From<VerifyMaxMintMustNotRevertCall>
    for CryticERC4626MustNotRevertCalls {
        fn from(value: VerifyMaxMintMustNotRevertCall) -> Self {
            Self::VerifyMaxMintMustNotRevert(value)
        }
    }
    impl ::core::convert::From<VerifyMaxRedeemMustNotRevertCall>
    for CryticERC4626MustNotRevertCalls {
        fn from(value: VerifyMaxRedeemMustNotRevertCall) -> Self {
            Self::VerifyMaxRedeemMustNotRevert(value)
        }
    }
    impl ::core::convert::From<VerifyMaxWithdrawMustNotRevertCall>
    for CryticERC4626MustNotRevertCalls {
        fn from(value: VerifyMaxWithdrawMustNotRevertCall) -> Self {
            Self::VerifyMaxWithdrawMustNotRevert(value)
        }
    }
    impl ::core::convert::From<VerifyTotalAssetsMustNotRevertCall>
    for CryticERC4626MustNotRevertCalls {
        fn from(value: VerifyTotalAssetsMustNotRevertCall) -> Self {
            Self::VerifyTotalAssetsMustNotRevert(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for CryticERC4626MustNotRevertCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
}
