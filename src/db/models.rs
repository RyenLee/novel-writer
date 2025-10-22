use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use rusqlite::types::ToSql;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Novel {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub word_count: i32,
    pub status: NovelStatus,
    pub is_pinned: bool,
    pub pinned_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NovelStatus {
    Draft,
    Writing,
    Completed,
    Abandoned,
}

impl NovelStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Writing => "writing",
            Self::Completed => "completed",
            Self::Abandoned => "abandoned",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "writing" => Self::Writing,
            "completed" => Self::Completed,
            "abandoned" => Self::Abandoned,
            _ => Self::Draft,
        }
    }
}

impl ToSql for NovelStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.as_str().into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Chapter {
    pub id: i64,
    pub novel_id: i64,
    pub parent_id: Option<i64>,
    pub title: String,
    pub content: String,
    pub sort_path: String,
    pub word_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub chapter_type: ChapterType,
    pub is_archived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChapterType {
    Volume,
    Chapter,
    Scene,
}

impl ChapterType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Volume => "volume",
            Self::Chapter => "chapter",
            Self::Scene => "scene",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "volume" => Self::Volume,
            "scene" => Self::Scene,
            _ => Self::Chapter,
        }
    }
}

impl ToSql for ChapterType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.as_str().into())
    }
}

// 已移除思维导图相关模型定义

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChapterVersion {
    pub id: i64,
    pub chapter_id: i64,
    pub parent_version_id: Option<i64>,
    pub version_type: VersionType,
    pub content: String,
    pub diff_data: Option<String>,
    pub word_count: i32,
    pub created_at: DateTime<Utc>,
    pub commit_message: String,
    pub is_auto_save: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VersionType {
    Snapshot,
    Diff,
}

impl VersionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
            Self::Diff => "diff",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "snapshot" => Self::Snapshot,
            _ => Self::Diff,
        }
    }
}

impl ToSql for VersionType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.as_str().into())
    }
}