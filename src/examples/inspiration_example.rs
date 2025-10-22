// 灵感模块使用示例
// 这个文件展示了如何使用灵感模块的各种功能

use crate::db;
use anyhow::Result;

pub struct InspirationExample;

impl InspirationExample {
    /// 演示完整的灵感管理流程
    pub fn demonstrate_inspiration_workflow() -> Result<()> {
        println!("=== 灵感模块使用示例 ===");
        
        // 1. 创建新灵感
        println!("\n1. 创建新灵感");
        let db = db::get_database()?;
        let inspiration = db.create_inspiration(
            1, // 小说ID
            "主角性格发展想法", 
            "主角应该从内向逐渐变得外向，通过经历一系列事件..."
        )?;
        println!("创建灵感成功: ID={}, 标题={}", inspiration.id, inspiration.title);
        
        // 2. 添加标签
        println!("\n2. 添加标签");
        db.add_inspiration_tags(inspiration.id, &[
            "角色".to_string(),
            "性格".to_string(),
            "发展".to_string()
        ])?;
        println!("添加标签成功");
        
        // 3. 关联章节
        println!("\n3. 关联章节");
        db.link_inspiration_to_chapter(inspiration.id, 5)?; // 关联到第5章
        println!("关联章节成功");
        
        // 4. 置顶灵感
        println!("\n4. 置顶灵感");
        let is_pinned = db.toggle_inspiration_pin(inspiration.id)?;
        println!("置顶状态: {}", is_pinned);
        
        // 5. 获取灵感详情
        println!("\n5. 获取灵感详情");
        if let Some(full_inspiration) = db.get_inspiration(inspiration.id)? {
            println!("灵感详情:");
            println!("  ID: {}", full_inspiration.id);
            println!("  标题: {}", full_inspiration.title);
            println!("  内容: {}", full_inspiration.content);
            println!("  标签: {:?}", full_inspiration.tags);
            println!("  关联章节: {:?}", full_inspiration.linked_chapters);
            println!("  置顶: {}", full_inspiration.is_pinned);
            println!("  创建时间: {}", full_inspiration.created_at);
            println!("  更新时间: {}", full_inspiration.updated_at);
        }
        
        // 6. 搜索灵感
        println!("\n6. 搜索灵感");
        let search_results = db.search_inspirations(1, "性格")?;
        println!("搜索到 {} 个相关灵感", search_results.len());
        for result in search_results {
            println!("  - {} (标签: {:?})", result.title, result.tags);
        }
        
        // 7. 获取小说所有灵感
        println!("\n7. 获取小说所有灵感");
        let all_inspirations = db.get_inspirations_by_novel(1)?;
        println!("小说共有 {} 个灵感", all_inspirations.len());
        
        // 8. 更新灵感
        println!("\n8. 更新灵感");
        db.update_inspiration(inspiration.id, "更新的标题", "更新后的详细内容...")?;
        println!("更新灵感成功");
        
        // 9. 演示删除操作（注释掉，实际使用时取消注释）
        // println!("\n9. 删除灵感");
        // db.delete_inspiration(inspiration.id)?;
        // println!("删除灵感成功");
        
        println!("\n=== 示例完成 ===");
        Ok(())
    }
    
    /// 批量创建示例灵感
    pub fn create_sample_inspirations(novel_id: i64) -> Result<()> {
        let db = db::get_database()?;
        
        let sample_inspirations = vec![
            ("世界观设定想法", "建立一个魔法与科技并存的世界...", vec!["世界观", "设定"]),
            ("情节转折点", "在第15章加入一个意外的盟友背叛...", vec!["情节", "转折"]),
            ("角色关系发展", "主角和反派之间应该有复杂的过去...", vec!["角色", "关系"]),
            ("战斗场景构思", "设计一场在雨中的剑术对决...", vec!["场景", "战斗"]),
            ("伏笔设置", "在早期章节埋下关于神秘组织的线索...", vec!["伏笔", "线索"]),
        ];
        
        for (title, content, tags) in sample_inspirations {
            let inspiration = db.create_inspiration(novel_id, title, content)?;
            db.add_inspiration_tags(inspiration.id, &tags.into_iter().map(String::from).collect::<Vec<_>>())?;
            println!("创建示例灵感: {}", title);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_inspiration_workflow() {
        // 注意：这个测试需要数据库支持
        // 在实际项目中，应该使用测试数据库
        println!("灵感模块测试（需要数据库连接）");
    }
}