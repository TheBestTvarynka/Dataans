use serde::{Deserialize, Serialize};

use crate::note::{Id as NoteId, OwnedNote};
use crate::profile::UserContext;
use crate::space::{Id as SpaceId, OwnedSpace};

/// An event name for the [UserContextEvent].
///
/// It includes sign in, sign out, and related events.
pub const USER_CONTEXT_EVENT: &str = "user-context";
/// An event name for the [DataEvent].
///
/// This event happens every time the local database is updated (e.g. during the sync process).
pub const DATA_EVENT: &str = "data-event";

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

/// An event related to data changes.
///
/// This event happens every time the local database is updated (e.g. during the sync process).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataEvent {
    /// A new space has been added.
    SpaceAdded(OwnedSpace),
    /// The space has been updated.
    SpaceUpdated(OwnedSpace),
    /// The space has been deleted.
    SpaceDeleted(SpaceId),
    /// A new note has been added.
    NoteAdded(OwnedNote),
    /// The note has been updated.
    NoteUpdated(OwnedNote),
    /// The note has been deleted.
    NoteDeleted(SpaceId, NoteId),
}
