/// Application configuration module
/// Manages application settings and preferences

use serde::{Deserialize, Serialize};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Application name
    pub app_name: String,
    
    /// Application version
    pub version: String,
    
    /// Database path
    pub database_path: String,
    
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    
    /// Theme settings
    pub theme: ThemeConfig,
    
    /// Editor settings
    pub editor: EditorConfig,
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Theme name (light, dark, custom)
    pub name: String,
    
    /// Font family
    pub font_family: String,
    
    /// Font size
    pub font_size: u32,
}

/// Editor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    /// Enable spell check
    pub spell_check: bool,
    
    /// Enable auto-save
    pub auto_save: bool,
    
    /// Tab size
    pub tab_size: u32,
    
    /// Word wrap
    pub word_wrap: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_name: "Novel Writer".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            database_path: "./novel_writer.db".to_string(),
            auto_save_interval: 30,
            theme: ThemeConfig::default(),
            editor: EditorConfig::default(),
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "light".to_string(),
            font_family: "Microsoft YaHei, SimSun, sans-serif".to_string(),
            font_size: 14,
        }
    }
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            spell_check: true,
            auto_save: true,
            tab_size: 4,
            word_wrap: true,
        }
    }
}

impl AppConfig {
    /// Get the default config file path
    pub fn get_config_path() -> String {
        #[cfg(target_os = "windows")]
        {
            format!("{}\\{}\\{}", 
                std::env::var("APPDATA").unwrap_or_else(|_| std::env::current_dir().unwrap().to_string_lossy().to_string()),
                "NovelWriter", 
                "config.json")
        }
        #[cfg(not(target_os = "windows"))]
        {
            format!("{}/.novelwriter/config.json", 
                std::env::var("HOME").unwrap_or_else(|_| std::env::current_dir().unwrap().to_string_lossy().to_string()))
        }
    }
    
    /// Load configuration from file
    /// If file doesn't exist, create directories and return default configuration
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        log::debug!("尝试加载配置文件: {}", config_path);
        
        // 检查文件是否存在
        if std::path::Path::new(&config_path).exists() {
            match std::fs::read_to_string(&config_path) {
                Ok(content) => {
                    log::debug!("成功读取配置文件");
                    match serde_json::from_str(&content) {
                        Ok(config) => Ok(config),
                        Err(e) => {
                            log::error!("解析配置文件失败: {}, 使用默认配置", e);
                            Ok(Self::default())
                        }
                    }
                },
                Err(e) => {
                    log::error!("读取配置文件失败: {}, 使用默认配置", e);
                    Ok(Self::default())
                }
            }
        } else {
            log::info!("配置文件不存在: {}, 将使用默认配置", config_path);
            // 尝试创建配置目录
            if let Some(parent) = std::path::Path::new(&config_path).parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    log::warn!("创建配置目录失败: {}", e);
                }
            }
            Ok(Self::default())
        }
    }
    
    /// Save configuration to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        log::debug!("保存配置到文件: {}", config_path);
        
        // 确保目录存在
        if let Some(parent) = std::path::Path::new(&config_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // 序列化配置
        let content = serde_json::to_string_pretty(self)?;
        
        // 写入文件
        std::fs::write(&config_path, content)?;
        log::debug!("配置保存成功");
        Ok(())
    }
    
    /// Get application name
    pub fn get_app_name(&self) -> &str {
        &self.app_name
    }
    
    /// Get version
    pub fn get_version(&self) -> &str {
        &self.version
    }
    
    /// Update theme settings
    pub fn update_theme(&mut self, name: String, font_family: String, font_size: u32) {
        self.theme.name = name;
        self.theme.font_family = font_family;
        self.theme.font_size = font_size;
    }
    
    /// Update editor settings
    pub fn update_editor(&mut self, spell_check: bool, auto_save: bool, tab_size: u32, word_wrap: bool) {
        self.editor.spell_check = spell_check;
        self.editor.auto_save = auto_save;
        self.editor.tab_size = tab_size;
        self.editor.word_wrap = word_wrap;
    }
    
    /// Update auto-save interval
    pub fn update_auto_save_interval(&mut self, interval: u64) {
        self.auto_save_interval = interval;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.app_name, "Novel Writer");
        assert_eq!(config.theme.name, "light");
        assert!(config.editor.auto_save);
    }
    
    #[test]
    fn test_config_load() {
        let config = AppConfig::load();
        assert!(config.is_ok());
    }
}
