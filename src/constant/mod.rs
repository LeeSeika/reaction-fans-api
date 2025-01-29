mod constant;

pub use constant::*;

pub const REGISTER_CODE_EXPIRE_TIME: u64 = 60 * 5;

// order
// pub enum OrderBy {
//     CreatedAt = "created_at".parse().unwrap(),
//     UpdatedAt,
//     PostedAt,
//     Rating,
//     ViewCount,
// }
// pub const ORDER_BY_CREATED_AT : &str = "created_at";
// pub const ORDER_BY_UPDATED_AT : &str = "updated_at";
// pub const ORDER_BY_POSTED_AT : &str = "posted_at";
// pub const ORDER_BY_RATING : &str = "rating";
// pub const ORDER_BY_VIEW_COUNT : &str = "view_count";
//
// // order direction
// pub const ORDER_DIRECTION_ASC : &str = "asc";
// pub const ORDER_DIRECTION_DESC : &str = "desc";