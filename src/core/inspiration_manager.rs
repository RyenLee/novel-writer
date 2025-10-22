use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, DateTime, Utc};
use anyhow::Result;
use crate::db::get_database;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Inspiration {
    pub id: i64,
    pub novel_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_pinned: bool,
    pub tags: Vec<String>,
    pub linked_chapters: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct InspirationStats {
    pub total_inspirations: usize,
    pub pinned_inspirations: usize,
    pub linked_chapters_count: usize,
    pub total_tags: usize,
    pub unique_tags_count: usize,
    pub most_used_tag: Option<(String, usize)>,
    pub inspirations_per_day: f64,
    pub recent_inspirations_count: usize,
    pub last_inspiration_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct InspirationTrends {
    pub daily_counts: Vec<DailyInspirationCount>,
    pub tag_distribution: Vec<(String, usize)>,
    pub inspiration_bursts: Vec<InspirationBurst>,
    pub trend_direction: TrendDirection,
}

#[derive(Debug, Clone)]
pub struct DailyInspirationCount {
    pub date: NaiveDate,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub struct InspirationBurst {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_inspirations: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    InsufficientData,
}

pub struct InspirationManager;

impl InspirationManager {
    pub fn new() -> Self {
        Self
    }
    
    /// 获取灵感统计信息
    pub async fn get_inspiration_stats(&self, novel_id: i64) -> Result<InspirationStats> {
        let db = get_database()?;
        let inspirations = db.get_inspirations_by_novel(novel_id)?;
        
        if inspirations.is_empty() {
            return Ok(InspirationStats {
                total_inspirations: 0,
                pinned_inspirations: 0,
                linked_chapters_count: 0,
                total_tags: 0,
                unique_tags_count: 0,
                most_used_tag: None,
                inspirations_per_day: 0.0,
                recent_inspirations_count: 0,
                last_inspiration_date: None,
            });
        }
        
        // 计算基本统计
        let total_inspirations = inspirations.len();
        let pinned_inspirations = inspirations.iter().filter(|i| i.is_pinned).count();
        let linked_chapters_count = inspirations.iter().map(|i| i.linked_chapters.len()).sum();
        
        // 统计标签信息
        let mut tag_counts: HashMap<String, usize> = HashMap::new();
        for inspiration in &inspirations {
            for tag in &inspiration.tags {
                *tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        
        let total_tags = tag_counts.values().sum();
        let unique_tags_count = tag_counts.len();
        
        // 找出使用最多的标签
        let most_used_tag = tag_counts
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(tag, count)| (tag.clone(), *count));
        
        // 计算每日平均灵感数
        let first_date = inspirations
            .iter()
            .min_by_key(|i| i.created_at.clone())
            .map(|i| DateTime::parse_from_rfc3339(&i.created_at).unwrap().with_timezone(&Utc).date_naive())
            .unwrap();
        
        let days_passed = (Utc::now().date_naive() - first_date).num_days().max(1);
        let inspirations_per_day = total_inspirations as f64 / days_passed as f64;
        
        // 最近7天的灵感数
        let seven_days_ago = Utc::now().date_naive() - chrono::Duration::days(7);
        let recent_inspirations_count = inspirations
            .iter()
            .filter(|i| {
                DateTime::parse_from_rfc3339(&i.created_at)
                    .unwrap()
                    .with_timezone(&Utc)
                    .date_naive() >= seven_days_ago
            })
            .count();
        
        // 最后一条灵感的日期
        let last_inspiration_date = inspirations
            .iter()
            .max_by_key(|i| i.updated_at.clone())
            .map(|i| DateTime::parse_from_rfc3339(&i.updated_at).unwrap().with_timezone(&Utc));
        
        Ok(InspirationStats {
            total_inspirations,
            pinned_inspirations,
            linked_chapters_count,
            total_tags,
            unique_tags_count,
            most_used_tag,
            inspirations_per_day,
            recent_inspirations_count,
            last_inspiration_date,
        })
    }
    
    /// 获取灵感趋势分析
    pub async fn get_inspiration_trends(&self, novel_id: i64, _period_days: i64) -> Result<InspirationTrends> {
        let db = get_database()?;
        let inspirations = db.get_inspirations_by_novel(novel_id)?;
        
        if inspirations.is_empty() {
            return Ok(InspirationTrends {
                daily_counts: Vec::new(),
                tag_distribution: Vec::new(),
                inspiration_bursts: Vec::new(),
                trend_direction: TrendDirection::InsufficientData,
            });
        }
        
        // 计算每日灵感数
        let mut daily_counts_map: HashMap<NaiveDate, usize> = HashMap::new();
        for inspiration in &inspirations {
            let date = DateTime::parse_from_rfc3339(&inspiration.created_at)
            .unwrap()
            .with_timezone(&Utc)
            .date_naive();
            *daily_counts_map.entry(date).or_insert(0) += 1;
        }
        
        // 转换为有序的每日计数列表
        let mut daily_counts: Vec<DailyInspirationCount> = daily_counts_map
            .into_iter()
            .map(|(date, count)| DailyInspirationCount { date, count })
            .collect();
        
        daily_counts.sort_by(|a, b| a.date.cmp(&b.date));
        
        // 计算标签分布
        let mut tag_counts: HashMap<String, usize> = HashMap::new();
        for inspiration in &inspirations {
            for tag in &inspiration.tags {
                *tag_counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        
        let mut tag_distribution: Vec<(String, usize)> = tag_counts
            .into_iter()
            .collect();
        
        tag_distribution.sort_by(|a, b| b.1.cmp(&a.1)); // 按计数降序排列
        
        // 检测灵感爆发期
        let inspiration_bursts = self.detect_inspiration_bursts(&daily_counts);
        
        // 分析趋势方向
        let trend_direction = self.analyze_trend_direction(&daily_counts);
        
        Ok(InspirationTrends {
            daily_counts,
            tag_distribution,
            inspiration_bursts,
            trend_direction,
        })
    }
    
    /// 检测灵感爆发期
    fn detect_inspiration_bursts(&self, daily_counts: &[DailyInspirationCount]) -> Vec<InspirationBurst> {
        if daily_counts.len() < 3 {
            return Vec::new();
        }
        
        let mut bursts = Vec::new();
        let mut current_burst: Option<(NaiveDate, NaiveDate, usize)> = None;
        let avg_count: f64 = daily_counts.iter().map(|d| d.count as f64).sum::<f64>() / daily_counts.len() as f64;
        let burst_threshold = (avg_count * 1.5).max(2.0);
        
        for daily in daily_counts {
            if daily.count as f64 >= burst_threshold {
                match current_burst {
                    Some((start, _, count)) => {
                        // 检查日期是否连续
                        let prev_date = daily.date.pred_opt().unwrap();
                        if prev_date == start {
                            // 更新结束日期和计数
                            current_burst = Some((start, daily.date, count + daily.count));
                        } else {
                            // 结束当前爆发期，开始新的
                            bursts.push(InspirationBurst {
                                start_date: start,
                                end_date: prev_date,
                                total_inspirations: count,
                            });
                            current_burst = Some((daily.date, daily.date, daily.count));
                        }
                    },
                    None => {
                        current_burst = Some((daily.date, daily.date, daily.count));
                    }
                }
            } else if let Some((start, end, count)) = current_burst.take() {
                // 爆发期结束
                if count >= 3 { // 至少3个灵感才视为爆发期
                    bursts.push(InspirationBurst {
                        start_date: start,
                        end_date: end,
                        total_inspirations: count,
                    });
                }
            }
        }
        
        // 添加最后一个爆发期（如果有）
        if let Some((start, end, count)) = current_burst {
            if count >= 3 {
                bursts.push(InspirationBurst {
                    start_date: start,
                    end_date: end,
                    total_inspirations: count,
                });
            }
        }
        
        bursts
    }
    
    /// 分析趋势方向
    fn analyze_trend_direction(&self, daily_counts: &[DailyInspirationCount]) -> TrendDirection {
        if daily_counts.len() < 7 {
            return TrendDirection::InsufficientData;
        }
        
        // 分割为前半段和后半段
        let mid = daily_counts.len() / 2;
        let first_half_avg = daily_counts[..mid]
            .iter()
            .map(|d| d.count as f64)
            .sum::<f64>() / mid as f64;
        
        let second_half_avg = daily_counts[mid..]
            .iter()
            .map(|d| d.count as f64)
            .sum::<f64>() / (daily_counts.len() - mid) as f64;
        
        // 计算变化百分比
        let change_percentage = if first_half_avg > 0.0 {
            ((second_half_avg - first_half_avg) / first_half_avg) * 100.0
        } else if second_half_avg > 0.0 {
            100.0 // 从零增加到正数，视为增长
        } else {
            0.0
        };
        
        if change_percentage >= 20.0 {
            TrendDirection::Increasing
        } else if change_percentage <= -20.0 {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        }
    }
    
    /// 生成灵感使用建议
    pub fn generate_inspiration_recommendations(&self, stats: &InspirationStats, trends: &InspirationTrends) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // 基于灵感总数的建议
        if stats.total_inspirations == 0 {
            recommendations.push("开始记录你的创作灵感，这将帮助你保持创意源源不断".to_string());
        } else if stats.total_inspirations < 10 {
            recommendations.push("尝试每天记录至少一个灵感，积少成多".to_string());
        }
        
        // 基于最近活动的建议
        if let Some(last_date) = stats.last_inspiration_date {
            let days_since_last = (Utc::now() - last_date).num_days();
            if days_since_last > 7 {
                recommendations.push("已经一周没记录新灵感了，尝试重新开始记录".to_string());
            }
        }
        
        // 基于标签使用的建议
        if stats.unique_tags_count < 3 && stats.total_inspirations > 5 {
            recommendations.push("尝试使用更多标签来组织你的灵感，便于后续查找和分类".to_string());
        }
        
        // 基于章节关联的建议
        if stats.linked_chapters_count == 0 && stats.total_inspirations > 3 {
            recommendations.push("考虑将灵感与具体章节关联，这样更容易在写作时应用它们".to_string());
        }
        
        // 基于趋势的建议
        match trends.trend_direction {
            TrendDirection::Increasing => {
                recommendations.push("你的灵感数量正在增加，继续保持这种创意状态！".to_string());
            },
            TrendDirection::Decreasing => {
                recommendations.push("最近灵感数量有所下降，尝试改变环境或阅读相关素材来激发创意".to_string());
            },
            TrendDirection::Stable => {
                recommendations.push("你的灵感产出比较稳定，尝试设定更高的创意目标".to_string());
            },
            TrendDirection::InsufficientData => {}
        }
        
        recommendations
    }
}