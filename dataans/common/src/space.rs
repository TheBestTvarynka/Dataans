use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::note::{File, FileId};
use crate::{CreationDate, UpdateDate};

/// Represent a space ID.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Id(Uuid);

impl Id {
    /// Returns the inner ID.
    pub fn inner(&self) -> Uuid {
        self.0
    }
}

impl AsRef<Uuid> for Id {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<Id> for Uuid {
    fn from(value: Id) -> Self {
        value.0
    }
}

/// Represents a space name.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Name<'name>(Cow<'name, str>);

impl From<String> for Name<'static> {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl From<Name<'_>> for String {
    fn from(value: Name<'_>) -> Self {
        match value.0 {
            Cow::Borrowed(s) => s.to_owned(),
            Cow::Owned(s) => s,
        }
    }
}

impl Display for Name<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<'name> From<&'name str> for Name<'name> {
    fn from(value: &'name str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl AsRef<str> for Name<'_> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// Represents space avatar file name.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Avatar<'avatar> {
    id: FileId,
    path: Cow<'avatar, str>,
}

impl<'avatar> Avatar<'avatar> {
    /// Creates a new [Avatar] based on `id` and `path`.
    pub fn new(id: FileId, path: impl Into<Cow<'avatar, str>>) -> Self {
        Self { id, path: path.into() }
    }

    /// Returns avatar [Uuid].
    pub fn id(&self) -> Uuid {
        *self.id.as_ref()
    }

    /// Returns path to the avatar file.
    pub fn path(&self) -> &str {
        self.path.as_ref()
    }
}

impl From<File> for Avatar<'_> {
    fn from(file: File) -> Self {
        let File {
            id,
            name: _,
            path,
            status: _,
        } = file;

        Self {
            id,
            path: path.to_str().expect("UTF8-path").to_owned().into(),
        }
    }
}

/// Represents a space.
///
/// Space - a collection of notes.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(from = "SerializedSpace<'name, 'avatar>")]
pub struct Space<'name, 'avatar> {
    /// Space ID.
    pub id: Id,
    /// Space name.
    pub name: Name<'name>,
    /// Creation date.
    pub created_at: CreationDate,
    /// Update date.
    pub updated_at: UpdateDate,
    /// Avatar image name.
    pub avatar: Avatar<'avatar>,
    /// Creation date of the newest non-deleted space note.
    ///
    /// INVARIANT: The value is the latest of [Space::created_at] and the creation dates of all non-deleted space notes.
    /// So it equals [Space::created_at] when the space has no notes.
    /// The space creation date is also taken into account when the space has notes,
    /// because a note can be older than its space when device clocks differ.
    /// This keeps the value monotonic on note creation, so it can be updated without knowing the other space notes.
    pub newest_note: CreationDate,
}

impl Space<'_, '_> {
    /// Records that a note created at `note_created_at` has been added to the space.
    pub fn note_added(&mut self, note_created_at: &CreationDate) {
        if self.newest_note < *note_created_at {
            self.newest_note = note_created_at.clone();
        }
    }

    /// Recomputes [Space::newest_note] after a note deletion.
    ///
    /// `newest_note_created_at` is the creation date of the newest remaining space note, or [None] if no notes are left.
    pub fn note_deleted(&mut self, newest_note_created_at: Option<&CreationDate>) {
        self.newest_note = newest_note_created_at
            .filter(|newest_note_created_at| self.created_at < **newest_note_created_at)
            .unwrap_or(&self.created_at)
            .clone();
    }
}

/// [Space] as it appears in serialized data.
///
/// JSON exports made by older app versions do not have the `newest_note` field.
/// Then it falls back to the space creation date, so the [Space::newest_note] invariant holds.
#[derive(Deserialize)]
struct SerializedSpace<'name, 'avatar> {
    id: Id,
    name: Name<'name>,
    created_at: CreationDate,
    updated_at: UpdateDate,
    avatar: Avatar<'avatar>,
    newest_note: Option<CreationDate>,
}

impl<'name, 'avatar> From<SerializedSpace<'name, 'avatar>> for Space<'name, 'avatar> {
    fn from(space: SerializedSpace<'name, 'avatar>) -> Self {
        let SerializedSpace {
            id,
            name,
            created_at,
            updated_at,
            avatar,
            newest_note,
        } = space;

        Self {
            id,
            name,
            newest_note: newest_note.unwrap_or_else(|| created_at.clone()),
            created_at,
            updated_at,
            avatar,
        }
    }
}

/// Sorts spaces from the most recently active to the least recently active.
///
/// Spaces are ordered by [Space::newest_note] in descending order.
/// Spaces with equal dates are ordered by id, so the order is deterministic.
/// This is the only place that defines the spaces order: the backend and the frontend must both use it.
pub fn sort_spaces(spaces: &mut [Space<'_, '_>]) {
    spaces.sort_by(|a, b| b.newest_note.cmp(&a.newest_note).then_with(|| a.id.cmp(&b.id)));
}

/// Owned version of [Space].
pub type OwnedSpace = Space<'static, 'static>;

/// Data that the app need to create the space.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct CreateSpace<'name, 'avatar> {
    /// Space ID.
    pub id: Id,
    /// Space name.
    pub name: Name<'name>,
    /// Avatar image name.
    pub avatar: Avatar<'avatar>,
}

/// Owned version of [CreateSpace];
pub type CreateSpaceOwned = CreateSpace<'static, 'static>;

/// Data that the app need to update the space.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateSpace<'name> {
    /// Space ID.
    pub id: Id,
    /// Space name.
    pub name: Name<'name>,
    /// Space avatar.
    pub avatar: Avatar<'static>,
}

/// Data that the app need to delete the space.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteSpace {
    /// Space ID.
    pub id: Id,
}

#[cfg(test)]
mod tests {
    use time::OffsetDateTime;
    use uuid::Uuid;

    use super::*;

    fn date(unix_timestamp: i64) -> CreationDate {
        OffsetDateTime::from_unix_timestamp(unix_timestamp)
            .expect("valid timestamp")
            .into()
    }

    fn space(id: u128, created_at: i64, newest_note: i64) -> OwnedSpace {
        Space {
            id: Uuid::from_u128(id).into(),
            name: Name::from(id.to_string()),
            created_at: date(created_at),
            updated_at: OffsetDateTime::from(date(created_at)).into(),
            avatar: Avatar::new(Uuid::nil().into(), ""),
            newest_note: date(newest_note),
        }
    }

    fn ids(spaces: &[OwnedSpace]) -> Vec<u128> {
        spaces.iter().map(|space| space.id.inner().as_u128()).collect()
    }

    #[test]
    fn sorts_by_newest_note() {
        let mut spaces = vec![space(1, 10, 20), space(2, 50, 50), space(3, 5, 40), space(4, 1, 1)];

        sort_spaces(&mut spaces);

        assert_eq!(ids(&spaces), [2, 3, 1, 4]);
    }

    #[test]
    fn breaks_ties_by_id() {
        let mut spaces = vec![space(3, 10, 10), space(1, 0, 10), space(2, 10, 10)];

        sort_spaces(&mut spaces);

        assert_eq!(ids(&spaces), [1, 2, 3]);
    }

    #[test]
    fn note_added_keeps_the_newest_date() {
        let mut space = space(1, 0, 20);

        space.note_added(&date(10));
        assert_eq!(space.newest_note, date(20));

        space.note_added(&date(30));
        assert_eq!(space.newest_note, date(30));
    }

    #[test]
    fn note_deleted_falls_back_to_space_creation_date() {
        let mut space = space(1, 10, 30);

        space.note_deleted(Some(&date(20)));
        assert_eq!(space.newest_note, date(20));

        // A note older than its space.
        space.note_deleted(Some(&date(5)));
        assert_eq!(space.newest_note, date(10));

        space.note_deleted(None);
        assert_eq!(space.newest_note, date(10));
    }

    #[test]
    fn deserializes_space_without_newest_note() {
        let mut value = serde_json::to_value(space(1, 10, 30)).expect("serializable space");
        value.as_object_mut().expect("space object").remove("newest_note");

        let space: OwnedSpace = serde_json::from_value(value).expect("deserializable space");

        assert_eq!(space.newest_note, date(10));
    }
}
