use dioxus::prelude::*;
use crate::core::inspiration_manager::{InspirationStats, InspirationTrends};
use chrono::{NaiveDate};
use chrono::Datelike;

#[derive(Props, Clone, PartialEq)]
pub struct InspirationStatsProps {
    pub stats: Signal<Option<InspirationStats>>,
    pub trends: Signal<Option<InspirationTrends>>,
    pub recommendations: Signal<Vec<String>>,
}

fn format_naive_date(date: &NaiveDate) -> String {
    format!("{}-{:02}-{:02}", date.year(), date.month(), date.day())
}

#[component]
pub fn InspirationStatsView(props: InspirationStatsProps) -> Element {    
    rsx! {
        div {
            class: "inspiration-stats-container",
            
            // 统计概览卡片
            div {
                class: "stats-cards",
                
                // 总灵感数卡片
                div {
                    class: "stat-card",
                    div { 
                        class: "stat-icon",
                        "💡"
                    }
                    div { 
                        class: "stat-value", 
                        { props.stats.read().as_ref().map_or("0".to_string(), |s| s.total_inspirations.to_string()) }
                    }
                    div { 
                        class: "stat-label", 
                        "总灵感数"
                    }
                }
            }
        }
    }
}