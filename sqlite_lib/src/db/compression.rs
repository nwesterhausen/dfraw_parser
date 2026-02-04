use std::{fmt::Debug, sync::Arc};
use zstd::{
    bulk::{Compressor, Decompressor},
    dict::{DecoderDictionary, EncoderDictionary, from_samples},
};

pub struct DbCodec {
    encoder_dict: Arc<EncoderDictionary<'static>>,
    decoder_dict: Arc<DecoderDictionary<'static>>,
    compression_level: i32,
}

impl DbCodec {
    pub fn new(dict_bytes: &[u8], level: i32) -> Self {
        Self {
            encoder_dict: Arc::new(EncoderDictionary::copy(dict_bytes, level)),
            decoder_dict: Arc::new(DecoderDictionary::copy(dict_bytes)),
            compression_level: level,
        }
    }

    /// Serialize with Ciborium and compress with Zstd Dictionary
    pub fn compress_record<T>(&self, data: &T) -> anyhow::Result<Vec<u8>>
    where
        T: serde::Serialize,
    {
        // Serialize to CBOR first
        // Optimization: You could reuse this buffer if DbCodec had a RefCell<Vec<u8>>
        let mut cbor_buffer = Vec::new();
        ciborium::into_writer(data, &mut cbor_buffer)?;

        // Setup Compressor with the shared dictionary
        let mut compressor = Compressor::with_prepared_dictionary(&self.encoder_dict)?;

        // Compress
        // "compress" automatically calculates the bound and allocates the Vec
        let compressed_buffer = compressor.compress(&cbor_buffer)?;

        Ok(compressed_buffer)
    }

    pub fn decompress_record<T>(&self, compressed_data: &[u8]) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut decompressor = Decompressor::with_prepared_dictionary(&self.decoder_dict)?;

        // Use .decompress() which returns a Vec<u8>.
        // We pass an initial capacity hint (e.g., 10x size), but Zstd will
        // automatically resize the vector if the data turns out to be larger.
        let capacity_hint = compressed_data.len() * 10;
        let cbor_buffer = decompressor.decompress(compressed_data, capacity_hint)?;

        let data = ciborium::from_reader(&cbor_buffer[..])?;
        Ok(data)
    }
}

/// Helper: Trains a dictionary from samples, limiting to 110KB
pub fn train_encoder_dictionary(samples: &[Vec<u8>]) -> anyhow::Result<Vec<u8>> {
    // 110KB is standard for Zstd dictionaries
    let dict = from_samples(samples, 112_640)?;
    Ok(dict)
}

/// Helper: Trains a dictionary directly from a list of serializable structs.
pub fn train_dictionary_from_objects<I, T>(objects: I) -> anyhow::Result<Vec<u8>>
where
    I: IntoIterator<Item = T>,
    T: serde::Serialize,
{
    const SAFETY_COUNT_LIMIT: usize = 100_000;
    // Stop if we have plenty of data (20MB) to keep training fast
    const TARGET_SAMPLE_SIZE: usize = 20 * 1024 * 1024;

    let mut samples = Vec::new();
    let mut total_size = 0;

    // Collect samples until we hit a reasonable limit (e.g., 20MB of sample data)
    // or the count limit. Zstd usually needs ~100x the dictionary size (11MB) for optimal results.
    for obj in objects.into_iter().take(SAFETY_COUNT_LIMIT) {
        let mut buffer = Vec::new();
        ciborium::into_writer(&obj, &mut buffer)?;
        total_size += buffer.len();
        samples.push(buffer);

        if total_size > TARGET_SAMPLE_SIZE {
            break;
        }
    }

    train_encoder_dictionary(&samples)
}

impl Debug for DbCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Codec for compression level {}", self.compression_level)
    }
}
