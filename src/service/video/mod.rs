mod svc;
mod get_video;

use crate::entity::video::ActiveModel as VideoActiveModel;
use crate::entity::video::Column as VideoColumn;
use crate::entity::video::Entity as VideoEntity;
use crate::entity::video::Model as VideoModel;

pub use svc::VideoService;
pub use svc::new;