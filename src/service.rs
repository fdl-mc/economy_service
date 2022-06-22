use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tonic::{transport::Channel, Request, Response, Status};

use crate::models::economy_state;
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
    pub conn: DatabaseConnection,
    pub users_client: UsersClient<Channel>,
}

#[tonic::async_trait]
impl EconomyServiceTrait for EconomyService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterReply>, Status> {
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
            Ok(res) => res.into_inner().user.unwrap(),
            // TODO: handle it better
            Err(err) => return Err(err),
        };

        // Register or ignore
        let state = economy_state::ActiveModel {
            user_id: Set(user.id),
            ..Default::default()
        };

        match state.insert(&self.conn).await {
            Ok(_) => (),
            // TODO: handle unique key violation
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        Ok(Response::new(RegisterReply {}))
    }

    async fn get_economy_state(
        &self,
        request: Request<GetEconomyStateRequest>,
    ) -> Result<Response<GetEconomyStateReply>, Status> {
        let message = request.get_ref();

        // Fetch the state
        let state = match economy_state::Entity::find()
            .filter(economy_state::Column::UserId.eq(message.user_id))
            .one(&self.conn)
            .await
        {
            Ok(res) => match res {
                Some(res) => res,
                None => return Err(Status::not_found("State not found")),
            },
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Prepare a reply
        let reply = GetEconomyStateReply {
            economy_state: Some(state.into_message()),
        };
        Ok(Response::new(reply))
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

        // Fetch user state
        let state = match economy_state::Entity::find()
            .filter(economy_state::Column::UserId.eq(user.id))
            .one(&self.conn)
            .await
        {
            Ok(res) => match res {
                Some(res) => res,
                None => return Err(Status::not_found("State not found")),
            },
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Prepare reply
        let reply = GetSelfEconomyStateReply {
            economy_state: Some(state.into_message()),
        };
        Ok(Response::new(reply))
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
