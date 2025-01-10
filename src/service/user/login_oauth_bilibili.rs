use crate::entity::user::Model as UserModel;
use crate::errs::http::Error as HttpError;

use super::svc::UserService;

impl UserService {
    async fn login_oauth_bilibili(&self, oauth_code: &str) -> Result<(String, UserModel), HttpError> {
        Ok((String::from("token"), UserModel::default()))
    }
}