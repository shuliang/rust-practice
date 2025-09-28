mod get;
mod post;

pub use get::change_password_form;
pub use post::change_password;

pub const PASSWORD_LEN_SHORTEST: usize = 12;
pub const PASSWORD_LEN_LONGEST: usize = 128;
