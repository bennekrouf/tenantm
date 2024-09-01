pub mod generated {
    tonic::include_proto!("tenantm");
}

use std::{env, fs};
use std::path::PathBuf;
use tonic::{transport::Server, Request, Response, Status};
use dotenvy::from_path;
use tokio;
use std::sync::Arc;
use tokio::sync::Mutex;
use messengerc::{connect_to_messenger_service, MessagingService};
use generated::{ListTenantsRequest, ListTenantsResponse, ListDatetimeFoldersRequest, ListDatetimeFoldersResponse};

#[derive(Debug, Default)]
pub struct MyTenantManager;

#[tonic::async_trait]
impl generated::tenant_manager_server::TenantManager for MyTenantManager {
    async fn list_tenants(
        &self,
        _request: Request<ListTenantsRequest>,
    ) -> Result<Response<ListTenantsResponse>, Status> {
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

        Ok(Response::new(ListTenantsResponse { tenants }))
    }

    async fn list_datetime_folders(
        &self,
        request: Request<ListDatetimeFoldersRequest>,
    ) -> Result<Response<ListDatetimeFoldersResponse>, Status> {
        // Load the environment variables from a custom file
        let custom_env_path = PathBuf::from("proto-definitions/.service");
        from_path(&custom_env_path).expect("Failed to load environment variables from custom path");

        // Get the target folder from the environment variable
        let target_folder = env::var("TARGET_FOLDER").unwrap_or_else(|_| "generated".to_string());
        let tenant = request.into_inner().tenant;
        let tenant_path = PathBuf::from(target_folder).join(&tenant);

        // List datetime-named folders under the tenant
        let mut datetime_folders = Vec::new();
        if tenant_path.is_dir() {
            for entry in fs::read_dir(&tenant_path).expect("Failed to read directory") {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        // Assuming datetime folders are named in a specific format, e.g., YYYY-MM-DD
                        if name.chars().all(|c| c.is_numeric() || c == '-') && name.len() == 10 {
                            datetime_folders.push(name.to_string());
                        }
                    }
                }
            }
        } else {
            return Err(Status::not_found("Tenant folder does not exist"));
        }

        Ok(Response::new(ListDatetimeFoldersResponse { datetime_folders }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::from_path(PathBuf::from("proto-definitions/.service")).expect("Failed to load environment variables from custom path");

    let ip = env::var("TENANTM_DOMAIN").expect("Missing 'domain' environment variable");
    let port = env::var("TENANTM_PORT").expect("Missing 'port' environment variable");
    let addr = format!("{}:{}", ip, port).parse()?;

    let tag = env::var("TENANTM_TAG").expect("Missing 'tag' environment variable");

    // Create and initialize the gRPC client for the messaging service
    let messenger_client = connect_to_messenger_service().await
        .ok_or("Failed to connect to messenger service")?;

    let messaging_service = MessagingService::new(
        Arc::new(Mutex::new(messenger_client)),
        tag.clone(),
    );

    let mes = format!("Tenantm listening on {}", &addr);
    let _ = messaging_service.publish_message(mes.to_string(), Some(vec![tag])).await;

    // Create the gRPC server
    let tenant_manager = MyTenantManager::default();

    // Start the server
    Server::builder()
        .add_service(generated::tenant_manager_server::TenantManagerServer::new(tenant_manager))
        .serve(addr)
        .await?;

    Ok(())
}

