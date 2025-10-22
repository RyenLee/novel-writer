mod models;
mod migrations;

use anyhow::Result;
use rusqlite::{Connection, params};
pub use models::*;
use chrono::{DateTime, Utc};
use crate::core::inspiration_manager::Inspiration;
use log::{info, warn, error};

use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DB: Mutex<Option<Database>> = Mutex::new(None);
}

pub struct Database {
    conn: Connection,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        // 重新创建数据库连接而不是克隆
        Database::new().expect("Failed to clone database connection")
    }
}

impl Database {
    pub fn new() -> Result<Self> {
        info!("Creating data directory if it doesn't exist...");
        std::fs::create_dir_all("data")?;
        info!("Data directory created successfully");
        
        info!("Opening database connection to data/novels.db");
        let conn = Connection::open("data/novels.db")?;
        info!("Database connection established successfully");
        
        info!("Running database migrations...");
        migrations::run_migrations(&conn)?;
        info!("Database migrations completed successfully");
        
        Ok(Self { conn })
    }
    
    // 小说操作
    pub fn create_novel(&self, title: &str) -> Result<Novel> {
        let novel = Novel {
            id: 0,
            title: title.to_string(),
            author: "".to_string(),
            description: "".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            word_count: 0,
            status: NovelStatus::Draft,
            is_pinned: false,
            pinned_order: None,
        };
        
        self.conn.execute(
            "INSERT INTO novels (title, author, description, created_at, updated_at, word_count, status) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                novel.title,
                novel.author,
                novel.description,
                novel.created_at.to_rfc3339(),
                novel.updated_at.to_rfc3339(),
                novel.word_count,
                novel.status.as_str(),
            ],
        )?;
        
        let id = self.conn.last_insert_rowid();
        Ok(Novel { id, ..novel })
    }
    
    pub fn get_all_novels(&self) -> Result<Vec<Novel>> {
        log::debug!("查询所有小说数据");
        let mut stmt = self.conn.prepare(
            "SELECT id, title, author, description, created_at, updated_at, word_count, status 
             FROM novels ORDER BY updated_at DESC"
        )?;
        
        let novel_iter = stmt.query_map([], |row| {
            // 获取字段
            let id = row.get(0)?;
            let title = row.get(1)?;
            let author = row.get(2)?;
            let description = row.get(3)?;
            
            // 处理created_at字段，添加更健壮的错误处理
            let created_at_str: String = match row.get(4) {
                Ok(s) => s,
                Err(e) => {
                    log::warn!("获取created_at字段失败: {}, 使用当前时间", e);
                    current_timestamp()
                }
            };
            
            let created_at = match DateTime::parse_from_rfc3339(&created_at_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    log::warn!("解析created_at时间失败: {}，格式: {}, 使用当前时间", e, created_at_str);
                    Utc::now()
                }
            };
            
            // 处理updated_at字段，添加更健壮的错误处理
            let updated_at_str: String = match row.get(5) {
                Ok(s) => s,
                Err(e) => {
                    log::warn!("获取updated_at字段失败: {}, 使用当前时间", e);
                    current_timestamp()
                }
            };
            
            let updated_at = match DateTime::parse_from_rfc3339(&updated_at_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    log::warn!("解析updated_at时间失败: {}，格式: {}, 使用当前时间", e, updated_at_str);
                    Utc::now()
                }
            };
            
            let word_count = row.get(6)?;
            let status_str: String = match row.get(7) {
                Ok(s) => s,
                Err(e) => {
                    log::warn!("获取status字段失败: {}, 使用默认状态Draft", e);
                    "draft".to_string()
                }
            };
            
            Ok(Novel {
                id,
                title,
                author,
                description,
                created_at,
                updated_at,
                word_count,
                status: NovelStatus::from_str(&status_str),
                is_pinned: false,
                pinned_order: None,
            })
        })?;
        
        let mut novels = Vec::new();
        for novel in novel_iter {
            novels.push(novel?);
        }
        
        Ok(novels)
    }
    
    pub fn get_novel_by_id(&self, novel_id: i64) -> Result<Option<Novel>> {
        let novels = self.get_all_novels()?;
        Ok(novels.into_iter().find(|n| n.id == novel_id))
    }
    
    pub fn update_novel(&self, novel: &Novel) -> Result<()> {
        self.conn.execute(
            "UPDATE novels SET title = ?1, author = ?2, description = ?3, updated_at = ?4, word_count = ?5, status = ?6 WHERE id = ?7",
            params![
                novel.title,
                novel.author,
                novel.description,
                novel.updated_at.to_rfc3339(),
                novel.word_count,
                novel.status.as_str(),
                novel.id,
            ],
        )?;
        Ok(())
    }
    
    pub fn delete_novel(&self, novel_id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM novels WHERE id = ?1", [novel_id])?;
        Ok(())
    }
    
    // 章节操作
    pub fn create_chapter(&self, novel_id: i64, title: &str, parent_id: Option<i64>) -> Result<Chapter> {
        let sort_path = self.calculate_next_sort_path(novel_id, parent_id)?;
        
        // 即使是空内容，也使用统一的字数统计方法
        let content = "".to_string();
        let word_count = content.chars().filter(|c| !c.is_whitespace()).count() as i32;
        
        let chapter = Chapter {
            id: 0,
            novel_id,
            parent_id,
            title: title.to_string(),
            content,
            sort_path,
            word_count,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            chapter_type: ChapterType::Chapter,
            is_archived: false,
        };
        
        self.conn.execute(
            "INSERT INTO chapters (novel_id, parent_id, title, content, sort_path, word_count, created_at, updated_at, chapter_type, is_archived) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                chapter.novel_id,
                chapter.parent_id,
                chapter.title,
                chapter.content,
                chapter.sort_path,
                chapter.word_count,
                chapter.created_at.to_rfc3339(),
                chapter.updated_at.to_rfc3339(),
                chapter.chapter_type.as_str(),
                chapter.is_archived,
            ],
        )?;
        
        let id = self.conn.last_insert_rowid();
        Ok(Chapter { id, ..chapter })
    }
    
    pub fn get_chapters_by_novel(&self, novel_id: i64) -> Result<Vec<Chapter>> {
        log::debug!("查询小说 {} 的章节数据", novel_id);
        let mut stmt = self.conn.prepare(
            "SELECT id, novel_id, parent_id, title, content, sort_path, word_count, created_at, updated_at, chapter_type, is_archived 
             FROM chapters WHERE novel_id = ?1 AND is_archived = 0 ORDER BY sort_path"
        )?;
        
        let chapter_iter = stmt.query_map([novel_id], |row| {
            // 获取字段
            let id = row.get(0)?;
            let novel_id = row.get(1)?;
            let parent_id = row.get(2)?;
            let title = row.get(3)?;
            let content = row.get(4)?;
            let sort_path = row.get(5)?;
            let word_count = row.get(6)?;
            
            // 处理created_at字段，添加更健壮的错误处理
            let created_at_str: String = match row.get(7) {
                Ok(s) => s,
                Err(e) => {
                    log::warn!("获取chapter created_at字段失败: {}, 使用当前时间", e);
                    current_timestamp()
                }
            };
            
            let created_at = match DateTime::parse_from_rfc3339(&created_at_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    log::warn!("解析chapter created_at时间失败: {}，格式: {}, 使用当前时间", e, created_at_str);
                    Utc::now()
                }
            };
            
            // 处理updated_at字段，添加更健壮的错误处理
            let updated_at_str: String = match row.get(8) {
                Ok(s) => s,
                Err(e) => {
                    log::warn!("获取chapter updated_at字段失败: {}, 使用当前时间", e);
                    current_timestamp()
                }
            };
            
            let updated_at = match DateTime::parse_from_rfc3339(&updated_at_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    log::warn!("解析chapter updated_at时间失败: {}，格式: {}, 使用当前时间", e, updated_at_str);
                    Utc::now()
                }
            };
            
            // 处理chapter_type字段
            let chapter_type_str: String = match row.get(9) {
                Ok(s) => s,
                Err(e) => {
                    log::warn!("获取chapter_type字段失败: {}, 使用默认值Chapter", e);
                    "chapter".to_string()
                }
            };
            
            let chapter_type = ChapterType::from_str(&chapter_type_str);
            let is_archived = row.get(10)?;
            
            Ok(Chapter {
                id,
                novel_id,
                parent_id,
                title,
                content,
                sort_path,
                word_count,
                created_at,
                updated_at,
                chapter_type,
                is_archived,
            })
        })?;
        
        let mut chapters = Vec::new();
        for chapter in chapter_iter {
            match chapter {
                Ok(chapter) => chapters.push(chapter),
                Err(e) => {
                    log::error!("解析章节数据失败: {}, 跳过该章节", e);
                }
            }
        }
        
        log::debug!("成功查询到 {} 个章节", chapters.len());
        Ok(chapters)
    }
    
    pub fn update_chapter_content(&self, chapter_id: i64, content: &str) -> Result<()> {
        // 改进的字数统计方法：统计所有非空白字符，对中英文都更准确
        let word_count = content.chars().filter(|c| !c.is_whitespace()).count() as i32;
        
        self.conn.execute(
            "UPDATE chapters SET content = ?1, word_count = ?2, updated_at = ?3 WHERE id = ?4",
            params![
                content,
                word_count,
                chrono::Utc::now().to_rfc3339(),
                chapter_id,
            ],
        )?;
        
        Ok(())
    }
    
    // 移除思维导图相关功能
    
    fn calculate_next_sort_path(&self, _novel_id: i64, _parent_id: Option<i64>) -> Result<String> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        Ok(format!("{:020}", timestamp))
    }


    pub fn get_chapter(&self, chapter_id: i64) -> Result<Chapter> {
        let mut stmt = self.conn.prepare(
            "SELECT id, novel_id, parent_id, title, content, sort_path, word_count, created_at, updated_at, chapter_type, is_archived 
             FROM chapters WHERE id = ?1"
        )?;
        
        stmt.query_row([chapter_id], |row| {
            Ok(Chapter {
                id: row.get(0)?,
                novel_id: row.get(1)?,
                parent_id: row.get(2)?,
                title: row.get(3)?,
                content: row.get(4)?,
                sort_path: row.get(5)?,
                word_count: row.get(6)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(7, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc),
                chapter_type: ChapterType::from_str(&row.get::<_, String>(9)?),
                is_archived: row.get(10)?,
            })
        }).map_err(|e| e.into())
    }
    
    pub fn update_chapter_parent(&self, chapter_id: i64, parent_id: Option<i64>, sort_path: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE chapters SET parent_id = ?1, sort_path = ?2, updated_at = ?3 WHERE id = ?4",
            params![parent_id, sort_path, chrono::Utc::now().to_rfc3339(), chapter_id],
        )?;
        Ok(())
    }
    
    pub fn update_chapter(&self, chapter: &Chapter) -> Result<()> {
        self.conn.execute(
            "UPDATE chapters SET title = ?1, content = ?2, word_count = ?3, chapter_type = ?4, updated_at = ?5 WHERE id = ?6",
            params![
                chapter.title,
                chapter.content,
                chapter.word_count,
                chapter.chapter_type.as_str(),
                chrono::Utc::now().to_rfc3339(),
                chapter.id,
            ],
        )?;
        Ok(())
    }
    
    pub fn delete_chapter(&self, chapter_id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM chapters WHERE id = ?1", [chapter_id])?;
        Ok(())
    }
    
    pub fn create_chapter_version(&self, version: ChapterVersion) -> Result<ChapterVersion> {
        self.conn.execute(
            "INSERT INTO chapter_versions (chapter_id, parent_version_id, version_type, content, diff_data, word_count, created_at, commit_message, is_auto_save) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                version.chapter_id,
                version.parent_version_id,
                version.version_type.as_str(),
                version.content,
                version.diff_data,
                version.word_count,
                version.created_at.to_rfc3339(),
                version.commit_message,
                version.is_auto_save,
            ],
        )?;
        
        let id = self.conn.last_insert_rowid();
        Ok(ChapterVersion { id, ..version })
    }
    
    pub fn get_chapter_versions(&self, chapter_id: i64) -> Result<Vec<ChapterVersion>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, chapter_id, parent_version_id, version_type, content, diff_data, word_count, created_at, commit_message, is_auto_save 
             FROM chapter_versions WHERE chapter_id = ?1 ORDER BY created_at DESC"
        )?;
        
        let version_iter = stmt.query_map([chapter_id], |row| {
            Ok(ChapterVersion {
                id: row.get(0)?,
                chapter_id: row.get(1)?,
                parent_version_id: row.get(2)?,
                version_type: VersionType::from_str(&row.get::<_, String>(3)?),
                content: row.get(4)?,
                diff_data: row.get(5)?,
                word_count: row.get(6)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(7, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc),
                commit_message: row.get(8)?,
                is_auto_save: row.get(9)?,
            })
        })?;
        
        let mut versions = Vec::new();
        for version in version_iter {
            versions.push(version?);
        }
        
        Ok(versions)
    }
    
    pub fn get_chapter_version(&self, version_id: i64) -> Result<ChapterVersion> {
        let mut stmt = self.conn.prepare(
            "SELECT id, chapter_id, parent_version_id, version_type, content, diff_data, word_count, created_at, commit_message, is_auto_save 
             FROM chapter_versions WHERE id = ?1"
        )?;
        
        stmt.query_row([version_id], |row| {
            Ok(ChapterVersion {
                id: row.get(0)?,
                chapter_id: row.get(1)?,
                parent_version_id: row.get(2)?,
                version_type: VersionType::from_str(&row.get::<_, String>(3)?),
                content: row.get(4)?,
                diff_data: row.get(5)?,
                word_count: row.get(6)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(7, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc),
                commit_message: row.get(8)?,
                is_auto_save: row.get(9)?,
            })
        }).map_err(|e| e.into())
    }
}

pub fn init_database() -> Result<()> {
    info!("Initializing database...");
    let db = Database::new()?;
    info!("Database instance created successfully");
    
    info!("Locking database mutex...");
    match DB.lock() {
        Ok(mut guard) => {
            *guard = Some(db);
            info!("Database initialized and stored in mutex successfully");
            Ok(())
        },
        Err(e) => {
            error!("Failed to lock database mutex: {}", e);
            Err(anyhow::anyhow!("Failed to lock database mutex: {}", e))
        }
    }
}

pub fn get_database() -> Result<Database> {
    info!("Getting database connection...");
    match DB.lock() {
        Ok(guard) => {
            match guard.as_ref().cloned() {
                Some(db) => {
                    info!("Database connection retrieved successfully");
                    Ok(db)
                },
                None => {
                    error!("Attempted to get database before initialization");
                    Err(anyhow::anyhow!("Database not initialized"))
                }
            }
        },
        Err(e) => {
            error!("Failed to lock database mutex: {}", e);
            Err(anyhow::anyhow!("Failed to access database: {}", e))
        }
    }
}

// 灵感相关操作方法
impl Database {
    pub fn create_inspiration(&self, novel_id: i64, title: &str, content: &str) -> Result<Inspiration> {
        let now = current_timestamp();
        let mut stmt = self.conn.prepare(
            "INSERT INTO inspirations (novel_id, title, content, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?4)"
        )?;
        
        stmt.execute(params![novel_id, title, content, now])?;
        let id = self.conn.last_insert_rowid();
        
        Ok(Inspiration {
            id,
            novel_id,
            title: title.to_string(),
            content: content.to_string(),
            created_at: now.clone(),
            updated_at: now,
            is_pinned: false,
            tags: Vec::new(),
            linked_chapters: Vec::new(),
        })
    }
    
    pub fn get_inspirations_by_novel(&self, novel_id: i64) -> Result<Vec<Inspiration>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, novel_id, title, content, created_at, updated_at, is_pinned 
             FROM inspirations WHERE novel_id = ?1 ORDER BY is_pinned DESC, updated_at DESC"
        )?;
        
        let rows = stmt.query_map(params![novel_id], |row| {
            Ok(Inspiration {
                id: row.get(0)?,
                novel_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                is_pinned: row.get(6)?,
                tags: Vec::new(),
                linked_chapters: Vec::new(),
            })
        })?;

        let mut inspirations: Vec<Inspiration> = rows.collect::<Result<_, _>>()?;
        
        // 加载标签和关联章节
        for insp in &mut inspirations {
            insp.tags = self.get_inspiration_tags(insp.id)?;
            insp.linked_chapters = self.get_inspiration_linked_chapters(insp.id)?;
        }

        Ok(inspirations)
    }
    
    pub fn get_inspiration(&self, inspiration_id: i64) -> Result<Option<Inspiration>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, novel_id, title, content, created_at, updated_at, is_pinned 
             FROM inspirations WHERE id = ?1"
        )?;
        
        let mut rows = stmt.query_map(params![inspiration_id], |row| {
            Ok(Inspiration {
                id: row.get(0)?,
                novel_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                is_pinned: row.get(6)?,
                tags: Vec::new(),
                linked_chapters: Vec::new(),
            })
        })?;

        if let Some(row) = rows.next() {
            let mut inspiration = row?;
            inspiration.tags = self.get_inspiration_tags(inspiration.id)?;
            inspiration.linked_chapters = self.get_inspiration_linked_chapters(inspiration.id)?;
            Ok(Some(inspiration))
        } else {
            Ok(None)
        }
    }
    
    pub fn update_inspiration(&self, inspiration_id: i64, title: &str, content: &str) -> Result<()> {
        let now = current_timestamp();
        self.conn.execute(
            "UPDATE inspirations SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
            params![title, content, now, inspiration_id],
        )?;
        Ok(())
    }
    
    pub fn delete_inspiration(&self, inspiration_id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM inspirations WHERE id = ?1", params![inspiration_id])?;
        Ok(())
    }
    
    pub fn toggle_inspiration_pin(&self, inspiration_id: i64) -> Result<bool> {
        let current: bool = self.conn.query_row(
            "SELECT is_pinned FROM inspirations WHERE id = ?1",
            params![inspiration_id],
            |row| row.get(0),
        )?;
        
        let new_value = !current;
        self.conn.execute(
            "UPDATE inspirations SET is_pinned = ?1, updated_at = ?2 WHERE id = ?3",
            params![new_value, current_timestamp(), inspiration_id],
        )?;
        
        Ok(new_value)
    }
    
    pub fn add_inspiration_tags(&self, inspiration_id: i64, tags: &[String]) -> Result<()> {
        for tag in tags {
            if !tag.trim().is_empty() {
                self.conn.execute(
                    "INSERT OR IGNORE INTO inspiration_tags (inspiration_id, tag) VALUES (?1, ?2)",
                    params![inspiration_id, tag.trim()],
                )?;
            }
        }
        Ok(())
    }
    
    pub fn remove_inspiration_tag(&self, inspiration_id: i64, tag: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM inspiration_tags WHERE inspiration_id = ?1 AND tag = ?2",
            params![inspiration_id, tag],
        )?;
        Ok(())
    }
    
    pub fn link_inspiration_to_chapter(&self, inspiration_id: i64, chapter_id: i64) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO inspiration_chapter_links (inspiration_id, chapter_id) VALUES (?1, ?2)",
            params![inspiration_id, chapter_id],
        )?;
        Ok(())
    }
    
    pub fn unlink_inspiration_from_chapter(&self, inspiration_id: i64, chapter_id: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM inspiration_chapter_links WHERE inspiration_id = ?1 AND chapter_id = ?2",
            params![inspiration_id, chapter_id],
        )?;
        Ok(())
    }
    
    pub fn search_inspirations(&self, novel_id: i64, query: &str) -> Result<Vec<Inspiration>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, novel_id, title, content, created_at, updated_at, is_pinned 
             FROM inspirations 
             WHERE novel_id = ?1 AND (title LIKE ?2 OR content LIKE ?2)
             ORDER BY is_pinned DESC, updated_at DESC"
        )?;
        
        let search_pattern = format!("%{}%", query);
        let rows = stmt.query_map(params![novel_id, search_pattern], |row| {
            Ok(Inspiration {
                id: row.get(0)?,
                novel_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                is_pinned: row.get(6)?,
                tags: Vec::new(),
                linked_chapters: Vec::new(),
            })
        })?;

        let mut inspirations: Vec<Inspiration> = rows.collect::<Result<_, _>>()?;
        
        for insp in &mut inspirations {
            insp.tags = self.get_inspiration_tags(insp.id)?;
            insp.linked_chapters = self.get_inspiration_linked_chapters(insp.id)?;
        }

        Ok(inspirations)
    }
    
    fn get_inspiration_tags(&self, inspiration_id: i64) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT tag FROM inspiration_tags WHERE inspiration_id = ?1",
        )?;
        
        let tags = stmt.query_map(params![inspiration_id], |row| row.get::<_, String>(0))?;
        tags.collect::<Result<Vec<String>, _>>().map_err(|e| e.into())
    }
    
    fn get_inspiration_linked_chapters(&self, inspiration_id: i64) -> Result<Vec<i64>> {
        let mut stmt = self.conn.prepare(
            "SELECT chapter_id FROM inspiration_chapter_links WHERE inspiration_id = ?1",
        )?;
        
        let chapters = stmt.query_map(params![inspiration_id], |row| row.get(0))?;
        chapters.collect::<Result<Vec<i64>, _>>().map_err(|e| e.into())
    }
}

pub fn current_timestamp() -> String {
    chrono::Utc::now().to_rfc3339()
}


