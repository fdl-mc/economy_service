use tonic::{Request, Response, Status};

use crate::proto::economy::economy_server::Economy as EconomyServiceTrait;
use crate::proto::economy::{GetEconomyStateReply, GetEconomyStateRequest, PayReply, PayRequest};

pub struct EconomyService {}

#[tonic::async_trait]
impl EconomyServiceTrait for EconomyService {
    async fn get_economy_state(
        &self,
        _request: Request<GetEconomyStateRequest>,
    ) -> Result<Response<GetEconomyStateReply>, Status> {
        Err(Status::unimplemented(""))
    }

    async fn pay(&self, _request: Request<PayRequest>) -> Result<Response<PayReply>, Status> {
        Err(Status::unimplemented(""))
    }
}
