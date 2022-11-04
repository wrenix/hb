//! The group that an [`Account`][crate::account::account::Account] belongs to.
//! 
//! Either `Active` or `Archived`.

pub mod group;
pub mod group_error;
pub mod group_query;

pub use group::Group;
pub use group_error::GroupError;
pub use group_query::QueryGroups;
