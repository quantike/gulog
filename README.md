# MinIO Write-Ahead Log (WAL) Example

This project demonstrates the implementation of a Write-Ahead Log (WAL) using MinIO as the backend for storing log records. The WAL is implemented with Rust, showcasing how to write, read, and validate records stored in MinIO. Each record is uniquely identified using a [ULID](https://github.com/ulid/spec) and includes data integrity validation with a SHA-256 checksum.

## Features
- **ULID-based Record Identification**: Each record has a unique identifier generated using ULID.
- **Data Integrity with SHA-256**: Every record is accompanied by a SHA-256 checksum, which is used to ensure data integrity when reading records.
- **Varying Data Sizes**: Demonstrates writing records with different amounts of data to MinIO, ranging from no data to large amounts (e.g., 1MB).

## Requirements
- **Rust**: Make sure Rust is installed. You can use [rustup](https://rustup.rs/) to install Rust.
- **MinIO**: A local or remote MinIO instance to store WAL records. This example assumes a local instance is running at `http://127.0.0.1:9000` with credentials `admin` / `password`.
- **Tokio**: The project uses the `tokio` runtime for asynchronous operations.

## Setup

1. **Install Dependencies**:
   - Add the following dependencies to your `Cargo.toml`:
     ```toml
     [dependencies]
     aws-sdk-s3 = "*"
     sha2 = "*"
     tokio = { version = "*", features = ["full"] }
     ulid = "*"
     ```

2. **Run MinIO Locally**:
   - You can run MinIO using Docker:
     ```sh
     docker run -p 9000:9000 -p 9001:9001 -e "MINIO_ROOT_USER=admin" -e "MINIO_ROOT_PASSWORD=password" quay.io/minio/minio server /data --console-address ":9001"
     ```

3. **Configure MinIO**:
   - Create a bucket named `gulog-dev` in MinIO, which is used to store the WAL records.

## Usage

### Running the Code
The main function in this project demonstrates the following:

1. **Creating a MinIO WAL Client**: A client is created to interact with the MinIO instance.
2. **Appending Records**: Several records with different data sizes are appended to MinIO. Each record gets a unique ULID.
3. **Reading Records**: The records are read back using their ULID, and their content is printed.
4. **Validating Checksums**: The checksum of each record is validated to ensure data integrity.

To run the code:

```sh
cargo run
```

### Example Output
When running the program, you should see output similar to the following:

```
Successfully appended record 1 with ULID: 01FZXYZABC1234567890ABCDEFG
Successfully appended record 2 with ULID: 01FZXYZDEF1234567890ABCDEFG
Successfully appended record 3 with ULID: 01FZXYZGHI1234567890ABCDEFG
...
Fetched Record 1: [Data]
Checksum for record 1 is valid!
...
```

## Code Overview

### `Record` Struct
- Represents a record in the WAL, consisting of:
  - `ulid`: Unique identifier for each record.
  - `data`: The payload of the record.
  - `checksum`: SHA-256 checksum for data integrity.

- Key Methods:
  - **`new(data: Vec<u8>) -> Self`**: Creates a new `Record` with a ULID and checksum.
  - **`validate_checksum(&self) -> bool`**: Validates the checksum of the record.
  - **`to_bytes(&self) -> io::Result<Vec<u8>>`**: Serializes the record to bytes.
  - **`from_bytes(ulid: Ulid, bytes: &[u8]) -> io::Result<Self>`**: Deserializes bytes into a `Record`.

### `MinioWAL` Struct
- Represents the Write-Ahead Log backed by MinIO.
- Key Methods:
  - **`append(&mut self, data: Vec<u8>) -> Result<Ulid, Box<dyn Error + Send + Sync>>`**: Appends a record to the WAL and returns its ULID.
  - **`read(&self, ulid: Ulid) -> Result<Record, Box<dyn Error + Send + Sync>>`**: Reads a record from the WAL using its ULID.

### Main Function
- Demonstrates appending records of varying sizes, reading them back, and validating their checksums.

## Configuration
The MinIO endpoint and credentials are configured directly in the `MinioWAL::new()` method. You can modify these values to point to your specific MinIO setup.

## Notes
- This example uses the `aws-sdk-s3` crate to communicate with MinIO. The SDK is compatible with MinIO as it implements the S3 API.
- Ensure your MinIO instance is up and running before executing the code.

## License
This project is licensed under the MIT License.

## Contributions
Contributions are welcome! Feel free to open issues or submit pull requests to improve the project.

## Acknowledgments
- [MinIO](https://min.io/) for providing a high-performance object storage system.
- [ULID](https://github.com/ulid/spec) for generating unique identifiers.
- [Rust](https://www.rust-lang.org/) community for making this project possible.


