use anyhow::Result;
use statedb_service::state_db_service_client::StateDbServiceClient;
use statedb_service::{
    Fea, FlushResponse, GetProgramRequest, GetProgramResponse, GetRequest, GetResponse,
    SetProgramRequest, SetProgramResponse, SetRequest, SetResponse,
};
use tonic::transport::Channel;
use tonic::Response;

pub mod statedb_service {
    tonic::include_proto!("statedb.v1");
}

pub struct StateDBClientCli {
    client: StateDbServiceClient<Channel>,
}

impl StateDBClientCli {
    pub async fn new(addr: String) -> Result<Self> {
        let client = StateDbServiceClient::connect(addr.clone()).await?;
        Ok(StateDBClientCli { client })
    }

    pub async fn get(mut self, root: Fea, key: Fea) -> Result<Response<GetResponse>> {
        let req = tonic::Request::new(GetRequest {
            root: Some(root),
            key: Some(key),
            details: true,
            get_db_read_log: true,
        });

        Ok(self.client.get(req).await?)
    }

    pub async fn set(mut self, root: Fea, key: Fea, val: String) -> Result<Response<SetResponse>> {
        let req = tonic::Request::new(SetRequest {
            old_root: Some(root),
            key: Some(key),
            value: val,
            persistent: true,
            details: true,
            get_db_read_log: true,
        });
        Ok(self.client.set(req).await?)
    }

    pub async fn get_program(mut self, key: Fea) -> Result<Response<GetProgramResponse>> {
        let req = tonic::Request::new(GetProgramRequest { key: Some(key) });
        Ok(self.client.get_program(req).await?)
    }

    pub async fn set_program(
        mut self,
        key: Fea,
        data: Vec<u8>,
    ) -> Result<Response<SetProgramResponse>> {
        let req = tonic::Request::new(SetProgramRequest {
            key: Some(key),
            data,
            persistent: true,
        });
        Ok(self.client.set_program(req).await?)
    }

    pub async fn flush(mut self) -> Result<Response<FlushResponse>> {
        Ok(self.client.flush(()).await?)
    }
}
