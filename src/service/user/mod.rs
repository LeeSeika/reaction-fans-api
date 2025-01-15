mod svc;
mod login_oauth_qq;
mod login_oauth_bilibili;
mod register;

use crate::entity::user::Entity as UserModel;
use crate::entity::user::Column as UserColumn;

pub use svc::UserService;
pub use svc::new;