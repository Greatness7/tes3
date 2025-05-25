use crate::prelude::*;

pub mod base64_bytes {
    use super::*;

    const BASE64: base64_simd::Base64 = base64_simd::STANDARD;

    pub fn serialize<T, S>(data: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Save,
        S: serde::Serializer,
    {
        let mut stream = Writer::new(vec![]);
        if stream.save(data).is_err() {
            return Err(serde::ser::Error::custom("esp serialize save error"));
        }

        #[allow(unused_mut)]
        let mut bytes = stream.cursor.into_inner();

        #[cfg(feature = "zstd")]
        {
            bytes = zstd::encode_all(&*bytes, 0).unwrap();
        }

        let encoded = BASE64.encode_to_string(bytes);

        serializer.serialize_str(&encoded)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        T: Load,
        D: serde::Deserializer<'de>,
    {
        let encoded: String = serde::Deserialize::deserialize(deserializer)?;

        #[allow(unused_mut)]
        let Ok(mut decoded) = BASE64.decode_to_vec(encoded.as_bytes()) else {
            return Err(serde::de::Error::custom("esp deserialize decode error"));
        };

        #[cfg(feature = "zstd")]
        {
            let Ok(decompressed) = zstd::stream::decode_all(&*decoded) else {
                return Err(serde::de::Error::custom("esp deserialize decompress error"));
            };
            decoded = decompressed;
        }

        let mut stream = Reader::new(&decoded);
        let Ok(data) = stream.load() else {
            return Err(serde::de::Error::custom("esp deserialize load error"));
        };

        Ok(data)
    }
}

pub mod cell_references {
    use super::*;

    type T = HashMap<(u32, u32), Reference>;

    pub fn serialize<S>(data: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut references: Vec<_> = data.values().collect();

        references.sort_by_key(|reference| {
            (
                !reference.persistent(),
                match reference.mast_index {
                    0 => u32::MAX,
                    i => i,
                },
                reference.refr_index,
            )
        });

        serializer.collect_seq(references)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let references: Vec<Reference> = serde::Deserialize::deserialize(deserializer)?;

        let hashmap = references
            .into_iter()
            .map(|reference| ((reference.mast_index, reference.refr_index), reference))
            .collect();

        Ok(hashmap)
    }
}
