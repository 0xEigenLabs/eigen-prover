use algebraic::errors::EigenError;
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
    pub async fn new(addr: String) -> Result<Self, EigenError> {
        let result = StateDbServiceClient::connect(addr.clone())
            .await
            .map_err(|e| EigenError::from(format!("connect err: {:?}", e)));
        match result {
            Ok(c) => Ok(StateDBClientCli { client: c }),
            Err(e) => Err(e),
        }
    }

    pub async fn get(mut self, root: Fea, key: Fea) -> Result<Response<GetResponse>, EigenError> {
        let req = tonic::Request::new(GetRequest {
            root: Some(root),
            key: Some(key),
            details: true,
            get_db_read_log: true,
        });

        self
            .client
            .get(req)
            .await
            .map_err(|e| EigenError::from(format!("get status: {:?}", e)))
    }

    pub async fn set(
        mut self,
        root: Fea,
        key: Fea,
        val: String,
    ) -> Result<Response<SetResponse>, EigenError> {
        let req = tonic::Request::new(SetRequest {
            old_root: Some(root),
            key: Some(key),
            value: val,
            persistent: true,
            details: true,
            get_db_read_log: true,
        });
        self
            .client
            .set(req)
            .await
            .map_err(|e| EigenError::from(format!("set status: {:?}", e)))
    }

    pub async fn get_program(
        mut self,
        key: Fea,
    ) -> Result<Response<GetProgramResponse>, EigenError> {
        let req = tonic::Request::new(GetProgramRequest { key: Some(key) });
        self
            .client
            .get_program(req)
            .await
            .map_err(|e| EigenError::from(format!("get_program status: {:?}", e)))
    }

    pub async fn set_program(
        mut self,
        key: Fea,
        data: Vec<u8>,
    ) -> Result<Response<SetProgramResponse>, EigenError> {
        let req = tonic::Request::new(SetProgramRequest {
            key: Some(key),
            data,
            persistent: true,
        });
        self
            .client
            .set_program(req)
            .await
            .map_err(|e| EigenError::from(format!("set_program status: {:?}", e)))
    }

    pub async fn flush(mut self) -> Result<Response<FlushResponse>, EigenError> {
        self
            .client
            .flush(())
            .await
            .map_err(|e| EigenError::from(format!("set_program status: {:?}", e)))
    }
}
