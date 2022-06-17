use sqlx::PgPool;
use tonic::{transport::Channel, Request, Response, Status};

use crate::models::economy_state::EconomyStateModel;
use crate::proto::economy::economy_server::Economy as EconomyServiceTrait;
use crate::proto::economy::{
    DepositReply, DepositRequest, GetEconomyStateReply, GetEconomyStateRequest,
    GetSelfEconomyStateReply, GetSelfEconomyStateRequest, PayReply, PayRequest, RegisterReply,
    RegisterRequest, WithdrawReply, WithdrawRequest,
};
use crate::proto::users::users_client::UsersClient;
use crate::proto::users::GetSelfUserRequest;
use crate::Config;

pub struct EconomyService {
    pub config: Config,
    pub pool: PgPool,
    pub users_client: UsersClient<Channel>,
}

#[tonic::async_trait]
impl EconomyServiceTrait for EconomyService {
    async fn register(
        &self,
        _request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterReply>, Status> {
        Err(Status::unimplemented(""))
    }

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
        request: Request<GetSelfEconomyStateRequest>,
    ) -> Result<Response<GetSelfEconomyStateReply>, Status> {
        let mut users_client = self.users_client.clone();

        // Extract token from metadata
        let token = match request.metadata().get("x-token") {
            Some(res) => res.to_str().unwrap().to_string(),
            None => return Err(Status::unauthenticated("No token provided")),
        };

        // Prepare request
        let mut user_request = Request::new(GetSelfUserRequest {});
        user_request
            .metadata_mut()
            .append("x-token", token.parse().unwrap());

        // Fetch user
        let user = match users_client.get_self_user(user_request).await {
            //                             KABOoom!!!!
            Ok(res) => res.into_inner().user.unwrap(),
            // TODO: maybe handle it a liiiiiittle better?
            Err(err) => return Err(err),
        };

        // Fetch or create user state
        match EconomyStateModel::get_by_user_id_or_create(user.id, &self.pool).await {
            Ok(res) => {
                let state = res.into_message();
                let reply = GetSelfEconomyStateReply {
                    economy_state: Some(state),
                };
                Ok(Response::new(reply))
            }
            Err(err) => Err(Status::internal(err.to_string())),
        }
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
