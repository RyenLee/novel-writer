use crate::db::{get_database, Novel};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc, NaiveDate};
use std::collections::HashMap;

pub struct StatsManager;

impl StatsManager {
    pub fn new() -> Self {
        Self
    }
    
    /// 获取小说统计信息
    pub async fn get_novel_stats(&self, novel_id: i64) -> Result<NovelStats> {
        let db = get_database()?;
        let novel = db.get_novel_by_id(novel_id)?
            .ok_or_else(|| anyhow::anyhow!("小说不存在"))?;
        
        let chapters = db.get_chapters_by_novel(novel_id)?;
        let writing_stats = self.get_writing_stats(novel_id).await?;
        
        let current_streak = self.calculate_current_streak(&writing_stats);
        let longest_streak = self.calculate_longest_streak(&writing_stats);
        let average_daily_words = self.calculate_average_daily_words(&writing_stats);
        let progress_percentage = self.calculate_progress_percentage(&novel);
        let last_updated = novel.updated_at;
        let total_words = novel.word_count;
        
        Ok(NovelStats {
            novel,
            total_chapters: chapters.len(),
            total_words,
            writing_days: writing_stats.len(),
            current_streak,
            longest_streak,
            average_daily_words,
            last_updated,
            progress_percentage,
        })
    }
    
    /// 获取写作统计记录
    pub async fn get_writing_stats(&self, novel_id: i64) -> Result<Vec<DailyWritingStats>> {
        let db = get_database()?;
        let mut stats_map: HashMap<NaiveDate, DailyWritingStats> = HashMap::new();
        
        // 从数据库获取写作统计（这里需要实现对应的数据库方法）
        // 暂时使用章节更新日期来模拟写作统计
        let chapters = db.get_chapters_by_novel(novel_id)?;
        
        for chapter in chapters {
            let date = chapter.updated_at.date_naive();
            let entry = stats_map.entry(date).or_insert(DailyWritingStats {
                date,
                word_count: 0,
                writing_time: 0,
                session_count: 0,
            });
            
            entry.word_count += chapter.word_count as u32;
            entry.session_count += 1;
        }
        
        let mut stats: Vec<DailyWritingStats> = stats_map.into_values().collect();
        stats.sort_by(|a, b| b.date.cmp(&a.date)); // 按日期倒序排列
        
        Ok(stats)
    }
    
    /// 计算当前连续写作天数
    fn calculate_current_streak(&self, stats: &[DailyWritingStats]) -> u32 {
        let mut current_streak = 0;
        let mut current_date = Utc::now().date_naive();
        
        for stat in stats {
            if stat.date == current_date {
                current_streak += 1;
                current_date = current_date.pred_opt().unwrap_or(current_date);
            } else if stat.date < current_date {
                // 日期不连续，终止统计
                break;
            }
        }
        
        current_streak
    }
    
    /// 计算最长连续写作天数
    fn calculate_longest_streak(&self, stats: &[DailyWritingStats]) -> u32 {
        if stats.is_empty() {
            return 0;
        }
        
        let mut longest_streak = 0;
        let mut current_streak = 1;
        let mut prev_date = stats[0].date;
        
        for i in 1..stats.len() {
            let current_date = stats[i].date;
            let gap = prev_date.signed_duration_since(current_date).num_days();
            
            if gap == 1 {
                // 连续日期
                current_streak += 1;
            } else if gap > 1 {
                // 日期不连续，重置计数
                longest_streak = longest_streak.max(current_streak);
                current_streak = 1;
            }
            
            prev_date = current_date;
        }
        
        longest_streak.max(current_streak)
    }
    
    /// 计算平均每日字数
    fn calculate_average_daily_words(&self, stats: &[DailyWritingStats]) -> f64 {
        if stats.is_empty() {
            return 0.0;
        }
        
        let total_words: u32 = stats.iter().map(|s| s.word_count).sum();
        total_words as f64 / stats.len() as f64
    }
    
    /// 计算进度百分比
    fn calculate_progress_percentage(&self, novel: &Novel) -> f64 {
        // 这里可以根据目标字数计算进度
        // 暂时使用一个简单的逻辑
        let target_words = 50000; // 5万字目标
        if target_words > 0 {
            (novel.word_count as f64 / target_words as f64 * 100.0).min(100.0)
        } else {
            0.0
        }
    }
    
    /// 获取写作趋势分析
    pub async fn get_writing_trends(&self, novel_id: i64, period_days: i64) -> Result<WritingTrends> {
        let stats = self.get_writing_stats(novel_id).await?;
        let cutoff_date = Utc::now().date_naive() - Duration::days(period_days);
        
        let recent_stats: Vec<&DailyWritingStats> = stats.iter()
            .filter(|s| s.date >= cutoff_date)
            .collect();
        
        let total_stats: Vec<&DailyWritingStats> = stats.iter().collect();
        
        Ok(WritingTrends {
            period_days,
            recent_daily_average: self.calculate_period_average(&recent_stats),
            total_daily_average: self.calculate_period_average(&total_stats),
            best_day: self.find_best_day(&stats),
            most_productive_time: self.analyze_productive_times(&stats),
            consistency_score: self.calculate_consistency_score(&stats),
        })
    }
    
    /// 计算时间段内的日均字数
    fn calculate_period_average(&self, stats: &[&DailyWritingStats]) -> f64 {
        if stats.is_empty() {
            return 0.0;
        }
        
        let total_words: u32 = stats.iter().map(|s| s.word_count).sum();
        total_words as f64 / stats.len() as f64
    }
    
    /// 找到写作量最高的一天
    fn find_best_day(&self, stats: &[DailyWritingStats]) -> Option<BestDay> {
        stats.iter()
            .max_by_key(|s| s.word_count)
            .map(|s| BestDay {
                date: s.date,
                word_count: s.word_count,
            })
    }
    
    /// 分析高效写作时间段（简化实现）
    fn analyze_productive_times(&self, _stats: &[DailyWritingStats]) -> String {
        // 这里可以分析一天中哪个时间段写作效率最高
        // 暂时返回一个固定值
        "上午 9-11 点".to_string()
    }
    
    /// 计算写作一致性分数
    fn calculate_consistency_score(&self, stats: &[DailyWritingStats]) -> f64 {
        if stats.len() < 2 {
            return 0.0;
        }
        
        let total_days = stats.len() as f64;
        let writing_days = stats.iter()
            .filter(|s| s.word_count > 0)
            .count() as f64;
        
        (writing_days / total_days * 100.0).min(100.0)
    }
    
    /// 设置写作目标
    pub async fn set_writing_goal(&self, _novel_id: i64, _goal: WritingGoal) -> Result<()> {
        // 这里可以实现目标设置逻辑
        // 暂时只是保存目标信息
        Ok(())
    }
    
    /// 获取目标进度
    pub async fn get_goal_progress(&self, novel_id: i64) -> Result<GoalProgress> {
        let novel_stats = self.get_novel_stats(novel_id).await?;
        
        Ok(GoalProgress {
            current_words: novel_stats.total_words as u32,
            target_words: 50000, // 默认5万字目标
            deadline: None, // 可以添加截止日期功能
            days_remaining: None,
            progress_percentage: novel_stats.progress_percentage,
            estimated_completion_date: self.estimate_completion_date(&novel_stats),
        })
    }
    
    /// 估算完成日期
    fn estimate_completion_date(&self, stats: &NovelStats) -> Option<NaiveDate> {
        let remaining_words = 50000u32.saturating_sub(stats.total_words as u32);
        
        if stats.average_daily_words > 0.0 {
            let days_needed = (remaining_words as f64 / stats.average_daily_words).ceil() as i64;
            Some(Utc::now().date_naive() + Duration::days(days_needed))
        } else {
            None
        }
    }
    
    /// 生成写作报告
    pub async fn generate_writing_report(&self, novel_id: i64) -> Result<WritingReport> {
        let novel_stats = self.get_novel_stats(novel_id).await?;
        let trends = self.get_writing_trends(novel_id, 30).await?;
        let goal_progress = self.get_goal_progress(novel_id).await?;
        
        let recommendations = self.generate_recommendations(&novel_stats, &trends);
        
        Ok(WritingReport {
            novel_stats,
            trends,
            goal_progress,
            recommendations,
        })
    }
    
    /// 生成改进建议
    fn generate_recommendations(&self, stats: &NovelStats, trends: &WritingTrends) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if stats.current_streak == 0 {
            recommendations.push("今天开始写作，建立连续写作习惯".to_string());
        }
        
        if trends.recent_daily_average < 500.0 {
            recommendations.push("尝试提高每日写作量，目标500字以上".to_string());
        }
        
        if stats.progress_percentage < 50.0 {
            recommendations.push("加快写作进度，保持稳定的创作节奏".to_string());
        }
        
        if trends.consistency_score < 70.0 {
            recommendations.push("提高写作频率，保持每周至少写作3-4天".to_string());
        }
        
        recommendations
    }
}

#[derive(Debug, Clone)]
pub struct NovelStats {
    pub novel: Novel,
    pub total_chapters: usize,
    pub total_words: i32,
    pub writing_days: usize,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub average_daily_words: f64,
    pub last_updated: DateTime<Utc>,
    pub progress_percentage: f64,
}

#[derive(Debug, Clone)]
pub struct DailyWritingStats {
    pub date: NaiveDate,
    pub word_count: u32,
    pub writing_time: u32, // 秒
    pub session_count: u32,
}

#[derive(Debug, Clone)]
pub struct WritingTrends {
    pub period_days: i64,
    pub recent_daily_average: f64,
    pub total_daily_average: f64,
    pub best_day: Option<BestDay>,
    pub most_productive_time: String,
    pub consistency_score: f64,
}

#[derive(Debug, Clone)]
pub struct BestDay {
    pub date: NaiveDate,
    pub word_count: u32,
}

#[derive(Debug, Clone)]
pub struct WritingGoal {
    pub target_words: u32,
    pub deadline: Option<NaiveDate>,
    pub daily_target: u32,
}

#[derive(Debug, Clone)]
pub struct GoalProgress {
    pub current_words: u32,
    pub target_words: u32,
    pub deadline: Option<NaiveDate>,
    pub days_remaining: Option<i64>,
    pub progress_percentage: f64,
    pub estimated_completion_date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct WritingReport {
    pub novel_stats: NovelStats,
    pub trends: WritingTrends,
    pub goal_progress: GoalProgress,
    pub recommendations: Vec<String>,
}