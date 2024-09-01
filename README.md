
# Tenant Management Service (tenantm)

`tenantm` is a Rust-based gRPC service designed to manage tenants and their associated configurations. The service allows for querying and deployment of tenant-related resources.

## Features

- **List Tenants:** Retrieve a list of all generated tenant directories from the configured `TARGET_FOLDER`.
- **Deployment Management:** Facilitates future enhancements for launching and managing tenant deployments.

## Prerequisites

Before running the service or testing, ensure you have the following:

- **Rust**: The Rust programming language and its tooling installed. 
- **`grpcurl`**: A command-line tool for interacting with gRPC services.



## gRPC API

The `tenantm` service provides the following gRPC methods:

### `ListTenants`

**Request:**

- `ListTenantsRequest`

**Response:**

- `ListTenantsResponse` with a list of tenant names.

### `ListDatetimeFolders`

**Request:**

- `ListDatetimeFoldersRequest` with the tenant name.

**Response:**

- `ListDatetimeFoldersResponse` with a list of datetime-named folders under the specified tenant.

## Example Usage

To test the `ListDatetimeFolders` method using `grpcurl`, you can use the following command:

```bash
grpcurl -plaintext -proto proto-definitions/tenantm.proto localhost:50059 tenantm.TenantManager/ListDatetimeFolders -d '{"tenant": "my_tenant"}'

## Running the Service

To run the `tenantm` service, follow these steps:

1. **Build the Project:**

   cargo build

2. **Run the Service:**

   cargo run

   The service will start and listen on port `50059`.

## Testing the Service

To test the `tenantm` service, use the `test_tenantm.sh` script provided in this repository. This script uses `grpcurl` to invoke the `ListTenants` method and output the results.

### Test Script

1. **Make the Script Executable:**

   chmod +x test_tenantm.sh

2. **Run the Script:**

   ./test_tenantm.sh

   This will send a request to the `ListTenants` method and display the results.

## Configuration

- **Environment Variables:**
  - `TARGET_FOLDER`: Directory where tenant configurations are stored. The service reads tenant directories from this location.

## Project Structure

- `src/`: Contains Rust source code for the gRPC service.
- `proto-definitions/`: Contains `.proto` files defining the gRPC service.
- `Cargo.toml`: Project configuration and dependencies.

## Contributing

Contributions are welcome! Please open an issue or a pull request to suggest improvements or add new features.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions or further information, please contact [your-email@example.com].
