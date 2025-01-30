use serde::{Deserialize, Serialize};

use crate::profile::UserContext;

/// An event name for the [UserContextEvent].
///
/// It includes sign in, sign out, and related events.
pub const USER_CONTEXT_EVENT: &str = "user-context";

/// An event related to the user context.
///
/// It includes sign in, sign out, and related events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserContextEvent {
    /// User signed in.
    SignedIn(UserContext),
    /// User context has been updates.
    ContextUpdated(UserContext),
    /// User signed out.
    SignedOut,
}
