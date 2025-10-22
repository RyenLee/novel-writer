use dioxus::prelude::*;
use crate::core::stats_manager::WritingReport;

#[component]
pub fn StatsView(writing_report: Signal<Option<WritingReport>>) -> Element {
    let mut selected_tab = use_signal(|| "overview".to_string());

    rsx! {
        div {
            class: "stats-view",
            h2 { "📊 写作统计" }
            
            // 标签页导航
            div {
                class: "stats-tabs",
                button {
                    class: if selected_tab() == "overview" { "active" } else { "" },
                    onclick: move |_| selected_tab.set("overview".to_string()),
                    "📋 概览"
                }
                button {
                    class: if selected_tab() == "trends" { "active" } else { "" },
                    onclick: move |_| selected_tab.set("trends".to_string()),
                    "📈 写作趋势"
                }
                button {
                    class: if selected_tab() == "goals" { "active" } else { "" },
                    onclick: move |_| selected_tab.set("goals".to_string()),
                    "🎯 目标进度"
                }
                button {
                    class: if selected_tab() == "recommendations" { "active" } else { "" },
                    onclick: move |_| selected_tab.set("recommendations".to_string()),
                    "💡 改进建议"
                }
            }
            
            // 标签页内容
            div {
                class: "stats-content",
                
                // 概览标签页
                if selected_tab() == "overview" {
                    div {
                        class: "overview-section",
                        h3 { "📋 小说概览" }
                        div {
                            class: "stats-card",
                            p { class: "stats-label", "总字数" }
                            p { class: "stats-value", "125,342" }
                        }
                        div {
                            class: "stats-card",
                            p { class: "stats-label", "已完成章节" }
                            p { class: "stats-value", "15 章" }
                        }
                        div {
                            class: "stats-card",
                            p { class: "stats-label", "写作天数" }
                            p { class: "stats-value", "45 天" }
                        }
                        div {
                            class: "stats-card",
                            p { class: "stats-label", "平均日产量" }
                            p { class: "stats-value", "2,785 字" }
                        }
                    }
                }
                
                // 写作趋势标签页
                else if selected_tab() == "trends" {
                    div {
                        class: "trends-section",
                        h3 { "📈 写作趋势" }
                        div {
                            class: "chart-placeholder",
                            p { "📊 最近30天写作趋势图" }
                            div {
                                class: "trend-bar",
                                style: "height: 30%; background-color: #4CAF50; width: 10px; margin: 0 2px; display: inline-block;"
                            }
                            div {
                                class: "trend-bar",
                                style: "height: 50%; background-color: #4CAF50; width: 10px; margin: 0 2px; display: inline-block;"
                            }
                            div {
                                class: "trend-bar",
                                style: "height: 70%; background-color: #4CAF50; width: 10px; margin: 0 2px; display: inline-block;"
                            }
                            div {
                                class: "trend-bar",
                                style: "height: 45%; background-color: #4CAF50; width: 10px; margin: 0 2px; display: inline-block;"
                            }
                            div {
                                class: "trend-bar",
                                style: "height: 60%; background-color: #4CAF50; width: 10px; margin: 0 2px; display: inline-block;"
                            }
                        }
                        div {
                            class: "best-day-info",
                            h4 { "🏆 最佳写作日" }
                            p { "2023-05-15: 5,687 字" }
                        }
                    }
                }
                
                // 目标进度标签页
                else if selected_tab() == "goals" {
                    div {
                        class: "goals-section",
                        h3 { "🎯 目标进度" }
                        div {
                            class: "progress-container",
                            div {
                                class: "progress-bar",
                                style: "width: 65%; background-color: #2196F3; height: 24px; border-radius: 12px;"
                            }
                            p { class: "progress-text", "65% 完成" }
                        }
                        p { class: "goal-details", "目标：200,000 字" }
                        p { class: "goal-details", "已完成：125,342 字" }
                        p { class: "goal-details", "剩余：74,658 字" }
                        p { class: "goal-details", "预计完成日期：2023-12-15" }
                    }
                }
                
                // 改进建议标签页
                else if selected_tab() == "recommendations" {
                    div {
                        class: "recommendations",
                        h3 { "改进建议" }
                        ul {
                            class: "recommendations-list",
                            // 静态建议数据
                            li {
                                class: "recommendation-item",
                                span { class: "recommendation-icon", "💡" }
                                span { "建议增加角色对话，提升故事生动性" }
                            }
                            li {
                                class: "recommendation-item",
                                span { class: "recommendation-icon", "💡" }
                                span { "可以适当增加场景描写，增强画面感" }
                            }
                            li {
                                class: "recommendation-item",
                                span { class: "recommendation-icon", "💡" }
                                span { "考虑设定每周写作目标，增强写作动力" }
                            }
                        }
                    }
                }
            }
        }
    }
}