use crate::Num;
use bincode::{
    de::{BorrowDecoder, Decoder},
    enc::Encoder,
    error::{DecodeError, EncodeError},
    BorrowDecode, Decode, Encode,
};

impl<const N: i64> Encode for Num<N> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&N, encoder)?;
        Ok(())
    }
}

impl<const N: i64, Context> Decode<Context> for Num<N> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let n: i64 = bincode::Decode::<Context>::decode(decoder)?;
        if n != N {
            return Err(DecodeError::OtherString(format!("not {N}")));
        }
        Ok(Default::default())
    }
}

impl<'de, const N: i64, Context> BorrowDecode<'de, Context> for Num<N> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let n: i64 = bincode::BorrowDecode::<Context>::borrow_decode(decoder)?;
        if n != N {
            return Err(DecodeError::OtherString(format!("not {N}")));
        }
        Ok(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::Num;
    use bincode::{Decode as BincodeDecode, Encode as BincodeEncode};

    #[test]
    fn test_bincode_typed_num() {
        #[derive(Debug, PartialEq, Eq, BincodeEncode, BincodeDecode)]
        struct BincodeConfig<const V: i64> {
            version: Num<V>,
            data: String,
        }

        const TEST_VERSION: i64 = 42;
        const ANOTHER_VERSION: i64 = 43;

        let config_correct_version = BincodeConfig::<TEST_VERSION> {
            version: Num,
            data: "test_data".to_string(),
        };

        let bincode_config = bincode::config::standard();

        let encoded_vec = bincode::encode_to_vec(&config_correct_version, bincode_config).unwrap();

        let (decoded_correct, len_correct): (BincodeConfig<TEST_VERSION>, usize) =
            bincode::decode_from_slice(&encoded_vec, bincode_config).unwrap();
        assert_eq!(decoded_correct, config_correct_version);
        assert_eq!(len_correct, encoded_vec.len());

        let decode_result_wrong_version: Result<(BincodeConfig<ANOTHER_VERSION>, usize), _> =
            bincode::decode_from_slice(&encoded_vec, bincode_config);

        assert!(decode_result_wrong_version.is_err());
        if let Err(e) = decode_result_wrong_version {
            match e {
                bincode::error::DecodeError::OtherString(s) => {
                    assert_eq!(s, format!("not {}", ANOTHER_VERSION));
                }
                _ => panic!("Expected OtherString error, got {:?}", e),
            }
        }
    }

    #[test]
    fn test_bincode_typed_num_negative() {
        #[derive(Debug, PartialEq, Eq, BincodeEncode, BincodeDecode)]
        struct BincodeConfigNegative<const V: i64> {
            version: Num<V>,
            payload: Vec<u8>,
        }

        const TEST_NEGATIVE_VERSION: i64 = -5;
        const ANOTHER_NEGATIVE_VERSION: i64 = -6;

        let config_correct_negative_version = BincodeConfigNegative::<TEST_NEGATIVE_VERSION> {
            version: Num,
            payload: vec![1, 2, 3],
        };

        let bincode_config = bincode::config::standard();

        let encoded_vec_negative =
            bincode::encode_to_vec(&config_correct_negative_version, bincode_config).unwrap();

        let (decoded_correct_negative, len_correct_negative): (
            BincodeConfigNegative<TEST_NEGATIVE_VERSION>,
            usize,
        ) = bincode::decode_from_slice(&encoded_vec_negative, bincode_config).unwrap();
        assert_eq!(decoded_correct_negative, config_correct_negative_version);
        assert_eq!(len_correct_negative, encoded_vec_negative.len());

        let decode_result_wrong_negative_version: Result<
            (BincodeConfigNegative<ANOTHER_NEGATIVE_VERSION>, usize),
            _,
        > = bincode::decode_from_slice(&encoded_vec_negative, bincode_config);

        assert!(decode_result_wrong_negative_version.is_err());
        if let Err(e) = decode_result_wrong_negative_version {
            match e {
                bincode::error::DecodeError::OtherString(s) => {
                    assert_eq!(s, format!("not {}", ANOTHER_NEGATIVE_VERSION));
                }
                _ => panic!("Expected OtherString error, got {:?}", e),
            }
        }
    }
    #[test]
    fn test_bincode_typed_num_i64_min() {
        #[derive(Debug, PartialEq, Eq, BincodeEncode, BincodeDecode)]
        struct BincodeConfigMin {
            version: Num<{ i64::MIN }>,
            data: String,
        }

        #[derive(Debug, PartialEq, Eq, BincodeEncode, BincodeDecode)]
        struct BincodeConfigMinMinusOne {
            version: Num<{ i64::MIN + 1 }>,
            data: String,
        }

        let config_min = BincodeConfigMin {
            version: Num,
            data: "test_data_min".to_string(),
        };

        let bincode_config = bincode::config::standard();

        let encoded_vec_min = bincode::encode_to_vec(&config_min, bincode_config).unwrap();

        let (decoded_min, len_min): (BincodeConfigMin, usize) =
            bincode::decode_from_slice(&encoded_vec_min, bincode_config).unwrap();
        assert_eq!(decoded_min, config_min);
        assert_eq!(len_min, encoded_vec_min.len());

        let config_min_minus_one = BincodeConfigMinMinusOne {
            version: Num,
            data: "test_data_min_minus_one".to_string(),
        };

        let encoded_vec_min_minus_one =
            bincode::encode_to_vec(&config_min_minus_one, bincode_config).unwrap();

        let decode_result_min_minus_one: Result<(BincodeConfigMin, usize), _> =
            bincode::decode_from_slice(&encoded_vec_min_minus_one, bincode_config);

        assert!(decode_result_min_minus_one.is_err());
        if let Err(e) = decode_result_min_minus_one {
            match e {
                bincode::error::DecodeError::OtherString(s) => {
                    assert_eq!(s, format!("not {}", i64::MIN));
                }
                _ => panic!("Expected OtherString error, got {:?}", e),
            }
        }
    }

    #[test]
    fn test_bincode_typed_num_i64_max() {
        #[derive(Debug, PartialEq, Eq, BincodeEncode, BincodeDecode)]
        struct BincodeConfigMax {
            version: Num<{ i64::MAX }>,
            data: String,
        }

        #[derive(Debug, PartialEq, Eq, BincodeEncode, BincodeDecode)]
        struct BincodeConfigMaxMinusOne {
            version: Num<{ i64::MAX - 1 }>,
            data: String,
        }

        let config_max = BincodeConfigMax {
            version: Num,
            data: "test_data_max".to_string(),
        };

        let bincode_config = bincode::config::standard();

        let encoded_vec_max = bincode::encode_to_vec(&config_max, bincode_config).unwrap();

        let (decoded_max, len_max): (BincodeConfigMax, usize) =
            bincode::decode_from_slice(&encoded_vec_max, bincode_config).unwrap();
        assert_eq!(decoded_max, config_max);
        assert_eq!(len_max, encoded_vec_max.len());

        let config_max_minus_one = BincodeConfigMaxMinusOne {
            version: Num,
            data: "test_data_max_minus_one".to_string(),
        };

        let encoded_vec_max_minus_one =
            bincode::encode_to_vec(&config_max_minus_one, bincode_config).unwrap();

        let decode_result_max_minus_one: Result<(BincodeConfigMax, usize), _> =
            bincode::decode_from_slice(&encoded_vec_max_minus_one, bincode_config);

        assert!(decode_result_max_minus_one.is_err());
        if let Err(e) = decode_result_max_minus_one {
            match e {
                bincode::error::DecodeError::OtherString(s) => {
                    assert_eq!(s, format!("not {}", i64::MAX));
                }
                _ => panic!("Expected OtherString error, got {:?}", e),
            }
        }
    }
}
