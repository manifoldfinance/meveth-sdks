pub use actor::*;
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
pub mod actor {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_vault"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IERC4626"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("approveFunds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approveFunds"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositFunds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositFunds"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sharesMinted"),
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
                    ::std::borrow::ToOwned::to_owned("depositFundsOnBehalf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "depositFundsOnBehalf",
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sharesMinted"),
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
                    ::std::borrow::ToOwned::to_owned("fund"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fund"),
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
                    ::std::borrow::ToOwned::to_owned("mintShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintShares"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokensDeposited"),
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
                    ::std::borrow::ToOwned::to_owned("mintSharesOnBehalf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintSharesOnBehalf"),
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
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokensDeposited"),
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
                    ::std::borrow::ToOwned::to_owned("redeemShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeemShares"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokensWithdrawn"),
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
                    ::std::borrow::ToOwned::to_owned("redeemSharesOnBehalf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "redeemSharesOnBehalf",
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokensWithdrawn"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawTokens"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sharesBurned"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawTokensOnBehalf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawTokensOnBehalf",
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sharesBurned"),
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
    pub static ACTOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\xE3W` \x81a\x0B\xA6\x808\x03\x80\x91a\0\x1F\x82\x85a\0\xE8V[\x839\x81\x01\x03\x12a\0\xE3WQ`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x91\x82\x90\x03a\0\xE3W`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x84\x17\x90\x91U`@Qc8\xD5.\x0F`\xE0\x1B\x81R\x90\x92` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x90\x81\x15a\0\xD7W`\0\x91a\0\x96W[P\x16\x90`\0T\x16\x17`\0U`@Qa\n\x84\x90\x81a\x01\"\x829\xF3[` \x81=\x82\x11a\0\xCFW[\x81a\0\xAE` \x93\x83a\0\xE8V[\x81\x01\x03\x12a\0\xCBWQ\x90\x82\x82\x16\x82\x03a\0\xC8WP8a\0|V[\x80\xFD[P\x80\xFD[=\x91Pa\0\xA1V[`@Q=`\0\x82>=\x90\xFD[`\0\x80\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x01\x0BW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c$K\xD6\x1E\x14a\x06\x1FWP\x80c1Z\t]\x14a\x05zW\x80c4t\xCE\xFE\x14a\x04\xBDW\x80c;vYM\x14a\x04\x1FW\x80c`j. \x14a\x03\xCAW\x80c\x89\xC6\xC0\x9B\x14a\x036W\x80c\x90\x83BG\x14a\x02\x84W\x80c\xAD\xBA\x98\x04\x14a\x01\xDFW\x80c\xB1\xAA\x90\xA1\x14a\x01%Wc\xCA\x1D \x9D\x14a\0\x92W`\0\x80\xFD[\x82\x914a\x01!W` 6`\x03\x19\x01\x12a\x01!W`\x01`\x01`\xA0\x1B\x03\x83T\x16\x80;\x15a\x01\x1CW\x83\x90`D\x84Q\x80\x96\x81\x93c@\xC1\x0F\x19`\xE0\x1B\x83R0\x87\x84\x01R\x865`$\x84\x01RZ\xF1\x80\x15a\x01\x12Wa\0\xE7W\x83\x80\xF3[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xFFWPR8\x80\x80\x83\x80\xF3[cNH{q`\xE0\x1B\x84R`A\x90R`$\x83\xFD[\x82Q=\x86\x82>=\x90\xFD[PPP\xFD[PP\xFD[P\x82\x904a\x01\xDBW` \x92\x83`\x03\x196\x01\x12a\x01\xD7W`\x01T\x82Qc\x94\xBF\x80M`\xE0\x1B\x81R\x825\x92\x81\x01\x83\x81R0` \x82\x01R\x90\x91\x86\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x90\x88\x90\x82\x90`@\x01\x03\x92Z\xF1\x93\x84\x15a\x01\xCCW\x80\x94a\x01\x95W[PPa\x01\x8F\x90\x83a\x07lV[Q\x90\x81R\xF3[\x90\x91\x93P\x84\x82\x81=\x83\x11a\x01\xC5W[a\x01\xAE\x81\x83a\x07'V[\x81\x01\x03\x12a\x01\xC2WPQ\x91a\x01\x8F\x85a\x01\x83V[\x80\xFD[P=a\x01\xA4V[\x83Q\x90=\x90\x82>=\x90\xFD[\x82\x80\xFD[P\x80\xFD[P\x82\x904a\x01\xDBW` \x92\x83`\x03\x196\x01\x12a\x01\xD7W`\x01T\x82Qc]\x04;)`\xE1\x1B\x81R\x825\x92\x81\x01\x83\x81R0` \x82\x01\x81\x90R`@\x82\x01R\x90\x91\x86\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x90\x88\x90\x82\x90``\x01\x03\x92Z\xF1\x93\x84\x15a\x01\xCCW\x80\x94a\x02PW[PPa\x01\x8F\x90\x83a\x07\x99V[\x90\x91\x93P\x84\x82\x81=\x83\x11a\x02}W[a\x02i\x81\x83a\x07'V[\x81\x01\x03\x12a\x01\xC2WPQ\x91a\x01\x8F\x85a\x02DV[P=a\x02_V[P4a\x01\xD7W` a\x02\xD9\x92a\x02\x996a\x06\xCAV[\x91\x90\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93c\x94\xBF\x80M`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x91\x82\x15a\x03,W\x83\x92a\x02\xF5W[` \x83\x83Q\x90\x81R\xF3[\x90\x91P` \x81=\x82\x11a\x03$W[\x81a\x03\x10` \x93\x83a\x07'V[\x81\x01\x03\x12a\x01\xD7W` \x92PQ\x908a\x02\xEBV[=\x91Pa\x03\x03V[\x81Q=\x85\x82>=\x90\xFD[P4a\x01\xD7W\x82`\x03\x196\x01\x12a\x01\xD7W` `\x01`\x01`\xA0\x1B\x03\x92`D\x84\x86T\x16\x94`\x01T\x16\x91\x86\x85Q\x96\x87\x94\x85\x93c\t^\xA7\xB3`\xE0\x1B\x85R\x84\x01R`\0\x19`$\x84\x01RZ\xF1\x90\x81\x15a\x03\xC1WPa\x03\x8DWP\x80\xF3[` \x81=\x82\x11a\x03\xB9W[\x81a\x03\xA5` \x93\x83a\x07'V[\x81\x01\x03\x12a\x01\xDBWQ\x80\x15\x15\x03a\x01\xC2W\x80\xF3[=\x91Pa\x03\x98V[Q=\x84\x82>=\x90\xFD[P4a\x01\xD7W` a\x02\xD9\x92a\x03\xDF6a\x06\xCAV[\x91\x90\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93cnU?e`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[P\x82\x904a\x01\xDBW` \x92\x83`\x03\x196\x01\x12a\x01\xD7W`\x01T\x82QcnU?e`\xE0\x1B\x81R\x825\x92\x81\x01\x83\x81R0` \x82\x01R\x90\x91\x86\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x90\x88\x90\x82\x90`@\x01\x03\x92Z\xF1\x93\x84\x15a\x01\xCCW\x80\x94a\x04\x89W[PP\x82a\x01\x8F\x91a\x07lV[\x90\x91\x93P\x84\x82\x81=\x83\x11a\x04\xB6W[a\x04\xA2\x81\x83a\x07'V[\x81\x01\x03\x12a\x01\xC2WPQ\x91a\x01\x8F\x85a\x04}V[P=a\x04\x98V[P\x82\x904a\x01\xDBWa\x05\x14` a\x04\xD36a\x06\xCAV[`\x01T\x85Qc-\x18+\xE5`\xE2\x1B\x81R\x97\x88\x01\x83\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x82\x01R0`@\x82\x01R\x92\x97\x94\x85\x93\x91\x90\x92\x16\x91\x83\x91\x88\x91\x83\x91``\x01\x90V[\x03\x92Z\xF1\x92\x83\x15a\x05oW\x80\x93a\x054W[PPa\x01\x8F\x82` \x94a\x07\x99V[\x90\x93\x92P` \x84=\x82\x11a\x05gW[\x81a\x05P` \x93\x83a\x07'V[\x81\x01\x03\x12a\x01\xC2WPa\x01\x8F` \x93Q\x92\x93a\x05&V[=\x91Pa\x05CV[\x82Q\x90=\x90\x82>=\x90\xFD[P\x82\x904a\x01\xDBW` \x92\x83`\x03\x196\x01\x12a\x01\xD7W`\x01T\x82Qc-\x18+\xE5`\xE2\x1B\x81R\x825\x92\x81\x01\x83\x81R0` \x82\x01\x81\x90R`@\x82\x01R\x90\x91\x86\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x90\x88\x90\x82\x90``\x01\x03\x92Z\xF1\x93\x84\x15a\x01\xCCW\x80\x94a\x05\xEBW[PP\x82a\x01\x8F\x91a\x07\x99V[\x90\x91\x93P\x84\x82\x81=\x83\x11a\x06\x18W[a\x06\x04\x81\x83a\x07'V[\x81\x01\x03\x12a\x01\xC2WPQ\x91a\x01\x8F\x85a\x05\xDFV[P=a\x05\xFAV[\x84\x92\x91P4a\x01\xD7W` \x81a\x06o\x81\x86\x81a\x06:6a\x06\xCAV[`\x01Tc]\x04;)`\xE1\x1B\x84R\x9B\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x82\x01R0`@\x82\x01R\x91\x9B\x16\x94``\x90\x91\x01\x90V[\x03\x92Z\xF1\x92\x83\x15a\x05oW\x80\x93a\x06\x8FW[PPa\x01\x8F` \x93\x83a\x07\x99V[\x90\x93\x92P` \x84=\x82\x11a\x06\xC2W[\x81a\x06\xAB` \x93\x83a\x07'V[\x81\x01\x03\x12a\x01\xC2WPa\x01\x8F` \x93Q\x92\x93a\x06\x81V[=\x91Pa\x06\x9EV[`@\x90`\x03\x19\x01\x12a\x06\xF0W`\x045\x90`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x06\xF0W\x90V[`\0\x80\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\x11W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\x11W`@RV[\x91\x90\x82\x01\x80\x92\x11a\x07VWV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90a\x07|a\x07\x87\x92`\x02Ta\x07IV[`\x02U`\x03Ta\x07IV[`\x03UV[\x91\x90\x82\x03\x91\x82\x11a\x07VWV[\x90a\x07\x87\x91a\x08\xC5`\x03T\x91a\x081`@Qa\x07\xB4\x81a\x06\xF5V[`Z\x81R\x7FActor has burned more shares tha` \x82\x01R\x7Fn they ever minted. Implies a ro`@\x82\x01R\x7Funding or accounting error\0\0\0\0\0\0``\x82\x01R\x84\x86a\x08\xF0V[`\x02Ta\x08\xC0`@Qa\x08C\x81a\x06\xF5V[``\x81R\x7FActor has withdrawn more tokens ` \x82\x01R\x7Fthan they ever deposited. Implie`@\x82\x01R\x7Fs a rounding or accounting error``\x82\x01R\x82\x84a\x08\xF0V[a\x07\x8CV[`\x02Ua\x07\x8CV[`\0[\x83\x81\x10a\x08\xE0WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x08\xD0V[\x91\x90\x91\x82\x81\x11a\x08\xFFWPPPV[\x7Fb\xBD\xDA\x9A\x05\xCD\xBC\xDB\xF9\x05\xCB\xAD\x99\xC6\xEB\xDC\t\x8Bo\t3\xD8\xF2\xEB<\xFA\xB7@\x0B`%\x14\x92\x91a\t6a\t0`@\x93a\n\x0FV[\x93a\n\x0FV[\x92a\t\xD0`;\x84Q\x80\x94` \x97\x88\x83\x01\x95h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x87Ra\ti\x81Q\x80\x92\x8C`)\x88\x01\x91\x01a\x08\xCDV[\x83\x01`\x1F`\xF9\x1B`)\x82\x01Ra\t\x88\x82Q\x80\x93\x8C`*\x85\x01\x91\x01a\x08\xCDV[\x01\x7F failed, reason: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`*\x82\x01Ra\t\xC1\x82Q\x80\x93\x8B\x87\x85\x01\x91\x01a\x08\xCDV[\x01\x03`\x1B\x81\x01\x85R\x01\x83a\x07'V[a\t\xEC\x83Q\x94\x85\x93\x81\x85RQ\x92\x83\x80\x92\x86\x01R\x85\x85\x01\x90a\x08\xCDV[`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xA1cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\n%W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \xB0\xE2\x1D\x8DQ\xCCWw\xF2\xAA\x8C\x15\xFE\x06T/\t\xC3\xFD\xBC\x1C\xED\x91\xE0\xD3\xB9h\xBC\x98\x0CF1dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static ACTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c$K\xD6\x1E\x14a\x06\x1FWP\x80c1Z\t]\x14a\x05zW\x80c4t\xCE\xFE\x14a\x04\xBDW\x80c;vYM\x14a\x04\x1FW\x80c`j. \x14a\x03\xCAW\x80c\x89\xC6\xC0\x9B\x14a\x036W\x80c\x90\x83BG\x14a\x02\x84W\x80c\xAD\xBA\x98\x04\x14a\x01\xDFW\x80c\xB1\xAA\x90\xA1\x14a\x01%Wc\xCA\x1D \x9D\x14a\0\x92W`\0\x80\xFD[\x82\x914a\x01!W` 6`\x03\x19\x01\x12a\x01!W`\x01`\x01`\xA0\x1B\x03\x83T\x16\x80;\x15a\x01\x1CW\x83\x90`D\x84Q\x80\x96\x81\x93c@\xC1\x0F\x19`\xE0\x1B\x83R0\x87\x84\x01R\x865`$\x84\x01RZ\xF1\x80\x15a\x01\x12Wa\0\xE7W\x83\x80\xF3[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\0\xFFWPR8\x80\x80\x83\x80\xF3[cNH{q`\xE0\x1B\x84R`A\x90R`$\x83\xFD[\x82Q=\x86\x82>=\x90\xFD[PPP\xFD[PP\xFD[P\x82\x904a\x01\xDBW` \x92\x83`\x03\x196\x01\x12a\x01\xD7W`\x01T\x82Qc\x94\xBF\x80M`\xE0\x1B\x81R\x825\x92\x81\x01\x83\x81R0` \x82\x01R\x90\x91\x86\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x90\x88\x90\x82\x90`@\x01\x03\x92Z\xF1\x93\x84\x15a\x01\xCCW\x80\x94a\x01\x95W[PPa\x01\x8F\x90\x83a\x07lV[Q\x90\x81R\xF3[\x90\x91\x93P\x84\x82\x81=\x83\x11a\x01\xC5W[a\x01\xAE\x81\x83a\x07'V[\x81\x01\x03\x12a\x01\xC2WPQ\x91a\x01\x8F\x85a\x01\x83V[\x80\xFD[P=a\x01\xA4V[\x83Q\x90=\x90\x82>=\x90\xFD[\x82\x80\xFD[P\x80\xFD[P\x82\x904a\x01\xDBW` \x92\x83`\x03\x196\x01\x12a\x01\xD7W`\x01T\x82Qc]\x04;)`\xE1\x1B\x81R\x825\x92\x81\x01\x83\x81R0` \x82\x01\x81\x90R`@\x82\x01R\x90\x91\x86\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x90\x88\x90\x82\x90``\x01\x03\x92Z\xF1\x93\x84\x15a\x01\xCCW\x80\x94a\x02PW[PPa\x01\x8F\x90\x83a\x07\x99V[\x90\x91\x93P\x84\x82\x81=\x83\x11a\x02}W[a\x02i\x81\x83a\x07'V[\x81\x01\x03\x12a\x01\xC2WPQ\x91a\x01\x8F\x85a\x02DV[P=a\x02_V[P4a\x01\xD7W` a\x02\xD9\x92a\x02\x996a\x06\xCAV[\x91\x90\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93c\x94\xBF\x80M`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x91\x82\x15a\x03,W\x83\x92a\x02\xF5W[` \x83\x83Q\x90\x81R\xF3[\x90\x91P` \x81=\x82\x11a\x03$W[\x81a\x03\x10` \x93\x83a\x07'V[\x81\x01\x03\x12a\x01\xD7W` \x92PQ\x908a\x02\xEBV[=\x91Pa\x03\x03V[\x81Q=\x85\x82>=\x90\xFD[P4a\x01\xD7W\x82`\x03\x196\x01\x12a\x01\xD7W` `\x01`\x01`\xA0\x1B\x03\x92`D\x84\x86T\x16\x94`\x01T\x16\x91\x86\x85Q\x96\x87\x94\x85\x93c\t^\xA7\xB3`\xE0\x1B\x85R\x84\x01R`\0\x19`$\x84\x01RZ\xF1\x90\x81\x15a\x03\xC1WPa\x03\x8DWP\x80\xF3[` \x81=\x82\x11a\x03\xB9W[\x81a\x03\xA5` \x93\x83a\x07'V[\x81\x01\x03\x12a\x01\xDBWQ\x80\x15\x15\x03a\x01\xC2W\x80\xF3[=\x91Pa\x03\x98V[Q=\x84\x82>=\x90\xFD[P4a\x01\xD7W` a\x02\xD9\x92a\x03\xDF6a\x06\xCAV[\x91\x90\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93cnU?e`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[P\x82\x904a\x01\xDBW` \x92\x83`\x03\x196\x01\x12a\x01\xD7W`\x01T\x82QcnU?e`\xE0\x1B\x81R\x825\x92\x81\x01\x83\x81R0` \x82\x01R\x90\x91\x86\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x90\x88\x90\x82\x90`@\x01\x03\x92Z\xF1\x93\x84\x15a\x01\xCCW\x80\x94a\x04\x89W[PP\x82a\x01\x8F\x91a\x07lV[\x90\x91\x93P\x84\x82\x81=\x83\x11a\x04\xB6W[a\x04\xA2\x81\x83a\x07'V[\x81\x01\x03\x12a\x01\xC2WPQ\x91a\x01\x8F\x85a\x04}V[P=a\x04\x98V[P\x82\x904a\x01\xDBWa\x05\x14` a\x04\xD36a\x06\xCAV[`\x01T\x85Qc-\x18+\xE5`\xE2\x1B\x81R\x97\x88\x01\x83\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x82\x01R0`@\x82\x01R\x92\x97\x94\x85\x93\x91\x90\x92\x16\x91\x83\x91\x88\x91\x83\x91``\x01\x90V[\x03\x92Z\xF1\x92\x83\x15a\x05oW\x80\x93a\x054W[PPa\x01\x8F\x82` \x94a\x07\x99V[\x90\x93\x92P` \x84=\x82\x11a\x05gW[\x81a\x05P` \x93\x83a\x07'V[\x81\x01\x03\x12a\x01\xC2WPa\x01\x8F` \x93Q\x92\x93a\x05&V[=\x91Pa\x05CV[\x82Q\x90=\x90\x82>=\x90\xFD[P\x82\x904a\x01\xDBW` \x92\x83`\x03\x196\x01\x12a\x01\xD7W`\x01T\x82Qc-\x18+\xE5`\xE2\x1B\x81R\x825\x92\x81\x01\x83\x81R0` \x82\x01\x81\x90R`@\x82\x01R\x90\x91\x86\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x90\x88\x90\x82\x90``\x01\x03\x92Z\xF1\x93\x84\x15a\x01\xCCW\x80\x94a\x05\xEBW[PP\x82a\x01\x8F\x91a\x07\x99V[\x90\x91\x93P\x84\x82\x81=\x83\x11a\x06\x18W[a\x06\x04\x81\x83a\x07'V[\x81\x01\x03\x12a\x01\xC2WPQ\x91a\x01\x8F\x85a\x05\xDFV[P=a\x05\xFAV[\x84\x92\x91P4a\x01\xD7W` \x81a\x06o\x81\x86\x81a\x06:6a\x06\xCAV[`\x01Tc]\x04;)`\xE1\x1B\x84R\x9B\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x82\x01R0`@\x82\x01R\x91\x9B\x16\x94``\x90\x91\x01\x90V[\x03\x92Z\xF1\x92\x83\x15a\x05oW\x80\x93a\x06\x8FW[PPa\x01\x8F` \x93\x83a\x07\x99V[\x90\x93\x92P` \x84=\x82\x11a\x06\xC2W[\x81a\x06\xAB` \x93\x83a\x07'V[\x81\x01\x03\x12a\x01\xC2WPa\x01\x8F` \x93Q\x92\x93a\x06\x81V[=\x91Pa\x06\x9EV[`@\x90`\x03\x19\x01\x12a\x06\xF0W`\x045\x90`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x06\xF0W\x90V[`\0\x80\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\x11W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\x11W`@RV[\x91\x90\x82\x01\x80\x92\x11a\x07VWV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90a\x07|a\x07\x87\x92`\x02Ta\x07IV[`\x02U`\x03Ta\x07IV[`\x03UV[\x91\x90\x82\x03\x91\x82\x11a\x07VWV[\x90a\x07\x87\x91a\x08\xC5`\x03T\x91a\x081`@Qa\x07\xB4\x81a\x06\xF5V[`Z\x81R\x7FActor has burned more shares tha` \x82\x01R\x7Fn they ever minted. Implies a ro`@\x82\x01R\x7Funding or accounting error\0\0\0\0\0\0``\x82\x01R\x84\x86a\x08\xF0V[`\x02Ta\x08\xC0`@Qa\x08C\x81a\x06\xF5V[``\x81R\x7FActor has withdrawn more tokens ` \x82\x01R\x7Fthan they ever deposited. Implie`@\x82\x01R\x7Fs a rounding or accounting error``\x82\x01R\x82\x84a\x08\xF0V[a\x07\x8CV[`\x02Ua\x07\x8CV[`\0[\x83\x81\x10a\x08\xE0WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x08\xD0V[\x91\x90\x91\x82\x81\x11a\x08\xFFWPPPV[\x7Fb\xBD\xDA\x9A\x05\xCD\xBC\xDB\xF9\x05\xCB\xAD\x99\xC6\xEB\xDC\t\x8Bo\t3\xD8\xF2\xEB<\xFA\xB7@\x0B`%\x14\x92\x91a\t6a\t0`@\x93a\n\x0FV[\x93a\n\x0FV[\x92a\t\xD0`;\x84Q\x80\x94` \x97\x88\x83\x01\x95h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x87Ra\ti\x81Q\x80\x92\x8C`)\x88\x01\x91\x01a\x08\xCDV[\x83\x01`\x1F`\xF9\x1B`)\x82\x01Ra\t\x88\x82Q\x80\x93\x8C`*\x85\x01\x91\x01a\x08\xCDV[\x01\x7F failed, reason: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`*\x82\x01Ra\t\xC1\x82Q\x80\x93\x8B\x87\x85\x01\x91\x01a\x08\xCDV[\x01\x03`\x1B\x81\x01\x85R\x01\x83a\x07'V[a\t\xEC\x83Q\x94\x85\x93\x81\x85RQ\x92\x83\x80\x92\x86\x01R\x85\x85\x01\x90a\x08\xCDV[`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xA1cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\n%W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \xB0\xE2\x1D\x8DQ\xCCWw\xF2\xAA\x8C\x15\xFE\x06T/\t\xC3\xFD\xBC\x1C\xED\x91\xE0\xD3\xB9h\xBC\x98\x0CF1dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static ACTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Actor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Actor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Actor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Actor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Actor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Actor)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Actor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ACTOR_ABI.clone(),
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
                ACTOR_ABI.clone(),
                ACTOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `approveFunds` (0x89c6c09b) function
        pub fn approve_funds(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 198, 192, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositFunds` (0x3b76594d) function
        pub fn deposit_funds(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([59, 118, 89, 77], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositFundsOnBehalf` (0x606a2e20) function
        pub fn deposit_funds_on_behalf(
            &self,
            assets: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([96, 106, 46, 32], (assets, receiver))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fund` (0xca1d209d) function
        pub fn fund(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 29, 32, 157], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintShares` (0xb1aa90a1) function
        pub fn mint_shares(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([177, 170, 144, 161], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintSharesOnBehalf` (0x90834247) function
        pub fn mint_shares_on_behalf(
            &self,
            shares: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([144, 131, 66, 71], (shares, receiver))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeemShares` (0xadba9804) function
        pub fn redeem_shares(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 186, 152, 4], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeemSharesOnBehalf` (0x244bd61e) function
        pub fn redeem_shares_on_behalf(
            &self,
            shares: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([36, 75, 214, 30], (shares, receiver))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawTokens` (0x315a095d) function
        pub fn withdraw_tokens(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([49, 90, 9, 93], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawTokensOnBehalf` (0x3474cefe) function
        pub fn withdraw_tokens_on_behalf(
            &self,
            assets: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 116, 206, 254], (assets, receiver))
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ActorEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Actor<M> {
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
    pub enum ActorEvents {
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
    impl ::ethers::contract::EthLogDecode for ActorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssertEqFailFilter::decode_log(log) {
                return Ok(ActorEvents::AssertEqFailFilter(decoded));
            }
            if let Ok(decoded) = AssertFailFilter::decode_log(log) {
                return Ok(ActorEvents::AssertFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGtFailFilter::decode_log(log) {
                return Ok(ActorEvents::AssertGtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGteFailFilter::decode_log(log) {
                return Ok(ActorEvents::AssertGteFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLtFailFilter::decode_log(log) {
                return Ok(ActorEvents::AssertLtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLteFailFilter::decode_log(log) {
                return Ok(ActorEvents::AssertLteFailFilter(decoded));
            }
            if let Ok(decoded) = AssertNeqFailFilter::decode_log(log) {
                return Ok(ActorEvents::AssertNeqFailFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(ActorEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(ActorEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUint256Filter::decode_log(log) {
                return Ok(ActorEvents::LogUint256Filter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ActorEvents {
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
    impl ::core::convert::From<AssertEqFailFilter> for ActorEvents {
        fn from(value: AssertEqFailFilter) -> Self {
            Self::AssertEqFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertFailFilter> for ActorEvents {
        fn from(value: AssertFailFilter) -> Self {
            Self::AssertFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGtFailFilter> for ActorEvents {
        fn from(value: AssertGtFailFilter) -> Self {
            Self::AssertGtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGteFailFilter> for ActorEvents {
        fn from(value: AssertGteFailFilter) -> Self {
            Self::AssertGteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLtFailFilter> for ActorEvents {
        fn from(value: AssertLtFailFilter) -> Self {
            Self::AssertLtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLteFailFilter> for ActorEvents {
        fn from(value: AssertLteFailFilter) -> Self {
            Self::AssertLteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertNeqFailFilter> for ActorEvents {
        fn from(value: AssertNeqFailFilter) -> Self {
            Self::AssertNeqFailFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for ActorEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for ActorEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUint256Filter> for ActorEvents {
        fn from(value: LogUint256Filter) -> Self {
            Self::LogUint256Filter(value)
        }
    }
    ///Container type for all input parameters for the `approveFunds` function with signature `approveFunds()` and selector `0x89c6c09b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approveFunds", abi = "approveFunds()")]
    pub struct ApproveFundsCall;
    ///Container type for all input parameters for the `depositFunds` function with signature `depositFunds(uint256)` and selector `0x3b76594d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "depositFunds", abi = "depositFunds(uint256)")]
    pub struct DepositFundsCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `depositFundsOnBehalf` function with signature `depositFundsOnBehalf(uint256,address)` and selector `0x606a2e20`
    #[derive(
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
        name = "depositFundsOnBehalf",
        abi = "depositFundsOnBehalf(uint256,address)"
    )]
    pub struct DepositFundsOnBehalfCall {
        pub assets: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `fund` function with signature `fund(uint256)` and selector `0xca1d209d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "fund", abi = "fund(uint256)")]
    pub struct FundCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mintShares` function with signature `mintShares(uint256)` and selector `0xb1aa90a1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mintShares", abi = "mintShares(uint256)")]
    pub struct MintSharesCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mintSharesOnBehalf` function with signature `mintSharesOnBehalf(uint256,address)` and selector `0x90834247`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mintSharesOnBehalf", abi = "mintSharesOnBehalf(uint256,address)")]
    pub struct MintSharesOnBehalfCall {
        pub shares: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `redeemShares` function with signature `redeemShares(uint256)` and selector `0xadba9804`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "redeemShares", abi = "redeemShares(uint256)")]
    pub struct RedeemSharesCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `redeemSharesOnBehalf` function with signature `redeemSharesOnBehalf(uint256,address)` and selector `0x244bd61e`
    #[derive(
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
        name = "redeemSharesOnBehalf",
        abi = "redeemSharesOnBehalf(uint256,address)"
    )]
    pub struct RedeemSharesOnBehalfCall {
        pub shares: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawTokens` function with signature `withdrawTokens(uint256)` and selector `0x315a095d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdrawTokens", abi = "withdrawTokens(uint256)")]
    pub struct WithdrawTokensCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawTokensOnBehalf` function with signature `withdrawTokensOnBehalf(uint256,address)` and selector `0x3474cefe`
    #[derive(
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
        name = "withdrawTokensOnBehalf",
        abi = "withdrawTokensOnBehalf(uint256,address)"
    )]
    pub struct WithdrawTokensOnBehalfCall {
        pub assets: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ActorCalls {
        ApproveFunds(ApproveFundsCall),
        DepositFunds(DepositFundsCall),
        DepositFundsOnBehalf(DepositFundsOnBehalfCall),
        Fund(FundCall),
        MintShares(MintSharesCall),
        MintSharesOnBehalf(MintSharesOnBehalfCall),
        RedeemShares(RedeemSharesCall),
        RedeemSharesOnBehalf(RedeemSharesOnBehalfCall),
        WithdrawTokens(WithdrawTokensCall),
        WithdrawTokensOnBehalf(WithdrawTokensOnBehalfCall),
    }
    impl ::ethers::core::abi::AbiDecode for ActorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ApproveFundsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApproveFunds(decoded));
            }
            if let Ok(decoded)
                = <DepositFundsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositFunds(decoded));
            }
            if let Ok(decoded)
                = <DepositFundsOnBehalfCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DepositFundsOnBehalf(decoded));
            }
            if let Ok(decoded)
                = <FundCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fund(decoded));
            }
            if let Ok(decoded)
                = <MintSharesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MintShares(decoded));
            }
            if let Ok(decoded)
                = <MintSharesOnBehalfCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MintSharesOnBehalf(decoded));
            }
            if let Ok(decoded)
                = <RedeemSharesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RedeemShares(decoded));
            }
            if let Ok(decoded)
                = <RedeemSharesOnBehalfCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RedeemSharesOnBehalf(decoded));
            }
            if let Ok(decoded)
                = <WithdrawTokensCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawTokens(decoded));
            }
            if let Ok(decoded)
                = <WithdrawTokensOnBehalfCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WithdrawTokensOnBehalf(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ActorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ApproveFunds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositFunds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositFundsOnBehalf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Fund(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MintShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintSharesOnBehalf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedeemShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedeemSharesOnBehalf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawTokensOnBehalf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ActorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApproveFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositFundsOnBehalf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Fund(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintSharesOnBehalf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RedeemShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedeemSharesOnBehalf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawTokensOnBehalf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApproveFundsCall> for ActorCalls {
        fn from(value: ApproveFundsCall) -> Self {
            Self::ApproveFunds(value)
        }
    }
    impl ::core::convert::From<DepositFundsCall> for ActorCalls {
        fn from(value: DepositFundsCall) -> Self {
            Self::DepositFunds(value)
        }
    }
    impl ::core::convert::From<DepositFundsOnBehalfCall> for ActorCalls {
        fn from(value: DepositFundsOnBehalfCall) -> Self {
            Self::DepositFundsOnBehalf(value)
        }
    }
    impl ::core::convert::From<FundCall> for ActorCalls {
        fn from(value: FundCall) -> Self {
            Self::Fund(value)
        }
    }
    impl ::core::convert::From<MintSharesCall> for ActorCalls {
        fn from(value: MintSharesCall) -> Self {
            Self::MintShares(value)
        }
    }
    impl ::core::convert::From<MintSharesOnBehalfCall> for ActorCalls {
        fn from(value: MintSharesOnBehalfCall) -> Self {
            Self::MintSharesOnBehalf(value)
        }
    }
    impl ::core::convert::From<RedeemSharesCall> for ActorCalls {
        fn from(value: RedeemSharesCall) -> Self {
            Self::RedeemShares(value)
        }
    }
    impl ::core::convert::From<RedeemSharesOnBehalfCall> for ActorCalls {
        fn from(value: RedeemSharesOnBehalfCall) -> Self {
            Self::RedeemSharesOnBehalf(value)
        }
    }
    impl ::core::convert::From<WithdrawTokensCall> for ActorCalls {
        fn from(value: WithdrawTokensCall) -> Self {
            Self::WithdrawTokens(value)
        }
    }
    impl ::core::convert::From<WithdrawTokensOnBehalfCall> for ActorCalls {
        fn from(value: WithdrawTokensOnBehalfCall) -> Self {
            Self::WithdrawTokensOnBehalf(value)
        }
    }
    ///Container type for all return fields from the `depositFunds` function with signature `depositFunds(uint256)` and selector `0x3b76594d`
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
    pub struct DepositFundsReturn {
        pub shares_minted: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `depositFundsOnBehalf` function with signature `depositFundsOnBehalf(uint256,address)` and selector `0x606a2e20`
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
    pub struct DepositFundsOnBehalfReturn {
        pub shares_minted: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `mintShares` function with signature `mintShares(uint256)` and selector `0xb1aa90a1`
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
    pub struct MintSharesReturn {
        pub tokens_deposited: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `mintSharesOnBehalf` function with signature `mintSharesOnBehalf(uint256,address)` and selector `0x90834247`
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
    pub struct MintSharesOnBehalfReturn {
        pub tokens_deposited: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `redeemShares` function with signature `redeemShares(uint256)` and selector `0xadba9804`
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
    pub struct RedeemSharesReturn {
        pub tokens_withdrawn: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `redeemSharesOnBehalf` function with signature `redeemSharesOnBehalf(uint256,address)` and selector `0x244bd61e`
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
    pub struct RedeemSharesOnBehalfReturn {
        pub tokens_withdrawn: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `withdrawTokens` function with signature `withdrawTokens(uint256)` and selector `0x315a095d`
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
    pub struct WithdrawTokensReturn {
        pub shares_burned: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `withdrawTokensOnBehalf` function with signature `withdrawTokensOnBehalf(uint256,address)` and selector `0x3474cefe`
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
    pub struct WithdrawTokensOnBehalfReturn {
        pub shares_burned: ::ethers::core::types::U256,
    }
}
