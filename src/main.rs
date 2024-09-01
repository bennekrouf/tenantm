
use std::env;
use std::fs;
use std::path::PathBuf;
use tonic::{transport::Server, Request, Response, Status};
use dotenvy::from_path;
use tracing::{info, error};
use tokio;

// Generated code from the .proto file
pub mod generated {
    tonic::include_proto!("tenantm");
}

#[derive(Debug, Default)]
pub struct MyTenantManager;

#[tonic::async_trait]
impl generated::tenant_manager_server::TenantManager for MyTenantManager {
    async fn list_tenants(
        &self,
        _request: Request<generated::ListTenantsRequest>,
    ) -> Result<Response<generated::ListTenantsResponse>, Status> {
        // Load the environment variables from a custom file
        let custom_env_path = PathBuf::from("proto-definitions/.service");
        from_path(&custom_env_path).expect("Failed to load environment variables from custom path");

        // Get the target folder from the environment variable
        let target_folder = env::var("TARGET_FOLDER").unwrap_or_else(|_| "generated".to_string());
        let target_path = PathBuf::from(target_folder);

        // List the tenant directories
        let mut tenants = Vec::new();
        if target_path.is_dir() {
            for entry in fs::read_dir(&target_path).expect("Failed to read directory") {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        tenants.push(name.to_string());
                    }
                }
            }
        } else {
            return Err(Status::not_found("Target folder does not exist"));
        }

        Ok(Response::new(generated::ListTenantsResponse { tenants }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::from_path(PathBuf::from("proto-definitions/.service")).expect("Failed to load environment variables from custom path");
    
    // Get the server address from environment variables
    let addr = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:50051".to_string());
    let addr = addr.parse()?;

    // Create the gRPC server
    let tenant_manager = MyTenantManager::default();

    // Start the server
    Server::builder()
        .add_service(generated::tenant_manager_server::TenantManagerServer::new(tenant_manager))
        .serve(addr)
        .await?;

    Ok(())
}

