pub use crytic_erc4626_sender_independent::*;
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
pub mod crytic_erc4626_sender_independent {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "verify_maxDepositIgnoresSenderAssets",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_maxDepositIgnoresSenderAssets",
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
                    ::std::borrow::ToOwned::to_owned(
                        "verify_maxMintIgnoresSenderAssets",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_maxMintIgnoresSenderAssets",
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
                    ::std::borrow::ToOwned::to_owned(
                        "verify_previewDepositIgnoresSender",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_previewDepositIgnoresSender",
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
                    ::std::borrow::ToOwned::to_owned("verify_previewMintIgnoresSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_previewMintIgnoresSender",
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
                        "verify_previewRedeemIgnoresSender",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_previewRedeemIgnoresSender",
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
                        "verify_previewWithdrawIgnoresSender",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_previewWithdrawIgnoresSender",
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
    pub static CRYTICERC4626SENDERINDEPENDENT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x0E\xFC\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@\x81\x81R`\x04\x806\x10\x15a\0\x15W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x1D6\xAF\x9D\x14a\n\xC3WP\x80c:h\xD4G\x14a\x08BW\x80c\x8C\xBD0\xDA\x14a\x06\xA7W\x80c\x9AM9\x90\x14a\x03\x99W\x80c\xAB\xE7\x80c\x14a\x01\xD0Wc\xD0\x87\xBAl\x14a\0eW`\0\x80\xFD[4a\x01\xCCW\x81`\x03\x196\x01\x12a\x01\xCCW`$5`\x01`\x01`\xA0\x1B\x03\x80`\x01T\x16\x84Q\x93\x84\x91c\xB3\xD7\xF6\xB9`\xE0\x1B\x90\x81\x84R\x85\x83\x85\x01R\x83`$` \x98\x89\x93Z\xFA\x93\x84\x15a\x01\xC2W\x86\x93\x89\x95a\x01\x8EW[P\x90`$\x91a\0\xC5\x8450a\x0C}V[`\x01T\x16\x88Q\x96\x87\x94\x85\x93\x84R\x83\x01RZ\xFA\x91\x82\x15a\x01\x84W\x85\x92a\x01<W[Pn:\x107\xB7\x106\xB9\xB3\x979\xB2\xB722\xB9`\x89\x1B\x84\x7FpreviewMint must not be dependena\x019\x96Q\x95a\x01*\x87a\x0C?V[`/\x87R\x86\x01R\x84\x01Ra\rzV[\x80\xF3[\x93\x91P\x82\x84\x81=\x83\x11a\x01}W[a\x01T\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW\x92Q\x90\x92n:\x107\xB7\x106\xB9\xB3\x979\xB2\xB722\xB9`\x89\x1Ba\0\xE5V[`\0\x80\xFD[P=a\x01JV[\x84Q=\x87\x82>=\x90\xFD[\x84\x81\x93\x95\x92\x96P=\x83\x11a\x01\xBBW[a\x01\xA7\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xWQ\x92\x85\x92\x90`$a\0\xB5V[P=a\x01\x9DV[\x87Q=\x8A\x82>=\x90\xFD[\x82\x80\xFD[P4a\x01\xCCW` \x90\x81`\x03\x196\x01\x12a\x03\x95W`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16\x91\x84Q\x91\x84\x83`$\x81c@-&}`\xE0\x1B\x97\x88\x82R0\x86\x83\x01RZ\xFA\x92\x83\x15a\x03\x8BW\x87\x93a\x03\\W[P\x81\x87T\x16\x91\x82;\x15a\x03XW\x86Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x83\x82\x01\x90\x81R\x835` \x82\x01R\x90\x93\x89\x91\x82\x91\x86\x91\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x93\x84\x15a\x03LW\x87\x94a\x03/W[PP`$\x90`\x01T\x16\x94\x87Q\x95\x86\x93\x84\x92\x83R0\x90\x83\x01RZ\xFA\x91\x82\x15a\x01\x84W\x85\x92a\x02\xE7W[Ps has infinite assets``\x1B\x84\x7FmaxDeposit must assume the agenta\x019\x96Q\x95a\x02\xD8\x87a\x0C?V[`4\x87R\x86\x01R\x84\x01Ra\rzV[\x93\x91P\x82\x84\x81=\x83\x11a\x03(W[a\x02\xFF\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW\x92Q\x90\x92s has infinite assets``\x1Ba\x02\x8EV[P=a\x02\xF5V[a\x03;\x91\x92\x94Pa\x0C\x15V[a\x03HW\x84\x91\x878a\x02fV[\x86\x80\xFD[P\x87Q\x90=\x90\x82>=\x90\xFD[\x87\x80\xFD[\x90\x92P\x84\x81\x81=\x83\x11a\x03\x84W[a\x03t\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xWQ\x918a\x02\x1CV[P=a\x03jV[\x86Q=\x89\x82>=\x90\xFD[\x83\x80\xFD[P4a\x01\xCCW` \x91\x82`\x03\x196\x01\x12a\x03\x95W\x815\x91`\x01`\x01`\xA0\x1B\x03\x80`\x01T\x16\x90\x83Q\x90\x87c&mj\x83`\xE1\x1B\x93\x84\x84R\x87\x86\x85\x01R\x88\x84`$\x81\x84Z\xFA\x93\x84\x15a\x06iW\x90\x89\x91\x83\x95a\x06sW[P\x90`$\x91\x88Q\x92\x83\x80\x92c\xB3\xD7\xF6\xB9`\xE0\x1B\x82R\x8C\x8B\x83\x01RZ\xFA\x90\x81\x15a\x06iW\x90\x89\x91\x83\x91a\x066W[P\x80a\x04(a\x04T\x920a\x0C}V[`\x01T\x89QcnU?e`\xE0\x1B\x81R\x89\x81\x01\x92\x83R0` \x84\x01R\x94\x85\x93\x91\x87\x16\x92\x84\x92\x83\x91`@\x01\x90V[\x03\x92Z\xF1\x80\x15a\x06,W\x90\x88\x91a\x06\x03W[PP`\x01T\x16\x94\x84Q\x92\x83R\x83\x83\x01R\x85\x82`$\x81\x88Z\xFA\x80\x15a\x05\xF9W\x87\x90a\x05\xCAW[a\x04\xEF\x92P\x84Q\x91a\x04\x9C\x83a\x0C?V[`1\x83R\x7FpreviewRedeem must not be depend\x88\x84\x01R\x7Fent on msg.sender\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x84\x01Ra\rzV[\x81Qcp\xA0\x821`\xE0\x1B\x81R0\x82\x82\x01R\x90\x84\x82`$\x81\x87Z\xFA\x91\x82\x15a\x05\xC0W\x90\x85\x92\x91\x87\x92a\x05\x8DW[P\x83Qc]\x04;)`\xE1\x1B\x81R\x90\x81\x01\x91\x82R0` \x83\x01\x81\x90R`@\x83\x01R\x93\x84\x91\x82\x90\x88\x90\x82\x90``\x01[\x03\x92Z\xF1\x90\x81\x15a\x05\x84WPa\x05\\W\x82\x80\xF3[\x81=\x83\x11a\x05}W[a\x05o\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW8\x80\x82\x80\xF3[P=a\x05eV[Q=\x85\x82>=\x90\xFD[\x83\x81\x94\x92\x93P=\x83\x11a\x05\xB9W[a\x05\xA5\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW\x90Q\x84\x91a\x05Ha\x05\x1BV[P=a\x05\x9BV[\x83Q=\x88\x82>=\x90\xFD[P\x85\x82\x81=\x83\x11a\x05\xF2W[a\x05\xE0\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xWa\x04\xEF\x91Qa\x04\x8BV[P=a\x05\xD6V[\x84Q=\x89\x82>=\x90\xFD[\x81=\x83\x11a\x06%W[a\x06\x16\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW\x868a\x04fV[P=a\x06\x0CV[\x86Q=\x8B\x82>=\x90\xFD[\x92PP\x81\x81=\x83\x11a\x06bW[a\x06M\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW\x87a\x04T\x8A\x92Q\x90a\x04\x19V[P=a\x06CV[\x87Q=\x84\x82>=\x90\xFD[\x92P\x93P\x81\x81=\x83\x11a\x06\xA0W[a\x06\x8B\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xWQ\x91\x88\x90\x88\x90`$a\x03\xECV[P=a\x06\x81V[P4a\x01\xCCW` \x90\x81`\x03\x196\x01\x12a\x03\x95W`\x01`\x01`\xA0\x1B\x03\x80`\x01T\x16\x91\x84Q\x91\x84\x83`$\x81cc\x1E\xBA\xDB`\xE1\x1B\x97\x88\x82R0\x87\x83\x01RZ\xFA\x92\x83\x15a\x03\x8BW\x87\x93a\x08\x13W[P\x80\x87T\x16\x80;\x15a\x03XW\x86Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x84\x82\x01\x90\x81R\x845` \x82\x01R\x90\x91\x89\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x01\xC2Wa\x07\xFBW[P\x90`$\x85\x92`\x01T\x16\x94\x87Q\x95\x86\x93\x84\x92\x83R0\x90\x83\x01RZ\xFA\x91\x82\x15a\x01\x84W\x85\x92a\x07\xB6W[Pps infinite assets`x\x1B\x84\x7FmaxMint must assume the agent haa\x019\x96Q\x95a\x07\xA7\x87a\x0C?V[`1\x87R\x86\x01R\x84\x01Ra\rzV[\x93\x91P\x82\x84\x81=\x83\x11a\x07\xF4W[a\x07\xCE\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW\x92Q\x90\x92ps infinite assets`x\x1Ba\x07`V[P=a\x07\xC4V[\x85\x92\x91\x97a\x08\n`$\x92a\x0C\x15V[\x97\x91\x92Pa\x077V[\x90\x92P\x84\x81\x81=\x83\x11a\x08;W[a\x08+\x81\x83a\x0C[V[\x81\x01\x03\x12a\x03HWQ\x918a\x06\xF2V[P=a\x08!V[P4a\x01\xCCW` \x91\x82`\x03\x196\x01\x12a\x03\x95W\x815\x91`\x01`\x01`\xA0\x1B\x03\x80`\x01T\x16\x90\x83Q\x90\x86\x82`$\x81c\n(\xA4w`\xE0\x1B\x96\x87\x82R\x8A\x89\x83\x01RZ\xFA\x91\x82\x15a\n\xB9W\x88\x92a\n\x8AW[Pa\x08\x9B\x860a\x0C}V[`\x01T\x85QcnU?e`\xE0\x1B\x81R\x85\x81\x01\x88\x81R0` \x82\x01R\x90\x91\x89\x91\x83\x91\x85\x16\x90\x82\x90\x8D\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x06,W\x90\x88\x91a\naW[PP`\x01T\x16\x94\x84Q\x92\x83R\x83\x83\x01R\x85\x82`$\x81\x88Z\xFA\x80\x15a\x05\xF9W\x87\x90a\n2W[a\tc\x92P\x84Q\x91a\t\x10\x83a\x0C?V[`3\x83R\x7FpreviewWithdraw must not be depe\x88\x84\x01R\x7Fndent on msg.sender\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x84\x01Ra\rzV[\x81Qcp\xA0\x821`\xE0\x1B\x81R0\x82\x82\x01R\x90\x84\x82`$\x81\x87Z\xFA\x91\x82\x15a\x05\xC0W\x90\x85\x92\x91\x87\x92a\t\xFBW[P\x83Qc]\x04;)`\xE1\x1B\x81R\x90\x81\x01\x91\x82R0` \x83\x01\x81\x90R`@\x83\x01R\x93\x84\x91\x82\x90\x88\x90\x82\x90``\x01[\x03\x92Z\xF1\x90\x81\x15a\x05\x84WPa\t\xD0W\x82\x80\xF3[\x81=\x83\x11a\t\xF4W[a\t\xE3\x81\x83a\x0C[V[\x81\x01\x03\x12a\t\xF1W8\x80\x82\x80\xF3[\x80\xFD[P=a\t\xD9V[\x83\x81\x94\x92\x93P=\x83\x11a\n+W[a\n\x13\x81\x83a\x0C[V[\x81\x01\x03\x12a\n'W\x90Q\x84\x91a\t\xBCa\t\x8FV[\x85\x80\xFD[P=a\n\tV[P\x85\x82\x81=\x83\x11a\nZW[a\nH\x81\x83a\x0C[V[\x81\x01\x03\x12a\x03HWa\tc\x91Qa\x08\xFFV[P=a\n>V[\x81=\x83\x11a\n\x83W[a\nt\x81\x83a\x0C[V[\x81\x01\x03\x12a\x03XW\x868a\x08\xDAV[P=a\njV[\x90\x91P\x86\x81\x81=\x83\x11a\n\xB2W[a\n\xA2\x81\x83a\x0C[V[\x81\x01\x03\x12a\x03XWQ\x908a\x08\x90V[P=a\n\x98V[\x85Q=\x8A\x82>=\x90\xFD[\x90P4a\x03\x95W` \x91\x82`\x03\x196\x01\x12a\x0B\xD6W\x805\x91`\x01`\x01`\xA0\x1B\x03\x91\x82`\x01T\x16\x85\x83`$\x81c\xEF\x8B0\xF7`\xE0\x1B\x94\x85\x82R\x89\x87\x83\x01RZ\xFA\x93\x84\x15a\x01\xC2W\x86\x93\x89\x95a\x0B\xE1W[P\x90`$\x91a\x0B \x870a\x0C}V[`\x01T\x16\x88Q\x96\x87\x94\x85\x93\x84R\x83\x01RZ\xFA\x91\x82\x15a\x01\x84W\x85\x92a\x0B\x97W[Pq22\xB7:\x107\xB7\x106\xB9\xB3\x979\xB2\xB722\xB9`q\x1B\x84\x7FpreviewDeposit must not be depena\x019\x96Q\x95a\x0B\x88\x87a\x0C?V[`2\x87R\x86\x01R\x84\x01Ra\rzV[\x93\x91P\x82\x84\x81=\x83\x11a\x0B\xDAW[a\x0B\xAF\x81\x83a\x0C[V[\x81\x01\x03\x12a\x0B\xD6W\x92Q\x90\x92q22\xB7:\x107\xB7\x106\xB9\xB3\x979\xB2\xB722\xB9`q\x1Ba\x0B@V[\x84\x80\xFD[P=a\x0B\xA5V[\x84\x81\x93\x95\x92\x96P=\x83\x11a\x0C\x0EW[a\x0B\xFA\x81\x83a\x0C[V[\x81\x01\x03\x12a\x03XWQ\x92\x85\x92\x90`$a\x0B\x11V[P=a\x0B\xF0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0C)W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C)W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C)W`@RV[\x90`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x84T\x16\x80;\x15a\x0B\xD6W`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x90\x85\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\rLWa\r9W[P\x81\x84T\x16\x82`\x01T\x16\x93\x81;\x15a\n'W\x91\x85`d\x92\x81\x95\x94`@Q\x97\x88\x96\x87\x95cQ\xBB\x10\xCF`\xE0\x1B\x87R\x16`\x04\x86\x01R`$\x85\x01R`D\x84\x01RZ\xF1\x80\x15a\r.Wa\r\x1DWPPV[a\r'\x82\x91a\x0C\x15V[a\t\xF1WPV[`@Q=\x84\x82>=\x90\xFD[a\rE\x90\x94\x91\x94a\x0C\x15V[\x928a\x0C\xD1V[`@Q=\x87\x82>=\x90\xFD[`\0[\x83\x81\x10a\rjWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\rZV[\x91\x90\x91\x82\x81\x03a\r\x89WPPPV[\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x92\x91a\r\xC0a\r\xBA`@\x93a\x0E\x87V[\x93a\x0E\x87V[\x92a\x0EH`5\x84Q\x80\x94` \x97\x88\x83\x01\x95h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x87Ra\r\xF3\x81Q\x80\x92\x8C`)\x88\x01\x91\x01a\rWV[\x83\x01a!=`\xF0\x1B`)\x82\x01Ra\x0E\x13\x82Q\x80\x93\x8C`+\x85\x01\x91\x01a\rWV[\x01i\x01a\x03\x93+\x0B\x9B{q\xD1`\xB5\x1B`+\x82\x01Ra\x0E9\x82Q\x80\x93\x8B\x87\x85\x01\x91\x01a\rWV[\x01\x03`\x15\x81\x01\x85R\x01\x83a\x0C[V[a\x0Ed\x83Q\x94\x85\x93\x81\x85RQ\x92\x83\x80\x92\x86\x01R\x85\x85\x01\x90a\rWV[`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xA1cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\x0E\x9DW\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \xCB.\x19\x03\xF8\xE8\x92\xBF\xFD8_\x13\xED \x8E\x84\xDD\xC3\x028\xCA&E\xF9\x84\xD4\xB76^\xED\xB2pdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static CRYTICERC4626SENDERINDEPENDENT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x806\x10\x15a\0\x15W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x1D6\xAF\x9D\x14a\n\xC3WP\x80c:h\xD4G\x14a\x08BW\x80c\x8C\xBD0\xDA\x14a\x06\xA7W\x80c\x9AM9\x90\x14a\x03\x99W\x80c\xAB\xE7\x80c\x14a\x01\xD0Wc\xD0\x87\xBAl\x14a\0eW`\0\x80\xFD[4a\x01\xCCW\x81`\x03\x196\x01\x12a\x01\xCCW`$5`\x01`\x01`\xA0\x1B\x03\x80`\x01T\x16\x84Q\x93\x84\x91c\xB3\xD7\xF6\xB9`\xE0\x1B\x90\x81\x84R\x85\x83\x85\x01R\x83`$` \x98\x89\x93Z\xFA\x93\x84\x15a\x01\xC2W\x86\x93\x89\x95a\x01\x8EW[P\x90`$\x91a\0\xC5\x8450a\x0C}V[`\x01T\x16\x88Q\x96\x87\x94\x85\x93\x84R\x83\x01RZ\xFA\x91\x82\x15a\x01\x84W\x85\x92a\x01<W[Pn:\x107\xB7\x106\xB9\xB3\x979\xB2\xB722\xB9`\x89\x1B\x84\x7FpreviewMint must not be dependena\x019\x96Q\x95a\x01*\x87a\x0C?V[`/\x87R\x86\x01R\x84\x01Ra\rzV[\x80\xF3[\x93\x91P\x82\x84\x81=\x83\x11a\x01}W[a\x01T\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW\x92Q\x90\x92n:\x107\xB7\x106\xB9\xB3\x979\xB2\xB722\xB9`\x89\x1Ba\0\xE5V[`\0\x80\xFD[P=a\x01JV[\x84Q=\x87\x82>=\x90\xFD[\x84\x81\x93\x95\x92\x96P=\x83\x11a\x01\xBBW[a\x01\xA7\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xWQ\x92\x85\x92\x90`$a\0\xB5V[P=a\x01\x9DV[\x87Q=\x8A\x82>=\x90\xFD[\x82\x80\xFD[P4a\x01\xCCW` \x90\x81`\x03\x196\x01\x12a\x03\x95W`\x01`\x01`\xA0\x1B\x03\x90\x81`\x01T\x16\x91\x84Q\x91\x84\x83`$\x81c@-&}`\xE0\x1B\x97\x88\x82R0\x86\x83\x01RZ\xFA\x92\x83\x15a\x03\x8BW\x87\x93a\x03\\W[P\x81\x87T\x16\x91\x82;\x15a\x03XW\x86Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x83\x82\x01\x90\x81R\x835` \x82\x01R\x90\x93\x89\x91\x82\x91\x86\x91\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x93\x84\x15a\x03LW\x87\x94a\x03/W[PP`$\x90`\x01T\x16\x94\x87Q\x95\x86\x93\x84\x92\x83R0\x90\x83\x01RZ\xFA\x91\x82\x15a\x01\x84W\x85\x92a\x02\xE7W[Ps has infinite assets``\x1B\x84\x7FmaxDeposit must assume the agenta\x019\x96Q\x95a\x02\xD8\x87a\x0C?V[`4\x87R\x86\x01R\x84\x01Ra\rzV[\x93\x91P\x82\x84\x81=\x83\x11a\x03(W[a\x02\xFF\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW\x92Q\x90\x92s has infinite assets``\x1Ba\x02\x8EV[P=a\x02\xF5V[a\x03;\x91\x92\x94Pa\x0C\x15V[a\x03HW\x84\x91\x878a\x02fV[\x86\x80\xFD[P\x87Q\x90=\x90\x82>=\x90\xFD[\x87\x80\xFD[\x90\x92P\x84\x81\x81=\x83\x11a\x03\x84W[a\x03t\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xWQ\x918a\x02\x1CV[P=a\x03jV[\x86Q=\x89\x82>=\x90\xFD[\x83\x80\xFD[P4a\x01\xCCW` \x91\x82`\x03\x196\x01\x12a\x03\x95W\x815\x91`\x01`\x01`\xA0\x1B\x03\x80`\x01T\x16\x90\x83Q\x90\x87c&mj\x83`\xE1\x1B\x93\x84\x84R\x87\x86\x85\x01R\x88\x84`$\x81\x84Z\xFA\x93\x84\x15a\x06iW\x90\x89\x91\x83\x95a\x06sW[P\x90`$\x91\x88Q\x92\x83\x80\x92c\xB3\xD7\xF6\xB9`\xE0\x1B\x82R\x8C\x8B\x83\x01RZ\xFA\x90\x81\x15a\x06iW\x90\x89\x91\x83\x91a\x066W[P\x80a\x04(a\x04T\x920a\x0C}V[`\x01T\x89QcnU?e`\xE0\x1B\x81R\x89\x81\x01\x92\x83R0` \x84\x01R\x94\x85\x93\x91\x87\x16\x92\x84\x92\x83\x91`@\x01\x90V[\x03\x92Z\xF1\x80\x15a\x06,W\x90\x88\x91a\x06\x03W[PP`\x01T\x16\x94\x84Q\x92\x83R\x83\x83\x01R\x85\x82`$\x81\x88Z\xFA\x80\x15a\x05\xF9W\x87\x90a\x05\xCAW[a\x04\xEF\x92P\x84Q\x91a\x04\x9C\x83a\x0C?V[`1\x83R\x7FpreviewRedeem must not be depend\x88\x84\x01R\x7Fent on msg.sender\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x84\x01Ra\rzV[\x81Qcp\xA0\x821`\xE0\x1B\x81R0\x82\x82\x01R\x90\x84\x82`$\x81\x87Z\xFA\x91\x82\x15a\x05\xC0W\x90\x85\x92\x91\x87\x92a\x05\x8DW[P\x83Qc]\x04;)`\xE1\x1B\x81R\x90\x81\x01\x91\x82R0` \x83\x01\x81\x90R`@\x83\x01R\x93\x84\x91\x82\x90\x88\x90\x82\x90``\x01[\x03\x92Z\xF1\x90\x81\x15a\x05\x84WPa\x05\\W\x82\x80\xF3[\x81=\x83\x11a\x05}W[a\x05o\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW8\x80\x82\x80\xF3[P=a\x05eV[Q=\x85\x82>=\x90\xFD[\x83\x81\x94\x92\x93P=\x83\x11a\x05\xB9W[a\x05\xA5\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW\x90Q\x84\x91a\x05Ha\x05\x1BV[P=a\x05\x9BV[\x83Q=\x88\x82>=\x90\xFD[P\x85\x82\x81=\x83\x11a\x05\xF2W[a\x05\xE0\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xWa\x04\xEF\x91Qa\x04\x8BV[P=a\x05\xD6V[\x84Q=\x89\x82>=\x90\xFD[\x81=\x83\x11a\x06%W[a\x06\x16\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW\x868a\x04fV[P=a\x06\x0CV[\x86Q=\x8B\x82>=\x90\xFD[\x92PP\x81\x81=\x83\x11a\x06bW[a\x06M\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW\x87a\x04T\x8A\x92Q\x90a\x04\x19V[P=a\x06CV[\x87Q=\x84\x82>=\x90\xFD[\x92P\x93P\x81\x81=\x83\x11a\x06\xA0W[a\x06\x8B\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xWQ\x91\x88\x90\x88\x90`$a\x03\xECV[P=a\x06\x81V[P4a\x01\xCCW` \x90\x81`\x03\x196\x01\x12a\x03\x95W`\x01`\x01`\xA0\x1B\x03\x80`\x01T\x16\x91\x84Q\x91\x84\x83`$\x81cc\x1E\xBA\xDB`\xE1\x1B\x97\x88\x82R0\x87\x83\x01RZ\xFA\x92\x83\x15a\x03\x8BW\x87\x93a\x08\x13W[P\x80\x87T\x16\x80;\x15a\x03XW\x86Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x84\x82\x01\x90\x81R\x845` \x82\x01R\x90\x91\x89\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x01\xC2Wa\x07\xFBW[P\x90`$\x85\x92`\x01T\x16\x94\x87Q\x95\x86\x93\x84\x92\x83R0\x90\x83\x01RZ\xFA\x91\x82\x15a\x01\x84W\x85\x92a\x07\xB6W[Pps infinite assets`x\x1B\x84\x7FmaxMint must assume the agent haa\x019\x96Q\x95a\x07\xA7\x87a\x0C?V[`1\x87R\x86\x01R\x84\x01Ra\rzV[\x93\x91P\x82\x84\x81=\x83\x11a\x07\xF4W[a\x07\xCE\x81\x83a\x0C[V[\x81\x01\x03\x12a\x01xW\x92Q\x90\x92ps infinite assets`x\x1Ba\x07`V[P=a\x07\xC4V[\x85\x92\x91\x97a\x08\n`$\x92a\x0C\x15V[\x97\x91\x92Pa\x077V[\x90\x92P\x84\x81\x81=\x83\x11a\x08;W[a\x08+\x81\x83a\x0C[V[\x81\x01\x03\x12a\x03HWQ\x918a\x06\xF2V[P=a\x08!V[P4a\x01\xCCW` \x91\x82`\x03\x196\x01\x12a\x03\x95W\x815\x91`\x01`\x01`\xA0\x1B\x03\x80`\x01T\x16\x90\x83Q\x90\x86\x82`$\x81c\n(\xA4w`\xE0\x1B\x96\x87\x82R\x8A\x89\x83\x01RZ\xFA\x91\x82\x15a\n\xB9W\x88\x92a\n\x8AW[Pa\x08\x9B\x860a\x0C}V[`\x01T\x85QcnU?e`\xE0\x1B\x81R\x85\x81\x01\x88\x81R0` \x82\x01R\x90\x91\x89\x91\x83\x91\x85\x16\x90\x82\x90\x8D\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x06,W\x90\x88\x91a\naW[PP`\x01T\x16\x94\x84Q\x92\x83R\x83\x83\x01R\x85\x82`$\x81\x88Z\xFA\x80\x15a\x05\xF9W\x87\x90a\n2W[a\tc\x92P\x84Q\x91a\t\x10\x83a\x0C?V[`3\x83R\x7FpreviewWithdraw must not be depe\x88\x84\x01R\x7Fndent on msg.sender\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x84\x01Ra\rzV[\x81Qcp\xA0\x821`\xE0\x1B\x81R0\x82\x82\x01R\x90\x84\x82`$\x81\x87Z\xFA\x91\x82\x15a\x05\xC0W\x90\x85\x92\x91\x87\x92a\t\xFBW[P\x83Qc]\x04;)`\xE1\x1B\x81R\x90\x81\x01\x91\x82R0` \x83\x01\x81\x90R`@\x83\x01R\x93\x84\x91\x82\x90\x88\x90\x82\x90``\x01[\x03\x92Z\xF1\x90\x81\x15a\x05\x84WPa\t\xD0W\x82\x80\xF3[\x81=\x83\x11a\t\xF4W[a\t\xE3\x81\x83a\x0C[V[\x81\x01\x03\x12a\t\xF1W8\x80\x82\x80\xF3[\x80\xFD[P=a\t\xD9V[\x83\x81\x94\x92\x93P=\x83\x11a\n+W[a\n\x13\x81\x83a\x0C[V[\x81\x01\x03\x12a\n'W\x90Q\x84\x91a\t\xBCa\t\x8FV[\x85\x80\xFD[P=a\n\tV[P\x85\x82\x81=\x83\x11a\nZW[a\nH\x81\x83a\x0C[V[\x81\x01\x03\x12a\x03HWa\tc\x91Qa\x08\xFFV[P=a\n>V[\x81=\x83\x11a\n\x83W[a\nt\x81\x83a\x0C[V[\x81\x01\x03\x12a\x03XW\x868a\x08\xDAV[P=a\njV[\x90\x91P\x86\x81\x81=\x83\x11a\n\xB2W[a\n\xA2\x81\x83a\x0C[V[\x81\x01\x03\x12a\x03XWQ\x908a\x08\x90V[P=a\n\x98V[\x85Q=\x8A\x82>=\x90\xFD[\x90P4a\x03\x95W` \x91\x82`\x03\x196\x01\x12a\x0B\xD6W\x805\x91`\x01`\x01`\xA0\x1B\x03\x91\x82`\x01T\x16\x85\x83`$\x81c\xEF\x8B0\xF7`\xE0\x1B\x94\x85\x82R\x89\x87\x83\x01RZ\xFA\x93\x84\x15a\x01\xC2W\x86\x93\x89\x95a\x0B\xE1W[P\x90`$\x91a\x0B \x870a\x0C}V[`\x01T\x16\x88Q\x96\x87\x94\x85\x93\x84R\x83\x01RZ\xFA\x91\x82\x15a\x01\x84W\x85\x92a\x0B\x97W[Pq22\xB7:\x107\xB7\x106\xB9\xB3\x979\xB2\xB722\xB9`q\x1B\x84\x7FpreviewDeposit must not be depena\x019\x96Q\x95a\x0B\x88\x87a\x0C?V[`2\x87R\x86\x01R\x84\x01Ra\rzV[\x93\x91P\x82\x84\x81=\x83\x11a\x0B\xDAW[a\x0B\xAF\x81\x83a\x0C[V[\x81\x01\x03\x12a\x0B\xD6W\x92Q\x90\x92q22\xB7:\x107\xB7\x106\xB9\xB3\x979\xB2\xB722\xB9`q\x1Ba\x0B@V[\x84\x80\xFD[P=a\x0B\xA5V[\x84\x81\x93\x95\x92\x96P=\x83\x11a\x0C\x0EW[a\x0B\xFA\x81\x83a\x0C[V[\x81\x01\x03\x12a\x03XWQ\x92\x85\x92\x90`$a\x0B\x11V[P=a\x0B\xF0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0C)W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C)W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C)W`@RV[\x90`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x81\x84T\x16\x80;\x15a\x0B\xD6W`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x90\x85\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\rLWa\r9W[P\x81\x84T\x16\x82`\x01T\x16\x93\x81;\x15a\n'W\x91\x85`d\x92\x81\x95\x94`@Q\x97\x88\x96\x87\x95cQ\xBB\x10\xCF`\xE0\x1B\x87R\x16`\x04\x86\x01R`$\x85\x01R`D\x84\x01RZ\xF1\x80\x15a\r.Wa\r\x1DWPPV[a\r'\x82\x91a\x0C\x15V[a\t\xF1WPV[`@Q=\x84\x82>=\x90\xFD[a\rE\x90\x94\x91\x94a\x0C\x15V[\x928a\x0C\xD1V[`@Q=\x87\x82>=\x90\xFD[`\0[\x83\x81\x10a\rjWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\rZV[\x91\x90\x91\x82\x81\x03a\r\x89WPPPV[\x7F--8\xC9\xA3M\xF9\x88zm\xCB*T\xC1\xF7\x9F\xF8\xBF\x9CML\xAF\xAC\xD7\xD1\xF7'\x7FW\xBA\xABo\x92\x91a\r\xC0a\r\xBA`@\x93a\x0E\x87V[\x93a\x0E\x87V[\x92a\x0EH`5\x84Q\x80\x94` \x97\x88\x83\x01\x95h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x87Ra\r\xF3\x81Q\x80\x92\x8C`)\x88\x01\x91\x01a\rWV[\x83\x01a!=`\xF0\x1B`)\x82\x01Ra\x0E\x13\x82Q\x80\x93\x8C`+\x85\x01\x91\x01a\rWV[\x01i\x01a\x03\x93+\x0B\x9B{q\xD1`\xB5\x1B`+\x82\x01Ra\x0E9\x82Q\x80\x93\x8B\x87\x85\x01\x91\x01a\rWV[\x01\x03`\x15\x81\x01\x85R\x01\x83a\x0C[V[a\x0Ed\x83Q\x94\x85\x93\x81\x85RQ\x92\x83\x80\x92\x86\x01R\x85\x85\x01\x90a\rWV[`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xA1cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\x0E\x9DW\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \xCB.\x19\x03\xF8\xE8\x92\xBF\xFD8_\x13\xED \x8E\x84\xDD\xC3\x028\xCA&E\xF9\x84\xD4\xB76^\xED\xB2pdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static CRYTICERC4626SENDERINDEPENDENT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CryticERC4626SenderIndependent<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CryticERC4626SenderIndependent<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CryticERC4626SenderIndependent<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CryticERC4626SenderIndependent<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CryticERC4626SenderIndependent<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CryticERC4626SenderIndependent))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CryticERC4626SenderIndependent<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CRYTICERC4626SENDERINDEPENDENT_ABI.clone(),
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
                CRYTICERC4626SENDERINDEPENDENT_ABI.clone(),
                CRYTICERC4626SENDERINDEPENDENT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `verify_maxDepositIgnoresSenderAssets` (0xabe78063) function
        pub fn verify_max_deposit_ignores_sender_assets(
            &self,
            tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 231, 128, 99], tokens)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_maxMintIgnoresSenderAssets` (0x8cbd30da) function
        pub fn verify_max_mint_ignores_sender_assets(
            &self,
            tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 189, 48, 218], tokens)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_previewDepositIgnoresSender` (0x1d36af9d) function
        pub fn verify_preview_deposit_ignores_sender(
            &self,
            tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 54, 175, 157], tokens)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_previewMintIgnoresSender` (0xd087ba6c) function
        pub fn verify_preview_mint_ignores_sender(
            &self,
            tokens: ::ethers::core::types::U256,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 135, 186, 108], (tokens, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_previewRedeemIgnoresSender` (0x9a4d3990) function
        pub fn verify_preview_redeem_ignores_sender(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 77, 57, 144], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_previewWithdrawIgnoresSender` (0x3a68d447) function
        pub fn verify_preview_withdraw_ignores_sender(
            &self,
            tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([58, 104, 212, 71], tokens)
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
            CryticERC4626SenderIndependentEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CryticERC4626SenderIndependent<M> {
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
    pub enum CryticERC4626SenderIndependentEvents {
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
    impl ::ethers::contract::EthLogDecode for CryticERC4626SenderIndependentEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssertEqFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626SenderIndependentEvents::AssertEqFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626SenderIndependentEvents::AssertFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertGtFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626SenderIndependentEvents::AssertGtFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertGteFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626SenderIndependentEvents::AssertGteFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertLtFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626SenderIndependentEvents::AssertLtFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertLteFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626SenderIndependentEvents::AssertLteFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertNeqFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626SenderIndependentEvents::AssertNeqFailFilter(decoded),
                );
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(
                    CryticERC4626SenderIndependentEvents::LogAddressFilter(decoded),
                );
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(
                    CryticERC4626SenderIndependentEvents::LogStringFilter(decoded),
                );
            }
            if let Ok(decoded) = LogUint256Filter::decode_log(log) {
                return Ok(
                    CryticERC4626SenderIndependentEvents::LogUint256Filter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CryticERC4626SenderIndependentEvents {
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
    for CryticERC4626SenderIndependentEvents {
        fn from(value: AssertEqFailFilter) -> Self {
            Self::AssertEqFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertFailFilter>
    for CryticERC4626SenderIndependentEvents {
        fn from(value: AssertFailFilter) -> Self {
            Self::AssertFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGtFailFilter>
    for CryticERC4626SenderIndependentEvents {
        fn from(value: AssertGtFailFilter) -> Self {
            Self::AssertGtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGteFailFilter>
    for CryticERC4626SenderIndependentEvents {
        fn from(value: AssertGteFailFilter) -> Self {
            Self::AssertGteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLtFailFilter>
    for CryticERC4626SenderIndependentEvents {
        fn from(value: AssertLtFailFilter) -> Self {
            Self::AssertLtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLteFailFilter>
    for CryticERC4626SenderIndependentEvents {
        fn from(value: AssertLteFailFilter) -> Self {
            Self::AssertLteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertNeqFailFilter>
    for CryticERC4626SenderIndependentEvents {
        fn from(value: AssertNeqFailFilter) -> Self {
            Self::AssertNeqFailFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter>
    for CryticERC4626SenderIndependentEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter>
    for CryticERC4626SenderIndependentEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUint256Filter>
    for CryticERC4626SenderIndependentEvents {
        fn from(value: LogUint256Filter) -> Self {
            Self::LogUint256Filter(value)
        }
    }
    ///Container type for all input parameters for the `verify_maxDepositIgnoresSenderAssets` function with signature `verify_maxDepositIgnoresSenderAssets(uint256)` and selector `0xabe78063`
    #[derive(
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
        name = "verify_maxDepositIgnoresSenderAssets",
        abi = "verify_maxDepositIgnoresSenderAssets(uint256)"
    )]
    pub struct VerifyMaxDepositIgnoresSenderAssetsCall {
        pub tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_maxMintIgnoresSenderAssets` function with signature `verify_maxMintIgnoresSenderAssets(uint256)` and selector `0x8cbd30da`
    #[derive(
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
        name = "verify_maxMintIgnoresSenderAssets",
        abi = "verify_maxMintIgnoresSenderAssets(uint256)"
    )]
    pub struct VerifyMaxMintIgnoresSenderAssetsCall {
        pub tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_previewDepositIgnoresSender` function with signature `verify_previewDepositIgnoresSender(uint256)` and selector `0x1d36af9d`
    #[derive(
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
        name = "verify_previewDepositIgnoresSender",
        abi = "verify_previewDepositIgnoresSender(uint256)"
    )]
    pub struct VerifyPreviewDepositIgnoresSenderCall {
        pub tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_previewMintIgnoresSender` function with signature `verify_previewMintIgnoresSender(uint256,uint256)` and selector `0xd087ba6c`
    #[derive(
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
        name = "verify_previewMintIgnoresSender",
        abi = "verify_previewMintIgnoresSender(uint256,uint256)"
    )]
    pub struct VerifyPreviewMintIgnoresSenderCall {
        pub tokens: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_previewRedeemIgnoresSender` function with signature `verify_previewRedeemIgnoresSender(uint256)` and selector `0x9a4d3990`
    #[derive(
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
        name = "verify_previewRedeemIgnoresSender",
        abi = "verify_previewRedeemIgnoresSender(uint256)"
    )]
    pub struct VerifyPreviewRedeemIgnoresSenderCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `verify_previewWithdrawIgnoresSender` function with signature `verify_previewWithdrawIgnoresSender(uint256)` and selector `0x3a68d447`
    #[derive(
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
        name = "verify_previewWithdrawIgnoresSender",
        abi = "verify_previewWithdrawIgnoresSender(uint256)"
    )]
    pub struct VerifyPreviewWithdrawIgnoresSenderCall {
        pub tokens: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CryticERC4626SenderIndependentCalls {
        VerifyMaxDepositIgnoresSenderAssets(VerifyMaxDepositIgnoresSenderAssetsCall),
        VerifyMaxMintIgnoresSenderAssets(VerifyMaxMintIgnoresSenderAssetsCall),
        VerifyPreviewDepositIgnoresSender(VerifyPreviewDepositIgnoresSenderCall),
        VerifyPreviewMintIgnoresSender(VerifyPreviewMintIgnoresSenderCall),
        VerifyPreviewRedeemIgnoresSender(VerifyPreviewRedeemIgnoresSenderCall),
        VerifyPreviewWithdrawIgnoresSender(VerifyPreviewWithdrawIgnoresSenderCall),
    }
    impl ::ethers::core::abi::AbiDecode for CryticERC4626SenderIndependentCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <VerifyMaxDepositIgnoresSenderAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyMaxDepositIgnoresSenderAssets(decoded));
            }
            if let Ok(decoded)
                = <VerifyMaxMintIgnoresSenderAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyMaxMintIgnoresSenderAssets(decoded));
            }
            if let Ok(decoded)
                = <VerifyPreviewDepositIgnoresSenderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyPreviewDepositIgnoresSender(decoded));
            }
            if let Ok(decoded)
                = <VerifyPreviewMintIgnoresSenderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyPreviewMintIgnoresSender(decoded));
            }
            if let Ok(decoded)
                = <VerifyPreviewRedeemIgnoresSenderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyPreviewRedeemIgnoresSender(decoded));
            }
            if let Ok(decoded)
                = <VerifyPreviewWithdrawIgnoresSenderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyPreviewWithdrawIgnoresSender(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CryticERC4626SenderIndependentCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::VerifyMaxDepositIgnoresSenderAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyMaxMintIgnoresSenderAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyPreviewDepositIgnoresSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyPreviewMintIgnoresSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyPreviewRedeemIgnoresSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyPreviewWithdrawIgnoresSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CryticERC4626SenderIndependentCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::VerifyMaxDepositIgnoresSenderAssets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyMaxMintIgnoresSenderAssets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyPreviewDepositIgnoresSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyPreviewMintIgnoresSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyPreviewRedeemIgnoresSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifyPreviewWithdrawIgnoresSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<VerifyMaxDepositIgnoresSenderAssetsCall>
    for CryticERC4626SenderIndependentCalls {
        fn from(value: VerifyMaxDepositIgnoresSenderAssetsCall) -> Self {
            Self::VerifyMaxDepositIgnoresSenderAssets(value)
        }
    }
    impl ::core::convert::From<VerifyMaxMintIgnoresSenderAssetsCall>
    for CryticERC4626SenderIndependentCalls {
        fn from(value: VerifyMaxMintIgnoresSenderAssetsCall) -> Self {
            Self::VerifyMaxMintIgnoresSenderAssets(value)
        }
    }
    impl ::core::convert::From<VerifyPreviewDepositIgnoresSenderCall>
    for CryticERC4626SenderIndependentCalls {
        fn from(value: VerifyPreviewDepositIgnoresSenderCall) -> Self {
            Self::VerifyPreviewDepositIgnoresSender(value)
        }
    }
    impl ::core::convert::From<VerifyPreviewMintIgnoresSenderCall>
    for CryticERC4626SenderIndependentCalls {
        fn from(value: VerifyPreviewMintIgnoresSenderCall) -> Self {
            Self::VerifyPreviewMintIgnoresSender(value)
        }
    }
    impl ::core::convert::From<VerifyPreviewRedeemIgnoresSenderCall>
    for CryticERC4626SenderIndependentCalls {
        fn from(value: VerifyPreviewRedeemIgnoresSenderCall) -> Self {
            Self::VerifyPreviewRedeemIgnoresSender(value)
        }
    }
    impl ::core::convert::From<VerifyPreviewWithdrawIgnoresSenderCall>
    for CryticERC4626SenderIndependentCalls {
        fn from(value: VerifyPreviewWithdrawIgnoresSenderCall) -> Self {
            Self::VerifyPreviewWithdrawIgnoresSender(value)
        }
    }
}
