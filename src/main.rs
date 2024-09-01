pub mod generated {
    tonic::include_proto!("tenantm");
}

use std::{env, fs, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use messengerc::{connect_to_messenger_service, MessagingService};
use tonic::{transport::Server, Request, Response, Status};
use dotenvy::from_path;
use tokio;
use tracing::{info, error};
use generated::{ListTenantsRequest, ListTenantsResponse, ListDatetimeFoldersRequest, ListDatetimeFoldersResponse};
use generated::tenant_manager_server::TenantManager;
use chrono::NaiveDateTime;

#[derive(Debug, Default)]
pub struct MyTenantManager;

#[tonic::async_trait]
impl TenantManager for MyTenantManager {
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
        info!("Environment variables loaded from {:?}", custom_env_path);

        // Get the target folder from the environment variable
        let target_folder = env::var("TARGET_FOLDER").unwrap_or_else(|_| {
            error!("'TARGET_FOLDER' environment variable not set. Using default 'generated' folder.");
            "generated".to_string()
        });
        info!("Target folder: {}", target_folder);

        // Get tenant from the request
        let tenant = request.into_inner().tenant;
        info!("Received request for tenant: {}", tenant);

        let tenant_path = PathBuf::from(target_folder).join(&tenant);
        info!("Tenant path: {:?}", tenant_path);

        // Check if the tenant directory exists
        if !tenant_path.is_dir() {
            error!("Tenant folder {:?} does not exist.", tenant_path);
            return Err(Status::not_found("Tenant folder does not exist"));
        }

        // List datetime-named folders under the tenant
        let mut datetime_folders = Vec::new();
        for entry in fs::read_dir(&tenant_path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    info!("Found directory: {}", name);
                    // Assuming datetime folders are named in a specific format, e.g., YYYY-MM-DD
                    // if name.chars().all(|c| c.is_numeric() || c == '-') && name.len() == 10 {
                        info!("Adding datetime folder: {}", name);
                        datetime_folders.push(name.to_string());
                    // } else {
                        // info!("Skipping non-datetime folder: {}", name);
                    // }
                }
            }
        }

        info!("Returning datetime folders: {:?}", datetime_folders);
        Ok(Response::new(ListDatetimeFoldersResponse { datetime_folders }))
    }

    async fn get_most_recent_datetime_folder(
        &self,
        request: Request<generated::GetMostRecentDatetimeFolderRequest>,
    ) -> Result<Response<generated::GetMostRecentDatetimeFolderResponse>, Status> {
        // Load the environment variables from a custom file
        let custom_env_path = PathBuf::from("proto-definitions/.service");
        dotenvy::from_path(&custom_env_path).expect("Failed to load environment variables from custom path");
        info!("Environment variables loaded from {:?}", custom_env_path);

        // Get the target folder from the environment variable
        let target_folder = env::var("TARGET_FOLDER").unwrap_or_else(|_| {
            error!("'TARGET_FOLDER' environment variable not set. Using default 'generated' folder.");
            "generated".to_string()
        });
        info!("Target folder: {}", target_folder);

        // Get tenant from the request
        let tenant = request.into_inner().tenant;
        info!("Received request for tenant: {}", tenant);

        let tenant_path = PathBuf::from(target_folder).join(&tenant);
        info!("Tenant path: {:?}", tenant_path);

        // Check if the tenant directory exists
        if !tenant_path.is_dir() {
            error!("Tenant folder {:?} does not exist.", tenant_path);
            return Err(Status::not_found("Tenant folder does not exist"));
        }

        // Find the most recent datetime folder under the tenant
        let mut most_recent_datetime: Option<NaiveDateTime> = None;
        let mut most_recent_folder: Option<String> = None;

        let datetime_format = "%Y-%m-%d_%H-%M-%S"; // Adjust to your folder name format

        for entry in fs::read_dir(&tenant_path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    info!("Found directory: {}", name);
                    // Parse folder names in the format YYYY-MM-DD_HH-MM-SS
                    if let Ok(parsed_datetime) = NaiveDateTime::parse_from_str(name.trim_start_matches("generated_"), datetime_format) {
                        if most_recent_datetime.is_none() || parsed_datetime > most_recent_datetime.unwrap() {
                            most_recent_datetime = Some(parsed_datetime);
                            most_recent_folder = Some(name.to_string());
                        }
                    }
                }
            }
        }

        match most_recent_folder {
            Some(folder) => {
                info!("Most recent datetime folder: {}", folder);
                Ok(Response::new(generated::GetMostRecentDatetimeFolderResponse { most_recent_folder: folder }))
            },
            None => {
                error!("No valid datetime folders found.");
                Err(Status::not_found("No valid datetime folders found"))
            }
        }
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

