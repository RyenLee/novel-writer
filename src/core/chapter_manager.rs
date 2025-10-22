use crate::db::{Chapter, get_database};
use anyhow::Result;
use std::collections::HashMap;
use log::{info, warn, error};

#[derive(Debug, Clone)]
pub struct ChapterTree {
    pub nodes: HashMap<i64, ChapterNode>,
    pub root_nodes: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct ChapterNode {
    pub chapter: Chapter,
    pub children: Vec<i64>,
    pub depth: usize,
}

pub struct ChapterManager;

impl ChapterManager {
    pub fn new() -> Self {
        Self
    }
    
    pub fn build_chapter_tree(&self, chapters: Vec<Chapter>) -> ChapterTree {
        info!("Building chapter tree from {} chapters", chapters.len());
        let mut nodes = HashMap::new();
        let mut root_nodes = Vec::new();
        
        // 首先创建所有节点
        for chapter in chapters {
            let node = ChapterNode {
                chapter: chapter.clone(),
                children: Vec::new(),
                depth: 0,
            };
            nodes.insert(chapter.id, node);
        }
        info!("Created {} chapter nodes", nodes.len());
        
        // 然后建立父子关系
        let mut node_ids: Vec<i64> = nodes.keys().cloned().collect();
        node_ids.sort(); // 确保处理顺序一致
        
        for &node_id in &node_ids {
            if let Some(node) = nodes.get(&node_id) {
                if let Some(parent_id) = node.chapter.parent_id {
                    if let Some(parent_node) = nodes.get_mut(&parent_id) {
                        parent_node.children.push(node_id);
                    }
                } else {
                    root_nodes.push(node_id);
                }
            }
        }
        info!("Established parent-child relationships with {} root nodes", root_nodes.len());
        
        // 计算深度
        let mut tree = ChapterTree { nodes, root_nodes };
        Self::calculate_depths(&mut tree);
        info!("Chapter tree built successfully with depth calculations completed");
        tree
    }
    
    fn calculate_depths(tree: &mut ChapterTree) {
        let root_nodes = tree.root_nodes.clone();
        for &root_id in &root_nodes {
            Self::calculate_depth_recursive(tree, root_id, 0);
        }
    }
    
    fn calculate_depth_recursive(tree: &mut ChapterTree, node_id: i64, depth: usize) {
        if let Some(node) = tree.nodes.get_mut(&node_id) {
            node.depth = depth;
            for &child_id in &node.children.clone() {
                Self::calculate_depth_recursive(tree, child_id, depth + 1);
            }
        }
    }
    
    pub async fn create_chapter(
        &self, 
        novel_id: i64, 
        title: &str, 
        parent_id: Option<i64>
    ) -> Result<Chapter> {
        info!("Creating new chapter: novel_id={}, title='{}', parent_id={:?}", 
              novel_id, title, parent_id);
        let db = get_database()?;
        let chapter = db.create_chapter(novel_id, title, parent_id)?;
        info!("Chapter created successfully: id={}, title='{}'", chapter.id, chapter.title);
        Ok(chapter)
    }
    
    pub async fn move_chapter(
        &self,
        chapter_id: i64,
        new_parent_id: Option<i64>,
        new_position: usize
    ) -> Result<()> {
        info!("Moving chapter: id={}, new_parent_id={:?}, new_position={}", 
              chapter_id, new_parent_id, new_position);
        
        let db = get_database()?;
        
        // 获取当前章节
        let chapter = match db.get_chapter(chapter_id) {
            Ok(ch) => ch,
            Err(e) => {
                error!("Failed to get chapter for moving: id={}, error={}", chapter_id, e);
                return Err(anyhow::anyhow!("获取章节失败: {}", e));
            }
        };
        
        let chapters = db.get_chapters_by_novel(chapter.novel_id)?;
        info!("Retrieved {} chapters for novel_id={} to build chapter tree", chapters.len(), chapter.novel_id);
        
        let tree = self.build_chapter_tree(chapters);
        
        // 验证移动是否有效（防止循环引用）
        if let Some(new_parent_id) = new_parent_id {
            if Self::would_create_cycle(&tree, chapter_id, new_parent_id) {
                warn!("Move operation rejected: Would create cycle - chapter_id={} cannot be moved under parent_id={}", 
                      chapter_id, new_parent_id);
                return Err(anyhow::anyhow!("移动章节会导致循环引用"));
            }
        }
        
        // 更新父节点和排序路径
        let new_sort_path = match self.calculate_new_sort_path(&tree, new_parent_id, new_position) {
            Ok(path) => {
                info!("Calculated new sort path for chapter {}: {}", chapter_id, path);
                path
            },
            Err(e) => {
                error!("Failed to calculate new sort path: chapter_id={}, error={}", chapter_id, e);
                return Err(e);
            }
        };
        
        // 更新数据库
        if let Err(e) = db.update_chapter_parent(chapter_id, new_parent_id, &new_sort_path) {
            error!("Failed to update chapter parent in database: chapter_id={}, error={}", chapter_id, e);
            return Err(e);
        }
        
        info!("Successfully moved chapter: id={}, new_parent_id={:?}, new_position={}", 
              chapter_id, new_parent_id, new_position);
        Ok(())
    }
    
    fn would_create_cycle(tree: &ChapterTree, moving_id: i64, new_parent_id: i64) -> bool {
        let mut current = Some(new_parent_id);
        while let Some(node_id) = current {
            if node_id == moving_id {
                return true;
            }
            current = tree.nodes.get(&node_id)
                .and_then(|node| node.chapter.parent_id);
        }
        false
    }
    
    fn calculate_new_sort_path(
        &self,
        tree: &ChapterTree,
        parent_id: Option<i64>,
        position: usize
    ) -> Result<String> {
        let siblings = if let Some(parent_id) = parent_id {
            tree.nodes.get(&parent_id)
                .map(|node| &node.children)
                .unwrap_or(&Vec::new())
                .clone()
        } else {
            tree.root_nodes.clone()
        };
        
        if position > siblings.len() {
            return Err(anyhow::anyhow!("位置超出范围"));
        }
        
        // 简单的基于位置的排序路径
        let timestamp = chrono::Utc::now().timestamp_millis();
        let position_str = format!("{:06}", position);
        Ok(format!("{}_{}", position_str, timestamp))
    }
    
    pub fn flatten_tree(&self, tree: &ChapterTree) -> Vec<Chapter> {
        info!("Flattening chapter tree with {} root nodes", tree.root_nodes.len());
        let mut result = Vec::new();
        for &root_id in &tree.root_nodes {
            Self::flatten_recursive(tree, root_id, &mut result);
        }
        info!("Tree flattened successfully, resulting in {} chapters in ordered sequence", result.len());
        result
    }
    
    fn flatten_recursive(tree: &ChapterTree, node_id: i64, result: &mut Vec<Chapter>) {
        if let Some(node) = tree.nodes.get(&node_id) {
            result.push(node.chapter.clone());
            for &child_id in &node.children {
                Self::flatten_recursive(tree, child_id, result);
            }
        }
    }
}