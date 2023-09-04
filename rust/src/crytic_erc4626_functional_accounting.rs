pub use crytic_erc4626_functional_accounting::*;
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
pub mod crytic_erc4626_functional_accounting {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("verify_depositProperties"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_depositProperties",
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verify_mintProperties"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_mintProperties",
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verify_redeemProperties"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_redeemProperties",
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verify_withdrawProperties"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_withdrawProperties",
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
    pub static CRYTICERC4626FUNCTIONALACCOUNTING_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa!8\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@\x90\x80\x82R`\x04\x91\x826\x10\x15a\0\x17W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x90\x81c\"\x18\x03\xD8\x14a\x11\x98WP\x80c+\xEF\xB0\xC6\x14a\x0B\xB0W\x80c,(\xD0\xB7\x14a\x06%Wc\xEAx1\xE9\x14a\0QW`\0\x80\xFD[4a\x06!Wa\0j\x90a\0c6a\x17\x81V[\x92\x90a\x1A\xE3V[\x90\x82\x15a\x06\x1DW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x81Q\x92cl\x82\xBB\xBF`\xE1\x1B\x84R0\x82\x85\x01R` \x94`$\x94\x86\x81\x87\x81\x8CZ\xFA\x90\x81\x15a\x05\xADW\x88\x91a\x05\xF0W[P\x80\x15a\x05AW\x84Qcp\xA0\x821`\xE0\x1B\x81R0\x85\x82\x01R\x91\x87\x83\x88\x81\x8DZ\xFA\x92\x83\x15a\x05\xE6W\x89\x93a\x05\xB7W[P\x82\x15a\x05gWa\0\xF3\x92\x91a\0\xEE\x91a\x1EZV[a\x1EZV[\x93\x83Qc\x03\xD1h\x9D`\xE1\x1B\x81R\x85\x84\x82\x01R\x86\x81\x83\x81\x8CZ\xFA\x90\x81\x15a\x05\xADW\x90\x89\x91\x89\x91a\x05|W[P\x85Q\x88\x81\x85\x81c&mj\x83`\xE1\x1B\x96\x87\x82R\x8C\x8B\x83\x01RZ\xFA\x80\x15a\x05rW\x90\x89\x91a\x05EW[PP\x15a\x05AW\x91`\x08\x98\x91\x87\x93\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-`\x80\x88Q\x89\x81R`\x1C\x8A\x82\x01R\x7FShares to use in redemption:\0\0\0\0``\x82\x01R\x8A\x88\x82\x01R\xA1\x86Q\x95a\x01\xB9\x87a\x17\xC6V[`\x05\x87Ra\x02;a\x01\xFEd7\xBB\xB72\xB9`\xD9\x1B\x98\x89\x89\x82\x01R\x8AQ\x90a\x01\xDE\x82a\x17\xC6V[`\r\x82Rlbefore redeem`\x98\x1B\x80\x9A\x83\x01R0a\x18\xA7V[\x97\x90P\x89Q\x9D\x8Ea\x02\x0E\x81a\x17\xC6V[Rg92\xB1\xB2\xB4\xBB2\xB9`\xC1\x1B\x9D\x8E\x8D\x82\x01R\x8AQ\x91a\x02-\x83a\x17\xC6V[`\r\x83R\x8D\x83\x01R\x86a\x18\xA7V[P\x94\x88Q\x92\x83R\x89\x82\x84\x01R\x8A\x83\x85\x81\x84Z\xFA\x92\x83\x15a\x04\xF9W\x8C\x96\x95\x94\x93\x92\x91\x8C\x91\x88\x94a\x05\x03W[P\x8AQc]\x04;)`\xE1\x1B\x81R\x83\x81\x01\x8D\x81R`\x01`\x01`\xA0\x1B\x03\x88\x16` \x82\x01R0`@\x82\x01R\x90\x98\x89\x92\x91\x83\x91\x90\x82\x90``\x01[\x03\x92Z\xF1\x95\x86\x15a\x04\xF9W\x8C\x96a\x04\xC6W[P\x88Q\x92`\xA0\x84\x01\x91\x84\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x04\xB5WPP\x93a\x04\x01\x8Ba\x04\xB2\x9D\x9Ea\x04q\x96a\x03\x90\x7F number of shares from the owner\x9C\x9A\x97a\x04\x07\x97\x8F\x80a\x04v\x9F\x9D\x7Fhan or equal to the number of as\x92R`a\x84R\x7Fredeem() must withdraw greater t\x88\x85\x01R\x83\x01R\x7Fsets predicted by previewRedeem(``\x83\x01R`)`\xF8\x1B`\x80\x83\x01R\x89a\x1C\xFAV[\x8CQ\x99a\x03\x9C\x8Ba\x17\xC6V[`\x05\x8BR\x83\x8B\x01Ra\x03\xD2\x8DQa\x03\xB2\x81a\x17\xC6V[`\x0C\x81Rkafter redeem`\xA0\x1B\x9B\x8C\x86\x83\x01R0a\x18\xA7V[\x9A\x90P\x8DQ\x92a\x03\xE1\x84a\x17\xC6V[`\x08\x84R\x84\x84\x01R\x8DQ\x93a\x03\xF5\x85a\x17\xC6V[`\x0C\x85R\x84\x01Ra\x18\xA7V[Pa\x18<V[\x90\x87Q\x91a\x04\x14\x83a\x17\xE2V[`A\x83R\x7Fredeem() must credit the correct\x8B\x84\x01R\x7F number of assets to the receive\x89\x84\x01R`9`\xF9\x1B``\x84\x01Ra\x1C\x05V[a\x18<V[\x91\x7Fredeem() must deduct the correct\x81Q\x95a\x04\xA4\x87a\x17\xFEV[\x82\x87R\x86\x01R\x84\x01Ra\x1C\x05V[\x80\xF3[cNH{q`\xE0\x1B\x8ER`A\x90R\x8C\xFD[\x90\x95P\x8A\x81\x81=\x83\x11a\x04\xF2W[a\x04\xDE\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x04\xEEWQ\x948a\x02\xADV[\x8B\x80\xFD[P=a\x04\xD4V[\x89Q=\x8E\x82>=\x90\xFD[\x92\x93P\x93\x94\x95\x96P\x81\x81=\x83\x11a\x05:W[a\x05\x1F\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x04\xEEW\x90\x8Aa\x02\x9B\x8D\x97\x96\x95\x94\x93Q\x93\x90a\x02eV[P=a\x05\x15V[\x87\x80\xFD[\x81=\x83\x11a\x05kW[a\x05X\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05gW\x878a\x01EV[\x88\x80\xFD[P=a\x05NV[\x87Q=\x8C\x82>=\x90\xFD[\x80\x92P\x88\x80\x92P=\x83\x11a\x05\xA6W[a\x05\x95\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05AW\x88\x90Q8a\x01\x1DV[P=a\x05\x8BV[\x85Q=\x8A\x82>=\x90\xFD[\x90\x92P\x87\x81\x81=\x83\x11a\x05\xDFW[a\x05\xCF\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05gWQ\x918a\0\xD9V[P=a\x05\xC5V[\x86Q=\x8B\x82>=\x90\xFD[\x90P\x86\x81\x81=\x83\x11a\x06\x16W[a\x06\x07\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05AWQ8a\0\xABV[P=a\x05\xFDV[\x83\x80\xFD[P\x80\xFD[P\x82\x904a\x0B\xACWa\x06A\x91a\x06:6a\x17\x81V[\x93\x90a\x1A\xE3V[\x91\x84`\x01`\x01`\xA0\x1B\x03\x92\x83`\x01T\x16\x94\x83Qc\xB3\xD7\xF6\xB9`\xE0\x1B\x95\x86\x82R\x88\x84\x83\x01R` \x97\x88\x83`$\x81\x84Z\xFA\x92\x83\x15a\x0B8W\x86\x93a\x0B}W[P\x86Qcc\x1E\xBA\xDB`\xE1\x1B\x81R\x84\x83\x16\x86\x82\x01\x81\x90R\x9A\x90\x8A\x81`$\x81\x86Z\xFA\x90\x81\x15a\x0BsW\x90\x8B\x91\x89\x91a\x0BBW[Pa\x06\xBA\x91\x92a\x1EZV[\x98`$\x89Q\x80\x94\x81\x93\x82R\x8C\x8A\x83\x01RZ\xFA\x80\x15a\x0B8W\x86\x90a\x0B\x05W[a\x06\xE4\x91P0a\x1B6V[`\x01T\x16\x97\x85Q\x90cp\xA0\x821`\xE0\x1B\x82R\x84\x82\x01R\x87\x81`$\x81\x8CZ\xFA\x80\x15a\n\xC6W\x87\x90\x86\x90a\n\xD0W[a\x07\x1B\x92Pa\x1B)V[\x85Q\x90c&mj\x83`\xE1\x1B\x82R\x84\x82\x01R\x87\x81`$\x81\x8CZ\xFA\x80\x15a\n\xC6W\x91\x87\x93\x91\x89\x93a\n\x95W[P\x86Q\x87\x81R\x87\x81\x01`\x16\x90R``\x81\x01\x7FShares to use in mint:\0\0\0\0\0\0\0\0\0\0\x90R\x84\x84\x82\x01R`\x80\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x91\xA1\x82\x82\x88Q\x9Ba\x07\xB1\x8Da\x17\xC6V[`\x06\x8DR\x8Ce9\xB2\xB722\xB9`\xD1\x1B\x96\x87\x91\x01R\x89Q\x9C\x8Da\x07\xD2\x81a\x17\xC6V[`\x0B\x90Rj\x18\x99Y\x9B\xDC\x99H\x1BZ[\x9D`\xAA\x1B\x9D\x8E\x85\x82\x01R0\x91a\x07\xF6\x92a\x18\xA7V[P\x97\x8AQa\x08\x03\x81a\x17\xC6V[`\x08\x81Rg92\xB1\xB2\xB4\xBB2\xB9`\xC1\x1B\x9E\x8F\x86\x83\x01R\x8CQ\x90a\x08%\x82a\x17\xC6V[`\x0B\x82R\x86\x82\x01Ra\x087\x91\x85a\x18\xA7V[\x8CQc\x94\xBF\x80M`\xE0\x1B\x81R\x92\x83\x01\x99\x8AR`\x01`\x01`\xA0\x1B\x03\x90\x94\x16` \x8A\x01RP\x91\x98\x91\x96\x87\x92\x83\x91\x82\x90`@\x01\x03\x92Z\xF1\x93\x84\x15a\n\x8BW\x8B\x94a\nNW[P\x92a\t\xB1a\n\x11\x95\x93a\t\xA9\x8Ba\x04\xB2\x9Da\x04q\x96a\t;\x8D\x7Fumber of shares to the receiver\0\x9D\x9B\x7Fthan or equal to the tokens pred\x82Q\x92a\x08\xE3\x84a\x17\xE2V[`V\x84R\x7Fmint() must always consume less \x88\x85\x01R\x83\x01R\x7Ficted by previewMint()\0\0\0\0\0\0\0\0\0\0``\x83\x01R\x89a\x1D\xCEV[\x8CQ\x94a\tG\x86a\x17\xC6V[`\x06\x86R\x83\x86\x01Ra\t{\x8DQa\t]\x81a\x17\xC6V[`\n\x81Ri\x18Y\x9D\x19\\\x88\x1BZ[\x9D`\xB2\x1B\x96\x87\x86\x83\x01R0a\x18\xA7V[P\x94\x8DQ\x92a\t\x89\x84a\x17\xC6V[`\x08\x84R\x84\x84\x01R\x8DQ\x93a\t\x9D\x85a\x17\xC6V[`\n\x85R\x84\x01Ra\x18\xA7V[\x95\x90Pa\x18<V[\x90\x87Q\x91a\t\xBE\x83a\x17\xFEV[`:\x83R\x7Fmint() must consume exactly the \x8B\x84\x01R\x7Fnumber of tokens requested\0\0\0\0\0\0\x89\x84\x01Ra\x1C\x05V[\x91\x7Fmint() must credit the correct n\x81Q\x95a\n?\x87a\x17\xFEV[`?\x87R\x86\x01R\x84\x01Ra\x1C\x05V[\x91\x94\x92\x95\x93P\x88\x82\x81=\x83\x11a\n\x84W[a\ni\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\n\x80W\x90Q\x92\x94\x91\x93\x90a\t\xB1a\x08yV[\x8A\x80\xFD[P=a\n_V[\x87Q=\x8D\x82>=\x90\xFD[\x90\x92\x80\x92\x94P=\x83\x11a\n\xBFW[a\n\xAD\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x06\x1DW\x85\x91\x87\x91\x8Ba\x07EV[P=a\n\xA3V[\x86Q=\x87\x82>=\x90\xFD[PP\x87\x81\x81=\x83\x11a\n\xFEW[a\n\xE7\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\n\xFAW\x86a\x07\x1B\x91Qa\x07\x11V[\x84\x80\xFD[P=a\n\xDDV[P\x88\x81\x81=\x83\x11a\x0B1W[a\x0B\x1B\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x0B-Wa\x06\xE4\x90Qa\x06\xD9V[\x85\x80\xFD[P=a\x0B\x11V[\x87Q=\x88\x82>=\x90\xFD[\x82\x81\x93\x92P=\x83\x11a\x0BlW[a\x0BY\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05AWQ\x8A\x90a\x06\xBAa\x06\xAFV[P=a\x0BOV[\x89Q=\x8A\x82>=\x90\xFD[\x90\x92P\x88\x81\x81=\x83\x11a\x0B\xA5W[a\x0B\x95\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x0B-WQ\x91\x8Ba\x06~V[P=a\x0B\x8BV[\x82\x80\xFD[P\x824a\x0B\xACWa\x0B\xCAa\x0B\xC36a\x17\x81V[\x91\x90a\x1A\xE3V[\x92`\x01`\x01`\xA0\x1B\x03\x92\x83`\x01T\x16\x92\x82Q\x94\x85\x91c@-&}`\xE0\x1B\x83R\x82\x82\x89\x16\x96\x87\x86\x83\x01R` \x98\x89\x91`$\x96\x87\x91Z\xFA\x90\x81\x15a\x10\xCEW\x8A\x91a\x11iW[Pa\x0C\x1B\x90a\x0C \x92a\x1EZV[a\x1FIV[\x90a\x0C+\x820a\x1B6V[`\x01T\x16\x94\x84Qccsz\xC9`\xE1\x1B\x81R\x82\x85\x82\x01R\x87\x81\x85\x81\x8AZ\xFA\x90\x81\x15a\x10\xCEW\x8A\x91a\x11<W[P\x85Q\x91cp\xA0\x821`\xE0\x1B\x83R\x85\x83\x01R\x87\x82\x85\x81\x8AZ\xFA\x90\x81\x15a\x10\xCEW\x8A\x91a\x11\x0BW[a\x0C\x87\x92Pa\x1B)V[\x84Q\x90c\x03\xD1h\x9D`\xE1\x1B\x82R\x84\x82\x01R\x86\x81\x84\x81\x89Z\xFA\x80\x15a\x11\x01W\x90\x87\x91a\x10\xD8W[PP\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-`\x80\x85Q\x86\x81R`\x19\x87\x82\x01R\x7FTokens to use in deposit:\0\0\0\0\0\0\0``\x82\x01R\x83\x89\x82\x01R\xA1\x83Qc\xEF\x8B0\xF7`\xE0\x1B\x80\x82R\x84\x82\x01\x83\x90R\x93\x90\x87\x81\x85\x81\x8AZ\xFA\x90\x81\x15a\x10\xCEW\x8A\x91a\x10\x9DW[P\x15a\x05gW`\x08\x97\x85Q\x93a\rH\x85a\x17\xC6V[`\x06\x85R\x8Ae9\xB2\xB722\xB9`\xD1\x1B\x93\x84\x8B\x88\x01R\x8Aa\r\xDCa\r\xA0\x8BQa\ro\x81a\x17\xC6V[`\x0E\x81R\x7Fbefore deposit\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9A\x8B\x85\x83\x01R0a\x18\xA7V[P\x98\x8BQ\x9E\x8Fa\r\xAF\x81a\x17\xC6V[Rg92\xB1\xB2\xB4\xBB2\xB9`\xC1\x1B\x9E\x8F\x84\x82\x01R\x8CQ\x91a\r\xCE\x83a\x17\xC6V[`\x0E\x83R\x84\x83\x01R\x87a\x18\xA7V[\x99\x90P\x8AQ\x94\x85\x91\x82R\x88\x84\x83\x01R\x81\x8DZ\xFA\x92\x83\x15a\x10\x93W\x90\x82\x91\x8C\x93\x94a\x10]W[P\x89QcnU?e`\xE0\x1B\x81R\x90\x81\x01\x87\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16` \x82\x01R\x90\x9A\x8B\x92\x91\x83\x90\x03`@\x01\x91\x83\x91Z\xF1\x97\x88\x15a\n\x8BW\x8B\x98a\x10\x1DW[P\x92a\x0Fua\x0F\xD5\x95\x93a\t\xA9\x8Ba\x04\xB2\x9Da\x04q\x96a\x0F\x04\x8D\x8F\x9B\x7Ft number of shares to the receiv\x9E\x9C\x7Fer than or equal to the shares p\x83Q\x93a\x0E\xAD\x85a\x17\xE2V[`\\\x85R\x7Fdeposit() must always mint great\x89\x86\x01R\x84\x01R\x7Fredicted by previewDeposit()\0\0\0\0``\x84\x01Ra\x1C\xFAV[\x8CQ\x94a\x0F\x10\x86a\x17\xC6V[`\x06\x86R\x83\x86\x01Ra\x0FG\x8DQa\x0F&\x81a\x17\xC6V[`\r\x81Rl\x18Y\x9D\x19\\\x88\x19\x19\\\x1B\xDC\xDA]`\x9A\x1B\x96\x87\x86\x83\x01R0a\x18\xA7V[P\x94\x8DQ\x92a\x0FU\x84a\x17\xC6V[`\x08\x84R\x84\x84\x01R\x8DQ\x93a\x0Fi\x85a\x17\xC6V[`\r\x85R\x84\x01Ra\x18\xA7V[\x90\x87Q\x91a\x0F\x82\x83a\x17\xFEV[`=\x83R\x7Fdeposit() must consume exactly t\x8B\x84\x01R\x7Fhe number of tokens requested\0\0\0\x89\x84\x01Ra\x1C\x05V[\x91\x7Fdeposit() must credit the correc\x81Q\x95a\x10\x03\x87a\x17\xE2V[`B\x87R\x86\x01R\x84\x01Ra2\xB9`\xF1\x1B``\x84\x01Ra\x1C\x05V[\x91\x94\x92\x97P\x97\x98\x94\x92\x89\x82\x81=\x83\x11a\x10VW[a\x10;\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\n\x80W\x89\x98\x87\x92Q\x98\x93\x95\x92P\x90\x93\x95\x99a\x0EAV[P=a\x101V[\x83\x81\x94\x92\x95P=\x83\x11a\x10\x8CW[a\x10u\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\x89W\x90Q\x91\x8A\x91\x90\x86a\x0E\x01V[\x80\xFD[P=a\x10kV[\x89Q=\x84\x82>=\x90\xFD[\x90P\x87\x81\x81=\x83\x11a\x10\xC7W[a\x10\xB4\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\xC3WQ\x8Aa\r3V[\x89\x80\xFD[P=a\x10\xAAV[\x86Q=\x8C\x82>=\x90\xFD[\x81=\x83\x11a\x10\xFAW[a\x10\xEB\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05AW\x85\x89a\x0C\xADV[P=a\x10\xE1V[\x85Q=\x8B\x82>=\x90\xFD[\x90P\x87\x82\x81=\x83\x11a\x115W[a\x11\"\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\xC3Wa\x0C\x87\x91Q\x90a\x0C}V[P=a\x11\x18V[\x90P\x87\x81\x81=\x83\x11a\x11bW[a\x11S\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\xC3WQ\x8Aa\x0CVV[P=a\x11IV[\x90P\x87\x81\x81=\x83\x11a\x11\x91W[a\x11\x80\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\xC3WQa\x0C a\x0C\rV[P=a\x11vV[\x83\x90\x85\x82\x854a\x06!Wa\x11\xB6\x93a\x11\xAF6a\x17\x81V[\x95\x90a\x1A\xE3V[`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91c\xCE\x96\xCBw`\xE0\x1B\x81R0\x86\x82\x01R` \x95`$\x97\x87\x83\x8A\x81\x88Z\xFA\x92\x83\x15a\x17wW\x87\x93a\x17HW[P\x82\x15a\x16tW\x85Qcp\xA0\x821`\xE0\x1B\x80\x82R0\x84\x83\x01R\x89\x82\x8C\x81\x8AZ\xFA\x91\x82\x15a\x16\xA7W\x89\x92a\x17\x19W[P\x81\x15a\x05gW\x87Qc\n(\xA4w`\xE0\x1B\x93\x84\x82R\x85\x82\x01R\x8A\x81\x8D\x81\x8BZ\xFA\x90\x81\x15a\x16\xE2W\x8A\x91a\x16\xECW[P\x88Q\x91\x82R0\x85\x83\x01R\x8A\x82\x8D\x81\x8BZ\xFA\x90\x81\x15a\x16\xE2W\x8A\x91a\x16\xB1W[a\x12s\x92Pa\x1EZV[\x90\x81\x11a\x05AW\x86Qc&mj\x83`\xE1\x1B\x81R\x83\x81\x01\x82\x90R\x93\x89\x85\x8C\x81\x8AZ\xFA\x94\x85\x15a\x16\xA7W\x89\x95a\x16xW[P\x84\x11a\x05AW\x15a\x16tW\x87\x90\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-`\x80\x88Q\x89\x81R`\x1A\x8A\x82\x01R\x7FTokens to use in withdraw:\0\0\0\0\0\0``\x82\x01R\x86\x85\x82\x01R\xA1\x86Q\x99\x8A\x91\x82R\x84\x84\x83\x01R\x81\x87Z\xFA\x92\x83\x15a\x16jW\x86\x93a\x16:W[`\x08\x98P\x85Q\x93a\x138\x85a\x17\xC6V[`\x05\x85R\x88a\x14\td7\xBB\xB72\xB9`\xD9\x1B\x97\x88\x83\x89\x01Ra\x13\xCEa\x13\x91\x8BQa\x13`\x81a\x17\xC6V[`\x0F\x81R\x7Fbefore withdraw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9A\x8B\x87\x83\x01R0a\x18\xA7V[\x99\x90P\x8BQ\x9E\x8Fa\x13\xA1\x81a\x17\xC6V[Rg92\xB1\xB2\xB4\xBB2\xB9`\xC1\x1B\x9E\x8F\x86\x82\x01R\x8CQ\x91a\x13\xC0\x83a\x17\xC6V[`\x0F\x83R\x86\x83\x01R\x87a\x18\xA7V[P\x8AQc-\x18+\xE5`\xE2\x1B\x81R\x96\x87\x01\x88\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R0`@\x82\x01R\x96\x9B\x90\x96\x8C\x93\x84\x92\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x97\x88\x15a\n\x8BW\x8B\x98a\x15\xFAW[P\x92a\x15`a\x15\xCC\x95\x93a\x04\x01\x8Ba\x04\xB2\x9Da\x04q\x96a\x14\xDE\x8D\x8F\x9B\x7Fct number of shares from the own\x9E\x9C\x7F or equal to the number of share\x83Q\x93a\x14\x87\x85a\x17\xE2V[``\x85R\x7Fwithdraw() must redeem less than\x89\x86\x01R\x84\x01R\x7Fs predicted by previewWithdraw()``\x84\x01Ra\x1D\xCEV[\x8CQ\x99a\x14\xEA\x8Ba\x17\xC6V[`\x05\x8BR\x83\x8B\x01Ra\x151\x8DQa\x15\0\x81a\x17\xC6V[`\x0E\x81R\x7Fafter withdraw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9B\x8C\x86\x83\x01R0a\x18\xA7V[\x9A\x90P\x8DQ\x92a\x15@\x84a\x17\xC6V[`\x08\x84R\x84\x84\x01R\x8DQ\x93a\x15T\x85a\x17\xC6V[`\x0E\x85R\x84\x01Ra\x18\xA7V[\x90\x87Q\x91a\x15m\x83a\x17\xE2V[`C\x83R\x7Fwithdraw() must credit the corre\x8B\x84\x01R\x7Fct number of assets to the recei\x89\x84\x01Rb;2\xB9`\xE9\x1B``\x84\x01Ra\x1C\x05V[\x91\x7Fwithdraw() must deduct the corre\x81Q\x95a\x10\x03\x87a\x17\xE2V[\x91\x94\x92\x97P\x97\x98\x94\x92\x89\x82\x81=\x83\x11a\x163W[a\x16\x18\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\n\x80W\x89\x98\x87\x92Q\x98\x93\x95\x92P\x90\x93\x95\x99a\x14\x1BV[P=a\x16\x0EV[\x92P\x86\x88\x81=\x83\x11a\x16cW[a\x16Q\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x0B-W`\x08\x97Q\x92a\x13(V[P=a\x16GV[\x85Q=\x88\x82>=\x90\xFD[\x86\x80\xFD[\x90\x94P\x89\x81\x81=\x83\x11a\x16\xA0W[a\x16\x90\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05gWQ\x93\x8Ca\x12\xA2V[P=a\x16\x86V[\x88Q=\x8B\x82>=\x90\xFD[\x90P\x8A\x82\x81=\x83\x11a\x16\xDBW[a\x16\xC8\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\xC3Wa\x12s\x91Q\x90a\x12iV[P=a\x16\xBEV[\x89Q=\x8C\x82>=\x90\xFD[\x90P\x8A\x81\x81=\x83\x11a\x17\x12W[a\x17\x03\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\xC3WQ\x8Da\x12IV[P=a\x16\xF9V[\x90\x91P\x89\x81\x81=\x83\x11a\x17AW[a\x171\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05gWQ\x90\x8Ca\x12\x1BV[P=a\x17'V[\x90\x92P\x87\x81\x81=\x83\x11a\x17pW[a\x17`\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x16tWQ\x91\x8Aa\x11\xEDV[P=a\x17VV[\x86Q=\x89\x82>=\x90\xFD[`@\x90`\x03\x19\x01\x12a\x17\x97W`\x045\x90`$5\x90V[`\0\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x17\xB0W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x17\xB0W`@RV[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x17\xB0W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x17\xB0W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x17\xB0W`@RV[\x91\x90\x82\x03\x91\x82\x11a\x18IWV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0[\x83\x81\x10a\x18rWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x18bV[\x90` \x91a\x18\x9B\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x18_V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x92\x90\x91\x92`\x01`\x01`\xA0\x1B\x03\x93`\0\x85\x81T\x16`@\x91\x82Q\x94\x85\x92\x89cp\xA0\x821`\xE0\x1B\x92\x83\x86R\x16\x90\x81`\x04\x86\x01R\x84`$` \x96\x87\x93Z\xFA\x96\x87\x15a\x1A\xD9W\x83\x97a\x1A\xAAW[P\x83\x90`$\x88\x9B`\x01T\x16\x93\x87Q\x94\x85\x93\x84\x92\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a\x1A\x9FW\x80\x92a\x1AoW[PP\x82Q\x7Fasset.balanceOf(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x82\x01R\x87Q\x91\x97\x90\x95\x83\x82\x01\x95\x89\x93a\x19\\\x81`0\x8B\x01\x8Aa\x18_V[\x88\x01\x90b\x05$\x05`\xEB\x1B\x90\x81`0\x84\x01R\x80Q\x87\x82\x01\x93\x81`3\x82\x01\x90a\x19\x83\x91\x87a\x18_V[\x01\x93\x8A`)`\xF8\x1B\x95\x86`3\x82\x01R\x03`\x14\x81\x01\x8CR`4\x01a\x19\xA6\x90\x8Ca\x18\x1AV[\x88Q\x9A\x89\x8CR\x89\x8C\x01a\x19\xB8\x91a\x18\x82V[\x90\x88\x8C\x01R\x8A\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x9B\x8C\x92\x03\x90\xA1\x87Q\x98\x89\x95\x88\x87\x01\x7Fvault.balanceOf(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90RQ\x90\x81`0\x88\x01a\x1A \x92a\x18_V[\x85\x01\x91`0\x83\x01RQ\x91\x82`3\x83\x01a\x1A8\x92a\x18_V[\x01\x90`3\x82\x01R\x03`\x14\x81\x01\x85R`4\x01a\x1AS\x90\x85a\x18\x1AV[\x82Q\x93\x83\x85\x94\x85R\x84\x01a\x1Af\x91a\x18\x82V[\x91\x83\x01R\x03\x90\xA1V[\x90\x91P\x82\x82\x81=\x83\x11a\x1A\x98W[a\x1A\x87\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\x89WPQ8\x80a\x19\x19V[P=a\x1A}V[\x84Q\x90=\x90\x82>=\x90\xFD[\x90\x96P\x83\x81\x81=\x83\x11a\x1A\xD2W[a\x1A\xC2\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x0B\xACWQ\x95\x83a\x18\xEFV[P=a\x1A\xB8V[\x85Q=\x85\x82>=\x90\xFD[`\x03\x90\x06\x80\x15a\x1B$W`\x01\x14a\x1B\x0CWs\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\x90V[s\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\x90V[P0\x90V[\x91\x90\x82\x01\x80\x92\x11a\x18IWV[\x90`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x84T\x16\x90\x81;\x15a\n\xFAW\x84\x83`D\x82\x93`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R\x16\x96\x87`\x04\x84\x01R\x89`$\x84\x01RZ\xF1\x80\x15a\x1B\xFAWa\x1B\xE7W[P\x83T`\x01T\x83\x16\x92\x16\x80;\x15a\n\xFAW\x84\x92\x91\x83`d\x92`@Q\x96\x87\x95\x86\x94cQ\xBB\x10\xCF`\xE0\x1B\x86R`\x04\x86\x01R`$\x85\x01R`D\x84\x01RZ\xF1\x80\x15a\x1B\xDCWa\x1B\xCBWPPV[a\x1B\xD5\x82\x91a\x17\x9CV[a\x10\x89WPV[`@Q=\x84\x82>=\x90\xFD[a\x1B\xF3\x90\x94\x91\x94a\x17\x9CV[\x928a\x1B\x82V[`@Q=\x87\x82>=\x90\xFD[\x81\x81\x03a\x1C\x11WPPPV[\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x93P`5a\x1CKa\x1CEa\x1C\xCD\x93a \xC3V[\x93a \xC3V[`@Q\x94\x85\x91` \x95h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x87\x85\x01Ra\x1Cx\x81Q\x80\x92\x89`)\x88\x01\x91\x01a\x18_V[\x83\x01a!=`\xF0\x1B`)\x82\x01Ra\x1C\x98\x82Q\x80\x93\x89`+\x85\x01\x91\x01a\x18_V[\x01i\x01a\x03\x93+\x0B\x9B{q\xD1`\xB5\x1B`+\x82\x01Ra\x1C\xBE\x82Q\x80\x93\x88\x87\x85\x01\x91\x01a\x18_V[\x01\x03`\x15\x81\x01\x85R\x01\x83a\x18\x1AV[a\x1C\xE1`@Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x18\x82V[\x03\x90\xA1cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x81\x81\x10a\x1D\x06WPPPV[\x7F\x94BN\xD2O\xB3\x968\xB6H\x17\xC77\xDDD?8z\xAA\x14\x86aM\xA4I\xB6hjd-ml\x93P`;a\x1D:a\x1CEa\x1C\xCD\x93a \xC3V[`@Q\x94\x85\x91` \x95h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x87\x85\x01Ra\x1Dg\x81Q\x80\x92\x89`)\x88\x01\x91\x01a\x18_V[\x83\x01`\x0F`\xFA\x1B`)\x82\x01Ra\x1D\x86\x82Q\x80\x93\x89`*\x85\x01\x91\x01a\x18_V[\x01\x7F failed, reason: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`*\x82\x01Ra\x1D\xBF\x82Q\x80\x93\x88\x87\x85\x01\x91\x01a\x18_V[\x01\x03`\x1B\x81\x01\x85R\x01\x83a\x18\x1AV[\x81\x81\x11a\x1D\xDAWPPPV[\x7Fb\xBD\xDA\x9A\x05\xCD\xBC\xDB\xF9\x05\xCB\xAD\x99\xC6\xEB\xDC\t\x8Bo\t3\xD8\xF2\xEB<\xFA\xB7@\x0B`%\x14\x93P`;a\x1E\x0Ea\x1CEa\x1C\xCD\x93a \xC3V[`@Q\x94\x85\x91` \x95h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x87\x85\x01Ra\x1E;\x81Q\x80\x92\x89`)\x88\x01\x91\x01a\x18_V[\x83\x01`\x1F`\xF9\x1B`)\x82\x01Ra\x1D\x86\x82Q\x80\x93\x89`*\x85\x01\x91\x01a\x18_V[\x90\x80\x82\x11a\x1EfWP\x90V[`\x01\x81\x01\x80\x91\x11a\x18IW\x80\x15a\x1F3Wa\x1F-a\x1E\xA7\x7F\xA9^n*\x18$\x11\xE7\xA6\xF9\xED\x11J\x85\xC3v\x1D\x87\xF9\xB8\xF4S\xD8B\xC7\x125\xAAd\xFF\xF9\x9F\x92\x84\x06\x93a \xC3V[a\x1F\x19`3a\x1E\xB5\x86a \xC3V[\x92`@Q\x93\x84\x91n\x02\x1Bc\x0Bk\x83Ks9\x03\xB3\x0Bc\xAB)`\x8D\x1B` \x84\x01Ra\x1E\xE8\x81Q\x80\x92` `/\x87\x01\x91\x01a\x18_V[\x82\x01c\x01\x03\xA3y`\xE5\x1B`/\x82\x01Ra\x1F\n\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x18_V[\x01\x03`\x13\x81\x01\x84R\x01\x82a\x18\x1AV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x18\x82V[\x03\x90\xA1\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x80a \xC0Wa\x1F\xD8`@\x91\x7Fk your inputs/assumptions.\0\0\0\0\0\0``\x84Qa\x1F\x82\x81a\x17\xE2V[`Z\x81R\x7FclampGt cannot clamp value a to ` \x82\x01R\x7Fbe larger than uint256.max. Chec\x86\x82\x01R\x01Ra \xC3V[\x90\x80Q\x90`\xA0\x82\x01\x81R`\x80\x82\x01\x92`\0\x84R`\x01\x93\x84\x80[a \xA2W[P\x91a \x90`3a \x81\x93\x85\x7F\xA9^n*\x18$\x11\xE7\xA6\xF9\xED\x11J\x85\xC3v\x1D\x87\xF9\xB8\xF4S\xD8B\xC7\x125\xAAd\xFF\xF9\x9F\x97`\x80a\x1F-\x98`\x1F\x19\x81\x01\x92\x03\x01\x81R\x85Q\x96\x87\x93n\x02\x1Bc\x0Bk\x83Ks9\x03\xB3\x0Bc\xAB)`\x8D\x1B` \x86\x01Ra e\x81Q\x80\x92` `/\x89\x01\x91\x01a\x18_V[\x84\x01\x91c\x01\x03\xA3y`\xE5\x1B`/\x84\x01RQ\x80\x93\x86\x84\x01\x90a\x18_V[\x01\x03`\x13\x81\x01\x85R\x01\x83a\x18\x1AV[Q\x91\x82\x91` \x83R` \x83\x01\x90a\x18\x82V[\x90`\0\x19\x01\x90`\n\x90\x81\x81\x06`0\x01\x83S\x04\x85\x81a\x1F\xF1WPa\x1F\xF6V[\x90V[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a \xD9W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 2\x8A\xE7\x81em\xDE\xA4.R\x95!`\xDA\xE1\xC1\xB5\xC0N\x06\xF3\xE1MXE'\xD1\xEA v\xA91dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static CRYTICERC4626FUNCTIONALACCOUNTING_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x90\x80\x82R`\x04\x91\x826\x10\x15a\0\x17W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x90\x81c\"\x18\x03\xD8\x14a\x11\x98WP\x80c+\xEF\xB0\xC6\x14a\x0B\xB0W\x80c,(\xD0\xB7\x14a\x06%Wc\xEAx1\xE9\x14a\0QW`\0\x80\xFD[4a\x06!Wa\0j\x90a\0c6a\x17\x81V[\x92\x90a\x1A\xE3V[\x90\x82\x15a\x06\x1DW`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x94\x81Q\x92cl\x82\xBB\xBF`\xE1\x1B\x84R0\x82\x85\x01R` \x94`$\x94\x86\x81\x87\x81\x8CZ\xFA\x90\x81\x15a\x05\xADW\x88\x91a\x05\xF0W[P\x80\x15a\x05AW\x84Qcp\xA0\x821`\xE0\x1B\x81R0\x85\x82\x01R\x91\x87\x83\x88\x81\x8DZ\xFA\x92\x83\x15a\x05\xE6W\x89\x93a\x05\xB7W[P\x82\x15a\x05gWa\0\xF3\x92\x91a\0\xEE\x91a\x1EZV[a\x1EZV[\x93\x83Qc\x03\xD1h\x9D`\xE1\x1B\x81R\x85\x84\x82\x01R\x86\x81\x83\x81\x8CZ\xFA\x90\x81\x15a\x05\xADW\x90\x89\x91\x89\x91a\x05|W[P\x85Q\x88\x81\x85\x81c&mj\x83`\xE1\x1B\x96\x87\x82R\x8C\x8B\x83\x01RZ\xFA\x80\x15a\x05rW\x90\x89\x91a\x05EW[PP\x15a\x05AW\x91`\x08\x98\x91\x87\x93\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-`\x80\x88Q\x89\x81R`\x1C\x8A\x82\x01R\x7FShares to use in redemption:\0\0\0\0``\x82\x01R\x8A\x88\x82\x01R\xA1\x86Q\x95a\x01\xB9\x87a\x17\xC6V[`\x05\x87Ra\x02;a\x01\xFEd7\xBB\xB72\xB9`\xD9\x1B\x98\x89\x89\x82\x01R\x8AQ\x90a\x01\xDE\x82a\x17\xC6V[`\r\x82Rlbefore redeem`\x98\x1B\x80\x9A\x83\x01R0a\x18\xA7V[\x97\x90P\x89Q\x9D\x8Ea\x02\x0E\x81a\x17\xC6V[Rg92\xB1\xB2\xB4\xBB2\xB9`\xC1\x1B\x9D\x8E\x8D\x82\x01R\x8AQ\x91a\x02-\x83a\x17\xC6V[`\r\x83R\x8D\x83\x01R\x86a\x18\xA7V[P\x94\x88Q\x92\x83R\x89\x82\x84\x01R\x8A\x83\x85\x81\x84Z\xFA\x92\x83\x15a\x04\xF9W\x8C\x96\x95\x94\x93\x92\x91\x8C\x91\x88\x94a\x05\x03W[P\x8AQc]\x04;)`\xE1\x1B\x81R\x83\x81\x01\x8D\x81R`\x01`\x01`\xA0\x1B\x03\x88\x16` \x82\x01R0`@\x82\x01R\x90\x98\x89\x92\x91\x83\x91\x90\x82\x90``\x01[\x03\x92Z\xF1\x95\x86\x15a\x04\xF9W\x8C\x96a\x04\xC6W[P\x88Q\x92`\xA0\x84\x01\x91\x84\x83\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x17a\x04\xB5WPP\x93a\x04\x01\x8Ba\x04\xB2\x9D\x9Ea\x04q\x96a\x03\x90\x7F number of shares from the owner\x9C\x9A\x97a\x04\x07\x97\x8F\x80a\x04v\x9F\x9D\x7Fhan or equal to the number of as\x92R`a\x84R\x7Fredeem() must withdraw greater t\x88\x85\x01R\x83\x01R\x7Fsets predicted by previewRedeem(``\x83\x01R`)`\xF8\x1B`\x80\x83\x01R\x89a\x1C\xFAV[\x8CQ\x99a\x03\x9C\x8Ba\x17\xC6V[`\x05\x8BR\x83\x8B\x01Ra\x03\xD2\x8DQa\x03\xB2\x81a\x17\xC6V[`\x0C\x81Rkafter redeem`\xA0\x1B\x9B\x8C\x86\x83\x01R0a\x18\xA7V[\x9A\x90P\x8DQ\x92a\x03\xE1\x84a\x17\xC6V[`\x08\x84R\x84\x84\x01R\x8DQ\x93a\x03\xF5\x85a\x17\xC6V[`\x0C\x85R\x84\x01Ra\x18\xA7V[Pa\x18<V[\x90\x87Q\x91a\x04\x14\x83a\x17\xE2V[`A\x83R\x7Fredeem() must credit the correct\x8B\x84\x01R\x7F number of assets to the receive\x89\x84\x01R`9`\xF9\x1B``\x84\x01Ra\x1C\x05V[a\x18<V[\x91\x7Fredeem() must deduct the correct\x81Q\x95a\x04\xA4\x87a\x17\xFEV[\x82\x87R\x86\x01R\x84\x01Ra\x1C\x05V[\x80\xF3[cNH{q`\xE0\x1B\x8ER`A\x90R\x8C\xFD[\x90\x95P\x8A\x81\x81=\x83\x11a\x04\xF2W[a\x04\xDE\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x04\xEEWQ\x948a\x02\xADV[\x8B\x80\xFD[P=a\x04\xD4V[\x89Q=\x8E\x82>=\x90\xFD[\x92\x93P\x93\x94\x95\x96P\x81\x81=\x83\x11a\x05:W[a\x05\x1F\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x04\xEEW\x90\x8Aa\x02\x9B\x8D\x97\x96\x95\x94\x93Q\x93\x90a\x02eV[P=a\x05\x15V[\x87\x80\xFD[\x81=\x83\x11a\x05kW[a\x05X\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05gW\x878a\x01EV[\x88\x80\xFD[P=a\x05NV[\x87Q=\x8C\x82>=\x90\xFD[\x80\x92P\x88\x80\x92P=\x83\x11a\x05\xA6W[a\x05\x95\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05AW\x88\x90Q8a\x01\x1DV[P=a\x05\x8BV[\x85Q=\x8A\x82>=\x90\xFD[\x90\x92P\x87\x81\x81=\x83\x11a\x05\xDFW[a\x05\xCF\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05gWQ\x918a\0\xD9V[P=a\x05\xC5V[\x86Q=\x8B\x82>=\x90\xFD[\x90P\x86\x81\x81=\x83\x11a\x06\x16W[a\x06\x07\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05AWQ8a\0\xABV[P=a\x05\xFDV[\x83\x80\xFD[P\x80\xFD[P\x82\x904a\x0B\xACWa\x06A\x91a\x06:6a\x17\x81V[\x93\x90a\x1A\xE3V[\x91\x84`\x01`\x01`\xA0\x1B\x03\x92\x83`\x01T\x16\x94\x83Qc\xB3\xD7\xF6\xB9`\xE0\x1B\x95\x86\x82R\x88\x84\x83\x01R` \x97\x88\x83`$\x81\x84Z\xFA\x92\x83\x15a\x0B8W\x86\x93a\x0B}W[P\x86Qcc\x1E\xBA\xDB`\xE1\x1B\x81R\x84\x83\x16\x86\x82\x01\x81\x90R\x9A\x90\x8A\x81`$\x81\x86Z\xFA\x90\x81\x15a\x0BsW\x90\x8B\x91\x89\x91a\x0BBW[Pa\x06\xBA\x91\x92a\x1EZV[\x98`$\x89Q\x80\x94\x81\x93\x82R\x8C\x8A\x83\x01RZ\xFA\x80\x15a\x0B8W\x86\x90a\x0B\x05W[a\x06\xE4\x91P0a\x1B6V[`\x01T\x16\x97\x85Q\x90cp\xA0\x821`\xE0\x1B\x82R\x84\x82\x01R\x87\x81`$\x81\x8CZ\xFA\x80\x15a\n\xC6W\x87\x90\x86\x90a\n\xD0W[a\x07\x1B\x92Pa\x1B)V[\x85Q\x90c&mj\x83`\xE1\x1B\x82R\x84\x82\x01R\x87\x81`$\x81\x8CZ\xFA\x80\x15a\n\xC6W\x91\x87\x93\x91\x89\x93a\n\x95W[P\x86Q\x87\x81R\x87\x81\x01`\x16\x90R``\x81\x01\x7FShares to use in mint:\0\0\0\0\0\0\0\0\0\0\x90R\x84\x84\x82\x01R`\x80\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x91\xA1\x82\x82\x88Q\x9Ba\x07\xB1\x8Da\x17\xC6V[`\x06\x8DR\x8Ce9\xB2\xB722\xB9`\xD1\x1B\x96\x87\x91\x01R\x89Q\x9C\x8Da\x07\xD2\x81a\x17\xC6V[`\x0B\x90Rj\x18\x99Y\x9B\xDC\x99H\x1BZ[\x9D`\xAA\x1B\x9D\x8E\x85\x82\x01R0\x91a\x07\xF6\x92a\x18\xA7V[P\x97\x8AQa\x08\x03\x81a\x17\xC6V[`\x08\x81Rg92\xB1\xB2\xB4\xBB2\xB9`\xC1\x1B\x9E\x8F\x86\x83\x01R\x8CQ\x90a\x08%\x82a\x17\xC6V[`\x0B\x82R\x86\x82\x01Ra\x087\x91\x85a\x18\xA7V[\x8CQc\x94\xBF\x80M`\xE0\x1B\x81R\x92\x83\x01\x99\x8AR`\x01`\x01`\xA0\x1B\x03\x90\x94\x16` \x8A\x01RP\x91\x98\x91\x96\x87\x92\x83\x91\x82\x90`@\x01\x03\x92Z\xF1\x93\x84\x15a\n\x8BW\x8B\x94a\nNW[P\x92a\t\xB1a\n\x11\x95\x93a\t\xA9\x8Ba\x04\xB2\x9Da\x04q\x96a\t;\x8D\x7Fumber of shares to the receiver\0\x9D\x9B\x7Fthan or equal to the tokens pred\x82Q\x92a\x08\xE3\x84a\x17\xE2V[`V\x84R\x7Fmint() must always consume less \x88\x85\x01R\x83\x01R\x7Ficted by previewMint()\0\0\0\0\0\0\0\0\0\0``\x83\x01R\x89a\x1D\xCEV[\x8CQ\x94a\tG\x86a\x17\xC6V[`\x06\x86R\x83\x86\x01Ra\t{\x8DQa\t]\x81a\x17\xC6V[`\n\x81Ri\x18Y\x9D\x19\\\x88\x1BZ[\x9D`\xB2\x1B\x96\x87\x86\x83\x01R0a\x18\xA7V[P\x94\x8DQ\x92a\t\x89\x84a\x17\xC6V[`\x08\x84R\x84\x84\x01R\x8DQ\x93a\t\x9D\x85a\x17\xC6V[`\n\x85R\x84\x01Ra\x18\xA7V[\x95\x90Pa\x18<V[\x90\x87Q\x91a\t\xBE\x83a\x17\xFEV[`:\x83R\x7Fmint() must consume exactly the \x8B\x84\x01R\x7Fnumber of tokens requested\0\0\0\0\0\0\x89\x84\x01Ra\x1C\x05V[\x91\x7Fmint() must credit the correct n\x81Q\x95a\n?\x87a\x17\xFEV[`?\x87R\x86\x01R\x84\x01Ra\x1C\x05V[\x91\x94\x92\x95\x93P\x88\x82\x81=\x83\x11a\n\x84W[a\ni\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\n\x80W\x90Q\x92\x94\x91\x93\x90a\t\xB1a\x08yV[\x8A\x80\xFD[P=a\n_V[\x87Q=\x8D\x82>=\x90\xFD[\x90\x92\x80\x92\x94P=\x83\x11a\n\xBFW[a\n\xAD\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x06\x1DW\x85\x91\x87\x91\x8Ba\x07EV[P=a\n\xA3V[\x86Q=\x87\x82>=\x90\xFD[PP\x87\x81\x81=\x83\x11a\n\xFEW[a\n\xE7\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\n\xFAW\x86a\x07\x1B\x91Qa\x07\x11V[\x84\x80\xFD[P=a\n\xDDV[P\x88\x81\x81=\x83\x11a\x0B1W[a\x0B\x1B\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x0B-Wa\x06\xE4\x90Qa\x06\xD9V[\x85\x80\xFD[P=a\x0B\x11V[\x87Q=\x88\x82>=\x90\xFD[\x82\x81\x93\x92P=\x83\x11a\x0BlW[a\x0BY\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05AWQ\x8A\x90a\x06\xBAa\x06\xAFV[P=a\x0BOV[\x89Q=\x8A\x82>=\x90\xFD[\x90\x92P\x88\x81\x81=\x83\x11a\x0B\xA5W[a\x0B\x95\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x0B-WQ\x91\x8Ba\x06~V[P=a\x0B\x8BV[\x82\x80\xFD[P\x824a\x0B\xACWa\x0B\xCAa\x0B\xC36a\x17\x81V[\x91\x90a\x1A\xE3V[\x92`\x01`\x01`\xA0\x1B\x03\x92\x83`\x01T\x16\x92\x82Q\x94\x85\x91c@-&}`\xE0\x1B\x83R\x82\x82\x89\x16\x96\x87\x86\x83\x01R` \x98\x89\x91`$\x96\x87\x91Z\xFA\x90\x81\x15a\x10\xCEW\x8A\x91a\x11iW[Pa\x0C\x1B\x90a\x0C \x92a\x1EZV[a\x1FIV[\x90a\x0C+\x820a\x1B6V[`\x01T\x16\x94\x84Qccsz\xC9`\xE1\x1B\x81R\x82\x85\x82\x01R\x87\x81\x85\x81\x8AZ\xFA\x90\x81\x15a\x10\xCEW\x8A\x91a\x11<W[P\x85Q\x91cp\xA0\x821`\xE0\x1B\x83R\x85\x83\x01R\x87\x82\x85\x81\x8AZ\xFA\x90\x81\x15a\x10\xCEW\x8A\x91a\x11\x0BW[a\x0C\x87\x92Pa\x1B)V[\x84Q\x90c\x03\xD1h\x9D`\xE1\x1B\x82R\x84\x82\x01R\x86\x81\x84\x81\x89Z\xFA\x80\x15a\x11\x01W\x90\x87\x91a\x10\xD8W[PP\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-`\x80\x85Q\x86\x81R`\x19\x87\x82\x01R\x7FTokens to use in deposit:\0\0\0\0\0\0\0``\x82\x01R\x83\x89\x82\x01R\xA1\x83Qc\xEF\x8B0\xF7`\xE0\x1B\x80\x82R\x84\x82\x01\x83\x90R\x93\x90\x87\x81\x85\x81\x8AZ\xFA\x90\x81\x15a\x10\xCEW\x8A\x91a\x10\x9DW[P\x15a\x05gW`\x08\x97\x85Q\x93a\rH\x85a\x17\xC6V[`\x06\x85R\x8Ae9\xB2\xB722\xB9`\xD1\x1B\x93\x84\x8B\x88\x01R\x8Aa\r\xDCa\r\xA0\x8BQa\ro\x81a\x17\xC6V[`\x0E\x81R\x7Fbefore deposit\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9A\x8B\x85\x83\x01R0a\x18\xA7V[P\x98\x8BQ\x9E\x8Fa\r\xAF\x81a\x17\xC6V[Rg92\xB1\xB2\xB4\xBB2\xB9`\xC1\x1B\x9E\x8F\x84\x82\x01R\x8CQ\x91a\r\xCE\x83a\x17\xC6V[`\x0E\x83R\x84\x83\x01R\x87a\x18\xA7V[\x99\x90P\x8AQ\x94\x85\x91\x82R\x88\x84\x83\x01R\x81\x8DZ\xFA\x92\x83\x15a\x10\x93W\x90\x82\x91\x8C\x93\x94a\x10]W[P\x89QcnU?e`\xE0\x1B\x81R\x90\x81\x01\x87\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16` \x82\x01R\x90\x9A\x8B\x92\x91\x83\x90\x03`@\x01\x91\x83\x91Z\xF1\x97\x88\x15a\n\x8BW\x8B\x98a\x10\x1DW[P\x92a\x0Fua\x0F\xD5\x95\x93a\t\xA9\x8Ba\x04\xB2\x9Da\x04q\x96a\x0F\x04\x8D\x8F\x9B\x7Ft number of shares to the receiv\x9E\x9C\x7Fer than or equal to the shares p\x83Q\x93a\x0E\xAD\x85a\x17\xE2V[`\\\x85R\x7Fdeposit() must always mint great\x89\x86\x01R\x84\x01R\x7Fredicted by previewDeposit()\0\0\0\0``\x84\x01Ra\x1C\xFAV[\x8CQ\x94a\x0F\x10\x86a\x17\xC6V[`\x06\x86R\x83\x86\x01Ra\x0FG\x8DQa\x0F&\x81a\x17\xC6V[`\r\x81Rl\x18Y\x9D\x19\\\x88\x19\x19\\\x1B\xDC\xDA]`\x9A\x1B\x96\x87\x86\x83\x01R0a\x18\xA7V[P\x94\x8DQ\x92a\x0FU\x84a\x17\xC6V[`\x08\x84R\x84\x84\x01R\x8DQ\x93a\x0Fi\x85a\x17\xC6V[`\r\x85R\x84\x01Ra\x18\xA7V[\x90\x87Q\x91a\x0F\x82\x83a\x17\xFEV[`=\x83R\x7Fdeposit() must consume exactly t\x8B\x84\x01R\x7Fhe number of tokens requested\0\0\0\x89\x84\x01Ra\x1C\x05V[\x91\x7Fdeposit() must credit the correc\x81Q\x95a\x10\x03\x87a\x17\xE2V[`B\x87R\x86\x01R\x84\x01Ra2\xB9`\xF1\x1B``\x84\x01Ra\x1C\x05V[\x91\x94\x92\x97P\x97\x98\x94\x92\x89\x82\x81=\x83\x11a\x10VW[a\x10;\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\n\x80W\x89\x98\x87\x92Q\x98\x93\x95\x92P\x90\x93\x95\x99a\x0EAV[P=a\x101V[\x83\x81\x94\x92\x95P=\x83\x11a\x10\x8CW[a\x10u\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\x89W\x90Q\x91\x8A\x91\x90\x86a\x0E\x01V[\x80\xFD[P=a\x10kV[\x89Q=\x84\x82>=\x90\xFD[\x90P\x87\x81\x81=\x83\x11a\x10\xC7W[a\x10\xB4\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\xC3WQ\x8Aa\r3V[\x89\x80\xFD[P=a\x10\xAAV[\x86Q=\x8C\x82>=\x90\xFD[\x81=\x83\x11a\x10\xFAW[a\x10\xEB\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05AW\x85\x89a\x0C\xADV[P=a\x10\xE1V[\x85Q=\x8B\x82>=\x90\xFD[\x90P\x87\x82\x81=\x83\x11a\x115W[a\x11\"\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\xC3Wa\x0C\x87\x91Q\x90a\x0C}V[P=a\x11\x18V[\x90P\x87\x81\x81=\x83\x11a\x11bW[a\x11S\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\xC3WQ\x8Aa\x0CVV[P=a\x11IV[\x90P\x87\x81\x81=\x83\x11a\x11\x91W[a\x11\x80\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\xC3WQa\x0C a\x0C\rV[P=a\x11vV[\x83\x90\x85\x82\x854a\x06!Wa\x11\xB6\x93a\x11\xAF6a\x17\x81V[\x95\x90a\x1A\xE3V[`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91c\xCE\x96\xCBw`\xE0\x1B\x81R0\x86\x82\x01R` \x95`$\x97\x87\x83\x8A\x81\x88Z\xFA\x92\x83\x15a\x17wW\x87\x93a\x17HW[P\x82\x15a\x16tW\x85Qcp\xA0\x821`\xE0\x1B\x80\x82R0\x84\x83\x01R\x89\x82\x8C\x81\x8AZ\xFA\x91\x82\x15a\x16\xA7W\x89\x92a\x17\x19W[P\x81\x15a\x05gW\x87Qc\n(\xA4w`\xE0\x1B\x93\x84\x82R\x85\x82\x01R\x8A\x81\x8D\x81\x8BZ\xFA\x90\x81\x15a\x16\xE2W\x8A\x91a\x16\xECW[P\x88Q\x91\x82R0\x85\x83\x01R\x8A\x82\x8D\x81\x8BZ\xFA\x90\x81\x15a\x16\xE2W\x8A\x91a\x16\xB1W[a\x12s\x92Pa\x1EZV[\x90\x81\x11a\x05AW\x86Qc&mj\x83`\xE1\x1B\x81R\x83\x81\x01\x82\x90R\x93\x89\x85\x8C\x81\x8AZ\xFA\x94\x85\x15a\x16\xA7W\x89\x95a\x16xW[P\x84\x11a\x05AW\x15a\x16tW\x87\x90\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-`\x80\x88Q\x89\x81R`\x1A\x8A\x82\x01R\x7FTokens to use in withdraw:\0\0\0\0\0\0``\x82\x01R\x86\x85\x82\x01R\xA1\x86Q\x99\x8A\x91\x82R\x84\x84\x83\x01R\x81\x87Z\xFA\x92\x83\x15a\x16jW\x86\x93a\x16:W[`\x08\x98P\x85Q\x93a\x138\x85a\x17\xC6V[`\x05\x85R\x88a\x14\td7\xBB\xB72\xB9`\xD9\x1B\x97\x88\x83\x89\x01Ra\x13\xCEa\x13\x91\x8BQa\x13`\x81a\x17\xC6V[`\x0F\x81R\x7Fbefore withdraw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9A\x8B\x87\x83\x01R0a\x18\xA7V[\x99\x90P\x8BQ\x9E\x8Fa\x13\xA1\x81a\x17\xC6V[Rg92\xB1\xB2\xB4\xBB2\xB9`\xC1\x1B\x9E\x8F\x86\x82\x01R\x8CQ\x91a\x13\xC0\x83a\x17\xC6V[`\x0F\x83R\x86\x83\x01R\x87a\x18\xA7V[P\x8AQc-\x18+\xE5`\xE2\x1B\x81R\x96\x87\x01\x88\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R0`@\x82\x01R\x96\x9B\x90\x96\x8C\x93\x84\x92\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x97\x88\x15a\n\x8BW\x8B\x98a\x15\xFAW[P\x92a\x15`a\x15\xCC\x95\x93a\x04\x01\x8Ba\x04\xB2\x9Da\x04q\x96a\x14\xDE\x8D\x8F\x9B\x7Fct number of shares from the own\x9E\x9C\x7F or equal to the number of share\x83Q\x93a\x14\x87\x85a\x17\xE2V[``\x85R\x7Fwithdraw() must redeem less than\x89\x86\x01R\x84\x01R\x7Fs predicted by previewWithdraw()``\x84\x01Ra\x1D\xCEV[\x8CQ\x99a\x14\xEA\x8Ba\x17\xC6V[`\x05\x8BR\x83\x8B\x01Ra\x151\x8DQa\x15\0\x81a\x17\xC6V[`\x0E\x81R\x7Fafter withdraw\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9B\x8C\x86\x83\x01R0a\x18\xA7V[\x9A\x90P\x8DQ\x92a\x15@\x84a\x17\xC6V[`\x08\x84R\x84\x84\x01R\x8DQ\x93a\x15T\x85a\x17\xC6V[`\x0E\x85R\x84\x01Ra\x18\xA7V[\x90\x87Q\x91a\x15m\x83a\x17\xE2V[`C\x83R\x7Fwithdraw() must credit the corre\x8B\x84\x01R\x7Fct number of assets to the recei\x89\x84\x01Rb;2\xB9`\xE9\x1B``\x84\x01Ra\x1C\x05V[\x91\x7Fwithdraw() must deduct the corre\x81Q\x95a\x10\x03\x87a\x17\xE2V[\x91\x94\x92\x97P\x97\x98\x94\x92\x89\x82\x81=\x83\x11a\x163W[a\x16\x18\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\n\x80W\x89\x98\x87\x92Q\x98\x93\x95\x92P\x90\x93\x95\x99a\x14\x1BV[P=a\x16\x0EV[\x92P\x86\x88\x81=\x83\x11a\x16cW[a\x16Q\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x0B-W`\x08\x97Q\x92a\x13(V[P=a\x16GV[\x85Q=\x88\x82>=\x90\xFD[\x86\x80\xFD[\x90\x94P\x89\x81\x81=\x83\x11a\x16\xA0W[a\x16\x90\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05gWQ\x93\x8Ca\x12\xA2V[P=a\x16\x86V[\x88Q=\x8B\x82>=\x90\xFD[\x90P\x8A\x82\x81=\x83\x11a\x16\xDBW[a\x16\xC8\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\xC3Wa\x12s\x91Q\x90a\x12iV[P=a\x16\xBEV[\x89Q=\x8C\x82>=\x90\xFD[\x90P\x8A\x81\x81=\x83\x11a\x17\x12W[a\x17\x03\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\xC3WQ\x8Da\x12IV[P=a\x16\xF9V[\x90\x91P\x89\x81\x81=\x83\x11a\x17AW[a\x171\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x05gWQ\x90\x8Ca\x12\x1BV[P=a\x17'V[\x90\x92P\x87\x81\x81=\x83\x11a\x17pW[a\x17`\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x16tWQ\x91\x8Aa\x11\xEDV[P=a\x17VV[\x86Q=\x89\x82>=\x90\xFD[`@\x90`\x03\x19\x01\x12a\x17\x97W`\x045\x90`$5\x90V[`\0\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x17\xB0W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x17\xB0W`@RV[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x17\xB0W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x17\xB0W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x17\xB0W`@RV[\x91\x90\x82\x03\x91\x82\x11a\x18IWV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0[\x83\x81\x10a\x18rWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x18bV[\x90` \x91a\x18\x9B\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x18_V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x92\x90\x91\x92`\x01`\x01`\xA0\x1B\x03\x93`\0\x85\x81T\x16`@\x91\x82Q\x94\x85\x92\x89cp\xA0\x821`\xE0\x1B\x92\x83\x86R\x16\x90\x81`\x04\x86\x01R\x84`$` \x96\x87\x93Z\xFA\x96\x87\x15a\x1A\xD9W\x83\x97a\x1A\xAAW[P\x83\x90`$\x88\x9B`\x01T\x16\x93\x87Q\x94\x85\x93\x84\x92\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a\x1A\x9FW\x80\x92a\x1AoW[PP\x82Q\x7Fasset.balanceOf(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x82\x01R\x87Q\x91\x97\x90\x95\x83\x82\x01\x95\x89\x93a\x19\\\x81`0\x8B\x01\x8Aa\x18_V[\x88\x01\x90b\x05$\x05`\xEB\x1B\x90\x81`0\x84\x01R\x80Q\x87\x82\x01\x93\x81`3\x82\x01\x90a\x19\x83\x91\x87a\x18_V[\x01\x93\x8A`)`\xF8\x1B\x95\x86`3\x82\x01R\x03`\x14\x81\x01\x8CR`4\x01a\x19\xA6\x90\x8Ca\x18\x1AV[\x88Q\x9A\x89\x8CR\x89\x8C\x01a\x19\xB8\x91a\x18\x82V[\x90\x88\x8C\x01R\x8A\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x9B\x8C\x92\x03\x90\xA1\x87Q\x98\x89\x95\x88\x87\x01\x7Fvault.balanceOf(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90RQ\x90\x81`0\x88\x01a\x1A \x92a\x18_V[\x85\x01\x91`0\x83\x01RQ\x91\x82`3\x83\x01a\x1A8\x92a\x18_V[\x01\x90`3\x82\x01R\x03`\x14\x81\x01\x85R`4\x01a\x1AS\x90\x85a\x18\x1AV[\x82Q\x93\x83\x85\x94\x85R\x84\x01a\x1Af\x91a\x18\x82V[\x91\x83\x01R\x03\x90\xA1V[\x90\x91P\x82\x82\x81=\x83\x11a\x1A\x98W[a\x1A\x87\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x10\x89WPQ8\x80a\x19\x19V[P=a\x1A}V[\x84Q\x90=\x90\x82>=\x90\xFD[\x90\x96P\x83\x81\x81=\x83\x11a\x1A\xD2W[a\x1A\xC2\x81\x83a\x18\x1AV[\x81\x01\x03\x12a\x0B\xACWQ\x95\x83a\x18\xEFV[P=a\x1A\xB8V[\x85Q=\x85\x82>=\x90\xFD[`\x03\x90\x06\x80\x15a\x1B$W`\x01\x14a\x1B\x0CWs\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\x90V[s\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\x90V[P0\x90V[\x91\x90\x82\x01\x80\x92\x11a\x18IWV[\x90`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x84T\x16\x90\x81;\x15a\n\xFAW\x84\x83`D\x82\x93`@Q\x94\x85\x93\x84\x92c@\xC1\x0F\x19`\xE0\x1B\x84R\x16\x96\x87`\x04\x84\x01R\x89`$\x84\x01RZ\xF1\x80\x15a\x1B\xFAWa\x1B\xE7W[P\x83T`\x01T\x83\x16\x92\x16\x80;\x15a\n\xFAW\x84\x92\x91\x83`d\x92`@Q\x96\x87\x95\x86\x94cQ\xBB\x10\xCF`\xE0\x1B\x86R`\x04\x86\x01R`$\x85\x01R`D\x84\x01RZ\xF1\x80\x15a\x1B\xDCWa\x1B\xCBWPPV[a\x1B\xD5\x82\x91a\x17\x9CV[a\x10\x89WPV[`@Q=\x84\x82>=\x90\xFD[a\x1B\xF3\x90\x94\x91\x94a\x17\x9CV[\x928a\x1B\x82V[`@Q=\x87\x82>=\x90\xFD[\x81\x81\x03a\x1C\x11WPPPV[\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x93P`5a\x1CKa\x1CEa\x1C\xCD\x93a \xC3V[\x93a \xC3V[`@Q\x94\x85\x91` \x95h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x87\x85\x01Ra\x1Cx\x81Q\x80\x92\x89`)\x88\x01\x91\x01a\x18_V[\x83\x01a!=`\xF0\x1B`)\x82\x01Ra\x1C\x98\x82Q\x80\x93\x89`+\x85\x01\x91\x01a\x18_V[\x01i\x01a\x03\x93+\x0B\x9B{q\xD1`\xB5\x1B`+\x82\x01Ra\x1C\xBE\x82Q\x80\x93\x88\x87\x85\x01\x91\x01a\x18_V[\x01\x03`\x15\x81\x01\x85R\x01\x83a\x18\x1AV[a\x1C\xE1`@Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x18\x82V[\x03\x90\xA1cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x81\x81\x10a\x1D\x06WPPPV[\x7F\x94BN\xD2O\xB3\x968\xB6H\x17\xC77\xDDD?8z\xAA\x14\x86aM\xA4I\xB6hjd-ml\x93P`;a\x1D:a\x1CEa\x1C\xCD\x93a \xC3V[`@Q\x94\x85\x91` \x95h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x87\x85\x01Ra\x1Dg\x81Q\x80\x92\x89`)\x88\x01\x91\x01a\x18_V[\x83\x01`\x0F`\xFA\x1B`)\x82\x01Ra\x1D\x86\x82Q\x80\x93\x89`*\x85\x01\x91\x01a\x18_V[\x01\x7F failed, reason: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`*\x82\x01Ra\x1D\xBF\x82Q\x80\x93\x88\x87\x85\x01\x91\x01a\x18_V[\x01\x03`\x1B\x81\x01\x85R\x01\x83a\x18\x1AV[\x81\x81\x11a\x1D\xDAWPPPV[\x7Fb\xBD\xDA\x9A\x05\xCD\xBC\xDB\xF9\x05\xCB\xAD\x99\xC6\xEB\xDC\t\x8Bo\t3\xD8\xF2\xEB<\xFA\xB7@\x0B`%\x14\x93P`;a\x1E\x0Ea\x1CEa\x1C\xCD\x93a \xC3V[`@Q\x94\x85\x91` \x95h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x87\x85\x01Ra\x1E;\x81Q\x80\x92\x89`)\x88\x01\x91\x01a\x18_V[\x83\x01`\x1F`\xF9\x1B`)\x82\x01Ra\x1D\x86\x82Q\x80\x93\x89`*\x85\x01\x91\x01a\x18_V[\x90\x80\x82\x11a\x1EfWP\x90V[`\x01\x81\x01\x80\x91\x11a\x18IW\x80\x15a\x1F3Wa\x1F-a\x1E\xA7\x7F\xA9^n*\x18$\x11\xE7\xA6\xF9\xED\x11J\x85\xC3v\x1D\x87\xF9\xB8\xF4S\xD8B\xC7\x125\xAAd\xFF\xF9\x9F\x92\x84\x06\x93a \xC3V[a\x1F\x19`3a\x1E\xB5\x86a \xC3V[\x92`@Q\x93\x84\x91n\x02\x1Bc\x0Bk\x83Ks9\x03\xB3\x0Bc\xAB)`\x8D\x1B` \x84\x01Ra\x1E\xE8\x81Q\x80\x92` `/\x87\x01\x91\x01a\x18_V[\x82\x01c\x01\x03\xA3y`\xE5\x1B`/\x82\x01Ra\x1F\n\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x18_V[\x01\x03`\x13\x81\x01\x84R\x01\x82a\x18\x1AV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x18\x82V[\x03\x90\xA1\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x80a \xC0Wa\x1F\xD8`@\x91\x7Fk your inputs/assumptions.\0\0\0\0\0\0``\x84Qa\x1F\x82\x81a\x17\xE2V[`Z\x81R\x7FclampGt cannot clamp value a to ` \x82\x01R\x7Fbe larger than uint256.max. Chec\x86\x82\x01R\x01Ra \xC3V[\x90\x80Q\x90`\xA0\x82\x01\x81R`\x80\x82\x01\x92`\0\x84R`\x01\x93\x84\x80[a \xA2W[P\x91a \x90`3a \x81\x93\x85\x7F\xA9^n*\x18$\x11\xE7\xA6\xF9\xED\x11J\x85\xC3v\x1D\x87\xF9\xB8\xF4S\xD8B\xC7\x125\xAAd\xFF\xF9\x9F\x97`\x80a\x1F-\x98`\x1F\x19\x81\x01\x92\x03\x01\x81R\x85Q\x96\x87\x93n\x02\x1Bc\x0Bk\x83Ks9\x03\xB3\x0Bc\xAB)`\x8D\x1B` \x86\x01Ra e\x81Q\x80\x92` `/\x89\x01\x91\x01a\x18_V[\x84\x01\x91c\x01\x03\xA3y`\xE5\x1B`/\x84\x01RQ\x80\x93\x86\x84\x01\x90a\x18_V[\x01\x03`\x13\x81\x01\x85R\x01\x83a\x18\x1AV[Q\x91\x82\x91` \x83R` \x83\x01\x90a\x18\x82V[\x90`\0\x19\x01\x90`\n\x90\x81\x81\x06`0\x01\x83S\x04\x85\x81a\x1F\xF1WPa\x1F\xF6V[\x90V[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a \xD9W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 2\x8A\xE7\x81em\xDE\xA4.R\x95!`\xDA\xE1\xC1\xB5\xC0N\x06\xF3\xE1MXE'\xD1\xEA v\xA91dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static CRYTICERC4626FUNCTIONALACCOUNTING_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CryticERC4626FunctionalAccounting<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CryticERC4626FunctionalAccounting<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CryticERC4626FunctionalAccounting<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CryticERC4626FunctionalAccounting<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CryticERC4626FunctionalAccounting<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CryticERC4626FunctionalAccounting))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CryticERC4626FunctionalAccounting<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CRYTICERC4626FUNCTIONALACCOUNTING_ABI.clone(),
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
                CRYTICERC4626FUNCTIONALACCOUNTING_ABI.clone(),
                CRYTICERC4626FUNCTIONALACCOUNTING_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `verify_depositProperties` (0x2befb0c6) function
        pub fn verify_deposit_properties(
            &self,
            receiver_id: ::ethers::core::types::U256,
            tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 239, 176, 198], (receiver_id, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_mintProperties` (0x2c28d0b7) function
        pub fn verify_mint_properties(
            &self,
            receiver_id: ::ethers::core::types::U256,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 40, 208, 183], (receiver_id, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_redeemProperties` (0xea7831e9) function
        pub fn verify_redeem_properties(
            &self,
            receiver_id: ::ethers::core::types::U256,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 120, 49, 233], (receiver_id, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_withdrawProperties` (0x221803d8) function
        pub fn verify_withdraw_properties(
            &self,
            receiver_id: ::ethers::core::types::U256,
            tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 24, 3, 216], (receiver_id, tokens))
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
            CryticERC4626FunctionalAccountingEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CryticERC4626FunctionalAccounting<M> {
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
    pub enum CryticERC4626FunctionalAccountingEvents {
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
    impl ::ethers::contract::EthLogDecode for CryticERC4626FunctionalAccountingEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssertEqFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626FunctionalAccountingEvents::AssertEqFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626FunctionalAccountingEvents::AssertFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertGtFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626FunctionalAccountingEvents::AssertGtFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertGteFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626FunctionalAccountingEvents::AssertGteFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertLtFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626FunctionalAccountingEvents::AssertLtFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertLteFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626FunctionalAccountingEvents::AssertLteFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertNeqFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626FunctionalAccountingEvents::AssertNeqFailFilter(decoded),
                );
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(
                    CryticERC4626FunctionalAccountingEvents::LogAddressFilter(decoded),
                );
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(
                    CryticERC4626FunctionalAccountingEvents::LogStringFilter(decoded),
                );
            }
            if let Ok(decoded) = LogUint256Filter::decode_log(log) {
                return Ok(
                    CryticERC4626FunctionalAccountingEvents::LogUint256Filter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CryticERC4626FunctionalAccountingEvents {
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
    for CryticERC4626FunctionalAccountingEvents {
        fn from(value: AssertEqFailFilter) -> Self {
            Self::AssertEqFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertFailFilter>
    for CryticERC4626FunctionalAccountingEvents {
        fn from(value: AssertFailFilter) -> Self {
            Self::AssertFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGtFailFilter>
    for CryticERC4626FunctionalAccountingEvents {
        fn from(value: AssertGtFailFilter) -> Self {
            Self::AssertGtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGteFailFilter>
    for CryticERC4626FunctionalAccountingEvents {
        fn from(value: AssertGteFailFilter) -> Self {
            Self::AssertGteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLtFailFilter>
    for CryticERC4626FunctionalAccountingEvents {
        fn from(value: AssertLtFailFilter) -> Self {
            Self::AssertLtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLteFailFilter>
    for CryticERC4626FunctionalAccountingEvents {
        fn from(value: AssertLteFailFilter) -> Self {
            Self::AssertLteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertNeqFailFilter>
    for CryticERC4626FunctionalAccountingEvents {
        fn from(value: AssertNeqFailFilter) -> Self {
            Self::AssertNeqFailFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter>
    for CryticERC4626FunctionalAccountingEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter>
    for CryticERC4626FunctionalAccountingEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUint256Filter>
    for CryticERC4626FunctionalAccountingEvents {
        fn from(value: LogUint256Filter) -> Self {
            Self::LogUint256Filter(value)
        }
    }
    ///Container type for all input parameters for the `verify_depositProperties` function with signature `verify_depositProperties(uint256,uint256)` and selector `0x2befb0c6`
    #[derive(
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
        name = "verify_depositProperties",
        abi = "verify_depositProperties(uint256,uint256)"
    )]
    pub struct VerifyDepositPropertiesCall {
        pub receiver_id: ::ethers::core::types::U256,
        pub tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_mintProperties` function with signature `verify_mintProperties(uint256,uint256)` and selector `0x2c28d0b7`
    #[derive(
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
        name = "verify_mintProperties",
        abi = "verify_mintProperties(uint256,uint256)"
    )]
    pub struct VerifyMintPropertiesCall {
        pub receiver_id: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_redeemProperties` function with signature `verify_redeemProperties(uint256,uint256)` and selector `0xea7831e9`
    #[derive(
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
        name = "verify_redeemProperties",
        abi = "verify_redeemProperties(uint256,uint256)"
    )]
    pub struct VerifyRedeemPropertiesCall {
        pub receiver_id: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_withdrawProperties` function with signature `verify_withdrawProperties(uint256,uint256)` and selector `0x221803d8`
    #[derive(
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
        name = "verify_withdrawProperties",
        abi = "verify_withdrawProperties(uint256,uint256)"
    )]
    pub struct VerifyWithdrawPropertiesCall {
        pub receiver_id: ::ethers::core::types::U256,
        pub tokens: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CryticERC4626FunctionalAccountingCalls {
        VerifyDepositProperties(VerifyDepositPropertiesCall),
        VerifyMintProperties(VerifyMintPropertiesCall),
        VerifyRedeemProperties(VerifyRedeemPropertiesCall),
        VerifyWithdrawProperties(VerifyWithdrawPropertiesCall),
    }
    impl ::ethers::core::abi::AbiDecode for CryticERC4626FunctionalAccountingCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <VerifyDepositPropertiesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyDepositProperties(decoded));
            }
            if let Ok(decoded)
                = <VerifyMintPropertiesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyMintProperties(decoded));
            }
            if let Ok(decoded)
                = <VerifyRedeemPropertiesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyRedeemProperties(decoded));
            }
            if let Ok(decoded)
                = <VerifyWithdrawPropertiesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyWithdrawProperties(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CryticERC4626FunctionalAccountingCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::VerifyDepositProperties(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyMintProperties(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyRedeemProperties(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyWithdrawProperties(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CryticERC4626FunctionalAccountingCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::VerifyDepositProperties(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyMintProperties(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyRedeemProperties(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyWithdrawProperties(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<VerifyDepositPropertiesCall>
    for CryticERC4626FunctionalAccountingCalls {
        fn from(value: VerifyDepositPropertiesCall) -> Self {
            Self::VerifyDepositProperties(value)
        }
    }
    impl ::core::convert::From<VerifyMintPropertiesCall>
    for CryticERC4626FunctionalAccountingCalls {
        fn from(value: VerifyMintPropertiesCall) -> Self {
            Self::VerifyMintProperties(value)
        }
    }
    impl ::core::convert::From<VerifyRedeemPropertiesCall>
    for CryticERC4626FunctionalAccountingCalls {
        fn from(value: VerifyRedeemPropertiesCall) -> Self {
            Self::VerifyRedeemProperties(value)
        }
    }
    impl ::core::convert::From<VerifyWithdrawPropertiesCall>
    for CryticERC4626FunctionalAccountingCalls {
        fn from(value: VerifyWithdrawPropertiesCall) -> Self {
            Self::VerifyWithdrawProperties(value)
        }
    }
}
