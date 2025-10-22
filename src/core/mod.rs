pub mod app_state;
pub mod chapter_manager;
pub mod novel_manager;
pub mod version_manager;
pub mod formatter;
pub mod stats_manager;
pub mod inspiration_manager;

pub use app_state::AppState;
pub use chapter_manager::{ChapterManager, ChapterNode};
pub use novel_manager::*;
pub use version_manager::*;
pub use formatter::*;
pub use stats_manager::*;
pub use inspiration_manager::*;