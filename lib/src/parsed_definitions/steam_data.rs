//! Additional data specific to the steam workshop that may be included in the `info.txt` file for a raw module.

use serde::{Deserialize, Serialize};

/// The additional data specific to the steam workshop
#[derive(Serialize, Deserialize, Default, Clone, Debug, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SteamData {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_value_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    changelog: Option<String>,
    file_id: u64,
}

impl SteamData {
    /// Returns whether the steam data is empty
    ///
    /// # Returns
    ///
    /// * `true` if the steam data is empty, `false` otherwise.
    #[allow(dead_code)]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.description.is_none()
            && self.tags.is_none()
            && self.key_value_tags.is_none()
            && self.metadata.is_none()
            && self.changelog.is_none()
            && self.file_id == 0
    }
    /// Sets the title of the steam data
    ///
    /// # Arguments
    ///
    /// * `title` - The title to set
    pub fn set_title(&mut self, title: &str) {
        self.title = Some(String::from(title));
    }
    pub fn get_title(&self) -> Option<String> {
        self.title.clone()
    }
    /// Sets the description of the steam data
    ///
    /// # Arguments
    ///
    /// * `description` - The description to set
    pub fn set_description(&mut self, description: &str) {
        self.description = Some(String::from(description));
    }
    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
    /// Sets the changelog of the steam data
    ///
    /// # Arguments
    ///
    /// * `changelog` - The changelog to set
    pub fn set_changelog(&mut self, changelog: &str) {
        self.changelog = Some(String::from(changelog));
    }
    pub fn get_changelog(&self) -> Option<String> {
        self.changelog.clone()
    }
    /// Sets the file id of the steam data
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file id to set
    pub fn set_file_id(&mut self, file_id: u64) {
        self.file_id = file_id;
    }
    pub fn get_file_id(&self) -> u64 {
        self.file_id
    }
    /// Adds a tag to the steam data
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag to add
    pub fn add_tag(&mut self, tag: &str) {
        if self.tags.is_none() {
            self.tags = Some(Vec::new());
        }

        if let Some(tags) = &mut self.tags {
            tags.push(String::from(tag));
        }
    }
    pub fn get_tags(&self) -> Option<Vec<String>> {
        self.tags.clone()
    }
    /// Adds a key value tag to the steam data
    ///
    /// # Arguments
    ///
    /// * `tag` - The key and value to add
    pub fn add_key_value_tag(&mut self, tag: &str) {
        if self.key_value_tags.is_none() {
            self.key_value_tags = Some(Vec::new());
        }

        if let Some(tags) = &mut self.key_value_tags {
            tags.push(String::from(tag));
        }
    }
    /// Adds metadata to the steam data
    ///
    /// # Arguments
    ///
    /// * `metadata` - The metadata to add
    pub fn add_metadata(&mut self, metadata: &str) {
        if self.metadata.is_none() {
            self.metadata = Some(Vec::new());
        }

        if let Some(self_metadata) = &mut self.metadata {
            self_metadata.push(String::from(metadata));
        }
    }
}
