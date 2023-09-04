pub use crytic_erc4626_vault_proxy::*;
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
pub mod crytic_erc4626_vault_proxy {
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
    pub static CRYTICERC4626VAULTPROXY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\t\xCC\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x1B.\xF1\xCA\x14a\x06\xBDWP\x80c'#\xF9\xEE\x14a\x050W\x83\x81c\x96\xEB'\xA1\x14a\x04qWP\x80c\xA4\x1F\xE4\x9F\x14a\x04\x16W\x80c\xA8\x15\xC1\x0F\x14a\x03\x02W\x80c\xB8\x19\"\x05\x14a\x02\x96W\x83\x81c\xB8\x8D\xAB2\x14a\x02\x1CW\x81c\xCC\xC9J\xE9\x14a\x017WPc\xE2\xBB\xB1X\x14a\0\x8BW`\0\x80\xFD[4a\x013W` a\0\xAAa\0\xE9\x93a\0\xA26a\x07LV[\x92\x90\x92a\x07\xC8V[\x91\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93cnU?e`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x90\x81\x15a\x01*WPa\0\xFDWP\x80\xF3[` \x90\x81=\x81\x11a\x01#W[a\x01\x13\x81\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\x1EW\x80\xF3[`\0\x80\xFD[P=a\x01\tV[Q=\x84\x82>=\x90\xFD[\x82\x80\xFD[\x80\x84\x844a\x02\x18W` 6`\x03\x19\x01\x12a\x02\x18W`\xFF\x82T`\xA0\x1C\x16\x15a\x02\x18W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x02\x0EW\x85\x91a\x01\xDAW[P\x15a\x01\xD5W\x82;\x15a\x01\xD5W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cU\xDFp\r`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01*WPa\x01\xC2WP\xF3[a\x01\xCB\x90a\x07|V[a\x01\xD2W\x80\xF3[\x80\xFD[PPP\xFD[\x94PP` \x84=\x82\x11a\x02\x06W[\x81a\x01\xF5` \x93\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\x1EW\x84\x93Q\x86a\x01\x87V[=\x91Pa\x01\xE8V[\x83Q=\x87\x82>=\x90\xFD[PP\xFD[\x80\x84\x844a\x02\x18Wa\x029\x91a\x0216a\x07LV[\x93\x90\x93a\x07\xC8V[\x92`\x01`\x01`\xA0\x1B\x03\x85T\x16\x80;\x15a\x02\x92W\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x92\x85\x01\x92\x83R` \x83\x01\x91\x90\x91R\x84\x91\x84\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x01*WPa\x01\xC2WP\xF3[\x85\x80\xFD[P4a\x013W` a\x02\xB9a\x02\xBF\x93a\x02\xAE6a\x07bV[\x96\x91\x96\x93\x90\x93a\x07\xC8V[\x92a\x07\xC8V[`\x01T\x85Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\0\xE9V[P\x904a\x013W` \x91\x82`\x03\x196\x01\x12a\x04\x12W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qcp\xA0\x821`\xE0\x1B\x81R0\x82\x82\x01R\x84\x81`$\x81\x87Z\xFA\x90\x81\x15a\x04\x08W\x90\x85\x92\x91\x87\x91a\x03\xD5W[P\x93a\x03_a\x03\x90\x95\x835a\x081V[\x84Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x90\x81R0` \x82\x01\x81\x90R`@\x82\x01R\x91\x94\x85\x92\x83\x91\x89\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x90\x81\x15a\x03\xCCWPa\x03\xA4W\x82\x80\xF3[\x81=\x83\x11a\x03\xC5W[a\x03\xB7\x81\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\x1EW8\x80\x82\x80\xF3[P=a\x03\xADV[Q=\x85\x82>=\x90\xFD[\x83\x81\x94\x92P=\x83\x11a\x04\x01W[a\x03\xEC\x81\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\x1EW\x90Q\x84\x91\x90a\x03_a\x03OV[P=a\x03\xE2V[\x83Q=\x88\x82>=\x90\xFD[\x83\x80\xFD[P4a\x013W` a\x02\xB9a\x04.\x93a\x02\xAE6a\x07bV[`\x01T\x85Qc-\x18+\xE5`\xE2\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\0\xE9V[\x80\x84\x844a\x02\x18W` 6`\x03\x19\x01\x12a\x02\x18W`\xFF\x82T`\xA0\x1C\x16\x15a\x02\x18W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x02\x0EW\x85\x91a\x04\xFCW[P\x15a\x01\xD5W\x82;\x15a\x01\xD5W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cb!\xE4\xF1`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01*WPa\x01\xC2WP\xF3[\x94PP` \x84=\x82\x11a\x05(W[\x81a\x05\x17` \x93\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\x1EW\x84\x93Q\x86a\x04\xC1V[=\x91Pa\x05\nV[P\x82\x904a\x06\xB9W` \x90\x81`\x03\x196\x01\x12a\x013W\x835\x93`\x01`\x01`\xA0\x1B\x03\x94\x85\x85T\x16\x80;\x15a\x02\x92W\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x84\x82\x01\x90\x81R` \x81\x01\x84\x90R\x90\x91\x87\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x06\xAFWa\x06\x9AW[P\x85\x84\x82\x87\x98a\x05\xD9\x97\x98T\x16\x83`\x01T\x16\x8A\x88Q\x80\x9A\x81\x95\x82\x94c\t^\xA7\xB3`\xE0\x1B\x84R\x8B\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x06\x90W\x90\x86\x93\x92\x91a\x06TW[`\x01T\x85QcnU?e`\xE0\x1B\x81R\x93\x84\x01\x92\x83R0` \x84\x01R\x92\x95P\x85\x92\x16\x90\x82\x90\x88\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x03\xCCWPa\x06,W\x82\x80\xF3[\x81=\x83\x11a\x06MW[a\x06?\x81\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\x1EW\x81\x80\x82\x80\xF3[P=a\x065V[\x92\x85\x81\x96\x92\x96=\x83\x11a\x06\x89W[a\x06l\x81\x83a\x07\xA6V[\x81\x01\x03\x12a\x06\x85WQ\x80\x15\x15\x03a\x02\x92W\x83\x85\x92a\x05\xEDV[\x86\x80\xFD[P=a\x06bV[\x84Q=\x89\x82>=\x90\xFD[\x94a\x06\xA8a\x05\xD9\x95\x96a\x07|V[\x94\x93a\x05\x96V[\x84Q=\x88\x82>=\x90\xFD[P\x80\xFD[\x92\x90P4a\x04\x12W\x82` \x91\x81\x86\x81a\x07\x10a\x06\xE3a\x06\xDB6a\x07LV[\x91\x90\x91a\x07\xC8V[`\x01Tc\x94\xBF\x80M`\xE0\x1B\x85R\x96\x84\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R\x90\x95\x16\x94`@\x01\x90V[\x03\x92Z\xF1\x90\x81\x15a\x01*WPa\x07$WP\x80\xF3[` \x90\x81=\x81\x11a\x07EW[a\x07:\x81\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\xD2W\x80\xF3[P=a\x070V[`@\x90`\x03\x19\x01\x12a\x01\x1EW`\x045\x90`$5\x90V[``\x90`\x03\x19\x01\x12a\x01\x1EW`\x045\x90`$5\x90`D5\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x90W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\x90W`@RV[`\x03\x90\x06\x80\x15a\x08\tW`\x01\x14a\x07\xF1Ws\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\x90V[s\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\x90V[P0\x90V[`\0[\x83\x81\x10a\x08!WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x08\x11V[\x90\x80\x82\x11a\x08=WP\x90V[`\x01\x81\x01\x80\x91\x11a\tAW\x80\x15a\t+Wa\t\x1B`@a\x08\x80\x7F\xA9^n*\x18$\x11\xE7\xA6\xF9\xED\x11J\x85\xC3v\x1D\x87\xF9\xB8\xF4S\xD8B\xC7\x125\xAAd\xFF\xF9\x9F\x93\x85\x06\x94a\tWV[a\x08\x89\x85a\tWV[\x90a\t\0`3\x84Q\x80\x94` \x82\x01\x94\x7FClamping value \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86Ra\x08\xCF\x81Q\x80\x92` `/\x87\x01\x91\x01a\x08\x0EV[\x82\x01c\x01\x03\xA3y`\xE5\x1B`/\x82\x01Ra\x08\xF1\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x08\x0EV[\x01\x03`\x13\x81\x01\x85R\x01\x83a\x07\xA6V[\x82Q\x93\x84\x92` \x84RQ\x80\x92\x81` \x86\x01R\x85\x85\x01\x90a\x08\x0EV[`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xA1\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\tmW\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \x16\xE3S\xE4\xB4#\xCB\x04\x1C\xEEl\x89S\xAD(\xE5\xCE\x89\x8E\xA7[\xEC\x8E\x94Q\rO\xFE\xEB\x91\xDE\xE8dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static CRYTICERC4626VAULTPROXY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x1B.\xF1\xCA\x14a\x06\xBDWP\x80c'#\xF9\xEE\x14a\x050W\x83\x81c\x96\xEB'\xA1\x14a\x04qWP\x80c\xA4\x1F\xE4\x9F\x14a\x04\x16W\x80c\xA8\x15\xC1\x0F\x14a\x03\x02W\x80c\xB8\x19\"\x05\x14a\x02\x96W\x83\x81c\xB8\x8D\xAB2\x14a\x02\x1CW\x81c\xCC\xC9J\xE9\x14a\x017WPc\xE2\xBB\xB1X\x14a\0\x8BW`\0\x80\xFD[4a\x013W` a\0\xAAa\0\xE9\x93a\0\xA26a\x07LV[\x92\x90\x92a\x07\xC8V[\x91\x86`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x92\x86Q\x97\x88\x95\x86\x94\x85\x93cnU?e`\xE0\x1B\x85R\x84\x01\x90\x92\x91`\x01`\x01`\xA0\x1B\x03` \x91`@\x84\x01\x95\x84R\x16\x91\x01RV[\x03\x92Z\xF1\x90\x81\x15a\x01*WPa\0\xFDWP\x80\xF3[` \x90\x81=\x81\x11a\x01#W[a\x01\x13\x81\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\x1EW\x80\xF3[`\0\x80\xFD[P=a\x01\tV[Q=\x84\x82>=\x90\xFD[\x82\x80\xFD[\x80\x84\x844a\x02\x18W` 6`\x03\x19\x01\x12a\x02\x18W`\xFF\x82T`\xA0\x1C\x16\x15a\x02\x18W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x02\x0EW\x85\x91a\x01\xDAW[P\x15a\x01\xD5W\x82;\x15a\x01\xD5W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cU\xDFp\r`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01*WPa\x01\xC2WP\xF3[a\x01\xCB\x90a\x07|V[a\x01\xD2W\x80\xF3[\x80\xFD[PPP\xFD[\x94PP` \x84=\x82\x11a\x02\x06W[\x81a\x01\xF5` \x93\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\x1EW\x84\x93Q\x86a\x01\x87V[=\x91Pa\x01\xE8V[\x83Q=\x87\x82>=\x90\xFD[PP\xFD[\x80\x84\x844a\x02\x18Wa\x029\x91a\x0216a\x07LV[\x93\x90\x93a\x07\xC8V[\x92`\x01`\x01`\xA0\x1B\x03\x85T\x16\x80;\x15a\x02\x92W\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x92\x85\x01\x92\x83R` \x83\x01\x91\x90\x91R\x84\x91\x84\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x01*WPa\x01\xC2WP\xF3[\x85\x80\xFD[P4a\x013W` a\x02\xB9a\x02\xBF\x93a\x02\xAE6a\x07bV[\x96\x91\x96\x93\x90\x93a\x07\xC8V[\x92a\x07\xC8V[`\x01T\x85Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\0\xE9V[P\x904a\x013W` \x91\x82`\x03\x196\x01\x12a\x04\x12W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qcp\xA0\x821`\xE0\x1B\x81R0\x82\x82\x01R\x84\x81`$\x81\x87Z\xFA\x90\x81\x15a\x04\x08W\x90\x85\x92\x91\x87\x91a\x03\xD5W[P\x93a\x03_a\x03\x90\x95\x835a\x081V[\x84Qc]\x04;)`\xE1\x1B\x81R\x92\x83\x01\x90\x81R0` \x82\x01\x81\x90R`@\x82\x01R\x91\x94\x85\x92\x83\x91\x89\x91\x83\x91``\x90\x91\x01\x90V[\x03\x92Z\xF1\x90\x81\x15a\x03\xCCWPa\x03\xA4W\x82\x80\xF3[\x81=\x83\x11a\x03\xC5W[a\x03\xB7\x81\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\x1EW8\x80\x82\x80\xF3[P=a\x03\xADV[Q=\x85\x82>=\x90\xFD[\x83\x81\x94\x92P=\x83\x11a\x04\x01W[a\x03\xEC\x81\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\x1EW\x90Q\x84\x91\x90a\x03_a\x03OV[P=a\x03\xE2V[\x83Q=\x88\x82>=\x90\xFD[\x83\x80\xFD[P4a\x013W` a\x02\xB9a\x04.\x93a\x02\xAE6a\x07bV[`\x01T\x85Qc-\x18+\xE5`\xE2\x1B\x81R\x92\x83\x01\x96\x87R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x88\x01R\x90\x83\x16`@\x87\x01R\x90\x94\x85\x92\x90\x91\x16\x90\x82\x90\x87\x90\x82\x90``\x01a\0\xE9V[\x80\x84\x844a\x02\x18W` 6`\x03\x19\x01\x12a\x02\x18W`\xFF\x82T`\xA0\x1C\x16\x15a\x02\x18W`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x91\x81Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81\x83\x81\x87Z\xFA\x90\x81\x15a\x02\x0EW\x85\x91a\x04\xFCW[P\x15a\x01\xD5W\x82;\x15a\x01\xD5W\x83\x92`$\x84\x92\x84Q\x95\x86\x93\x84\x92cb!\xE4\xF1`\xE0\x1B\x84R\x805\x90\x84\x01RZ\xF1\x90\x81\x15a\x01*WPa\x01\xC2WP\xF3[\x94PP` \x84=\x82\x11a\x05(W[\x81a\x05\x17` \x93\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\x1EW\x84\x93Q\x86a\x04\xC1V[=\x91Pa\x05\nV[P\x82\x904a\x06\xB9W` \x90\x81`\x03\x196\x01\x12a\x013W\x835\x93`\x01`\x01`\xA0\x1B\x03\x94\x85\x85T\x16\x80;\x15a\x02\x92W\x83Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x84\x82\x01\x90\x81R` \x81\x01\x84\x90R\x90\x91\x87\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x06\xAFWa\x06\x9AW[P\x85\x84\x82\x87\x98a\x05\xD9\x97\x98T\x16\x83`\x01T\x16\x8A\x88Q\x80\x9A\x81\x95\x82\x94c\t^\xA7\xB3`\xE0\x1B\x84R\x8B\x84\x01` \x90\x93\x92\x91\x93`\x01`\x01`\xA0\x1B\x03`@\x82\x01\x95\x16\x81R\x01RV[\x03\x92Z\xF1\x80\x15a\x06\x90W\x90\x86\x93\x92\x91a\x06TW[`\x01T\x85QcnU?e`\xE0\x1B\x81R\x93\x84\x01\x92\x83R0` \x84\x01R\x92\x95P\x85\x92\x16\x90\x82\x90\x88\x90\x82\x90`@\x01\x03\x92Z\xF1\x90\x81\x15a\x03\xCCWPa\x06,W\x82\x80\xF3[\x81=\x83\x11a\x06MW[a\x06?\x81\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\x1EW\x81\x80\x82\x80\xF3[P=a\x065V[\x92\x85\x81\x96\x92\x96=\x83\x11a\x06\x89W[a\x06l\x81\x83a\x07\xA6V[\x81\x01\x03\x12a\x06\x85WQ\x80\x15\x15\x03a\x02\x92W\x83\x85\x92a\x05\xEDV[\x86\x80\xFD[P=a\x06bV[\x84Q=\x89\x82>=\x90\xFD[\x94a\x06\xA8a\x05\xD9\x95\x96a\x07|V[\x94\x93a\x05\x96V[\x84Q=\x88\x82>=\x90\xFD[P\x80\xFD[\x92\x90P4a\x04\x12W\x82` \x91\x81\x86\x81a\x07\x10a\x06\xE3a\x06\xDB6a\x07LV[\x91\x90\x91a\x07\xC8V[`\x01Tc\x94\xBF\x80M`\xE0\x1B\x85R\x96\x84\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R\x90\x95\x16\x94`@\x01\x90V[\x03\x92Z\xF1\x90\x81\x15a\x01*WPa\x07$WP\x80\xF3[` \x90\x81=\x81\x11a\x07EW[a\x07:\x81\x83a\x07\xA6V[\x81\x01\x03\x12a\x01\xD2W\x80\xF3[P=a\x070V[`@\x90`\x03\x19\x01\x12a\x01\x1EW`\x045\x90`$5\x90V[``\x90`\x03\x19\x01\x12a\x01\x1EW`\x045\x90`$5\x90`D5\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07\x90W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07\x90W`@RV[`\x03\x90\x06\x80\x15a\x08\tW`\x01\x14a\x07\xF1Ws\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\xBB\x90V[s\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\xAA\x90V[P0\x90V[`\0[\x83\x81\x10a\x08!WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x08\x11V[\x90\x80\x82\x11a\x08=WP\x90V[`\x01\x81\x01\x80\x91\x11a\tAW\x80\x15a\t+Wa\t\x1B`@a\x08\x80\x7F\xA9^n*\x18$\x11\xE7\xA6\xF9\xED\x11J\x85\xC3v\x1D\x87\xF9\xB8\xF4S\xD8B\xC7\x125\xAAd\xFF\xF9\x9F\x93\x85\x06\x94a\tWV[a\x08\x89\x85a\tWV[\x90a\t\0`3\x84Q\x80\x94` \x82\x01\x94\x7FClamping value \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86Ra\x08\xCF\x81Q\x80\x92` `/\x87\x01\x91\x01a\x08\x0EV[\x82\x01c\x01\x03\xA3y`\xE5\x1B`/\x82\x01Ra\x08\xF1\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x08\x0EV[\x01\x03`\x13\x81\x01\x85R\x01\x83a\x07\xA6V[\x82Q\x93\x84\x92` \x84RQ\x80\x92\x81` \x86\x01R\x85\x85\x01\x90a\x08\x0EV[`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xA1\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\tmW\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \x16\xE3S\xE4\xB4#\xCB\x04\x1C\xEEl\x89S\xAD(\xE5\xCE\x89\x8E\xA7[\xEC\x8E\x94Q\rO\xFE\xEB\x91\xDE\xE8dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static CRYTICERC4626VAULTPROXY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CryticERC4626VaultProxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CryticERC4626VaultProxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CryticERC4626VaultProxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CryticERC4626VaultProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CryticERC4626VaultProxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CryticERC4626VaultProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CryticERC4626VaultProxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CRYTICERC4626VAULTPROXY_ABI.clone(),
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
                CRYTICERC4626VAULTPROXY_ABI.clone(),
                CRYTICERC4626VAULTPROXY_BYTECODE.clone().into(),
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
            CryticERC4626VaultProxyEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CryticERC4626VaultProxy<M> {
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
    pub enum CryticERC4626VaultProxyEvents {
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
    impl ::ethers::contract::EthLogDecode for CryticERC4626VaultProxyEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssertEqFailFilter::decode_log(log) {
                return Ok(CryticERC4626VaultProxyEvents::AssertEqFailFilter(decoded));
            }
            if let Ok(decoded) = AssertFailFilter::decode_log(log) {
                return Ok(CryticERC4626VaultProxyEvents::AssertFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGtFailFilter::decode_log(log) {
                return Ok(CryticERC4626VaultProxyEvents::AssertGtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGteFailFilter::decode_log(log) {
                return Ok(CryticERC4626VaultProxyEvents::AssertGteFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLtFailFilter::decode_log(log) {
                return Ok(CryticERC4626VaultProxyEvents::AssertLtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLteFailFilter::decode_log(log) {
                return Ok(CryticERC4626VaultProxyEvents::AssertLteFailFilter(decoded));
            }
            if let Ok(decoded) = AssertNeqFailFilter::decode_log(log) {
                return Ok(CryticERC4626VaultProxyEvents::AssertNeqFailFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(CryticERC4626VaultProxyEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(CryticERC4626VaultProxyEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUint256Filter::decode_log(log) {
                return Ok(CryticERC4626VaultProxyEvents::LogUint256Filter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CryticERC4626VaultProxyEvents {
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
    impl ::core::convert::From<AssertEqFailFilter> for CryticERC4626VaultProxyEvents {
        fn from(value: AssertEqFailFilter) -> Self {
            Self::AssertEqFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertFailFilter> for CryticERC4626VaultProxyEvents {
        fn from(value: AssertFailFilter) -> Self {
            Self::AssertFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGtFailFilter> for CryticERC4626VaultProxyEvents {
        fn from(value: AssertGtFailFilter) -> Self {
            Self::AssertGtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGteFailFilter> for CryticERC4626VaultProxyEvents {
        fn from(value: AssertGteFailFilter) -> Self {
            Self::AssertGteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLtFailFilter> for CryticERC4626VaultProxyEvents {
        fn from(value: AssertLtFailFilter) -> Self {
            Self::AssertLtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLteFailFilter> for CryticERC4626VaultProxyEvents {
        fn from(value: AssertLteFailFilter) -> Self {
            Self::AssertLteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertNeqFailFilter> for CryticERC4626VaultProxyEvents {
        fn from(value: AssertNeqFailFilter) -> Self {
            Self::AssertNeqFailFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for CryticERC4626VaultProxyEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for CryticERC4626VaultProxyEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUint256Filter> for CryticERC4626VaultProxyEvents {
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
    pub enum CryticERC4626VaultProxyCalls {
        Deposit(DepositCall),
        DepositForSelfSimple(DepositForSelfSimpleCall),
        Mint(MintCall),
        MintAsset(MintAssetCall),
        RecognizeLossProxy(RecognizeLossProxyCall),
        RecognizeProfitProxy(RecognizeProfitProxyCall),
        Redeem(RedeemCall),
        RedeemForSelfSimple(RedeemForSelfSimpleCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for CryticERC4626VaultProxyCalls {
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
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CryticERC4626VaultProxyCalls {
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
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CryticERC4626VaultProxyCalls {
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
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositCall> for CryticERC4626VaultProxyCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositForSelfSimpleCall>
    for CryticERC4626VaultProxyCalls {
        fn from(value: DepositForSelfSimpleCall) -> Self {
            Self::DepositForSelfSimple(value)
        }
    }
    impl ::core::convert::From<MintCall> for CryticERC4626VaultProxyCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<MintAssetCall> for CryticERC4626VaultProxyCalls {
        fn from(value: MintAssetCall) -> Self {
            Self::MintAsset(value)
        }
    }
    impl ::core::convert::From<RecognizeLossProxyCall> for CryticERC4626VaultProxyCalls {
        fn from(value: RecognizeLossProxyCall) -> Self {
            Self::RecognizeLossProxy(value)
        }
    }
    impl ::core::convert::From<RecognizeProfitProxyCall>
    for CryticERC4626VaultProxyCalls {
        fn from(value: RecognizeProfitProxyCall) -> Self {
            Self::RecognizeProfitProxy(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for CryticERC4626VaultProxyCalls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<RedeemForSelfSimpleCall>
    for CryticERC4626VaultProxyCalls {
        fn from(value: RedeemForSelfSimpleCall) -> Self {
            Self::RedeemForSelfSimple(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for CryticERC4626VaultProxyCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
}
