mod login_oauth_qq;
mod register;
mod svc;

use crate::entity::user::Column as UserColumn;
use crate::entity::user::Entity as UserModel;

pub use svc::new;
pub use svc::UserService;
