pub use deposit_contract::*;
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
pub mod deposit_contract {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("VALIDATOR_DEPOSIT_SIZE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "VALIDATOR_DEPOSIT_SIZE",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawal_credentials",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deposit_data_root"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("get_deposit_count"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get_deposit_count"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("get_deposit_root"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get_deposit_root"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DepositEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DepositEvent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawal_credentials",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
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
    pub static DEPOSITCONTRACT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`@`\x80\x81R4a\x01'W`\0h\x01\xBC\x16\xD6t\xEC\x80\0\0\x81U\x80[`\x1F\x81\x10a\x000W\x82Qa\x0Cc\x90\x81a\x01-\x829\xF3[` \x80\x82\x10\x15a\0\xCFW`\"\x82\x01T\x84Q\x82\x81\x01\x82\x90R\x80\x86\x01\x91\x90\x91R\x84\x81R``\x80\x82\x01\x91\x90`\x01`\x01`@\x1B\x03\x83\x11\x82\x84\x10\x17a\x01\x13W\x82\x87R\x81Q\x86[\x81\x81\x10a\x01\x01WP\x91\x84\x93\x91\x87\x80\x94\x83\x01\x91\x82\x01R\x03\x90`\x02Z\xFA\x15a\0\xF7W\x82Q\x90`\x01\x83\x01\x80\x84\x11a\0\xE3W\x10\x15a\0\xCFW`#\x82\x01U`\0\x19\x81\x14a\0\xBBW`\x01\x01a\0\x1AV[cNH{q`\xE0\x1B\x82R`\x11`\x04R`$\x82\xFD[cNH{q`\xE0\x1B\x83R`2`\x04R`$\x83\xFD[cNH{q`\xE0\x1B\x85R`\x11`\x04R`$\x85\xFD[\x83Q=\x84\x82>=\x90\xFD[\x83\x81\x01\x80\x87\x01Q\x90\x84\x01R\x85\x01a\0qV[cNH{q`\xE0\x1B\x86R`A`\x04R`$\x86\xFD[`\0\x80\xFD\xFE`\x80`@\x81\x81R`\x04\x91\x826\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x91\x82c\x01\xFF\xC9\xA7\x14a\x02\x88WP\x81c\"\x89Q\x18\x14a\x02\nWP\x80cUR\xAAe\x14a\x01\xEEW\x80cb\x1F\xD10\x14a\x01\xADWc\xC5\xF2\x89/\x14a\0\\W`\0\x80\xFD[4a\x01\xA9W\x81`\x03\x196\x01\x12a\x01\xA9W\x81`!T\x90\x81\x92\x84\x92[` \x86\x81\x86\x10\x15a\x01&W`\x01\x94\x83\x86\x16\x86\x03a\0\xEBWa\0\xBF\x90\x87\x87\x01T\x90\x86Q\x90\x85\x82\x01\x92\x83R\x87\x82\x01R\x86\x81Ra\0\xAF\x81a\x03\x7FV[\x86Q\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90`\x02Z\xFA\x15a\0\xE1Wa\0\xDB\x90\x86Q\x93[\x1C\x93a\x03ZV[\x92a\0vV[\x81Q=\x87\x82>=\x90\xFD[a\x01\r\x90\x87`\"\x01T\x86Q\x90\x85\x82\x01\x92\x83R\x87\x82\x01R\x86\x81Ra\0\xAF\x81a\x03\x7FV[\x81\x01\x03\x90`\x02Z\xFA\x15a\0\xE1Wa\0\xDB\x90\x86Q\x93a\0\xD4V[\x83` \x82a\x01\x87\x88a\x01Ag\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x16a\x0B/V[a\x01w`X\x87Q\x80\x93\x88\x82\x01\x95\x86Ra\x01b\x81Q\x80\x92\x8B\x8D\x86\x01\x91\x01a\x03\x12V[\x81\x01\x87\x8A\x82\x01R\x03`8\x81\x01\x84R\x01\x82a\x03\xB1V[\x85Q\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90`\x02Z\xFA\x15a\x01\x9FW` \x91Q\x90Q\x90\x81R\xF3[Q\x90=\x90\x82>=\x90\xFD[P\x80\xFD[P4a\x01\xA9W\x81`\x03\x196\x01\x12a\x01\xA9Wa\x01\xEA\x90a\x01\xD7g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`!T\x16a\x0B/V[\x90Q\x91\x82\x91` \x83R` \x83\x01\x90a\x035V[\x03\x90\xF3[P4a\x01\xA9W\x81`\x03\x196\x01\x12a\x01\xA9W` \x91T\x90Q\x90\x81R\xF3[\x83\x90`\x806`\x03\x19\x01\x12a\x01\xA9Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x815\x81\x81\x11a\x02\x84Wa\x027\x906\x90\x84\x01a\x02\xDFV[`$\x92\x91\x925\x82\x81\x11a\x02\x80Wa\x02Q\x906\x90\x86\x01a\x02\xDFV[\x90\x92`D5\x90\x81\x11a\x02|Wa\x02y\x95a\x02m\x916\x91\x01a\x02\xDFV[\x93\x90\x92`d5\x95a\x03\xF4V[\x80\xF3[\x86\x80\xFD[\x85\x80\xFD[\x83\x80\xFD[\x84\x914a\x02\xDBW` 6`\x03\x19\x01\x12a\x02\xDBW5c\xFF\xFF\xFF\xFF`\xE0\x1B\x81\x16\x80\x91\x03a\x02\xDBW` \x92Pc\x01\xFF\xC9\xA7`\xE0\x1B\x81\x14\x90\x81\x15a\x02\xCAW[P\x15\x15\x81R\xF3[c\x85d\t\x07`\xE0\x1B\x14\x90P\x83a\x02\xC3V[\x82\x80\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x03\rW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\rW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03\rWV[`\0\x80\xFD[`\0[\x83\x81\x10a\x03%WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x03\x15V[\x90` \x91a\x03N\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x03\x12V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\0\x19\x81\x14a\x03iW`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x9BW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x9BW`@RV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90\x92\x96\x93\x95\x94`0\x84\x03a\n\xDBW` \x96\x87\x89\x03a\npW``\x82\x03a\n\x19Wg\r\xE0\xB6\xB3\xA7d\0\x004\x10a\t\xC5Wc;\x9A\xCA\0\x804\x06a\tZW4\x04\x98g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x99\x8A\x81\x11a\t\x05W\x90\x8A\x8A\x96\x95\x94\x93\x92\x16a\x04V\x90a\x0B/V[\x92\x86`!T\x9C\x8D\x16a\x04g\x90a\x0B/V[\x98`@\x99\x8AQ\x80\x91`\xA0\x82R`\xA0\x82\x01a\x04\x82\x90\x85\x8Da\x03\xD3V[\x82\x81\x03\x86\x84\x01Ra\x04\x94\x90\x88\x8Ca\x03\xD3V[\x82\x81\x03\x83\x8F\x01Ra\x04\xA5\x90\x8Aa\x035V[\x82\x81\x03``\x84\x01Ra\x04\xB8\x90\x87\x8Aa\x03\xD3V[\x82\x81\x03`\x80\x84\x01Ra\x04\xC9\x91a\x035V[\x03\x7Fd\x9B\xBCb\xD0\xE3\x13B\xAF\xEAN\\\xD8-@I\xE7\xE1\xEE\x91/\xC0\x88\x9A\xA7\x90\x80;\xE3\x908\xC5\x91\xA1\x80\x8AQ\x9D\x8E\x01\x98\x897\x8C\x01\x8C`\0\x9D\x82\x8F\x85\x81\x95\x01R\x03`\x10\x81\x01\x82R`0\x01a\x05\x17\x90\x82a\x03\xB1V[\x8AQ\x98\x89\x91Q\x80\x9Aa\x05(\x92a\x03\x12V[\x80`\x02\x99\x81\x01\x03\x90\x89Z\xFA\x15a\x08\xEDW\x8AQ\x92\x81\x89\x11a\t\x01W\x87\x8Ca\x05s\x8BQ\x83\x81\x01\x90\x8D\x86\x837\x83``\x82\x01R\x8D\x81Ra\x05c\x81a\x03\x7FV[\x8DQ\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90\x8AZ\xFA\x15a\x08\xF7W\x8Ba\x05\xC4\x8A\x8A\x93a\x05\xB4\x84Q\x96\x83Q\x92\x87\x84\x01\x94`?\x19\x83\x01\x91\x01\x857\x82\x01\x82`\x1F\x19\x91\x87\x83\x82\x01R\x03\x90\x81\x01\x83R\x82a\x03\xB1V[\x8CQ\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90\x89Z\xFA\x15a\x08\xEDW\x8Aa\x06\x01\x88\x92\x82Q\x8BQ\x90\x85\x82\x01\x92\x83R\x8C\x82\x01R\x8B\x81Ra\x05\xF1\x81a\x03\x7FV[\x8BQ\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90\x88Z\xFA\x15a\x08\xE3Wa\x06O\x86\x92\x8B\x92a\x06?\x8A\x85Q\x98\x83\x82Q\x94\x85\x92\x8A\x84\x01\x97\x88R\x84\x84\x017\x81\x01\x87\x83\x82\x01R\x03\x87\x81\x01\x84R\x01\x82a\x03\xB1V[\x89Q\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90\x86Z\xFA\x15a\x08\xD9W\x87a\x06\xAC\x85\x92\x82Q\x94a\x06\x9C`X\x8AQ\x80\x93\x88a\x06\x81\x81\x84\x01\x97\x88\x81Q\x93\x84\x92\x01a\x03\x12V[\x82\x01\x90\x88\x8A\x83\x01R`8\x82\x01R\x03`8\x81\x01\x84R\x01\x82a\x03\xB1V[\x88Q\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90\x85Z\xFA\x15a\x08\xCFW\x86a\x06\xE9\x84\x92\x82Q\x87Q\x90\x85\x82\x01\x92\x83R\x88\x82\x01R\x87\x81Ra\x06\xD9\x81a\x03\x7FV[\x87Q\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90\x84Z\xFA\x15a\x08\xC5W\x85Q\x94\x85\x03a\x086Wc\xFF\xFF\xFF\xFF\x87\x10\x15a\x07\xE9W`\x01\x94\x85\x88\x01\x80\x98\x11a\x07\xD5W\x87\x86\x95\x98`!U\x90\x87\x95[a\x07;W[cNH{q`\xE0\x1B\x88R`\x04\x87\x90R`$\x88\xFD[\x83\x86\x99\x98\x99\x10\x15\x80a\x07\xCCW\x87\x80\x84\x16\x14a\x07\xBCWa\x07\xA8W\x88a\x07x\x85\x92\x88\x8A\x01T\x90\x88Q\x90\x85\x82\x01\x92\x83R\x89\x82\x01R\x88\x81Ra\x06\x9C\x81a\x03\x7FV[\x81\x01\x03\x90\x85Z\xFA\x15a\x07\x9EW\x85\x96a\x07\x94\x89Q\x92\x88\x1C\x96a\x03ZV[\x95\x91\x90\x98\x97a\x07\"V[\x83Q=\x89\x82>=\x90\xFD[cNH{q`\xE0\x1B\x89R`2`\x04R`$\x89\xFD[P\x96\x97\x93PPPP\x91\x90\x91P\x01UV[P\x88\x97Pa\x07'V[cNH{q`\xE0\x1B\x87R`\x11`\x04R`$\x87\xFD[P`\x84\x91Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`!`$\x82\x01R\x7FDepositContract: merkle tree ful`D\x82\x01R`\x1B`\xFA\x1B`d\x82\x01R\xFD[P`\xA4\x91Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`T`$\x82\x01R\x7FDepositContract: reconstructed D`D\x82\x01R\x7FepositData does not match suppli`d\x82\x01R\x7Fed deposit_data_root\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R\xFD[\x82Q=\x87\x82>=\x90\xFD[\x83Q=\x88\x82>=\x90\xFD[\x84Q=\x89\x82>=\x90\xFD[\x86Q=\x8B\x82>=\x90\xFD[\x87Q=\x8C\x82>=\x90\xFD[\x88Q=\x8D\x82>=\x90\xFD[\x8B\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x8B\x90R`'`$\x82\x01R\x7FDepositContract: deposit value t`D\x82\x01Rf\r\xED\xE4\r\r,\xED`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x8A\x90R`3`$\x82\x01R\x7FDepositContract: deposit value n`D\x82\x01R\x7Fot multiple of gwei\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x89\x90R`&`$\x82\x01R\x7FDepositContract: deposit value t`D\x82\x01Reoo low`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x89\x90R`)`$\x82\x01R\x7FDepositContract: invalid signatu`D\x82\x01Rh\x0EL\xA4\r\x8C\xAD\xCC\xEE\x8D`\xBB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x89\x90R`6`$\x82\x01R\x7FDepositContract: invalid withdra`D\x82\x01R\x7Fwal_credentials length\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FDepositContract: invalid pubkey `D\x82\x01Re\r\x8C\xAD\xCC\xEE\x8D`\xD3\x1B`d\x82\x01R`\x84\x90\xFD[\x90`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x9BW`@R`\x08\x81R` \x81\x01` 6\x827\x81\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xC0\x1B\x90`\xC0\x1B\x16\x90\x82Q\x15a\x0C\x17W\x81`\x07\x1A\x90S\x81Q`\x01\x10\x15a\x0C\x17W\x80`\x06\x1A`!\x83\x01S\x81Q`\x02\x10\x15a\x0C\x17W\x80`\x05\x1A`\"\x83\x01S\x81Q`\x03\x10\x15a\x0C\x17W`\x04\x81\x81\x1A`#\x84\x01S\x82Q\x81\x10\x15a\x0C\x02W\x81`\x03\x1A`$\x84\x01S\x82Q`\x05\x10\x15a\x0C\x02W\x81`\x02\x1A`%\x84\x01S\x82Q`\x06\x10\x15a\x0C\x02W\x81`\x01\x1A`&\x84\x01S\x82Q`\x07\x10\x15a\x0C\x02WP`\0\x1A\x90`'\x01SV[`2\x90cNH{q`\xE0\x1B`\0RR`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 X\x1CM\xD4T7\xEC\xB5\x7F\xEFjg\x8F\xA1\x8F:\x84%\r`\xFF!A-\xB0\x8C+\xB4\xC1uPkdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static DEPOSITCONTRACT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x91\x826\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x91\x82c\x01\xFF\xC9\xA7\x14a\x02\x88WP\x81c\"\x89Q\x18\x14a\x02\nWP\x80cUR\xAAe\x14a\x01\xEEW\x80cb\x1F\xD10\x14a\x01\xADWc\xC5\xF2\x89/\x14a\0\\W`\0\x80\xFD[4a\x01\xA9W\x81`\x03\x196\x01\x12a\x01\xA9W\x81`!T\x90\x81\x92\x84\x92[` \x86\x81\x86\x10\x15a\x01&W`\x01\x94\x83\x86\x16\x86\x03a\0\xEBWa\0\xBF\x90\x87\x87\x01T\x90\x86Q\x90\x85\x82\x01\x92\x83R\x87\x82\x01R\x86\x81Ra\0\xAF\x81a\x03\x7FV[\x86Q\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90`\x02Z\xFA\x15a\0\xE1Wa\0\xDB\x90\x86Q\x93[\x1C\x93a\x03ZV[\x92a\0vV[\x81Q=\x87\x82>=\x90\xFD[a\x01\r\x90\x87`\"\x01T\x86Q\x90\x85\x82\x01\x92\x83R\x87\x82\x01R\x86\x81Ra\0\xAF\x81a\x03\x7FV[\x81\x01\x03\x90`\x02Z\xFA\x15a\0\xE1Wa\0\xDB\x90\x86Q\x93a\0\xD4V[\x83` \x82a\x01\x87\x88a\x01Ag\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x16a\x0B/V[a\x01w`X\x87Q\x80\x93\x88\x82\x01\x95\x86Ra\x01b\x81Q\x80\x92\x8B\x8D\x86\x01\x91\x01a\x03\x12V[\x81\x01\x87\x8A\x82\x01R\x03`8\x81\x01\x84R\x01\x82a\x03\xB1V[\x85Q\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90`\x02Z\xFA\x15a\x01\x9FW` \x91Q\x90Q\x90\x81R\xF3[Q\x90=\x90\x82>=\x90\xFD[P\x80\xFD[P4a\x01\xA9W\x81`\x03\x196\x01\x12a\x01\xA9Wa\x01\xEA\x90a\x01\xD7g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`!T\x16a\x0B/V[\x90Q\x91\x82\x91` \x83R` \x83\x01\x90a\x035V[\x03\x90\xF3[P4a\x01\xA9W\x81`\x03\x196\x01\x12a\x01\xA9W` \x91T\x90Q\x90\x81R\xF3[\x83\x90`\x806`\x03\x19\x01\x12a\x01\xA9Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x815\x81\x81\x11a\x02\x84Wa\x027\x906\x90\x84\x01a\x02\xDFV[`$\x92\x91\x925\x82\x81\x11a\x02\x80Wa\x02Q\x906\x90\x86\x01a\x02\xDFV[\x90\x92`D5\x90\x81\x11a\x02|Wa\x02y\x95a\x02m\x916\x91\x01a\x02\xDFV[\x93\x90\x92`d5\x95a\x03\xF4V[\x80\xF3[\x86\x80\xFD[\x85\x80\xFD[\x83\x80\xFD[\x84\x914a\x02\xDBW` 6`\x03\x19\x01\x12a\x02\xDBW5c\xFF\xFF\xFF\xFF`\xE0\x1B\x81\x16\x80\x91\x03a\x02\xDBW` \x92Pc\x01\xFF\xC9\xA7`\xE0\x1B\x81\x14\x90\x81\x15a\x02\xCAW[P\x15\x15\x81R\xF3[c\x85d\t\x07`\xE0\x1B\x14\x90P\x83a\x02\xC3V[\x82\x80\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x03\rW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03\rW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03\rWV[`\0\x80\xFD[`\0[\x83\x81\x10a\x03%WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x03\x15V[\x90` \x91a\x03N\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x03\x12V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\0\x19\x81\x14a\x03iW`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x9BW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x9BW`@RV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90\x92\x96\x93\x95\x94`0\x84\x03a\n\xDBW` \x96\x87\x89\x03a\npW``\x82\x03a\n\x19Wg\r\xE0\xB6\xB3\xA7d\0\x004\x10a\t\xC5Wc;\x9A\xCA\0\x804\x06a\tZW4\x04\x98g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x99\x8A\x81\x11a\t\x05W\x90\x8A\x8A\x96\x95\x94\x93\x92\x16a\x04V\x90a\x0B/V[\x92\x86`!T\x9C\x8D\x16a\x04g\x90a\x0B/V[\x98`@\x99\x8AQ\x80\x91`\xA0\x82R`\xA0\x82\x01a\x04\x82\x90\x85\x8Da\x03\xD3V[\x82\x81\x03\x86\x84\x01Ra\x04\x94\x90\x88\x8Ca\x03\xD3V[\x82\x81\x03\x83\x8F\x01Ra\x04\xA5\x90\x8Aa\x035V[\x82\x81\x03``\x84\x01Ra\x04\xB8\x90\x87\x8Aa\x03\xD3V[\x82\x81\x03`\x80\x84\x01Ra\x04\xC9\x91a\x035V[\x03\x7Fd\x9B\xBCb\xD0\xE3\x13B\xAF\xEAN\\\xD8-@I\xE7\xE1\xEE\x91/\xC0\x88\x9A\xA7\x90\x80;\xE3\x908\xC5\x91\xA1\x80\x8AQ\x9D\x8E\x01\x98\x897\x8C\x01\x8C`\0\x9D\x82\x8F\x85\x81\x95\x01R\x03`\x10\x81\x01\x82R`0\x01a\x05\x17\x90\x82a\x03\xB1V[\x8AQ\x98\x89\x91Q\x80\x9Aa\x05(\x92a\x03\x12V[\x80`\x02\x99\x81\x01\x03\x90\x89Z\xFA\x15a\x08\xEDW\x8AQ\x92\x81\x89\x11a\t\x01W\x87\x8Ca\x05s\x8BQ\x83\x81\x01\x90\x8D\x86\x837\x83``\x82\x01R\x8D\x81Ra\x05c\x81a\x03\x7FV[\x8DQ\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90\x8AZ\xFA\x15a\x08\xF7W\x8Ba\x05\xC4\x8A\x8A\x93a\x05\xB4\x84Q\x96\x83Q\x92\x87\x84\x01\x94`?\x19\x83\x01\x91\x01\x857\x82\x01\x82`\x1F\x19\x91\x87\x83\x82\x01R\x03\x90\x81\x01\x83R\x82a\x03\xB1V[\x8CQ\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90\x89Z\xFA\x15a\x08\xEDW\x8Aa\x06\x01\x88\x92\x82Q\x8BQ\x90\x85\x82\x01\x92\x83R\x8C\x82\x01R\x8B\x81Ra\x05\xF1\x81a\x03\x7FV[\x8BQ\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90\x88Z\xFA\x15a\x08\xE3Wa\x06O\x86\x92\x8B\x92a\x06?\x8A\x85Q\x98\x83\x82Q\x94\x85\x92\x8A\x84\x01\x97\x88R\x84\x84\x017\x81\x01\x87\x83\x82\x01R\x03\x87\x81\x01\x84R\x01\x82a\x03\xB1V[\x89Q\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90\x86Z\xFA\x15a\x08\xD9W\x87a\x06\xAC\x85\x92\x82Q\x94a\x06\x9C`X\x8AQ\x80\x93\x88a\x06\x81\x81\x84\x01\x97\x88\x81Q\x93\x84\x92\x01a\x03\x12V[\x82\x01\x90\x88\x8A\x83\x01R`8\x82\x01R\x03`8\x81\x01\x84R\x01\x82a\x03\xB1V[\x88Q\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90\x85Z\xFA\x15a\x08\xCFW\x86a\x06\xE9\x84\x92\x82Q\x87Q\x90\x85\x82\x01\x92\x83R\x88\x82\x01R\x87\x81Ra\x06\xD9\x81a\x03\x7FV[\x87Q\x92\x83\x92\x83\x92Q\x92\x83\x91a\x03\x12V[\x81\x01\x03\x90\x84Z\xFA\x15a\x08\xC5W\x85Q\x94\x85\x03a\x086Wc\xFF\xFF\xFF\xFF\x87\x10\x15a\x07\xE9W`\x01\x94\x85\x88\x01\x80\x98\x11a\x07\xD5W\x87\x86\x95\x98`!U\x90\x87\x95[a\x07;W[cNH{q`\xE0\x1B\x88R`\x04\x87\x90R`$\x88\xFD[\x83\x86\x99\x98\x99\x10\x15\x80a\x07\xCCW\x87\x80\x84\x16\x14a\x07\xBCWa\x07\xA8W\x88a\x07x\x85\x92\x88\x8A\x01T\x90\x88Q\x90\x85\x82\x01\x92\x83R\x89\x82\x01R\x88\x81Ra\x06\x9C\x81a\x03\x7FV[\x81\x01\x03\x90\x85Z\xFA\x15a\x07\x9EW\x85\x96a\x07\x94\x89Q\x92\x88\x1C\x96a\x03ZV[\x95\x91\x90\x98\x97a\x07\"V[\x83Q=\x89\x82>=\x90\xFD[cNH{q`\xE0\x1B\x89R`2`\x04R`$\x89\xFD[P\x96\x97\x93PPPP\x91\x90\x91P\x01UV[P\x88\x97Pa\x07'V[cNH{q`\xE0\x1B\x87R`\x11`\x04R`$\x87\xFD[P`\x84\x91Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`!`$\x82\x01R\x7FDepositContract: merkle tree ful`D\x82\x01R`\x1B`\xFA\x1B`d\x82\x01R\xFD[P`\xA4\x91Q\x90bF\x1B\xCD`\xE5\x1B\x82R`\x04\x82\x01R`T`$\x82\x01R\x7FDepositContract: reconstructed D`D\x82\x01R\x7FepositData does not match suppli`d\x82\x01R\x7Fed deposit_data_root\0\0\0\0\0\0\0\0\0\0\0\0`\x84\x82\x01R\xFD[\x82Q=\x87\x82>=\x90\xFD[\x83Q=\x88\x82>=\x90\xFD[\x84Q=\x89\x82>=\x90\xFD[\x86Q=\x8B\x82>=\x90\xFD[\x87Q=\x8C\x82>=\x90\xFD[\x88Q=\x8D\x82>=\x90\xFD[\x8B\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x8B\x90R`'`$\x82\x01R\x7FDepositContract: deposit value t`D\x82\x01Rf\r\xED\xE4\r\r,\xED`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x8A\x90R`3`$\x82\x01R\x7FDepositContract: deposit value n`D\x82\x01R\x7Fot multiple of gwei\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x89\x90R`&`$\x82\x01R\x7FDepositContract: deposit value t`D\x82\x01Reoo low`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x89\x90R`)`$\x82\x01R\x7FDepositContract: invalid signatu`D\x82\x01Rh\x0EL\xA4\r\x8C\xAD\xCC\xEE\x8D`\xBB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x89\x90R`6`$\x82\x01R\x7FDepositContract: invalid withdra`D\x82\x01R\x7Fwal_credentials length\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FDepositContract: invalid pubkey `D\x82\x01Re\r\x8C\xAD\xCC\xEE\x8D`\xD3\x1B`d\x82\x01R`\x84\x90\xFD[\x90`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\x9BW`@R`\x08\x81R` \x81\x01` 6\x827\x81\x93g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xC0\x1B\x90`\xC0\x1B\x16\x90\x82Q\x15a\x0C\x17W\x81`\x07\x1A\x90S\x81Q`\x01\x10\x15a\x0C\x17W\x80`\x06\x1A`!\x83\x01S\x81Q`\x02\x10\x15a\x0C\x17W\x80`\x05\x1A`\"\x83\x01S\x81Q`\x03\x10\x15a\x0C\x17W`\x04\x81\x81\x1A`#\x84\x01S\x82Q\x81\x10\x15a\x0C\x02W\x81`\x03\x1A`$\x84\x01S\x82Q`\x05\x10\x15a\x0C\x02W\x81`\x02\x1A`%\x84\x01S\x82Q`\x06\x10\x15a\x0C\x02W\x81`\x01\x1A`&\x84\x01S\x82Q`\x07\x10\x15a\x0C\x02WP`\0\x1A\x90`'\x01SV[`2\x90cNH{q`\xE0\x1B`\0RR`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 X\x1CM\xD4T7\xEC\xB5\x7F\xEFjg\x8F\xA1\x8F:\x84%\r`\xFF!A-\xB0\x8C+\xB4\xC1uPkdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static DEPOSITCONTRACT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DepositContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DepositContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DepositContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DepositContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DepositContract<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DepositContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DepositContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEPOSITCONTRACT_ABI.clone(),
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
                DEPOSITCONTRACT_ABI.clone(),
                DEPOSITCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `VALIDATOR_DEPOSIT_SIZE` (0x5552aa65) function
        pub fn validator_deposit_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([85, 82, 170, 101], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x22895118) function
        pub fn deposit(
            &self,
            pubkey: ::ethers::core::types::Bytes,
            withdrawal_credentials: ::ethers::core::types::Bytes,
            signature: ::ethers::core::types::Bytes,
            deposit_data_root: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [34, 137, 81, 24],
                    (pubkey, withdrawal_credentials, signature, deposit_data_root),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_deposit_count` (0x621fd130) function
        pub fn get_deposit_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([98, 31, 209, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_deposit_root` (0xc5f2892f) function
        pub fn get_deposit_root(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([197, 242, 137, 47], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DepositEvent` event
        pub fn deposit_event_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DepositEventFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DepositEventFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DepositContract<M> {
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
    #[ethevent(
        name = "DepositEvent",
        abi = "DepositEvent(bytes,bytes,bytes,bytes,bytes)"
    )]
    pub struct DepositEventFilter {
        pub pubkey: ::ethers::core::types::Bytes,
        pub withdrawal_credentials: ::ethers::core::types::Bytes,
        pub amount: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
        pub index: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `VALIDATOR_DEPOSIT_SIZE` function with signature `VALIDATOR_DEPOSIT_SIZE()` and selector `0x5552aa65`
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
    #[ethcall(name = "VALIDATOR_DEPOSIT_SIZE", abi = "VALIDATOR_DEPOSIT_SIZE()")]
    pub struct ValidatorDepositSizeCall;
    ///Container type for all input parameters for the `deposit` function with signature `deposit(bytes,bytes,bytes,bytes32)` and selector `0x22895118`
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
    #[ethcall(name = "deposit", abi = "deposit(bytes,bytes,bytes,bytes32)")]
    pub struct DepositCall {
        pub pubkey: ::ethers::core::types::Bytes,
        pub withdrawal_credentials: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
        pub deposit_data_root: [u8; 32],
    }
    ///Container type for all input parameters for the `get_deposit_count` function with signature `get_deposit_count()` and selector `0x621fd130`
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
    #[ethcall(name = "get_deposit_count", abi = "get_deposit_count()")]
    pub struct GetDepositCountCall;
    ///Container type for all input parameters for the `get_deposit_root` function with signature `get_deposit_root()` and selector `0xc5f2892f`
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
    #[ethcall(name = "get_deposit_root", abi = "get_deposit_root()")]
    pub struct GetDepositRootCall;
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DepositContractCalls {
        ValidatorDepositSize(ValidatorDepositSizeCall),
        Deposit(DepositCall),
        GetDepositCount(GetDepositCountCall),
        GetDepositRoot(GetDepositRootCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for DepositContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ValidatorDepositSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ValidatorDepositSize(decoded));
            }
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <GetDepositCountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDepositCount(decoded));
            }
            if let Ok(decoded)
                = <GetDepositRootCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDepositRoot(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DepositContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ValidatorDepositSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDepositCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDepositRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DepositContractCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ValidatorDepositSize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDepositCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDepositRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ValidatorDepositSizeCall> for DepositContractCalls {
        fn from(value: ValidatorDepositSizeCall) -> Self {
            Self::ValidatorDepositSize(value)
        }
    }
    impl ::core::convert::From<DepositCall> for DepositContractCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<GetDepositCountCall> for DepositContractCalls {
        fn from(value: GetDepositCountCall) -> Self {
            Self::GetDepositCount(value)
        }
    }
    impl ::core::convert::From<GetDepositRootCall> for DepositContractCalls {
        fn from(value: GetDepositRootCall) -> Self {
            Self::GetDepositRoot(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for DepositContractCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    ///Container type for all return fields from the `VALIDATOR_DEPOSIT_SIZE` function with signature `VALIDATOR_DEPOSIT_SIZE()` and selector `0x5552aa65`
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
    pub struct ValidatorDepositSizeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `get_deposit_count` function with signature `get_deposit_count()` and selector `0x621fd130`
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
    pub struct GetDepositCountReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `get_deposit_root` function with signature `get_deposit_root()` and selector `0xc5f2892f`
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
    pub struct GetDepositRootReturn(pub [u8; 32]);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
}
