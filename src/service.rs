use sqlx::PgPool;
use tonic::{transport::Channel, Request, Response, Status};

use crate::models::economy_state::EconomyStateModel;
use crate::proto::economy::economy_server::Economy as EconomyServiceTrait;
use crate::proto::economy::{
    DepositReply, DepositRequest, GetEconomyStateReply, GetEconomyStateRequest,
    GetSelfEconomyStateReply, GetSelfEconomyStateRequest, PayReply, PayRequest, WithdrawReply,
    WithdrawRequest,
};
use crate::proto::users::users_client::UsersClient;
use crate::Config;

pub struct EconomyService {
    pub config: Config,
    pub pool: PgPool,
    pub users_client: UsersClient<Channel>,
}

#[tonic::async_trait]
impl EconomyServiceTrait for EconomyService {
    async fn get_economy_state(
        &self,
        request: Request<GetEconomyStateRequest>,
    ) -> Result<Response<GetEconomyStateReply>, Status> {
        match EconomyStateModel::get_by_user_id(request.get_ref().user_id, &self.pool).await {
            Ok(res) => match res {
                Some(res) => {
                    let state = res.into_message();
                    let reply = GetEconomyStateReply {
                        economy_state: Some(state),
                    };
                    Ok(Response::new(reply))
                }
                None => Err(Status::not_found("State not found")),
            },
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }

    async fn get_self_economy_state(
        &self,
        _request: Request<GetSelfEconomyStateRequest>,
    ) -> Result<Response<GetSelfEconomyStateReply>, Status> {
        Err(Status::unimplemented(""))
    }

    async fn pay(&self, _request: Request<PayRequest>) -> Result<Response<PayReply>, Status> {
        Err(Status::unimplemented(""))
    }

    async fn deposit(
        &self,
        _request: Request<DepositRequest>,
    ) -> Result<Response<DepositReply>, Status> {
        Err(Status::unimplemented(""))
    }

    async fn withdraw(
        &self,
        _request: Request<WithdrawRequest>,
    ) -> Result<Response<WithdrawReply>, Status> {
        Err(Status::unimplemented(""))
    }
}
