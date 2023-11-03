pub use ssz_rs::serde::{as_hex, as_str, try_bytes_from_hex_str};

pub mod seq_of_str {
    use serde::{
        de::{Deserializer, Error},
        ser::SerializeSeq,
    };
    use std::{fmt, marker::PhantomData, str::FromStr};

    pub fn serialize<S, T, U>(data: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: AsRef<[U]>,
        U: fmt::Display,
    {
        let mut seq = serializer.serialize_seq(None)?;
        for elem in data.as_ref().iter() {
            let rendered_elem = format!("{elem}");
            seq.serialize_element(&rendered_elem)?;
        }
        seq.end()
    }

    struct Visitor<T>(PhantomData<Vec<T>>);

    impl<'de, T: FromStr> serde::de::Visitor<'de> for Visitor<T> {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("sequence of string")
        }

        fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            let mut coll = Vec::with_capacity(access.size_hint().unwrap_or(0));

            while let Some(elem) = access.next_element()? {
                let recovered_elem = T::from_str(elem).map_err(|_| {
                    Error::custom("failure to parse element of sequence from string")
                })?;
                coll.push(recovered_elem);
            }
            Ok(coll)
        }
    }

    pub fn deserialize<'de, D, T, U>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: TryFrom<Vec<U>>,
        U: FromStr,
    {
        let data = deserializer.deserialize_seq(Visitor(PhantomData))?;
        T::try_from(data).map_err(|_| serde::de::Error::custom("failure to parse collection"))
    }
}

#[cfg(test)]
mod tests {
    use crate::types::mainnet::SignedBeaconBlock;

    const EXPECTED_SIGNED_BLOCK_STR: &str = r#"
    { "message": {
        "slot": "0",
        "proposer_index": "0",
        "parent_root": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "state_root": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "body": {
          "randao_reveal": "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
          "eth1_data": {
            "deposit_root": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "deposit_count": "0",
            "block_hash": "0x0000000000000000000000000000000000000000000000000000000000000000"
          },
          "graffiti": "0x0000000000000000000000000000000000000000000000000000000000000000",
          "proposer_slashings": [],
          "attester_slashings": [],
          "attestations": [],
          "deposits": [],
          "voluntary_exits": [],
          "sync_aggregate": {
            "sync_committee_bits": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "sync_committee_signature": "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
          },
          "execution_payload": {
            "parent_hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "fee_recipient": "0x0000000000000000000000000000000000000000",
            "state_root": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "receipts_root": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "logs_bloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "prev_randao": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "block_number": "0",
            "gas_limit": "0",
            "gas_used": "0",
            "timestamp": "0",
            "extra_data": "0x",
            "base_fee_per_gas": "0",
            "block_hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "transactions": [],
            "withdrawals": [],
            "blob_gas_used": "0",
            "excess_blob_gas": "0"
          },
          "bls_to_execution_changes": [],
          "blob_kzg_commitments": []
        }
      },
      "signature": "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000" }
    "#;

    #[test]
    fn test_serde() {
        let signed_block = SignedBeaconBlock::Deneb(Default::default());
        let str = serde_json::to_string(&signed_block).unwrap();
        let expected_str =
            EXPECTED_SIGNED_BLOCK_STR.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        assert_eq!(str, expected_str);
        let recovered_signed_block: SignedBeaconBlock = serde_json::from_str(&str).unwrap();
        assert_eq!(signed_block, recovered_signed_block);
    }
}
