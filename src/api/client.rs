use std::net::SocketAddr;

use crate::{
    api::{
        routes::{call::CallRequest, upload::UploadResponse},
        API_VERSION,
    },
    exec::task::TaskResult,
    storage::FnEntry,
};

use super::routes::{get::GetResponse, list::ListResponse};

#[derive(Debug, Clone)]
pub struct Client {
    r_client: reqwest::Client,
}

impl Client {
    pub fn new() -> anyhow::Result<Self> {
        let r_client = reqwest::Client::builder().build()?;

        Ok(Self { r_client })
    }

    /// List all available functions in the server
    pub async fn list(&self, addr: SocketAddr) -> anyhow::Result<ListResponse> {
        let url = format!("http://{addr}/{API_VERSION}/functions");

        let response = self.r_client.get(url).send().await?;
        let response = response.json::<ListResponse>().await?;

        Ok(response)
    }

    /// Gets detailed information about a function
    pub async fn get(&self, addr: SocketAddr, fn_id: &uuid::Uuid) -> anyhow::Result<GetResponse> {
        let url = format!("http://{addr}/{API_VERSION}/functions/{fn_id}");

        let response = self.r_client.get(url).send().await?;
        let response = response.json::<GetResponse>().await?;

        Ok(response)
    }

    /// Calls a function on the server
    pub async fn call(
        &mut self,
        addr: SocketAddr,
        id: &uuid::Uuid,
        args: Vec<u8>,
    ) -> anyhow::Result<TaskResult> {
        let call_url = format!("http://{addr}/{API_VERSION}/functions/{id}");
        let call_request = CallRequest {
            id: id.to_owned(),
            args,
        };

        let response = self
            .r_client
            .post(call_url)
            .json(&call_request)
            .send()
            .await?;

        let response = response.json::<TaskResult>().await?;

        Ok(response)
    }

    /// Uploads a function to the server
    pub async fn upload(
        &mut self,
        addr: SocketAddr,
        fn_name: &str,
        fn_data: Vec<u8>,
    ) -> anyhow::Result<UploadResponse> {
        let form = reqwest::multipart::Form::new();
        let part = reqwest::multipart::Part::bytes(fn_data)
            .file_name(fn_name.to_string())
            .mime_str("application/wasm")?;

        let form = form.part("fn_file", part);

        let upload_url = format!("http://{addr}/{API_VERSION}/functions");

        let raw_response = self
            .r_client
            .post(upload_url)
            .multipart(form)
            .send()
            .await?;

        let response = raw_response.json::<UploadResponse>().await?;

        Ok(response)
    }

    /// Deletes a function from the server
    pub async fn delete(&mut self, addr: SocketAddr, fn_id: &uuid::Uuid) -> anyhow::Result<()> {
        let delete_url = format!("http://{addr}/{API_VERSION}/functions/{fn_id}");

        self.r_client.delete(delete_url).send().await?;

        Ok(())
    }
}
