use algebraic::errors::EigenError;
use tonic::{Request, Response};
use tonic::transport::Channel;
use statedb_service::state_db_service_client::StateDbServiceClient;
use statedb_service::{
    result_code::Code, Fea, FlushResponse, GetProgramRequest, GetProgramResponse, GetRequest,
    GetResponse, LoadDbRequest, LoadProgramDbRequest, ResultCode, SetProgramRequest,
    SetProgramResponse, SetRequest, SetResponse, SiblingList,
};
use crate::statedb::statedb_service;


pub struct StateDBClientCli {
    client: StateDbServiceClient<Channel>,
}

impl StateDBClientCli {
    pub fn new(addr: String) -> Self {
        let mut client = StateDbServiceClient::connect(addr.clone());
        StateDBClientCli {
            client: client
        }
    }

    async extern fn get(mut self, root: Fea, key: Fea) -> Result<Response<GetResponse>, EigenError> {
        // let response = self.client.get(req).await?;
        let req = tonic::Request::new(GetRequest {
            root: Some(root),
            key: Some(key),
            details: true,
            get_db_read_log: true,
        });

        let resp = self.client.get(req).await.map_err(|e| EigenError::from(format!("get status: {:?}", e)));
        return resp;
    }

    async extern fn set(mut self, root: Fea, key: Fea, val: String) -> Result<Response<SetResponse>, EigenError> {
        let req = tonic::Request::new(SetRequest {
            old_root: Some(root),
            key: Some(key),
            value: val,
            persistent: true,
            details: true,
            get_db_read_log: true,
        });
        // let response = self.client.get(req).await?;
        let res = self.client.set(req).await.map_err(|e| EigenError::from(format!("set status: {:?}", e)));
        return res;
    }

    async extern fn get_program(mut self, key: Fea) -> Result<Response<GetProgramResponse>, EigenError> {
        let req = tonic::Request::new(GetProgramRequest {
            key: Some(key),
        });
        // let response = self.client.get(req).await?;
        let res = self.client.get_program(req).await.map_err(|e| EigenError::from(format!("get_program status: {:?}", e)));
        return res;
    }

    async extern fn set_program(mut self, key: Fea, data: Vec<u8>) -> Result<Response<SetProgramResponse>, EigenError> {
        // let response = self.client.get(req).await?;
        let req = tonic::Request::new(SetProgramRequest {
            key: Some(key),
            data: data,
            persistent: true,
        });
        let res = self.client.set_program(req).await.map_err(|e| EigenError::from(format!("set_program status: {:?}", e)));
        return res;
    }

    async extern fn flush(mut self) -> Result<Response<FlushResponse>, EigenError> {
        let res = self.client.flush(()).await.map_err(|e| EigenError::from(format!("set_program status: {:?}", e)));
        return res;
    }
}

#[cfg(test)]
mod test {
    use crate::statedb_client::StateDBClientCli;
    use crate::statedb::statedb_service::Fea;

    lazy_static! {
        static ref CLIENT: StateDBClientCli = init();
    }

    fn init() -> StateDBClientCli {
        let addr = "127.0.0.1:50051".to_string();
        StateDBClientCli::new(addr)
    }

    #[test]
    async fn test_set_get() {
        let client = &*CLIENT;
        let root: Fea= Fea {
            fe0: 0,
            fe1: 0,
            fe2: 0,
            fe3: 0,
        };
        let key: Fea = Fea{
            fe0: 1,
            fe1: 1,
            fe2: 1,
            fe3: 1,
        };
        let val = "1".to_string();

        let set_resp = client.set(root, key, val).await;

        assert_eq!(set_resp.is_ok(), true);
        let result = set_resp.unwrap();
        let new_root = result.get_ref().new_root.unwrap().clone();

        let get_resp = client.get(new_root, key.clone()).await;
        assert_eq!(set_resp.is_ok(), true);
        assert_eq!(get_resp.unwrap().get_ref().value, "1");
    }
}