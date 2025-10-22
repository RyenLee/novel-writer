use crate::db::{Novel, Chapter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub current_novel: Option<Novel>,
    pub current_chapter: Option<Chapter>,
    pub chapter_tree: Option<ChapterTree>,
    pub editing_mode: EditingMode,
    pub view_settings: ViewSettings,
    pub auto_save_enabled: bool,
    pub last_save_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterTree {
    pub chapters: Vec<Chapter>,
    pub expanded_nodes: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EditingMode {
    Writing,
    Editing,
    Preview,
    Outline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewSettings {
    pub font_size: u32,
    pub font_family: String,
    pub line_height: f32,
    pub theme: Theme,
    pub sidebar_width: u32,
    pub show_word_count: bool,
    pub focus_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Sepia,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_novel: None,
            current_chapter: None,
            chapter_tree: None,
            editing_mode: EditingMode::Writing,
            view_settings: ViewSettings::default(),
            auto_save_enabled: true,
            last_save_time: None,
        }
    }
}

impl Default for ViewSettings {
    fn default() -> Self {
        Self {
            font_size: 16,
            font_family: "system-ui".to_string(),
            line_height: 1.6,
            theme: Theme::Light,
            sidebar_width: 300,
            show_word_count: true,
            focus_mode: false,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn set_current_novel(&mut self, novel: Novel) {
        self.current_novel = Some(novel);
        self.current_chapter = None;
    }
    
    pub fn set_current_chapter(&mut self, chapter: Chapter) {
        self.current_chapter = Some(chapter);
        self.editing_mode = EditingMode::Writing;
    }
    
    pub fn set_editing_mode(&mut self, mode: EditingMode) {
        self.editing_mode = mode;
    }
    
    pub fn toggle_focus_mode(&mut self) {
        self.view_settings.focus_mode = !self.view_settings.focus_mode;
    }
    
    pub fn toggle_theme(&mut self) {
        self.view_settings.theme = match self.view_settings.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Sepia,
            Theme::Sepia => Theme::Light,
        };
    }
    
    pub fn update_word_count(&mut self, word_count: u32) {
        if let Some(ref mut chapter) = self.current_chapter {
            chapter.word_count = word_count as i32;
        }
    }
    
    pub fn should_auto_save(&self) -> bool {
        self.auto_save_enabled && 
        self.last_save_time.map_or(true, |last_save| {
            chrono::Utc::now().signed_duration_since(last_save).num_seconds() > 30 // 30秒自动保存
        })
    }
    
    pub fn mark_saved(&mut self) {
        self.last_save_time = Some(chrono::Utc::now());
    }
}