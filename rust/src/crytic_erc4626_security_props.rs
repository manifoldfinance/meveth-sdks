pub use crytic_erc4626_security_props::*;
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
pub mod crytic_erc4626_security_props {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "verify_assetDecimalsLessThanVault",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_assetDecimalsLessThanVault",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verify_sharePriceInflationAttack"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_sharePriceInflationAttack",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inflateAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delta"),
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
    pub static CRYTICERC4626SECURITYPROPS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x0B\x8E\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x9B0<\xCD\x14a\x02\xEEWPc\xA2\x05\x82\xE9\x14a\0:W`\0\x80\xFD[4a\x02\xEAW\x82`\x03\x196\x01\x12a\x02\xEAW`\x01`\x01`\xA0\x1B\x03\x83\x81`\x01T\x16\x91\x83Q\x90c1<\xE5g`\xE0\x1B\x93\x84\x83R\x82\x87\x81` \x96\x87\x94Z\xF1\x91\x82\x15a\x02\xE0W\x90\x83\x91\x88\x93a\x02\xA1W[P\x87T\x16\x93\x86\x86Q\x80\x96\x81\x93\x82RZ\xFA\x92\x83\x15a\x02\x97W\x86\x93a\x02bW[P`\xFF\x16\x90\x83Q\x90`\xA0\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02MW\x85R`o\x82R\x80\x82\x01\x92\x7FThe vault's share token should h\x84R\x7Fave greater than or equal to the\x86\x84\x01R\x7F number of decimals as the vault``\x84\x01R\x7F's asset token.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x84\x01R\x84\x81\x10a\x01iW\x87\x80\xF3[\x94a\x023`;`$\x99\x98\x96\x7F\x94BN\xD2O\xB3\x968\xB6H\x17\xC77\xDDD?8z\xAA\x14\x86aM\xA4I\xB6hjd-ml\x96a\x02=\x96a\x02$a\x01\xFC\x97a\x01\xB5a\x01\xAF`\x01\x9Ea\x0B\x19V[\x95a\x0B\x19V[\x90\x88Q\x99\x8A\x96h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x83\x89\x01Ra\x01\xDF\x81Q\x80\x92\x85`)\x8C\x01\x91\x01a\n\xCAV[\x87\x01\x91`\x0F`\xFA\x1B`)\x84\x01R\x83Q\x93\x84\x91`*\x85\x01\x91\x01a\n\xCAV[\x01\x91p\x01\x033\x0BKc+!a\x03\x93+\x0B\x9B{q\xD1`}\x1B`*\x84\x01RQ\x80\x93\x86\x84\x01\x90a\n\xCAV[\x01\x03`\x1B\x81\x01\x85R\x01\x83a\n\xA8V[Q\x91\x82\x91\x82a\n\xEDV[\x03\x90\xA1cNH{q`\xE0\x1B\x83RR\xFD[`A\x87cNH{q`\xE0\x1B`\0RR`$`\0\xFD[\x90\x92P\x81\x81\x81=\x83\x11a\x02\x90W[a\x02z\x81\x83a\n\xA8V[\x81\x01\x03\x12a\x02\x8BWQ\x91`\xFFa\0\xA1V[`\0\x80\xFD[P=a\x02pV[\x84Q=\x88\x82>=\x90\xFD[\x82\x81\x93\x92\x94P=\x83\x11a\x02\xD9W[a\x02\xB9\x81\x83a\n\xA8V[\x81\x01\x03\x12a\x02\xD5WQ`\xFF\x81\x16\x81\x03a\x02\xD5W\x82\x90\x918a\0\x83V[\x86\x80\xFD[P=a\x02\xAFV[\x85Q=\x89\x82>=\x90\xFD[\x82\x80\xFD[\x84\x91P\x834a\x02\xEAW\x83`\x03\x196\x01\x12a\x02\xEAW\x805\x93`\x01`\x01`\xA0\x1B\x03\x92`\x01\x93\x80\x85T\x16\x91bxtE`\xE2\x1B\x92\x83\x82R` \x91\x82\x81\x88\x81\x85Z\xFA\x90\x81\x15a\ntW\x89\x91a\nGW[Pa\t\x16W\x81\x86\x91\x86Q\x92\x83\x80\x92c\x18\x16\r\xDD`\xE0\x1B\x82RZ\xFA\x90\x81\x15a\t\x0CW\x88\x91a\n\x1AW[Pa\x02\xD5Wa'\x10\x88\x11\x15a\x02\xD5W`$\x97\x885\x81\x01\x93\x84\x82\x11a\n\x08W\x83\x89T\x16\x80;\x15a\n\x04W\x86Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x81\x8A\x01\x90\x81R` \x81\x01\x85\x90R\x90\x91\x8B\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x08\xB3Wa\t\xF1W[P\x83\x89T\x16\x84\x89T\x16\x90\x80;\x15a\t\xEDW\x8B`d\x8A\x8D\x80\x94\x8CQ\x96\x87\x95\x86\x94cQ\xBB\x10\xCF`\xE0\x1B\x86R0\x90\x86\x01R\x84\x01R\x88`D\x84\x01RZ\xF1\x80\x15a\x08\xB3W\x90\x8A\x91a\t\xD9W[P\x83\x89`D\x8A\x8E\x94\x89\x84T\x16\x8CQ\x96\x87\x95\x86\x94cnU?e`\xE0\x1B\x86R\x85\x01R0\x90\x84\x01RZ\xF1\x90\x81\x15a\x08\xB3W\x90\x89\x91\x8B\x91a\t\xA8W[P\x03a\tsW\x83\x88T\x16\x90\x86Q\x90\x81R\x83\x81\x89\x81\x85Z\xFA\x90\x81\x15a\x08\xB3W\x90\x89\x91\x8B\x91a\twW[P\x03a\tsW\x83\x89T\x16\x90`\0\x19\x83\x01\x92\x83\x11a\taW\x86Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81\x89\x01\x90\x81R` \x81\x01\x93\x90\x93R\x91\x83\x91\x83\x91\x82\x90\x8C\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\t\x0CWa\t.W[P\x81`\x02T\x16\x80;\x15a\t\x16W\x87\x80\x91\x8A\x87Q\x80\x94\x81\x93c\xCA\x1D \x9D`\xE0\x1B\x83R\x89\x8C\x84\x01RZ\xF1\x80\x15a\t\x0CW\x90\x88\x91a\t\x1AW[PP\x81`\x02T\x16\x80;\x15a\t\x16W\x87\x80\x91\x87\x87Q\x80\x94\x81\x93c\x89\xC6\xC0\x9B`\xE0\x1B\x83RZ\xF1\x80\x15a\t\x0CW\x90\x88\x91a\x08\xF8W[PP\x83Q\x90\x84\x82R`\x1A\x85\x83\x01R\x87``\x93\x7FAmount of alice's deposit:\0\0\0\0\0\0\x85\x85\x01R\x85\x83\x85\x01R\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x90\x81`\x80\x80\x96\xA1\x83\x87\x8D\x8B\x84`\x02T\x16\x96\x8CQ\x97\x88\x94\x85\x93c;vYM`\xE0\x1B\x85R\x84\x01RZ\xF1\x92\x83\x15a\x08\xEEW\x90\x84\x91\x8C\x94a\x08\xBDW[P\x82\x86\x8AQ\x8B\x81R`\r\x8C\x82\x01Rl \xB64\xB1\xB2\x90)\xB40\xB92\xB9\x9D`\x99\x1B\x8A\x82\x01R\x86\x85\x82\x01R\xA1`\x02T\x16\x92\x8C\x8A\x8D\x8BQ\x96\x87\x94\x85\x93c+n\xA6\x01`\xE2\x1B\x85R\x84\x01RZ\xF1\x91\x82\x15a\x08\xB3W\x8A\x92a\x08\x84W[P\x80\x84\x88Q\x89\x81R\x85\x8A\x82\x01R\x7FAmount of tokens alice withdrew:\x88\x82\x01R\x84\x86\x82\x01R\xA1\x81\x86\x03\x86\x81\x11a\x08pW\x84\x82\x91\x89Q\x90\x8A\x82R`\x0B\x8B\x83\x01Rj \xB64\xB1\xB2\x90&7\xB9\xB9\x9D`\xA9\x1B\x89\x83\x01R\x86\x82\x01R\xA1g\r\xDD)5\x02\x9D\x80\0\x95\x86\x81\x02\x90\x80\x82\x04\x88\x14\x90\x15\x17\x15a\x08^W\x84g\r\xE0\xB6\xB3\xA7d\0\0\x83\x92\x04\x97\x89Q\x90\x8A\x82R`\r\x8B\x83\x01Rl\x1B\x1B\xDC\xDC\xD5\x1A\x1C\x99\\\xDA\x1B\xDB\x19`\x9A\x1B\x89\x83\x01R\x86\x82\x01R\xA1\x83\x87Q\x88\x81R`\x15\x89\x82\x01R\x7FminRedeemedAmountNorm\0\0\0\0\0\0\0\0\0\0\0\x87\x82\x01R\x87\x85\x82\x01R\xA1\x85Q\x92\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08JW\x86R`J\x83RiThreshold%`\xB0\x1B\x82\x84\x01\x94\x7FShare inflation attack possible,\x86R\x7F victim lost an amount over loss\x88\x86\x01R\x84\x01R\x84\x81\x11\x15a\x07\x8BW\x88\x80\xF3[a\x02=\x94a\x08\x13\x94\x7Fp{\x8CV\xE4\xC2\x11\xCF\x13!\xFA\xEBAH#pb\"\x8D\xB2\xFC\xEC\xC9\xBEH~\x83\xA2h\x0E~P\x97\x94a\x08;a\x023\x95a\x07\xCBa\x01\xAF`<\x97a\x0B\x19V[\x90\x88Q\x99\x8A\x96h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x83\x89\x01Ra\x07\xF5\x81Q\x80\x92\x85`)\x8C\x01\x91\x01a\n\xCAV[\x87\x01\x91a<=`\xF0\x1B`)\x84\x01R\x83Q\x93\x84\x91`+\x85\x01\x91\x01a\n\xCAV[\x01\x91p\x01\x033\x0BKc+!a\x03\x93+\x0B\x9B{q\xD1`}\x1B`+\x84\x01RQ\x80\x93\x86\x84\x01\x90a\n\xCAV[\x01\x03`\x1C\x81\x01\x85R\x01\x83a\n\xA8V[\x8A`A\x89cNH{q`\xE0\x1B`\0RR`\0\xFD[cNH{q`\xE0\x1B\x8BR`\x11\x89R\x8B\x8B\xFD[\x8B`\x11\x8AcNH{q`\xE0\x1B`\0RR`\0\xFD[\x90\x91P\x82\x81\x81=\x83\x11a\x08\xACW[a\x08\x9C\x81\x83a\n\xA8V[\x81\x01\x03\x12a\x02\x8BWQ\x90\x8Ba\x06\x15V[P=a\x08\x92V[\x87Q=\x8C\x82>=\x90\xFD[\x82\x81\x93\x92\x95P=\x83\x11a\x08\xE7W[a\x08\xD5\x81\x83a\n\xA8V[\x81\x01\x03\x12a\x02\x8BW\x83\x90Q\x92\x8Da\x05\xC0V[P=a\x08\xCBV[\x88Q=\x8D\x82>=\x90\xFD[a\t\x01\x90a\n~V[a\x02\xD5W\x86\x89a\x05+V[\x85Q=\x8A\x82>=\x90\xFD[\x87\x80\xFD[a\t#\x90a\n~V[a\x02\xD5W\x86\x89a\x04\xF9V[\x81\x81\x81=\x83\x11a\tZW[a\tC\x81\x83a\n\xA8V[\x81\x01\x03\x12a\t\x16WQ\x80\x15\x15\x03a\x02\xD5W\x88a\x04\xC3V[P=a\t9V[cNH{q`\xE0\x1B\x8AR`\x11\x88R\x8A\x8A\xFD[\x88\x80\xFD[\x80\x92P\x85\x80\x92P=\x83\x11a\t\xA1W[a\t\x90\x81\x83a\n\xA8V[\x81\x01\x03\x12a\x02\x8BW\x88\x90Q\x8Ca\x04jV[P=a\t\x86V[\x80\x92P\x85\x80\x92P=\x83\x11a\t\xD2W[a\t\xC1\x81\x83a\n\xA8V[\x81\x01\x03\x12a\x02\x8BW\x88\x90Q\x8Ca\x04BV[P=a\t\xB7V[a\t\xE2\x90a\n~V[a\tsW\x88\x8Ba\x04\nV[\x8A\x80\xFD[a\t\xFD\x90\x99\x91\x99a\n~V[\x97\x8Aa\x03\xC3V[\x89\x80\xFD[cNH{q`\xE0\x1B\x89R`\x11\x87R\x89\x89\xFD[\x90P\x81\x81\x81=\x83\x11a\n@W[a\n1\x81\x83a\n\xA8V[\x81\x01\x03\x12a\t\x16WQ\x89a\x03aV[P=a\n'V[\x90P\x82\x81\x81=\x83\x11a\nmW[a\n^\x81\x83a\n\xA8V[\x81\x01\x03\x12a\tsWQ\x8Aa\x039V[P=a\nTV[\x86Q=\x8B\x82>=\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\x92W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\x92W`@RV[`\0[\x83\x81\x10a\n\xDDWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\n\xCDV[`@\x91` \x82Ra\x0B\r\x81Q\x80\x92\x81` \x86\x01R` \x86\x86\x01\x91\x01a\n\xCAV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\x0B/W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \x07W\xF4*2\xD4\xDCm}\xAC]\xE3\0\x9E\xA5\xBA\x84\x1E'L\xC0\xCD\x1B\xD8{u[:d>\x02\xC2dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static CRYTICERC4626SECURITYPROPS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x90\x816\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x9B0<\xCD\x14a\x02\xEEWPc\xA2\x05\x82\xE9\x14a\0:W`\0\x80\xFD[4a\x02\xEAW\x82`\x03\x196\x01\x12a\x02\xEAW`\x01`\x01`\xA0\x1B\x03\x83\x81`\x01T\x16\x91\x83Q\x90c1<\xE5g`\xE0\x1B\x93\x84\x83R\x82\x87\x81` \x96\x87\x94Z\xF1\x91\x82\x15a\x02\xE0W\x90\x83\x91\x88\x93a\x02\xA1W[P\x87T\x16\x93\x86\x86Q\x80\x96\x81\x93\x82RZ\xFA\x92\x83\x15a\x02\x97W\x86\x93a\x02bW[P`\xFF\x16\x90\x83Q\x90`\xA0\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02MW\x85R`o\x82R\x80\x82\x01\x92\x7FThe vault's share token should h\x84R\x7Fave greater than or equal to the\x86\x84\x01R\x7F number of decimals as the vault``\x84\x01R\x7F's asset token.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x84\x01R\x84\x81\x10a\x01iW\x87\x80\xF3[\x94a\x023`;`$\x99\x98\x96\x7F\x94BN\xD2O\xB3\x968\xB6H\x17\xC77\xDDD?8z\xAA\x14\x86aM\xA4I\xB6hjd-ml\x96a\x02=\x96a\x02$a\x01\xFC\x97a\x01\xB5a\x01\xAF`\x01\x9Ea\x0B\x19V[\x95a\x0B\x19V[\x90\x88Q\x99\x8A\x96h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x83\x89\x01Ra\x01\xDF\x81Q\x80\x92\x85`)\x8C\x01\x91\x01a\n\xCAV[\x87\x01\x91`\x0F`\xFA\x1B`)\x84\x01R\x83Q\x93\x84\x91`*\x85\x01\x91\x01a\n\xCAV[\x01\x91p\x01\x033\x0BKc+!a\x03\x93+\x0B\x9B{q\xD1`}\x1B`*\x84\x01RQ\x80\x93\x86\x84\x01\x90a\n\xCAV[\x01\x03`\x1B\x81\x01\x85R\x01\x83a\n\xA8V[Q\x91\x82\x91\x82a\n\xEDV[\x03\x90\xA1cNH{q`\xE0\x1B\x83RR\xFD[`A\x87cNH{q`\xE0\x1B`\0RR`$`\0\xFD[\x90\x92P\x81\x81\x81=\x83\x11a\x02\x90W[a\x02z\x81\x83a\n\xA8V[\x81\x01\x03\x12a\x02\x8BWQ\x91`\xFFa\0\xA1V[`\0\x80\xFD[P=a\x02pV[\x84Q=\x88\x82>=\x90\xFD[\x82\x81\x93\x92\x94P=\x83\x11a\x02\xD9W[a\x02\xB9\x81\x83a\n\xA8V[\x81\x01\x03\x12a\x02\xD5WQ`\xFF\x81\x16\x81\x03a\x02\xD5W\x82\x90\x918a\0\x83V[\x86\x80\xFD[P=a\x02\xAFV[\x85Q=\x89\x82>=\x90\xFD[\x82\x80\xFD[\x84\x91P\x834a\x02\xEAW\x83`\x03\x196\x01\x12a\x02\xEAW\x805\x93`\x01`\x01`\xA0\x1B\x03\x92`\x01\x93\x80\x85T\x16\x91bxtE`\xE2\x1B\x92\x83\x82R` \x91\x82\x81\x88\x81\x85Z\xFA\x90\x81\x15a\ntW\x89\x91a\nGW[Pa\t\x16W\x81\x86\x91\x86Q\x92\x83\x80\x92c\x18\x16\r\xDD`\xE0\x1B\x82RZ\xFA\x90\x81\x15a\t\x0CW\x88\x91a\n\x1AW[Pa\x02\xD5Wa'\x10\x88\x11\x15a\x02\xD5W`$\x97\x885\x81\x01\x93\x84\x82\x11a\n\x08W\x83\x89T\x16\x80;\x15a\n\x04W\x86Qc@\xC1\x0F\x19`\xE0\x1B\x81R0\x81\x8A\x01\x90\x81R` \x81\x01\x85\x90R\x90\x91\x8B\x91\x83\x91\x90\x82\x90\x84\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\x08\xB3Wa\t\xF1W[P\x83\x89T\x16\x84\x89T\x16\x90\x80;\x15a\t\xEDW\x8B`d\x8A\x8D\x80\x94\x8CQ\x96\x87\x95\x86\x94cQ\xBB\x10\xCF`\xE0\x1B\x86R0\x90\x86\x01R\x84\x01R\x88`D\x84\x01RZ\xF1\x80\x15a\x08\xB3W\x90\x8A\x91a\t\xD9W[P\x83\x89`D\x8A\x8E\x94\x89\x84T\x16\x8CQ\x96\x87\x95\x86\x94cnU?e`\xE0\x1B\x86R\x85\x01R0\x90\x84\x01RZ\xF1\x90\x81\x15a\x08\xB3W\x90\x89\x91\x8B\x91a\t\xA8W[P\x03a\tsW\x83\x88T\x16\x90\x86Q\x90\x81R\x83\x81\x89\x81\x85Z\xFA\x90\x81\x15a\x08\xB3W\x90\x89\x91\x8B\x91a\twW[P\x03a\tsW\x83\x89T\x16\x90`\0\x19\x83\x01\x92\x83\x11a\taW\x86Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81\x89\x01\x90\x81R` \x81\x01\x93\x90\x93R\x91\x83\x91\x83\x91\x82\x90\x8C\x90\x82\x90`@\x01\x03\x92Z\xF1\x80\x15a\t\x0CWa\t.W[P\x81`\x02T\x16\x80;\x15a\t\x16W\x87\x80\x91\x8A\x87Q\x80\x94\x81\x93c\xCA\x1D \x9D`\xE0\x1B\x83R\x89\x8C\x84\x01RZ\xF1\x80\x15a\t\x0CW\x90\x88\x91a\t\x1AW[PP\x81`\x02T\x16\x80;\x15a\t\x16W\x87\x80\x91\x87\x87Q\x80\x94\x81\x93c\x89\xC6\xC0\x9B`\xE0\x1B\x83RZ\xF1\x80\x15a\t\x0CW\x90\x88\x91a\x08\xF8W[PP\x83Q\x90\x84\x82R`\x1A\x85\x83\x01R\x87``\x93\x7FAmount of alice's deposit:\0\0\0\0\0\0\x85\x85\x01R\x85\x83\x85\x01R\x7F1\xC3i\xD7\x02\x9A\xFB\xA3K!6\x9B\xCF\x9Aj\xC12\xFB&!\xC3EX\xB9\x14\x85\x9Bv\x8D\x05#-\x90\x81`\x80\x80\x96\xA1\x83\x87\x8D\x8B\x84`\x02T\x16\x96\x8CQ\x97\x88\x94\x85\x93c;vYM`\xE0\x1B\x85R\x84\x01RZ\xF1\x92\x83\x15a\x08\xEEW\x90\x84\x91\x8C\x94a\x08\xBDW[P\x82\x86\x8AQ\x8B\x81R`\r\x8C\x82\x01Rl \xB64\xB1\xB2\x90)\xB40\xB92\xB9\x9D`\x99\x1B\x8A\x82\x01R\x86\x85\x82\x01R\xA1`\x02T\x16\x92\x8C\x8A\x8D\x8BQ\x96\x87\x94\x85\x93c+n\xA6\x01`\xE2\x1B\x85R\x84\x01RZ\xF1\x91\x82\x15a\x08\xB3W\x8A\x92a\x08\x84W[P\x80\x84\x88Q\x89\x81R\x85\x8A\x82\x01R\x7FAmount of tokens alice withdrew:\x88\x82\x01R\x84\x86\x82\x01R\xA1\x81\x86\x03\x86\x81\x11a\x08pW\x84\x82\x91\x89Q\x90\x8A\x82R`\x0B\x8B\x83\x01Rj \xB64\xB1\xB2\x90&7\xB9\xB9\x9D`\xA9\x1B\x89\x83\x01R\x86\x82\x01R\xA1g\r\xDD)5\x02\x9D\x80\0\x95\x86\x81\x02\x90\x80\x82\x04\x88\x14\x90\x15\x17\x15a\x08^W\x84g\r\xE0\xB6\xB3\xA7d\0\0\x83\x92\x04\x97\x89Q\x90\x8A\x82R`\r\x8B\x83\x01Rl\x1B\x1B\xDC\xDC\xD5\x1A\x1C\x99\\\xDA\x1B\xDB\x19`\x9A\x1B\x89\x83\x01R\x86\x82\x01R\xA1\x83\x87Q\x88\x81R`\x15\x89\x82\x01R\x7FminRedeemedAmountNorm\0\0\0\0\0\0\0\0\0\0\0\x87\x82\x01R\x87\x85\x82\x01R\xA1\x85Q\x92\x83\x01\x83\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08JW\x86R`J\x83RiThreshold%`\xB0\x1B\x82\x84\x01\x94\x7FShare inflation attack possible,\x86R\x7F victim lost an amount over loss\x88\x86\x01R\x84\x01R\x84\x81\x11\x15a\x07\x8BW\x88\x80\xF3[a\x02=\x94a\x08\x13\x94\x7Fp{\x8CV\xE4\xC2\x11\xCF\x13!\xFA\xEBAH#pb\"\x8D\xB2\xFC\xEC\xC9\xBEH~\x83\xA2h\x0E~P\x97\x94a\x08;a\x023\x95a\x07\xCBa\x01\xAF`<\x97a\x0B\x19V[\x90\x88Q\x99\x8A\x96h\x02Ks\xB3\x0BcK!\xD1`\xBD\x1B\x83\x89\x01Ra\x07\xF5\x81Q\x80\x92\x85`)\x8C\x01\x91\x01a\n\xCAV[\x87\x01\x91a<=`\xF0\x1B`)\x84\x01R\x83Q\x93\x84\x91`+\x85\x01\x91\x01a\n\xCAV[\x01\x91p\x01\x033\x0BKc+!a\x03\x93+\x0B\x9B{q\xD1`}\x1B`+\x84\x01RQ\x80\x93\x86\x84\x01\x90a\n\xCAV[\x01\x03`\x1C\x81\x01\x85R\x01\x83a\n\xA8V[\x8A`A\x89cNH{q`\xE0\x1B`\0RR`\0\xFD[cNH{q`\xE0\x1B\x8BR`\x11\x89R\x8B\x8B\xFD[\x8B`\x11\x8AcNH{q`\xE0\x1B`\0RR`\0\xFD[\x90\x91P\x82\x81\x81=\x83\x11a\x08\xACW[a\x08\x9C\x81\x83a\n\xA8V[\x81\x01\x03\x12a\x02\x8BWQ\x90\x8Ba\x06\x15V[P=a\x08\x92V[\x87Q=\x8C\x82>=\x90\xFD[\x82\x81\x93\x92\x95P=\x83\x11a\x08\xE7W[a\x08\xD5\x81\x83a\n\xA8V[\x81\x01\x03\x12a\x02\x8BW\x83\x90Q\x92\x8Da\x05\xC0V[P=a\x08\xCBV[\x88Q=\x8D\x82>=\x90\xFD[a\t\x01\x90a\n~V[a\x02\xD5W\x86\x89a\x05+V[\x85Q=\x8A\x82>=\x90\xFD[\x87\x80\xFD[a\t#\x90a\n~V[a\x02\xD5W\x86\x89a\x04\xF9V[\x81\x81\x81=\x83\x11a\tZW[a\tC\x81\x83a\n\xA8V[\x81\x01\x03\x12a\t\x16WQ\x80\x15\x15\x03a\x02\xD5W\x88a\x04\xC3V[P=a\t9V[cNH{q`\xE0\x1B\x8AR`\x11\x88R\x8A\x8A\xFD[\x88\x80\xFD[\x80\x92P\x85\x80\x92P=\x83\x11a\t\xA1W[a\t\x90\x81\x83a\n\xA8V[\x81\x01\x03\x12a\x02\x8BW\x88\x90Q\x8Ca\x04jV[P=a\t\x86V[\x80\x92P\x85\x80\x92P=\x83\x11a\t\xD2W[a\t\xC1\x81\x83a\n\xA8V[\x81\x01\x03\x12a\x02\x8BW\x88\x90Q\x8Ca\x04BV[P=a\t\xB7V[a\t\xE2\x90a\n~V[a\tsW\x88\x8Ba\x04\nV[\x8A\x80\xFD[a\t\xFD\x90\x99\x91\x99a\n~V[\x97\x8Aa\x03\xC3V[\x89\x80\xFD[cNH{q`\xE0\x1B\x89R`\x11\x87R\x89\x89\xFD[\x90P\x81\x81\x81=\x83\x11a\n@W[a\n1\x81\x83a\n\xA8V[\x81\x01\x03\x12a\t\x16WQ\x89a\x03aV[P=a\n'V[\x90P\x82\x81\x81=\x83\x11a\nmW[a\n^\x81\x83a\n\xA8V[\x81\x01\x03\x12a\tsWQ\x8Aa\x039V[P=a\nTV[\x86Q=\x8B\x82>=\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\n\x92W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\n\x92W`@RV[`\0[\x83\x81\x10a\n\xDDWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\n\xCDV[`@\x91` \x82Ra\x0B\r\x81Q\x80\x92\x81` \x86\x01R` \x86\x86\x01\x91\x01a\n\xCAV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90`@Q`\xA0\x81\x01`@R`\x80\x81\x01\x92`\0\x84R\x92[`\0\x19\x01\x92`\n\x90`0\x82\x82\x06\x01\x85S\x04\x92\x83a\x0B/W\x80\x93P`\x80\x91\x03\x01\x91`\x1F\x19\x01\x91\x82RV\xFE\xA2dipfsX\"\x12 \x07W\xF4*2\xD4\xDCm}\xAC]\xE3\0\x9E\xA5\xBA\x84\x1E'L\xC0\xCD\x1B\xD8{u[:d>\x02\xC2dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static CRYTICERC4626SECURITYPROPS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CryticERC4626SecurityProps<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CryticERC4626SecurityProps<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CryticERC4626SecurityProps<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CryticERC4626SecurityProps<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CryticERC4626SecurityProps<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CryticERC4626SecurityProps))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CryticERC4626SecurityProps<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CRYTICERC4626SECURITYPROPS_ABI.clone(),
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
                CRYTICERC4626SECURITYPROPS_ABI.clone(),
                CRYTICERC4626SECURITYPROPS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `verify_assetDecimalsLessThanVault` (0xa20582e9) function
        pub fn verify_asset_decimals_less_than_vault(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 5, 130, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_sharePriceInflationAttack` (0x9b303ccd) function
        pub fn verify_share_price_inflation_attack(
            &self,
            inflate_amount: ::ethers::core::types::U256,
            delta: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 48, 60, 205], (inflate_amount, delta))
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
            CryticERC4626SecurityPropsEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CryticERC4626SecurityProps<M> {
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
    pub enum CryticERC4626SecurityPropsEvents {
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
    impl ::ethers::contract::EthLogDecode for CryticERC4626SecurityPropsEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssertEqFailFilter::decode_log(log) {
                return Ok(CryticERC4626SecurityPropsEvents::AssertEqFailFilter(decoded));
            }
            if let Ok(decoded) = AssertFailFilter::decode_log(log) {
                return Ok(CryticERC4626SecurityPropsEvents::AssertFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGtFailFilter::decode_log(log) {
                return Ok(CryticERC4626SecurityPropsEvents::AssertGtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertGteFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626SecurityPropsEvents::AssertGteFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertLtFailFilter::decode_log(log) {
                return Ok(CryticERC4626SecurityPropsEvents::AssertLtFailFilter(decoded));
            }
            if let Ok(decoded) = AssertLteFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626SecurityPropsEvents::AssertLteFailFilter(decoded),
                );
            }
            if let Ok(decoded) = AssertNeqFailFilter::decode_log(log) {
                return Ok(
                    CryticERC4626SecurityPropsEvents::AssertNeqFailFilter(decoded),
                );
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(CryticERC4626SecurityPropsEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(CryticERC4626SecurityPropsEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUint256Filter::decode_log(log) {
                return Ok(CryticERC4626SecurityPropsEvents::LogUint256Filter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CryticERC4626SecurityPropsEvents {
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
    impl ::core::convert::From<AssertEqFailFilter> for CryticERC4626SecurityPropsEvents {
        fn from(value: AssertEqFailFilter) -> Self {
            Self::AssertEqFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertFailFilter> for CryticERC4626SecurityPropsEvents {
        fn from(value: AssertFailFilter) -> Self {
            Self::AssertFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGtFailFilter> for CryticERC4626SecurityPropsEvents {
        fn from(value: AssertGtFailFilter) -> Self {
            Self::AssertGtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertGteFailFilter>
    for CryticERC4626SecurityPropsEvents {
        fn from(value: AssertGteFailFilter) -> Self {
            Self::AssertGteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLtFailFilter> for CryticERC4626SecurityPropsEvents {
        fn from(value: AssertLtFailFilter) -> Self {
            Self::AssertLtFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertLteFailFilter>
    for CryticERC4626SecurityPropsEvents {
        fn from(value: AssertLteFailFilter) -> Self {
            Self::AssertLteFailFilter(value)
        }
    }
    impl ::core::convert::From<AssertNeqFailFilter>
    for CryticERC4626SecurityPropsEvents {
        fn from(value: AssertNeqFailFilter) -> Self {
            Self::AssertNeqFailFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for CryticERC4626SecurityPropsEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for CryticERC4626SecurityPropsEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUint256Filter> for CryticERC4626SecurityPropsEvents {
        fn from(value: LogUint256Filter) -> Self {
            Self::LogUint256Filter(value)
        }
    }
    ///Container type for all input parameters for the `verify_assetDecimalsLessThanVault` function with signature `verify_assetDecimalsLessThanVault()` and selector `0xa20582e9`
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
        name = "verify_assetDecimalsLessThanVault",
        abi = "verify_assetDecimalsLessThanVault()"
    )]
    pub struct VerifyAssetDecimalsLessThanVaultCall;
    ///Container type for all input parameters for the `verify_sharePriceInflationAttack` function with signature `verify_sharePriceInflationAttack(uint256,uint256)` and selector `0x9b303ccd`
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
        name = "verify_sharePriceInflationAttack",
        abi = "verify_sharePriceInflationAttack(uint256,uint256)"
    )]
    pub struct VerifySharePriceInflationAttackCall {
        pub inflate_amount: ::ethers::core::types::U256,
        pub delta: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CryticERC4626SecurityPropsCalls {
        VerifyAssetDecimalsLessThanVault(VerifyAssetDecimalsLessThanVaultCall),
        VerifySharePriceInflationAttack(VerifySharePriceInflationAttackCall),
    }
    impl ::ethers::core::abi::AbiDecode for CryticERC4626SecurityPropsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <VerifyAssetDecimalsLessThanVaultCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifyAssetDecimalsLessThanVault(decoded));
            }
            if let Ok(decoded)
                = <VerifySharePriceInflationAttackCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifySharePriceInflationAttack(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CryticERC4626SecurityPropsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::VerifyAssetDecimalsLessThanVault(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifySharePriceInflationAttack(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CryticERC4626SecurityPropsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::VerifyAssetDecimalsLessThanVault(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifySharePriceInflationAttack(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<VerifyAssetDecimalsLessThanVaultCall>
    for CryticERC4626SecurityPropsCalls {
        fn from(value: VerifyAssetDecimalsLessThanVaultCall) -> Self {
            Self::VerifyAssetDecimalsLessThanVault(value)
        }
    }
    impl ::core::convert::From<VerifySharePriceInflationAttackCall>
    for CryticERC4626SecurityPropsCalls {
        fn from(value: VerifySharePriceInflationAttackCall) -> Self {
            Self::VerifySharePriceInflationAttack(value)
        }
    }
}
