# `gulog`: A MinIO-backed Write-Ahead Log (WAL) Example

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

1. **Build the binary**:
   - Build the Rust project using Cargo
     ```sh
     cargo build # --release
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
5. **Accessing the Last Record**: Paginates the MinIO bucket to find the last record, and recovers the WAL from there. Useful for recovery.

To run the code:

```sh
cargo run # --release
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
  - `ulid`: Unique identifier for each record. ULIDs were chosen instead of a zero-indexed offset because of they encode timestamp information and (nearly) guarentee uniqueness within millisecond precision (1.21e+24 unique ULIDs per millisecond [per the docs](https://github.com/ulid/spec)).
  - `data`: The payload of the record.
  - `checksum`: SHA-256 checksum for data integrity. A 32 byte `u8` array.
- `ulid`s are used to create MinIO bucket keys via `ulid.to_string()`, which returns a Crockford Base32 encoded string that represents this Ulid.
- The `checksum` is used to guarentee data integrity. Because we rely on an external service (MinIO/S3), we want to make sure what we get back from the distributed storage engine, matches our expectations.

### `MinioWAL` Struct
- Represents the Write-Ahead Log backed by MinIO.

### Main Function
- Demonstrates appending records of varying sizes, reading them back, and validating their checksums.

## Configuration
The MinIO endpoint and credentials are configured directly in the `MinioWAL::new()` method. You can modify these values to point to your specific MinIO setup.

## Notes
- This example uses the `aws-sdk-s3` crate to communicate with MinIO. The SDK is compatible with MinIO as it implements the S3 API.
- Ensure your MinIO instance is up and running before executing the code.

## Upcoming Work
- Timestamps: because we use ULID, there is a host of timestamp-related optimizations we can do (windows, closest-to, before, after).
- 

## License
This project is licensed under the MIT License.

## Contributions
Contributions are welcome! Feel free to open issues or submit pull requests to improve the project. I think the biggest places to improve are:
- Last record retrieval
- Batch append
- Batch read
- Conditional writes (newish S3 feature, also available in MinIO)

## Acknowledgments
- [Building a distributed log using S3 (under 150 lines of Go)](https://avi.im/blag/2024/s3-log/) for providing an easy to understand, digestible intro to disaggregated storage concepts. Go subscribe to their blog.
- [s3-log](https://avi.im/blag/2024/s3-log/) for providing a nice go implementation.
- [MinIO](https://min.io/) for providing a high-performance object storage system that let's me run it locally and emulate S3 for free.
