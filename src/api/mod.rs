mod api_error;
mod routes;
mod server;

pub mod client;
pub use api_error::*;
pub use server::*;

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use super::*;
    use crate::{
        api::client::Client as ApiClient,
        exec::wasm_task_exec::WasmTaskExecutor,
        storage::test_utils::{read_file_to_bytes, setup_mock_local_storage_backend},
    };

    async fn setup_server() -> Server {
        let task_exec = WasmTaskExecutor::new();

        let storage = setup_mock_local_storage_backend().await;
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));

        Server::new(addr, task_exec, storage)
    }

    #[tokio::test]
    async fn test_upload_functions() {
        let mut server = setup_server().await;
        server.run().await.unwrap();

        let server_addr = server.local_addr();

        let fn_file = read_file_to_bytes("tests/fixtures/hello.wasm").unwrap();
        let fn_name = "hello_1.wasm";

        let mut api_client = ApiClient::new().unwrap();
        let response = api_client.upload(server_addr, fn_name, fn_file).await;
        assert!(response.is_ok());

        let fn_file = read_file_to_bytes("tests/fixtures/hello.wasm").unwrap();
        let fn_name = "hello_2.wasm";

        let mut api_client = ApiClient::new().unwrap();
        let response = api_client.upload(server_addr, fn_name, fn_file).await;

        assert!(response.is_ok());

        let stored_fns = api_client.list(server_addr).await.unwrap();
        assert_eq!(stored_fns.functions.len(), 2);

        server.stop();
    }

    #[tokio::test]
    async fn test_list_functions() {
        let mut server = setup_server().await;
        server.run().await.unwrap();

        let server_addr = server.local_addr();

        let fn_file = read_file_to_bytes("tests/fixtures/hello.wasm").unwrap();
        let fn_name = "hello_1.wasm";

        let mut api_client = ApiClient::new().unwrap();
        let response = api_client.upload(server_addr, fn_name, fn_file).await;
        assert!(response.is_ok());

        let functions = api_client.list(server_addr).await.unwrap().functions;

        assert_eq!(functions.len(), 1);

        server.stop();
    }

    #[tokio::test]
    async fn test_get_function() {
        let mut server = setup_server().await;
        server.run().await.unwrap();

        let server_addr = server.local_addr();

        let fn_file = read_file_to_bytes("tests/fixtures/hello.wasm").unwrap();
        let fn_name = "hello_1.wasm";

        let mut api_client = ApiClient::new().unwrap();
        let response = api_client
            .upload(server_addr, fn_name, fn_file)
            .await
            .unwrap();

        let response = api_client.get(server_addr, &response.id).await.unwrap();

        assert!(!response.fn_data.is_empty());

        server.stop();
    }

    #[tokio::test]
    async fn test_get_non_existent_function() {
        let mut server = setup_server().await;
        server.run().await.unwrap();

        let server_addr = server.local_addr();

        let api_client = ApiClient::new().unwrap();

        let fn_id = uuid::Uuid::new_v4();

        let response = api_client.get(server_addr, &fn_id).await;
        assert!(response.is_err());

        server.stop();
    }

    #[tokio::test]
    async fn test_call_function() {
        let mut server = setup_server().await;
        server.run().await.unwrap();

        let server_addr = server.local_addr();

        let fn_file = read_file_to_bytes("tests/fixtures/hello.wasm").unwrap();
        let fn_name = "hello_1.wasm";

        let mut api_client = ApiClient::new().unwrap();
        let response = api_client
            .upload(server_addr, fn_name, fn_file)
            .await
            .unwrap();

        let response = api_client
            .call(server_addr, &response.id, vec![])
            .await
            .unwrap();

        let expected_response = "Hello, world!\n";
        assert_eq!(response.stdout, expected_response);

        server.stop();
    }

    #[tokio::test]
    async fn test_call_invalid_function() {
        let mut server = setup_server().await;
        server.run().await.unwrap();

        let server_addr = server.local_addr();

        let fn_id = uuid::Uuid::new_v4();

        let mut api_client = ApiClient::new().unwrap();
        let response = api_client.call(server_addr, &fn_id, vec![]).await;
        assert!(response.is_err());

        server.stop();
    }

    #[tokio::test]
    async fn test_delete_functions() {
        let mut server = setup_server().await;
        server.run().await.unwrap();

        let server_addr = server.local_addr();

        let fn_file = read_file_to_bytes("tests/fixtures/hello.wasm").unwrap();
        let fn_name = "hello_1.wasm";

        let mut api_client = ApiClient::new().unwrap();

        let response = api_client
            .upload(server_addr, fn_name, fn_file)
            .await
            .unwrap();

        api_client.delete(server_addr, &response.id).await.unwrap();

        let get_response = api_client.get(server_addr, &response.id).await;
        assert!(get_response.is_err());

        server.stop();
    }
}
