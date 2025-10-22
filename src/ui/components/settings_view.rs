use dioxus::prelude::*;
use crate::config::AppConfig;
use log::{debug, error, info};

#[component]
pub fn SettingsView() -> Element {
    let mut auto_save_enabled = use_signal(|| true);
    
    // 简化版本，使用静态数据避免类型推断问题
    rsx! {
        div {
            class: "settings-view",
            h2 { "系统设置" }
            
            div {
                class: "settings-form",
                
                // 简单的主题设置
                div {
                    class: "settings-section",
                    h3 { "主题设置" }
                    
                    div {
                        class: "setting-item",
                        label { "主题模式:" }
                        select {
                            value: "light",
                            onchange: move |evt| println!("主题已更改为: {}", evt.value()),
                            option { value: "light", "浅色" }
                            option { value: "dark", "深色" }
                            option { value: "sepia", "护眼" }
                        }
                    }
                }
                
                // 编辑器设置
                div {
                    class: "settings-section",
                    h3 { "编辑器设置" }
                    
                    div {
                        class: "setting-item",
                        label {
                            class: "checkbox-label",
                            input {
                                r#type: "checkbox",
                                checked: auto_save_enabled(),
                                onchange: move |evt| {
                                    let new_value = evt.value() == "on";
                                    auto_save_enabled.set(new_value);
                                    println!("自动保存已{}", if new_value { "启用" } else { "禁用" });
                                    // 实际应用中这里应该更新配置
                                }
                            }
                            span { "自动保存" }
                        }
                    }
                }
                
                // 操作按钮
                div {
                    class: "settings-actions",
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| {
                            println!("重置为默认值");
                            auto_save_enabled.set(true);
                            // 实际应用中这里应该重置所有设置为默认值
                        },
                        "重置为默认值"
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            println!("保存设置: 自动保存 = {}", auto_save_enabled());
                            // 实际应用中这里应该保存所有设置到配置文件
                        },
                        "保存设置"
                    }
                }
            }
        }
    }
}