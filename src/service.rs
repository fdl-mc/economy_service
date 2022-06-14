use sqlx::PgPool;
use tonic::{transport::Channel, Request, Response, Status};

use crate::models::user_state::UserStateModel;
use crate::proto::economy::economy_server::Economy as EconomyServiceTrait;
use crate::proto::economy::{GetEconomyStateReply, GetEconomyStateRequest, PayReply, PayRequest};
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
        match UserStateModel::get_by_user_id(request.get_ref().user_id, &self.pool).await {
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

    async fn pay(&self, _request: Request<PayRequest>) -> Result<Response<PayReply>, Status> {
        Err(Status::unimplemented(""))
    }
}
