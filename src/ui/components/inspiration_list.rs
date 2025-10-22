// 灵感列表组件 - 简化版本
// 由于Dioxus版本兼容性问题，这里提供一个简化的组件结构
// 实际实现需要根据具体的UI框架进行调整

use crate::db;

pub struct InspirationList;

impl InspirationList {
    pub fn new() -> Self {
        Self
    }
    
    // 获取小说灵感列表
    pub fn get_inspirations(&self, novel_id: i64) -> Result<Vec<crate::core::inspiration_manager::Inspiration>, Box<dyn std::error::Error>> {
        let db = db::get_database()?;
        db.get_inspirations_by_novel(novel_id).map_err(|e| e.into())
    }
    
    // 搜索灵感
    pub fn search_inspirations(&self, novel_id: i64, query: &str) -> Result<Vec<crate::core::inspiration_manager::Inspiration>, Box<dyn std::error::Error>> {
        let db = db::get_database()?;
        db.search_inspirations(novel_id, query).map_err(|e| e.into())
    }
    
    // 创建新灵感
    pub fn create_inspiration(&self, novel_id: i64, title: &str, content: &str) -> Result<crate::core::inspiration_manager::Inspiration, Box<dyn std::error::Error>> {
        let db = db::get_database()?;
        db.create_inspiration(novel_id, title, content).map_err(|e| e.into())
    }
    
    // 更新灵感
    pub fn update_inspiration(&self, inspiration_id: i64, title: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let db = db::get_database()?;
        db.update_inspiration(inspiration_id, title, content).map_err(|e| e.into())
    }
    
    // 删除灵感
    pub fn delete_inspiration(&self, inspiration_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let db = db::get_database()?;
        db.delete_inspiration(inspiration_id).map_err(|e| e.into())
    }
    
    // 切换置顶状态
    pub fn toggle_pin(&self, inspiration_id: i64) -> Result<bool, Box<dyn std::error::Error>> {
        let db = db::get_database()?;
        db.toggle_inspiration_pin(inspiration_id).map_err(|e| e.into())
    }
    
    // 添加标签
    pub fn add_tags(&self, inspiration_id: i64, tags: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let db = db::get_database()?;
        db.add_inspiration_tags(inspiration_id, tags).map_err(|e| e.into())
    }
    
    // 关联章节
    pub fn link_chapter(&self, inspiration_id: i64, chapter_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let db = db::get_database()?;
        db.link_inspiration_to_chapter(inspiration_id, chapter_id).map_err(|e| e.into())
    }
}