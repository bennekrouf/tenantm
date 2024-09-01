
#!/bin/bash

# Set the server address and port
SERVER_ADDRESS="localhost:50059"

# Path to the .proto file
PROTO_FILE_PATH="proto-definitions/tenantm.proto"

# The fully qualified name of the service and method
SERVICE_METHOD="tenantm.TenantManager/GetMostRecentDatetimeFolder"

# Tenant to test
TENANT_NAME="mayorana"

# JSON payload for the gRPC request
JSON_PAYLOAD="{\"tenant\": \"${TENANT_NAME}\"}"

# Test the gRPC service using grpcurl
grpcurl -d "$JSON_PAYLOAD" -plaintext -proto $PROTO_FILE_PATH  $SERVER_ADDRESS $SERVICE_METHOD

