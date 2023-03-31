/// base64 encode + zstd compression
pub mod base64_zstd_compress {
    use crate::prelude::*;

    pub fn serialize<T, S>(data: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Save,
        S: serde::Serializer,
    {
        let mut stream = Writer::new(vec![]);
        if stream.save(data).is_err() {
            return Err(serde::ser::Error::custom("esp serialize error"));
        }

        let bytes = stream.cursor.into_inner();
        let compressed = zstd::encode_all(&*bytes, 0).unwrap();

        let base64 = base64_simd::STANDARD;
        let encoded = base64.encode_to_string(&compressed);

        serializer.serialize_str(&encoded)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        T: Load,
        D: serde::Deserializer<'de>,
    {
        let encoded: String = serde::Deserialize::deserialize(deserializer)?;

        let base64 = base64_simd::STANDARD;
        let compressed = match base64.decode_to_vec(encoded.as_bytes()) {
            Ok(v) => v,
            _ => return Err(serde::de::Error::custom("esp deserialize decode error")),
        };

        let decompressed = match zstd::stream::decode_all(&*compressed) {
            Ok(v) => v,
            _ => return Err(serde::de::Error::custom("esp deserialize decompress error")),
        };

        let mut stream = Reader::new(&decompressed);
        let value = match stream.load() {
            Ok(v) => v,
            _ => return Err(serde::de::Error::custom("esp deserialize load error")),
        };

        Ok(value)
    }
}
