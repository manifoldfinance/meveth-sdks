pub use auth_manager::*;
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
pub mod auth_manager {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialAdmin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialMevEth"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialShareVault"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialStaker"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("auth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("auth"),
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
                    ::std::borrow::ToOwned::to_owned("mevEth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mevEth"),
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
                    ::std::borrow::ToOwned::to_owned("mevEthShareVault"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mevEthShareVault"),
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
                    ::std::borrow::ToOwned::to_owned("updateMevEth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateMevEth"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newMevEth"),
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
                    ::std::borrow::ToOwned::to_owned("updateMevEthShareVault"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateMevEthShareVault",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newMevEthShareVault",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("updateWagyuStaker"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateWagyuStaker"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newWagyuStaker"),
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
                    ::std::borrow::ToOwned::to_owned("wagyuStaker"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wagyuStaker"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MevEthShareVaultAuthUpdateMissed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MevEthShareVaultAuthUpdateMissed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("changeAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
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
                    ::std::borrow::ToOwned::to_owned("Unauthorized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Unauthorized"),
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
    pub static AUTHMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA04a\0\xEAW`\x1Fa\t\xB78\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xEFW\x80\x84\x92`\x80\x94`@R\x839\x81\x01\x03\x12a\0\xEAWa\0G\x81a\x01\x05V[\x90a\0T` \x82\x01a\x01\x05V[\x91a\0m``a\0f`@\x85\x01a\x01\x05V[\x93\x01a\x01\x05V[\x90`\x80R`\x01\x80`\xA0\x1B\x03\x80\x92\x81`\x01\x80`\xA0\x1B\x03\x19\x95\x16\x85`\0T\x16\x17`\0U\x16\x83`\x01T\x16\x17`\x01U\x16\x90`\x02T\x16\x17`\x02U`@Qa\x08\x9D\x90\x81a\x01\x1A\x829`\x80Q\x81\x81\x81`\xC4\x01R\x81\x81a\x01`\x01R\x81\x81a\x01\xD5\x01R\x81\x81a\x03M\x01R\x81\x81a\x04\xA2\x01R\x81\x81a\x05\t\x01R\x81\x81a\x06\x88\x01Ra\x07\xEB\x01R\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0\xEAWV\xFE`\x80`@\x81\x81R`\x04\x806\x10\x15a\0\x15W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x03\r\xF0Y\x14a\x07\xC0WP\x80c'\xE1\xF7\xDF\x14a\x06]W\x80c,\xAC\xAB\xDE\x14a\x065W\x80cpH\x02u\x14a\x04\xDEW\x80c\x8Cd\x1F^\x14a\x04yW\x80c\x98p\xD7\xFE\x14a\x03\"W\x80c\xB4\t\x92\xA1\x14a\x01\xAAW\x80c\xBA\xDF&c\x14a\x01\x84W\x80c\xDE\x93u\xF2\x14a\x01@W\x80c\xF9\xCCE\xF2\x14a\x01\x14Wc\xFFN\x8BH\x14a\0\x9CW`\0\x80\xFD[4a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x805\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x01\0WPP`\x01`\x01`\xA0\x1B\x03\x19`\x01T\x16\x17`\x01U\x80\xF3[Qb\x82\xB4)`\xE8\x1B\x81R\xFD[\x84\x80\xFD[\x82\x80\xFD[PP4a\x01<W\x81`\x03\x196\x01\x12a\x01<W` \x90`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x90Q\x90\x81R\xF3[P\x80\xFD[PP4a\x01<W\x81`\x03\x196\x01\x12a\x01<W` \x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[PP4a\x01<W\x81`\x03\x196\x01\x12a\x01<W`\x01`\x01`\xA0\x1B\x03` \x92T\x16\x90Q\x90\x81R\xF3[P\x904a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x815\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03\x14W\x80\x85\x91\x82T\x16\x90\x81;\x15a\x01\x10W\x84Q\x83\x81`$\x81\x83c\xB4\t\x92\xA1`\xE0\x1B\x97\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x03\0W[PP\x80`\x02T\x16\x80;\x15a\x02\xDEW\x83\x80\x91`$\x88Q\x80\x94\x81\x93\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x02\xE2W[PP`\x01T\x16\x92\x83;\x15a\x01\x10W`$\x83\x92\x83\x88\x96\x88Q\x97\x88\x95\x86\x94\x85R\x84\x01RZ\xF1\x91\x82a\x02\xCAW[PPa\x02\xC5W\x7F\x01\xE2h\x0F{\xB5\xC7N;z\xFA\x1AA~\xCF\xEE\xB92\x07\xDF\xFAcC\x95\x8F%\xDDx\x1F\xEC\x07\xF7\x91\x81Q\x90\x81R`\x03` \x82\x01R\xA1\x80\xF3[PP\x80\xF3[a\x02\xD3\x90a\x08=V[a\x02\xDEW\x838a\x02\x8DV[\x83\x80\xFD[a\x02\xEB\x90a\x08=V[a\x01\x10W\x828a\x02cV[\x86Q=\x86\x82>=\x90\xFD[a\x03\t\x90a\x08=V[a\x01\x10W\x828a\x022V[P\x90Qb\x82\xB4)`\xE8\x1B\x81R\xFD[P\x904a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x815\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03\x14W\x80\x85\x91\x82T\x16\x90\x81;\x15a\x01\x10W\x84Q\x83\x81`$\x81\x83cL8k\xFF`\xE1\x1B\x97\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x04eW[PP\x80`\x02T\x16\x80;\x15a\x02\xDEW\x83\x80\x91`$\x88Q\x80\x94\x81\x93\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x04QW[PP`\x01T\x16\x92\x83;\x15a\x01\x10W`$\x83\x92\x83\x88\x96\x88Q\x97\x88\x95\x86\x94\x85R\x84\x01RZ\xF1\x91\x82a\x04=W[PPa\x02\xC5W\x7F\x01\xE2h\x0F{\xB5\xC7N;z\xFA\x1AA~\xCF\xEE\xB92\x07\xDF\xFAcC\x95\x8F%\xDDx\x1F\xEC\x07\xF7\x91\x81Q\x90\x81R`\x02` \x82\x01R\xA1\x80\xF3[a\x04F\x90a\x08=V[a\x02\xDEW\x838a\x04\x05V[a\x04Z\x90a\x08=V[a\x01\x10W\x828a\x03\xDBV[a\x04n\x90a\x08=V[a\x01\x10W\x828a\x03\xAAV[P4a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x805\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x01\0WPP`\x01`\x01`\xA0\x1B\x03\x19`\x02T\x16\x17`\x02U\x80\xF3[P\x904a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x815\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03\x14W\x80\x85\x91\x82T\x16\x90\x81;\x15a\x01\x10W\x84Q\x83\x81`$\x81\x83cpH\x02u`\xE0\x1B\x97\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x06!W[PP\x80`\x02T\x16\x80;\x15a\x02\xDEW\x83\x80\x91`$\x88Q\x80\x94\x81\x93\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x06\rW[PP`\x01T\x16\x92\x83;\x15a\x01\x10W`$\x83\x92\x83\x88\x96\x88Q\x97\x88\x95\x86\x94\x85R\x84\x01RZ\xF1\x91\x82a\x05\xF9W[PPa\x02\xC5W\x7F\x01\xE2h\x0F{\xB5\xC7N;z\xFA\x1AA~\xCF\xEE\xB92\x07\xDF\xFAcC\x95\x8F%\xDDx\x1F\xEC\x07\xF7\x91\x81Q\x90\x81R`\0` \x82\x01R\xA1\x80\xF3[a\x06\x02\x90a\x08=V[a\x02\xDEW\x838a\x05\xC1V[a\x06\x16\x90a\x08=V[a\x01\x10W\x828a\x05\x97V[a\x06*\x90a\x08=V[a\x01\x10W\x828a\x05fV[PP4a\x01<W\x81`\x03\x196\x01\x12a\x01<W` \x90`\x01`\x01`\xA0\x1B\x03`\x02T\x16\x90Q\x90\x81R\xF3[P\x904a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x815\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03\x14W\x80\x85T\x16\x90\x81;\x15a\x07\xBCW\x83Q\x86\x81`$\x81\x83c'\xE1\xF7\xDF`\xE0\x1B\x97\x88\x83R\x8B\x8A\x84\x01RZ\xF1\x80\x15a\x07\xB2Wa\x07\x9DW[P\x90\x81\x86\x92`\x02T\x16\x80;\x15a\x02\xDEW\x83\x80\x91`$\x88Q\x80\x94\x81\x93\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x07\x89W[PP`\x01T\x16\x92\x83;\x15a\x01\x10W`$\x83\x92\x83\x88\x96\x88Q\x97\x88\x95\x86\x94\x85R\x84\x01RZ\xF1\x91\x82a\x07uW[PPa\x02\xC5W\x7F\x01\xE2h\x0F{\xB5\xC7N;z\xFA\x1AA~\xCF\xEE\xB92\x07\xDF\xFAcC\x95\x8F%\xDDx\x1F\xEC\x07\xF7\x91\x81Q\x90\x81R`\x01` \x82\x01R\xA1\x80\xF3[a\x07~\x90a\x08=V[a\x02\xDEW\x838a\x07=V[a\x07\x92\x90a\x08=V[a\x01\x10W\x828a\x07\x13V[a\x07\xAA\x90\x96\x91\x92\x96a\x08=V[\x94\x908a\x06\xE0V[\x85Q=\x89\x82>=\x90\xFD[\x85\x80\xFD[\x92PP4a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x805\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x082WPP\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x81U\x80\xF3[b\x82\xB4)`\xE8\x1B\x81R\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08QW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 )P%\x1F\x81dJ\xA3\xA2\xAD\xED\xCB\x03R\x96\xE2O<\xC3_\x14\x951\x11\x8CG3\x0F\x02\x80\xE0&dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static AUTHMANAGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x806\x10\x15a\0\x15W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x03\r\xF0Y\x14a\x07\xC0WP\x80c'\xE1\xF7\xDF\x14a\x06]W\x80c,\xAC\xAB\xDE\x14a\x065W\x80cpH\x02u\x14a\x04\xDEW\x80c\x8Cd\x1F^\x14a\x04yW\x80c\x98p\xD7\xFE\x14a\x03\"W\x80c\xB4\t\x92\xA1\x14a\x01\xAAW\x80c\xBA\xDF&c\x14a\x01\x84W\x80c\xDE\x93u\xF2\x14a\x01@W\x80c\xF9\xCCE\xF2\x14a\x01\x14Wc\xFFN\x8BH\x14a\0\x9CW`\0\x80\xFD[4a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x805\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x01\0WPP`\x01`\x01`\xA0\x1B\x03\x19`\x01T\x16\x17`\x01U\x80\xF3[Qb\x82\xB4)`\xE8\x1B\x81R\xFD[\x84\x80\xFD[\x82\x80\xFD[PP4a\x01<W\x81`\x03\x196\x01\x12a\x01<W` \x90`\x01`\x01`\xA0\x1B\x03`\x01T\x16\x90Q\x90\x81R\xF3[P\x80\xFD[PP4a\x01<W\x81`\x03\x196\x01\x12a\x01<W` \x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[PP4a\x01<W\x81`\x03\x196\x01\x12a\x01<W`\x01`\x01`\xA0\x1B\x03` \x92T\x16\x90Q\x90\x81R\xF3[P\x904a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x815\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03\x14W\x80\x85\x91\x82T\x16\x90\x81;\x15a\x01\x10W\x84Q\x83\x81`$\x81\x83c\xB4\t\x92\xA1`\xE0\x1B\x97\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x03\0W[PP\x80`\x02T\x16\x80;\x15a\x02\xDEW\x83\x80\x91`$\x88Q\x80\x94\x81\x93\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x02\xE2W[PP`\x01T\x16\x92\x83;\x15a\x01\x10W`$\x83\x92\x83\x88\x96\x88Q\x97\x88\x95\x86\x94\x85R\x84\x01RZ\xF1\x91\x82a\x02\xCAW[PPa\x02\xC5W\x7F\x01\xE2h\x0F{\xB5\xC7N;z\xFA\x1AA~\xCF\xEE\xB92\x07\xDF\xFAcC\x95\x8F%\xDDx\x1F\xEC\x07\xF7\x91\x81Q\x90\x81R`\x03` \x82\x01R\xA1\x80\xF3[PP\x80\xF3[a\x02\xD3\x90a\x08=V[a\x02\xDEW\x838a\x02\x8DV[\x83\x80\xFD[a\x02\xEB\x90a\x08=V[a\x01\x10W\x828a\x02cV[\x86Q=\x86\x82>=\x90\xFD[a\x03\t\x90a\x08=V[a\x01\x10W\x828a\x022V[P\x90Qb\x82\xB4)`\xE8\x1B\x81R\xFD[P\x904a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x815\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03\x14W\x80\x85\x91\x82T\x16\x90\x81;\x15a\x01\x10W\x84Q\x83\x81`$\x81\x83cL8k\xFF`\xE1\x1B\x97\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x04eW[PP\x80`\x02T\x16\x80;\x15a\x02\xDEW\x83\x80\x91`$\x88Q\x80\x94\x81\x93\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x04QW[PP`\x01T\x16\x92\x83;\x15a\x01\x10W`$\x83\x92\x83\x88\x96\x88Q\x97\x88\x95\x86\x94\x85R\x84\x01RZ\xF1\x91\x82a\x04=W[PPa\x02\xC5W\x7F\x01\xE2h\x0F{\xB5\xC7N;z\xFA\x1AA~\xCF\xEE\xB92\x07\xDF\xFAcC\x95\x8F%\xDDx\x1F\xEC\x07\xF7\x91\x81Q\x90\x81R`\x02` \x82\x01R\xA1\x80\xF3[a\x04F\x90a\x08=V[a\x02\xDEW\x838a\x04\x05V[a\x04Z\x90a\x08=V[a\x01\x10W\x828a\x03\xDBV[a\x04n\x90a\x08=V[a\x01\x10W\x828a\x03\xAAV[P4a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x805\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x01\0WPP`\x01`\x01`\xA0\x1B\x03\x19`\x02T\x16\x17`\x02U\x80\xF3[P\x904a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x815\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03\x14W\x80\x85\x91\x82T\x16\x90\x81;\x15a\x01\x10W\x84Q\x83\x81`$\x81\x83cpH\x02u`\xE0\x1B\x97\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x06!W[PP\x80`\x02T\x16\x80;\x15a\x02\xDEW\x83\x80\x91`$\x88Q\x80\x94\x81\x93\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x06\rW[PP`\x01T\x16\x92\x83;\x15a\x01\x10W`$\x83\x92\x83\x88\x96\x88Q\x97\x88\x95\x86\x94\x85R\x84\x01RZ\xF1\x91\x82a\x05\xF9W[PPa\x02\xC5W\x7F\x01\xE2h\x0F{\xB5\xC7N;z\xFA\x1AA~\xCF\xEE\xB92\x07\xDF\xFAcC\x95\x8F%\xDDx\x1F\xEC\x07\xF7\x91\x81Q\x90\x81R`\0` \x82\x01R\xA1\x80\xF3[a\x06\x02\x90a\x08=V[a\x02\xDEW\x838a\x05\xC1V[a\x06\x16\x90a\x08=V[a\x01\x10W\x828a\x05\x97V[a\x06*\x90a\x08=V[a\x01\x10W\x828a\x05fV[PP4a\x01<W\x81`\x03\x196\x01\x12a\x01<W` \x90`\x01`\x01`\xA0\x1B\x03`\x02T\x16\x90Q\x90\x81R\xF3[P\x904a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x815\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03\x14W\x80\x85T\x16\x90\x81;\x15a\x07\xBCW\x83Q\x86\x81`$\x81\x83c'\xE1\xF7\xDF`\xE0\x1B\x97\x88\x83R\x8B\x8A\x84\x01RZ\xF1\x80\x15a\x07\xB2Wa\x07\x9DW[P\x90\x81\x86\x92`\x02T\x16\x80;\x15a\x02\xDEW\x83\x80\x91`$\x88Q\x80\x94\x81\x93\x88\x83R\x8C\x8B\x84\x01RZ\xF1\x80\x15a\x02\xF6W\x90\x84\x91a\x07\x89W[PP`\x01T\x16\x92\x83;\x15a\x01\x10W`$\x83\x92\x83\x88\x96\x88Q\x97\x88\x95\x86\x94\x85R\x84\x01RZ\xF1\x91\x82a\x07uW[PPa\x02\xC5W\x7F\x01\xE2h\x0F{\xB5\xC7N;z\xFA\x1AA~\xCF\xEE\xB92\x07\xDF\xFAcC\x95\x8F%\xDDx\x1F\xEC\x07\xF7\x91\x81Q\x90\x81R`\x01` \x82\x01R\xA1\x80\xF3[a\x07~\x90a\x08=V[a\x02\xDEW\x838a\x07=V[a\x07\x92\x90a\x08=V[a\x01\x10W\x828a\x07\x13V[a\x07\xAA\x90\x96\x91\x92\x96a\x08=V[\x94\x908a\x06\xE0V[\x85Q=\x89\x82>=\x90\xFD[\x85\x80\xFD[\x92PP4a\x01\x10W` 6`\x03\x19\x01\x12a\x01\x10W\x805\x91`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x80\x94\x03a\x01\x0CW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x082WPP\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x81U\x80\xF3[b\x82\xB4)`\xE8\x1B\x81R\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08QW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 )P%\x1F\x81dJ\xA3\xA2\xAD\xED\xCB\x03R\x96\xE2O<\xC3_\x14\x951\x11\x8CG3\x0F\x02\x80\xE0&dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static AUTHMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AuthManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AuthManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AuthManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AuthManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AuthManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AuthManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AuthManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AUTHMANAGER_ABI.clone(),
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
                AUTHMANAGER_ABI.clone(),
                AUTHMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `auth` (0xde9375f2) function
        pub fn auth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([222, 147, 117, 242], ())
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
        ///Calls the contract's `mevEth` (0xbadf2663) function
        pub fn mev_eth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([186, 223, 38, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mevEthShareVault` (0xf9cc45f2) function
        pub fn mev_eth_share_vault(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([249, 204, 69, 242], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMevEth` (0x030df059) function
        pub fn update_mev_eth(
            &self,
            new_mev_eth: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 13, 240, 89], new_mev_eth)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMevEthShareVault` (0xff4e8b48) function
        pub fn update_mev_eth_share_vault(
            &self,
            new_mev_eth_share_vault: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 78, 139, 72], new_mev_eth_share_vault)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateWagyuStaker` (0x8c641f5e) function
        pub fn update_wagyu_staker(
            &self,
            new_wagyu_staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 100, 31, 94], new_wagyu_staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wagyuStaker` (0x2cacabde) function
        pub fn wagyu_staker(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([44, 172, 171, 222], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `MevEthShareVaultAuthUpdateMissed` event
        pub fn mev_eth_share_vault_auth_update_missed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MevEthShareVaultAuthUpdateMissedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MevEthShareVaultAuthUpdateMissedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AuthManager<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
        name = "MevEthShareVaultAuthUpdateMissed",
        abi = "MevEthShareVaultAuthUpdateMissed(address,uint8)"
    )]
    pub struct MevEthShareVaultAuthUpdateMissedFilter {
        pub change_address: ::ethers::core::types::Address,
        pub operation: u8,
    }
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
    ///Container type for all input parameters for the `auth` function with signature `auth()` and selector `0xde9375f2`
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
    #[ethcall(name = "auth", abi = "auth()")]
    pub struct AuthCall;
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
    ///Container type for all input parameters for the `mevEth` function with signature `mevEth()` and selector `0xbadf2663`
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
    #[ethcall(name = "mevEth", abi = "mevEth()")]
    pub struct MevEthCall;
    ///Container type for all input parameters for the `mevEthShareVault` function with signature `mevEthShareVault()` and selector `0xf9cc45f2`
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
    #[ethcall(name = "mevEthShareVault", abi = "mevEthShareVault()")]
    pub struct MevEthShareVaultCall;
    ///Container type for all input parameters for the `updateMevEth` function with signature `updateMevEth(address)` and selector `0x030df059`
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
    #[ethcall(name = "updateMevEth", abi = "updateMevEth(address)")]
    pub struct UpdateMevEthCall {
        pub new_mev_eth: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateMevEthShareVault` function with signature `updateMevEthShareVault(address)` and selector `0xff4e8b48`
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
    #[ethcall(name = "updateMevEthShareVault", abi = "updateMevEthShareVault(address)")]
    pub struct UpdateMevEthShareVaultCall {
        pub new_mev_eth_share_vault: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateWagyuStaker` function with signature `updateWagyuStaker(address)` and selector `0x8c641f5e`
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
    #[ethcall(name = "updateWagyuStaker", abi = "updateWagyuStaker(address)")]
    pub struct UpdateWagyuStakerCall {
        pub new_wagyu_staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `wagyuStaker` function with signature `wagyuStaker()` and selector `0x2cacabde`
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
    #[ethcall(name = "wagyuStaker", abi = "wagyuStaker()")]
    pub struct WagyuStakerCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AuthManagerCalls {
        AddAdmin(AddAdminCall),
        AddOperator(AddOperatorCall),
        Auth(AuthCall),
        DeleteAdmin(DeleteAdminCall),
        DeleteOperator(DeleteOperatorCall),
        MevEth(MevEthCall),
        MevEthShareVault(MevEthShareVaultCall),
        UpdateMevEth(UpdateMevEthCall),
        UpdateMevEthShareVault(UpdateMevEthShareVaultCall),
        UpdateWagyuStaker(UpdateWagyuStakerCall),
        WagyuStaker(WagyuStakerCall),
    }
    impl ::ethers::core::abi::AbiDecode for AuthManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddAdmin(decoded));
            }
            if let Ok(decoded)
                = <AddOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddOperator(decoded));
            }
            if let Ok(decoded)
                = <AuthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Auth(decoded));
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
                = <MevEthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MevEth(decoded));
            }
            if let Ok(decoded)
                = <MevEthShareVaultCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MevEthShareVault(decoded));
            }
            if let Ok(decoded)
                = <UpdateMevEthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateMevEth(decoded));
            }
            if let Ok(decoded)
                = <UpdateMevEthShareVaultCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateMevEthShareVault(decoded));
            }
            if let Ok(decoded)
                = <UpdateWagyuStakerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateWagyuStaker(decoded));
            }
            if let Ok(decoded)
                = <WagyuStakerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WagyuStaker(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AuthManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Auth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeleteAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeleteOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MevEth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MevEthShareVault(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMevEth(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateMevEthShareVault(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateWagyuStaker(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WagyuStaker(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AuthManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Auth(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::MevEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::MevEthShareVault(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMevEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMevEthShareVault(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateWagyuStaker(element) => ::core::fmt::Display::fmt(element, f),
                Self::WagyuStaker(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddAdminCall> for AuthManagerCalls {
        fn from(value: AddAdminCall) -> Self {
            Self::AddAdmin(value)
        }
    }
    impl ::core::convert::From<AddOperatorCall> for AuthManagerCalls {
        fn from(value: AddOperatorCall) -> Self {
            Self::AddOperator(value)
        }
    }
    impl ::core::convert::From<AuthCall> for AuthManagerCalls {
        fn from(value: AuthCall) -> Self {
            Self::Auth(value)
        }
    }
    impl ::core::convert::From<DeleteAdminCall> for AuthManagerCalls {
        fn from(value: DeleteAdminCall) -> Self {
            Self::DeleteAdmin(value)
        }
    }
    impl ::core::convert::From<DeleteOperatorCall> for AuthManagerCalls {
        fn from(value: DeleteOperatorCall) -> Self {
            Self::DeleteOperator(value)
        }
    }
    impl ::core::convert::From<MevEthCall> for AuthManagerCalls {
        fn from(value: MevEthCall) -> Self {
            Self::MevEth(value)
        }
    }
    impl ::core::convert::From<MevEthShareVaultCall> for AuthManagerCalls {
        fn from(value: MevEthShareVaultCall) -> Self {
            Self::MevEthShareVault(value)
        }
    }
    impl ::core::convert::From<UpdateMevEthCall> for AuthManagerCalls {
        fn from(value: UpdateMevEthCall) -> Self {
            Self::UpdateMevEth(value)
        }
    }
    impl ::core::convert::From<UpdateMevEthShareVaultCall> for AuthManagerCalls {
        fn from(value: UpdateMevEthShareVaultCall) -> Self {
            Self::UpdateMevEthShareVault(value)
        }
    }
    impl ::core::convert::From<UpdateWagyuStakerCall> for AuthManagerCalls {
        fn from(value: UpdateWagyuStakerCall) -> Self {
            Self::UpdateWagyuStaker(value)
        }
    }
    impl ::core::convert::From<WagyuStakerCall> for AuthManagerCalls {
        fn from(value: WagyuStakerCall) -> Self {
            Self::WagyuStaker(value)
        }
    }
    ///Container type for all return fields from the `auth` function with signature `auth()` and selector `0xde9375f2`
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
    pub struct AuthReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `mevEth` function with signature `mevEth()` and selector `0xbadf2663`
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
    pub struct MevEthReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `mevEthShareVault` function with signature `mevEthShareVault()` and selector `0xf9cc45f2`
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
    pub struct MevEthShareVaultReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `wagyuStaker` function with signature `wagyuStaker()` and selector `0x2cacabde`
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
    pub struct WagyuStakerReturn(pub ::ethers::core::types::Address);
}
