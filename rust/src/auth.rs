pub use auth::*;
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
pub mod auth {
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
                    ::std::borrow::ToOwned::to_owned("admins"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("admins"),
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
                    ::std::borrow::ToOwned::to_owned("operators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("operators"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AdminAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AdminDeleted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminDeleted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OperatorAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOperator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorDeleted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OperatorDeleted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldOperator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("AlreadySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AlreadySet"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NoAdmin"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
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
    pub static AUTH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x804a\0\xA0W`\x1Fa\x04\xC88\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xA5W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0\xA0WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0\xA0W`\0R`\x02` R`@`\0 `\xFF\x19\x90`\x01\x82\x82T\x16\x17\x90U`\0T\x81`\xFF`\x01\x81\x84\x16\x01\x16\x91\x16\x17`\0U`\x01` R`\x01`@`\0 \x91\x82T\x16\x17\x90U`@Qa\x04\x0C\x90\x81a\0\xBC\x829\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x13\xE7\xC9\xD8\x14a\x03\x9BWP\x80c'\xE1\xF7\xDF\x14a\x02\xB6W\x80cB\x9Bb\xE5\x14a\x02xW\x80cpH\x02u\x14a\x01\x9AW\x80c\x98p\xD7\xFE\x14a\x01\x10Wc\xB4\t\x92\xA1\x14a\0fW`\0\x80\xFD[4a\x01\x0CW` 6`\x03\x19\x01\x12a\x01\x0CW\x815\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x80\x93\x03a\x01\x08W3\x84R`\x02` R`\xFF\x82\x85 T\x16\x15a\0\xFBW\x82\x84R`\x01` R`\xFF\x82\x85 T\x16\x15a\0\xEDWP\x81\x83R`\x01` R\x82 \x80T`\xFF\x19\x16\x90U\x7Fi\xDF,^\xC2\xEAM\x1F\xBE\x1EP5$\xF5\x93\xB3V\x16,\xA7\x10g\x12c\x82\x7F.\x19\x92\xB9Z\xE1\x82\x80\xA2\x80\xF3[\x90Qc\xA7A\xA0E`\xE0\x1B\x81R\xFD[\x90Qb\x82\xB4)`\xE8\x1B\x81R\xFD[\x83\x80\xFD[\x82\x80\xFD[P4a\x01\x0CW` 6`\x03\x19\x01\x12a\x01\x0CW\x815\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x80\x93\x03a\x01\x08W3\x84R`\x02` R`\xFF\x82\x85 T\x16\x15a\0\xFBW\x82\x84R`\x01` R`\xFF\x82\x85 T\x16a\0\xEDWP\x81\x83R`\x01` R\x82 `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\xACo\xA8X\xE95\nF\xCE\xC1e9\x92n\x0F\xDE%\xB7b\x9F\x84\xB5\xA7+\xFF\xAA\xE4\xDF\x88\x8A\xE8m\x82\x80\xA2\x80\xF3[P\x82\x904a\x02tW` 6`\x03\x19\x01\x12a\x02tW\x825\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x92\x03a\x01\x0CW3\x83R`\x02` R`\xFF\x81\x84 T\x16\x15a\x02fW\x81\x83R`\x02` R`\xFF\x81\x84 T\x16a\x02WW\x82T`\xFF\x81\x16`\xFF\x81\x14a\x02DW`\xFF\x19\x91\x82\x16`\x01\x91\x82\x01`\xFF\x16\x17\x85U\x83\x85R`\x02` R\x91\x84 \x80T\x90\x91\x16\x90\x91\x17\x90U\x7FD\xD6\xD2Yc\xF0\x97\xAD\x14\xF2\x9F\x06\x85J\x01\xF5ud\x8A\x1E\xF8/0\xE5b\xCC\xD3\x88\x97\x17\xE39\x82\x80\xA2\x80\xF3[cNH{q`\xE0\x1B\x85R`\x11\x86R`$\x85\xFD[Qc\xA7A\xA0E`\xE0\x1B\x81R\x83\x90\xFD[Qb\x82\xB4)`\xE8\x1B\x81R\x83\x90\xFD[P\x80\xFD[P\x904a\x01\x0CW` 6`\x03\x19\x01\x12a\x01\x0CW5`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x91\x03a\x01\x0CW\x81\x83`\xFF\x92` \x95R`\x02\x85R T\x16\x90Q\x90\x15\x15\x81R\xF3[P\x904a\x01\x0CW` 6`\x03\x19\x01\x12a\x01\x0CW\x805\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x80\x93\x03a\x01\x08W3\x84R`\x02` R`\xFF\x81\x85 T\x16\x15a\x03\x8FW\x82\x84R`\x02` R`\xFF\x81\x85 T\x16\x15a\x03\x82W\x83T\x91`\xFF\x83\x16\x80\x15a\x03oW`\xFF\x90`\0\x19\x01\x16\x80`\xFF\x19\x80\x95\x16\x17\x86U\x15a\x03aWP\x82\x84R`\x02` R\x83 \x90\x81T\x16\x90U\x7F\x98\x9D\xDF\xCE\x05}\xAD!\x9E\n\xE1oi\x1B\x12\x1B\xB0\xE3H\xF0\xD8\xAE\n\xD4\0\xB4\xD5\xAC\x8Dal\x8B\x82\x80\xA2\x80\xF3[\x90Qc\x1F\x8C\x1D\xBD`\xE1\x1B\x81R\xFD[cNH{q`\xE0\x1B\x86R`\x11\x82R`$\x86\xFD[Qc\xA7A\xA0E`\xE0\x1B\x81R\xFD[Qb\x82\xB4)`\xE8\x1B\x81R\xFD[\x92\x90P4a\x01\x08W` 6`\x03\x19\x01\x12a\x01\x08W5`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x91\x03a\x01\x08W\x83R`\x01` \x90\x81R\x92 T`\xFF\x16\x15\x15\x81R\xF3\xFE\xA2dipfsX\"\x12 1#\x02\xC1\xE9V)\x02\xAFy\x94\x12C\x97\xCF\xB6%\"\xA3\xF3n\xDD\xF2\xA0E\xCEx-\x98c\xCA3dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static AUTH_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x13\xE7\xC9\xD8\x14a\x03\x9BWP\x80c'\xE1\xF7\xDF\x14a\x02\xB6W\x80cB\x9Bb\xE5\x14a\x02xW\x80cpH\x02u\x14a\x01\x9AW\x80c\x98p\xD7\xFE\x14a\x01\x10Wc\xB4\t\x92\xA1\x14a\0fW`\0\x80\xFD[4a\x01\x0CW` 6`\x03\x19\x01\x12a\x01\x0CW\x815\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x80\x93\x03a\x01\x08W3\x84R`\x02` R`\xFF\x82\x85 T\x16\x15a\0\xFBW\x82\x84R`\x01` R`\xFF\x82\x85 T\x16\x15a\0\xEDWP\x81\x83R`\x01` R\x82 \x80T`\xFF\x19\x16\x90U\x7Fi\xDF,^\xC2\xEAM\x1F\xBE\x1EP5$\xF5\x93\xB3V\x16,\xA7\x10g\x12c\x82\x7F.\x19\x92\xB9Z\xE1\x82\x80\xA2\x80\xF3[\x90Qc\xA7A\xA0E`\xE0\x1B\x81R\xFD[\x90Qb\x82\xB4)`\xE8\x1B\x81R\xFD[\x83\x80\xFD[\x82\x80\xFD[P4a\x01\x0CW` 6`\x03\x19\x01\x12a\x01\x0CW\x815\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x80\x93\x03a\x01\x08W3\x84R`\x02` R`\xFF\x82\x85 T\x16\x15a\0\xFBW\x82\x84R`\x01` R`\xFF\x82\x85 T\x16a\0\xEDWP\x81\x83R`\x01` R\x82 `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\xACo\xA8X\xE95\nF\xCE\xC1e9\x92n\x0F\xDE%\xB7b\x9F\x84\xB5\xA7+\xFF\xAA\xE4\xDF\x88\x8A\xE8m\x82\x80\xA2\x80\xF3[P\x82\x904a\x02tW` 6`\x03\x19\x01\x12a\x02tW\x825\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x80\x92\x03a\x01\x0CW3\x83R`\x02` R`\xFF\x81\x84 T\x16\x15a\x02fW\x81\x83R`\x02` R`\xFF\x81\x84 T\x16a\x02WW\x82T`\xFF\x81\x16`\xFF\x81\x14a\x02DW`\xFF\x19\x91\x82\x16`\x01\x91\x82\x01`\xFF\x16\x17\x85U\x83\x85R`\x02` R\x91\x84 \x80T\x90\x91\x16\x90\x91\x17\x90U\x7FD\xD6\xD2Yc\xF0\x97\xAD\x14\xF2\x9F\x06\x85J\x01\xF5ud\x8A\x1E\xF8/0\xE5b\xCC\xD3\x88\x97\x17\xE39\x82\x80\xA2\x80\xF3[cNH{q`\xE0\x1B\x85R`\x11\x86R`$\x85\xFD[Qc\xA7A\xA0E`\xE0\x1B\x81R\x83\x90\xFD[Qb\x82\xB4)`\xE8\x1B\x81R\x83\x90\xFD[P\x80\xFD[P\x904a\x01\x0CW` 6`\x03\x19\x01\x12a\x01\x0CW5`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x91\x03a\x01\x0CW\x81\x83`\xFF\x92` \x95R`\x02\x85R T\x16\x90Q\x90\x15\x15\x81R\xF3[P\x904a\x01\x0CW` 6`\x03\x19\x01\x12a\x01\x0CW\x805\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x80\x93\x03a\x01\x08W3\x84R`\x02` R`\xFF\x81\x85 T\x16\x15a\x03\x8FW\x82\x84R`\x02` R`\xFF\x81\x85 T\x16\x15a\x03\x82W\x83T\x91`\xFF\x83\x16\x80\x15a\x03oW`\xFF\x90`\0\x19\x01\x16\x80`\xFF\x19\x80\x95\x16\x17\x86U\x15a\x03aWP\x82\x84R`\x02` R\x83 \x90\x81T\x16\x90U\x7F\x98\x9D\xDF\xCE\x05}\xAD!\x9E\n\xE1oi\x1B\x12\x1B\xB0\xE3H\xF0\xD8\xAE\n\xD4\0\xB4\xD5\xAC\x8Dal\x8B\x82\x80\xA2\x80\xF3[\x90Qc\x1F\x8C\x1D\xBD`\xE1\x1B\x81R\xFD[cNH{q`\xE0\x1B\x86R`\x11\x82R`$\x86\xFD[Qc\xA7A\xA0E`\xE0\x1B\x81R\xFD[Qb\x82\xB4)`\xE8\x1B\x81R\xFD[\x92\x90P4a\x01\x08W` 6`\x03\x19\x01\x12a\x01\x08W5`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x91\x03a\x01\x08W\x83R`\x01` \x90\x81R\x92 T`\xFF\x16\x15\x15\x81R\xF3\xFE\xA2dipfsX\"\x12 1#\x02\xC1\xE9V)\x02\xAFy\x94\x12C\x97\xCF\xB6%\"\xA3\xF3n\xDD\xF2\xA0E\xCEx-\x98c\xCA3dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static AUTH_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Auth<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Auth<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Auth<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Auth<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Auth<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Auth)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Auth<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AUTH_ABI.clone(),
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
                AUTH_ABI.clone(),
                AUTH_BYTECODE.clone().into(),
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
        ///Calls the contract's `admins` (0x429b62e5) function
        pub fn admins(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([66, 155, 98, 229], p0)
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
        ///Calls the contract's `operators` (0x13e7c9d8) function
        pub fn operators(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([19, 231, 201, 216], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AdminAdded` event
        pub fn admin_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AdminDeleted` event
        pub fn admin_deleted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminDeletedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorAdded` event
        pub fn operator_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorDeleted` event
        pub fn operator_deleted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorDeletedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AuthEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Auth<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AlreadySet` with signature `AlreadySet()` and selector `0xa741a045`
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
    #[etherror(name = "AlreadySet", abi = "AlreadySet()")]
    pub struct AlreadySet;
    ///Custom Error type `NoAdmin` with signature `NoAdmin()` and selector `0x3f183b7a`
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
    #[etherror(name = "NoAdmin", abi = "NoAdmin()")]
    pub struct NoAdmin;
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AuthErrors {
        AlreadySet(AlreadySet),
        NoAdmin(NoAdmin),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AuthErrors {
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
                = <AlreadySet as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AlreadySet(decoded));
            }
            if let Ok(decoded)
                = <NoAdmin as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoAdmin(decoded));
            }
            if let Ok(decoded)
                = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AuthErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AlreadySet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoAdmin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AuthErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AlreadySet as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NoAdmin as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AuthErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AlreadySet(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AuthErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AlreadySet> for AuthErrors {
        fn from(value: AlreadySet) -> Self {
            Self::AlreadySet(value)
        }
    }
    impl ::core::convert::From<NoAdmin> for AuthErrors {
        fn from(value: NoAdmin) -> Self {
            Self::NoAdmin(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for AuthErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
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
    #[ethevent(name = "AdminAdded", abi = "AdminAdded(address)")]
    pub struct AdminAddedFilter {
        #[ethevent(indexed)]
        pub new_admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "AdminDeleted", abi = "AdminDeleted(address)")]
    pub struct AdminDeletedFilter {
        #[ethevent(indexed)]
        pub old_admin: ::ethers::core::types::Address,
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
    #[ethevent(name = "OperatorAdded", abi = "OperatorAdded(address)")]
    pub struct OperatorAddedFilter {
        #[ethevent(indexed)]
        pub new_operator: ::ethers::core::types::Address,
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
    #[ethevent(name = "OperatorDeleted", abi = "OperatorDeleted(address)")]
    pub struct OperatorDeletedFilter {
        #[ethevent(indexed)]
        pub old_operator: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AuthEvents {
        AdminAddedFilter(AdminAddedFilter),
        AdminDeletedFilter(AdminDeletedFilter),
        OperatorAddedFilter(OperatorAddedFilter),
        OperatorDeletedFilter(OperatorDeletedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AuthEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminAddedFilter::decode_log(log) {
                return Ok(AuthEvents::AdminAddedFilter(decoded));
            }
            if let Ok(decoded) = AdminDeletedFilter::decode_log(log) {
                return Ok(AuthEvents::AdminDeletedFilter(decoded));
            }
            if let Ok(decoded) = OperatorAddedFilter::decode_log(log) {
                return Ok(AuthEvents::OperatorAddedFilter(decoded));
            }
            if let Ok(decoded) = OperatorDeletedFilter::decode_log(log) {
                return Ok(AuthEvents::OperatorDeletedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AuthEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminDeletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorDeletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AdminAddedFilter> for AuthEvents {
        fn from(value: AdminAddedFilter) -> Self {
            Self::AdminAddedFilter(value)
        }
    }
    impl ::core::convert::From<AdminDeletedFilter> for AuthEvents {
        fn from(value: AdminDeletedFilter) -> Self {
            Self::AdminDeletedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorAddedFilter> for AuthEvents {
        fn from(value: OperatorAddedFilter) -> Self {
            Self::OperatorAddedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorDeletedFilter> for AuthEvents {
        fn from(value: OperatorDeletedFilter) -> Self {
            Self::OperatorDeletedFilter(value)
        }
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
    ///Container type for all input parameters for the `admins` function with signature `admins(address)` and selector `0x429b62e5`
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
    #[ethcall(name = "admins", abi = "admins(address)")]
    pub struct AdminsCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `operators` function with signature `operators(address)` and selector `0x13e7c9d8`
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
    #[ethcall(name = "operators", abi = "operators(address)")]
    pub struct OperatorsCall(pub ::ethers::core::types::Address);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AuthCalls {
        AddAdmin(AddAdminCall),
        AddOperator(AddOperatorCall),
        Admins(AdminsCall),
        DeleteAdmin(DeleteAdminCall),
        DeleteOperator(DeleteOperatorCall),
        Operators(OperatorsCall),
    }
    impl ::ethers::core::abi::AbiDecode for AuthCalls {
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
                = <AdminsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Admins(decoded));
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
                = <OperatorsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Operators(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AuthCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admins(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeleteAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeleteOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Operators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AuthCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admins(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Operators(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddAdminCall> for AuthCalls {
        fn from(value: AddAdminCall) -> Self {
            Self::AddAdmin(value)
        }
    }
    impl ::core::convert::From<AddOperatorCall> for AuthCalls {
        fn from(value: AddOperatorCall) -> Self {
            Self::AddOperator(value)
        }
    }
    impl ::core::convert::From<AdminsCall> for AuthCalls {
        fn from(value: AdminsCall) -> Self {
            Self::Admins(value)
        }
    }
    impl ::core::convert::From<DeleteAdminCall> for AuthCalls {
        fn from(value: DeleteAdminCall) -> Self {
            Self::DeleteAdmin(value)
        }
    }
    impl ::core::convert::From<DeleteOperatorCall> for AuthCalls {
        fn from(value: DeleteOperatorCall) -> Self {
            Self::DeleteOperator(value)
        }
    }
    impl ::core::convert::From<OperatorsCall> for AuthCalls {
        fn from(value: OperatorsCall) -> Self {
            Self::Operators(value)
        }
    }
    ///Container type for all return fields from the `admins` function with signature `admins(address)` and selector `0x429b62e5`
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
    pub struct AdminsReturn(pub bool);
    ///Container type for all return fields from the `operators` function with signature `operators(address)` and selector `0x13e7c9d8`
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
    pub struct OperatorsReturn(pub bool);
}
