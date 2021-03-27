// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use super::ConfigKey;
use crate::prelude::*;

use strum::IntoStaticStr;

/// Notetype config packed into a collection config key. This may change
/// frequently, and we want to avoid the potentially expensive notetype
/// write/sync.
#[derive(Debug, Clone, Copy, IntoStaticStr)]
#[strum(serialize_all = "camelCase")]
enum NoteTypeConfigKey {
    #[strum(to_string = "lastDeck")]
    LastDeckAddedTo,
}

impl NoteTypeConfigKey {
    fn for_notetype(self, ntid: NoteTypeId) -> String {
        build_aux_notetype_key(ntid, <&'static str>::from(self))
    }
}

impl Collection {
    #[allow(dead_code)]
    pub(crate) fn get_current_notetype_id(&self) -> Option<NoteTypeId> {
        self.get_config_optional(ConfigKey::CurrentNoteTypeId)
    }

    pub(crate) fn set_current_notetype_id(&mut self, ntid: NoteTypeId) -> Result<()> {
        self.set_config(ConfigKey::CurrentNoteTypeId, &ntid)
    }

    pub(crate) fn clear_aux_config_for_notetype(&self, ntid: NoteTypeId) -> Result<()> {
        self.remove_config_prefix(&build_aux_notetype_key(ntid, ""))
    }

    pub(crate) fn get_last_deck_added_to_for_notetype(&self, id: NoteTypeId) -> Option<DeckId> {
        let key = NoteTypeConfigKey::LastDeckAddedTo.for_notetype(id);
        self.get_config_optional(key.as_str())
    }

    pub(crate) fn set_last_deck_for_notetype(&mut self, id: NoteTypeId, did: DeckId) -> Result<()> {
        let key = NoteTypeConfigKey::LastDeckAddedTo.for_notetype(id);
        self.set_config(key.as_str(), &did)
    }
}

fn build_aux_notetype_key(ntid: NoteTypeId, key: &str) -> String {
    format!("_nt_{ntid}_{key}", ntid = ntid, key = key)
}
