use crate::db::{Novel, Chapter, get_database};
use anyhow::Result;
use chrono::{DateTime, Utc};
use log::{info, warn, error};

pub struct NovelManager;

impl NovelManager {
    pub fn new() -> Self {
        Self
    }
    
    /// 创建新小说
    pub async fn create_novel(&self, title: &str, author: Option<&str>) -> Result<Novel> {
        info!("Creating new novel: title='{}', author={:?}", title, author);
        let db = get_database()?;
        let mut novel = db.create_novel(title)?;
        info!("Novel created with ID: {}", novel.id);
        
        if let Some(author_name) = author {
            novel.author = author_name.to_string();
            info!("Setting author for novel {}: '{}'", novel.id, author_name);
            // 更新数据库中的作者信息
            self.update_novel_author(novel.id, author_name).await?;
        }
        
        info!("Novel creation completed successfully: ID={}, title='{}'", novel.id, title);
        Ok(novel)
    }
    
    /// 获取所有小说
    pub async fn get_all_novels(&self) -> Result<Vec<Novel>> {
        info!("Retrieving all novels from database");
        let db = get_database()?;
        let novels = db.get_all_novels()?;
        info!("Retrieved {} novels successfully", novels.len());
        Ok(novels)
    }
    
    /// 根据ID获取小说
    pub async fn get_novel_by_id(&self, novel_id: i64) -> Result<Option<Novel>> {
        info!("Retrieving novel by ID: {}", novel_id);
        let novels = self.get_all_novels().await?;
        let result = novels.into_iter().find(|n| n.id == novel_id);
        
        if let Some(novel) = &result {
            info!("Found novel: ID={}, title='{}'", novel.id, novel.title);
        } else {
            info!("Novel not found: ID={}", novel_id);
        }
        
        Ok(result)
    }
    
    /// 更新小说标题
    pub async fn update_novel_title(&self, novel_id: i64, new_title: &str) -> Result<()> {
        info!("Updating novel title: ID={}, new_title='{}'", novel_id, new_title);
        let db = get_database()?;
        
        // 由于数据库层没有直接更新标题的方法，我们需要先获取小说，然后更新
        if let Some(mut novel) = self.get_novel_by_id(novel_id).await? {
            let old_title = novel.title.clone();
            novel.title = new_title.to_string();
            novel.updated_at = Utc::now();
            
            // 更新数据库
            db.update_novel(&novel)?;
            info!("Updated novel title: ID={}, old='{}', new='{}'", novel_id, old_title, new_title);
        } else {
            warn!("Failed to update novel title: Novel not found with ID={}", novel_id);
        }
        
        Ok(())
    }
    
    /// 更新小说作者
    pub async fn update_novel_author(&self, novel_id: i64, author: &str) -> Result<()> {
        let db = get_database()?;
        
        if let Some(mut novel) = self.get_novel_by_id(novel_id).await? {
            novel.author = author.to_string();
            novel.updated_at = Utc::now();
            
            db.update_novel(&novel)?;
        }
        
        Ok(())
    }
    
    /// 更新小说描述
    pub async fn update_novel_description(&self, novel_id: i64, description: &str) -> Result<()> {
        let db = get_database()?;
        
        if let Some(mut novel) = self.get_novel_by_id(novel_id).await? {
            novel.description = description.to_string();
            novel.updated_at = Utc::now();
            
            db.update_novel(&novel)?;
        }
        
        Ok(())
    }
    
    /// 更新小说状态
    pub async fn update_novel_status(&self, novel_id: i64, status: crate::db::NovelStatus) -> Result<()> {
        let db = get_database()?;
        
        if let Some(mut novel) = self.get_novel_by_id(novel_id).await? {
            novel.status = status;
            novel.updated_at = Utc::now();
            
            db.update_novel(&novel)?;
        }
        
        Ok(())
    }
    
    /// 切换小说置顶状态
    pub async fn toggle_novel_pin(&self, novel_id: i64) -> Result<bool> {
        info!("Toggle novel pin status: ID={}", novel_id);
        let db = get_database()?;
        
        // 获取要操作的小说
        if let Some(mut novel) = self.get_novel_by_id(novel_id).await? {
            let current_pinned = novel.is_pinned;
            
            if !current_pinned {
                // 检查当前置顶小说数量
                let all_novels = self.get_all_novels().await?;
                let pinned_count = all_novels.iter().filter(|n| n.is_pinned).count();
                
                if pinned_count >= 3 {
                    info!("Cannot pin novel {}: already 3 novels are pinned", novel_id);
                    return Err(anyhow::anyhow!("最多只能置顶3本小说"));
                }
                
                // 设置为置顶，并分配一个新的排序号
                novel.is_pinned = true;
                novel.pinned_order = Some((pinned_count + 1) as i32);
                info!("Pinning novel {} with order {}", novel_id, pinned_count + 1);
            } else {
                // 取消置顶
                novel.is_pinned = false;
                novel.pinned_order = None;
                info!("Unpinning novel {}", novel_id);
            }
            
            novel.updated_at = Utc::now();
            
            // 更新数据库
            db.update_novel(&novel)?;
            
            // 如果取消置顶，重新计算其他置顶小说的排序号
            if current_pinned {
                self.reorder_pinned_novels().await?;
            }
            
            info!("Successfully toggled novel pin status: ID={}, new_status={}", novel_id, novel.is_pinned);
            Ok(novel.is_pinned)
        } else {
            warn!("Failed to toggle novel pin status: Novel not found with ID={}", novel_id);
            Err(anyhow::anyhow!("小说不存在"))
        }
    }
    
    /// 重新排序置顶小说
    async fn reorder_pinned_novels(&self) -> Result<()> {
        info!("Reordering pinned novels");
        let db = get_database()?;
        
        // 获取所有置顶的小说
        let mut all_novels = self.get_all_novels().await?;
        let mut pinned_novels: Vec<_> = all_novels
            .iter_mut()
            .filter(|n| n.is_pinned)
            .collect();
        
        // 按更新时间排序（最新的在前面）
        pinned_novels.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        
        // 更新排序号
        for (index, novel) in pinned_novels.iter_mut().enumerate() {
            novel.pinned_order = Some((index + 1) as i32);
            db.update_novel(novel)?;
            info!("Updated pinned order for novel {}: {}", novel.id, index + 1);
        }
        
        Ok(())
    }
    
    /// 删除小说
    pub async fn delete_novel(&self, novel_id: i64) -> Result<()> {
        info!("Deleting novel: ID={}", novel_id);
        
        // 先检查小说是否存在
        if let Some(novel) = self.get_novel_by_id(novel_id).await? {
            info!("Found novel for deletion: ID={}, title='{}'", novel.id, novel.title);
            let db = get_database()?;
            
            // 由于外键约束，删除小说会自动删除相关的章节、节点等
            db.delete_novel(novel_id)?;
            info!("Successfully deleted novel and all associated data: ID={}, title='{}'", novel_id, novel.title);
        } else {
            warn!("Failed to delete novel: Novel not found with ID={}", novel_id);
        }
        
        Ok(())
    }
    
    /// 获取小说的章节统计
    pub async fn get_novel_statistics(&self, novel_id: i64) -> Result<NovelStatistics> {
        let db = get_database()?;
        let chapters = db.get_chapters_by_novel(novel_id)?;
        
        let total_chapters = chapters.len();
        let total_words: i32 = chapters.iter().map(|c| c.word_count).sum();
        let average_words = if total_chapters > 0 {
            total_words / total_chapters as i32
        } else {
            0
        };
        
        let last_updated = chapters.iter()
            .map(|c| c.updated_at)
            .max()
            .unwrap_or(Utc::now());
        
        Ok(NovelStatistics {
            novel_id,
            total_chapters,
            total_words,
            average_words,
            last_updated,
            chapter_types: self.analyze_chapter_types(&chapters),
        })
    }
    
    /// 分析章节类型分布
    fn analyze_chapter_types(&self, chapters: &[Chapter]) -> ChapterTypeDistribution {
        let mut distribution = ChapterTypeDistribution::default();
        
        for chapter in chapters {
            match chapter.chapter_type {
                crate::db::ChapterType::Volume => distribution.volumes += 1,
                crate::db::ChapterType::Chapter => distribution.chapters += 1,
                crate::db::ChapterType::Scene => distribution.scenes += 1,
            }
        }
        
        distribution
    }
    
    /// 搜索小说
    pub async fn search_novels(&self, query: &str) -> Result<Vec<Novel>> {
        info!("Searching novels with query: '{}'", query);
        let novels = self.get_all_novels().await?;
        
        let results: Vec<Novel> = novels.into_iter()
            .filter(|novel| {
                novel.title.to_lowercase().contains(&query.to_lowercase()) ||
                novel.author.to_lowercase().contains(&query.to_lowercase()) ||
                novel.description.to_lowercase().contains(&query.to_lowercase())
            })
            .collect();
        
        info!("Search completed: found {} novels matching query '{}'", results.len(), query);
        Ok(results)
    }
    
    /// 导出小说数据
    pub async fn export_novel_data(&self, novel_id: i64) -> Result<NovelExportData> {
        info!("Exporting novel data: ID={}", novel_id);
        let db = get_database()?;
        
        let novel = match self.get_novel_by_id(novel_id).await? {
            Some(n) => {
                info!("Found novel for export: ID={}, title='{}'", novel_id, n.title);
                n
            },
            None => {
                error!("Failed to export novel: Novel not found with ID={}", novel_id);
                return Err(anyhow::anyhow!("小说不存在"));
            }
        };
        
        info!("Retrieving chapters for novel: ID={}", novel_id);
        let chapters = db.get_chapters_by_novel(novel_id)?;
        info!("Retrieved {} chapters for export", chapters.len());
        
        let export_data = NovelExportData {
            novel,
            chapters,
            export_time: Utc::now(),
        };
        
        info!("Novel data export completed successfully: ID={}, title='{}', chapters={}", 
              novel_id, export_data.novel.title, export_data.chapters.len());
        
        Ok(export_data)
    }
}

#[derive(Debug, Clone)]
pub struct NovelStatistics {
    pub novel_id: i64,
    pub total_chapters: usize,
    pub total_words: i32,
    pub average_words: i32,
    pub last_updated: DateTime<Utc>,
    pub chapter_types: ChapterTypeDistribution,
}

#[derive(Debug, Clone, Default)]
pub struct ChapterTypeDistribution {
    pub volumes: usize,
    pub chapters: usize,
    pub scenes: usize,
}

#[derive(Debug, Clone)]
pub struct NovelExportData {
    pub novel: Novel,
    pub chapters: Vec<Chapter>,
    pub export_time: DateTime<Utc>,
}