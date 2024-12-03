use aws_sdk_s3::{config::{BehaviorVersion, Credentials, Region}, primitives::ByteStream, Client, Config};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    io::{self, Read, Write},
};
use tokio;
use ulid::Ulid;

/// Represents a record in the WAL. ULID and checksum are useful and provide a predictable amount
/// of metadata overhead (48 bytes metadata: 16 bytes for ULID, 32 for checksum).
#[derive(Debug)]
pub struct Record {
    pub ulid: Ulid,         // ULID as the unique identifier
    pub data: Vec<u8>,      // Data payload
    pub checksum: [u8; 32], // SHA-256 checkum for integrity
}

impl Record {
    /// Calculates the checksum for the record.
    fn calculate_checksum(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut checkum = [0u8; 32];
        checkum.copy_from_slice(&result);
        checkum
    }

    /// Creates a new record with a ULID and calculated checksum.
    pub fn new(data: Vec<u8>) -> Self {
        let ulid = Ulid::new();
        let checksum = Self::calculate_checksum(&data);
        Self {
            ulid,
            data,
            checksum,
        }
    }

    /// Validates the record's checksum.
    pub fn validate_checksum(&self) -> bool {
        self.checksum == Self::calculate_checksum(&self.data)
    }

    /// Serializes the record into a byte buffer (excluding the ULID).
    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        // We create a `buf` with the appropriate capacity by extending the `Record`'s `data` field
        // length by 32, which is the length of our checksum field.
        let mut buf = Vec::with_capacity(self.data.len() + 32);
        buf.write_all(&self.data)?; // Write `data` content
        buf.write_all(&self.checksum)?; // Write `checksum`
        Ok(buf)
    }

    /// Deserializes a record from a byte buffer.
    pub fn from_bytes(ulid: Ulid, mut bytes: &[u8]) -> io::Result<Self> {
        let data_len = bytes.len() - 32; // Remember: buf = data.len() + 32
        let mut data = vec![0u8; data_len];
        bytes.read_exact(&mut data)?;

        let mut checksum = [0u8; 32];
        bytes.read_exact(&mut checksum)?;

        Ok(Self {
            ulid,
            data,
            checksum,
        })
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.ulid == other.ulid
    }
}

impl Eq for Record {}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.ulid.cmp(&other.ulid))
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ulid.cmp(&other.ulid)
    }
}

pub struct MinioWAL {
    client: Client,
    bucket: String,
}

impl MinioWAL {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let config = Config::builder()
            .region(Region::new("us-east-1"))
            .endpoint_url("http://127.0.0.1:9000")
            .credentials_provider(Credentials::new(
                "admin", 
                "password", 
                None, 
                None, 
                "static"
            ))
            .behavior_version(BehaviorVersion::latest())
            .build();

        let client = Client::from_conf(config);

        Ok(Self { client, bucket: "gulog-dev".to_string() })
    }
}

trait WAL {
    /// Appends data to the log, returning the ULID of the written record.
    async fn append(&mut self, data: Vec<u8>) -> Result<String, Box<dyn Error + Send + Sync>>;

    /// Reads a record from the log using the ULID.
    async fn read(&self, key: String) -> Result<Record, Box<dyn Error + Send + Sync>>;
}

impl WAL for MinioWAL {
    async fn append(&mut self, data: Vec<u8>) -> Result<String, Box<dyn Error + Send + Sync>> {
        let record = Record::new(data);
        let ulid = record.ulid.to_string();
        let body = ByteStream::from(record.to_bytes()?);

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&ulid)
            .body(body)
            .send()
            .await?;

        Ok(ulid)
    }

    async fn read(&self, key: String) -> Result<Record, Box<dyn Error + Send + Sync>> {
        let response = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await?;

        let data = response.body.collect().await?.into_bytes();
        let ulid = Ulid::from_string(&key)?;

        Ok(Record::from_bytes(ulid, &data)?)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut wal = MinioWAL::new().await?;

    // Append a record
    let ulid = wal.append(b"Hello, MinIO!".to_vec()).await?;
    println!("Record appended with ULID: {}", ulid);

    // Read the record back
    let record = wal.read(ulid.clone()).await?;
    println!("Read record: {:?}", String::from_utf8(record.data.clone()));

    // Validate checksum
    assert!(record.validate_checksum());
    println!("Checksum is valid!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_ulid_generation() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_checksum_calculation() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_serialization() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_deserialization() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_deserialization_faulty() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_checksum_validation_positive() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_checksum_validation_negative() {
        unimplemented!()
    }

    #[test]
    fn test_initialize_empty_payload() {
        let record = Record::new(vec![]);
        assert!(record.validate_checksum());

        let bytes = record.to_bytes().unwrap();
        let deserialized_record = Record::from_bytes(record.ulid, &bytes).unwrap();
        assert_eq!(record, deserialized_record);
    }

    #[test]
    #[ignore]
    fn test_initialize_huge_payload() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn test_initialize_corrupted_payload() {
        unimplemented!()
    }

    #[test]
    fn test_record_equality() {
        let record1 = Record::new(b"First Record".to_vec());
        let record2 = Record::new(b"Second Record".to_vec());
        let record3 = Record::new(b"Third Record".to_vec());

        assert_ne!(record1, record2);
        assert_ne!(record1, record3);
    }
}
