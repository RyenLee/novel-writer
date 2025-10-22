use regex::Regex;

pub struct Formatter;

impl Formatter {
    pub fn new() -> Self {
        Self
    }
    
    /// 格式化文本内容
    pub fn format_text(&self, text: &str, options: &FormatOptions) -> String {
        let mut result = text.to_string();
        
        // 按顺序应用各种格式化规则
        if options.remove_extra_spaces {
            result = self.remove_extra_spaces(&result);
        }
        
        if options.remove_extra_newlines {
            result = self.remove_extra_newlines(&result);
        }
        
        if options.fix_punctuation_spacing {
            result = self.fix_punctuation_spacing(&result);
        }
        
        if options.auto_indent_paragraphs {
            result = self.auto_indent_paragraphs(&result, options.indent_spaces);
        }
        
        if options.trim_trailing_spaces {
            result = self.trim_trailing_spaces(&result);
        }
        
        if options.ensure_trailing_newline {
            result = self.ensure_trailing_newline(&result);
        }
        
        result
    }
    
    /// 移除多余的空格
    fn remove_extra_spaces(&self, text: &str) -> String {
        let re = Regex::new(r"\s+").expect("Invalid regex pattern for extra spaces");
        re.replace_all(text, " ").to_string()
    }
    
    /// 移除多余的空行
    fn remove_extra_newlines(&self, text: &str) -> String {
        let re = Regex::new(r"\n{3,}").expect("Invalid regex pattern for extra newlines");
        re.replace_all(text, "\n\n").to_string()
    }
    
    /// 修正标点符号间距（中英文混合排版）
    fn fix_punctuation_spacing(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        // 中文标点后加空格
        let chinese_punctuation = Regex::new(r"([。！？；：，])([^\s])").expect("Invalid regex pattern for Chinese punctuation");
        result = chinese_punctuation.replace_all(&result, "$1 $2").to_string();
        
        // 英文标点前后空格处理
        let english_punctuation = Regex::new(r"(\w)([,.!?;:])(\w)").expect("Invalid regex pattern for English punctuation");
        result = english_punctuation.replace_all(&result, "$1$2 $3").to_string();
        
        result
    }
    
    /// 自动段落缩进
    fn auto_indent_paragraphs(&self, text: &str, indent_spaces: usize) -> String {
        let indent = " ".repeat(indent_spaces);
        let lines: Vec<&str> = text.lines().collect();
        let mut result = String::new();
        let mut in_paragraph = false;
        
        for line in lines {
            let trimmed = line.trim();
            
            if trimmed.is_empty() {
                // 空行结束段落
                if in_paragraph {
                    result.push('\n');
                    in_paragraph = false;
                }
                result.push('\n');
            } else {
                // 非空行
                if !in_paragraph {
                    // 新段落开始，添加缩进
                    result.push_str(&indent);
                    in_paragraph = true;
                } else {
                    // 段落继续，换行但不缩进
                    result.push('\n');
                }
                result.push_str(line.trim_start());
            }
        }
        
        result
    }
    
    /// 移除行尾空格
    fn trim_trailing_spaces(&self, text: &str) -> String {
        text.lines()
            .map(|line| line.trim_end())
            .collect::<Vec<&str>>()
            .join("\n")
    }
    
    /// 确保文本以换行符结尾
    fn ensure_trailing_newline(&self, text: &str) -> String {
        let mut result = text.to_string();
        if !result.ends_with('\n') {
            result.push('\n');
        }
        result
    }
    
    /// 统计文本信息
    pub fn analyze_text(&self, text: &str) -> TextStatistics {
        let chinese_chars = self.count_chinese_characters(text);
        let english_words = self.count_english_words(text);
        let total_chars = text.chars().count();
        let paragraphs = self.count_paragraphs(text);
        let lines = text.lines().count();
        
        TextStatistics {
            total_chars,
            chinese_chars,
            english_words,
            total_words: chinese_chars + english_words,
            paragraphs,
            lines,
            reading_time: self.calculate_reading_time(chinese_chars + english_words),
        }
    }
    
    /// 统计中文字符数
    fn count_chinese_characters(&self, text: &str) -> usize {
        text.chars()
            .filter(|c| {
                let codepoint = *c as u32;
                // 中文字符的Unicode范围
                (0x4E00..=0x9FFF).contains(&codepoint) || // 基本汉字
                (0x3400..=0x4DBF).contains(&codepoint) || // 扩展A
                (0x20000..=0x2A6DF).contains(&codepoint) || // 扩展B
                (0x2A700..=0x2B73F).contains(&codepoint) || // 扩展C
                (0x2B740..=0x2B81F).contains(&codepoint) || // 扩展D
                (0xF900..=0xFAFF).contains(&codepoint) // 兼容汉字
            })
            .count()
    }
    
    /// 统计英文单词数
    fn count_english_words(&self, text: &str) -> usize {
        let re = Regex::new(r"[a-zA-Z]+(?:'[a-zA-Z]+)?").expect("Invalid regex pattern for English words");
        re.find_iter(text).count()
    }
    
    /// 统计段落数
    fn count_paragraphs(&self, text: &str) -> usize {
        if text.trim().is_empty() {
            return 0;
        }
        
        text.split("\n\n")
            .filter(|para| !para.trim().is_empty())
            .count()
    }
    
    /// 计算阅读时间（分钟）
    fn calculate_reading_time(&self, word_count: usize) -> f64 {
        // 假设平均阅读速度：中文200字/分钟，英文200词/分钟
        word_count as f64 / 200.0
    }
    
    /// 批量处理多个章节
    pub fn batch_format_chapters(&self, chapters: &[String], options: &FormatOptions) -> Vec<String> {
        chapters.iter()
            .map(|chapter| self.format_text(chapter, options))
            .collect()
    }
    
    /// 检查文本格式问题
    pub fn check_format_issues(&self, text: &str) -> Vec<FormatIssue> {
        let mut issues = Vec::new();
        
        // 检查多余空格
        if text.contains("  ") {
            issues.push(FormatIssue::ExtraSpaces);
        }
        
        // 检查多余空行
        if text.contains("\n\n\n") {
            issues.push(FormatIssue::ExtraNewlines);
        }
        
        // 检查行尾空格
        if text.lines().any(|line| line.ends_with(' ')) {
            issues.push(FormatIssue::TrailingSpaces);
        }
        
        // 检查缺少结尾换行
        if !text.ends_with('\n') && !text.is_empty() {
            issues.push(FormatIssue::MissingTrailingNewline);
        }
        
        // 检查标点符号问题
        if self.has_punctuation_issues(text) {
            issues.push(FormatIssue::PunctuationSpacing);
        }
        
        issues
    }
    
    /// 检查标点符号问题
    fn has_punctuation_issues(&self, text: &str) -> bool {
        let patterns = [
            r"[。！？；：，][^\s]", // 中文标点后无空格
            r"\w[,.!?;:]\w",       // 英文标点前后无空格
        ];
        
        patterns.iter().any(|pattern| {
            Regex::new(pattern).expect("Invalid regex pattern for punctuation check").is_match(text)
        })
    }
    
    /// 生成格式报告
    pub fn generate_format_report(&self, text: &str) -> FormatReport {
        let stats = self.analyze_text(text);
        let issues = self.check_format_issues(text);
        let suggestions = self.generate_suggestions(&issues);
        
        FormatReport {
            statistics: stats,
            issues,
            suggestions,
        }
    }
    
    /// 生成改进建议
    fn generate_suggestions(&self, issues: &[FormatIssue]) -> Vec<String> {
        issues.iter()
            .map(|issue| match issue {
                FormatIssue::ExtraSpaces => "移除多余空格".to_string(),
                FormatIssue::ExtraNewlines => "移除多余空行".to_string(),
                FormatIssue::TrailingSpaces => "移除行尾空格".to_string(),
                FormatIssue::MissingTrailingNewline => "添加结尾换行符".to_string(),
                FormatIssue::PunctuationSpacing => "修正标点符号间距".to_string(),
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct FormatOptions {
    pub remove_extra_spaces: bool,
    pub remove_extra_newlines: bool,
    pub fix_punctuation_spacing: bool,
    pub auto_indent_paragraphs: bool,
    pub trim_trailing_spaces: bool,
    pub ensure_trailing_newline: bool,
    pub indent_spaces: usize,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            remove_extra_spaces: true,
            remove_extra_newlines: true,
            fix_punctuation_spacing: true,
            auto_indent_paragraphs: true,
            trim_trailing_spaces: true,
            ensure_trailing_newline: true,
            indent_spaces: 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextStatistics {
    pub total_chars: usize,
    pub chinese_chars: usize,
    pub english_words: usize,
    pub total_words: usize,
    pub paragraphs: usize,
    pub lines: usize,
    pub reading_time: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormatIssue {
    ExtraSpaces,
    ExtraNewlines,
    TrailingSpaces,
    MissingTrailingNewline,
    PunctuationSpacing,
}

#[derive(Debug, Clone)]
pub struct FormatReport {
    pub statistics: TextStatistics,
    pub issues: Vec<FormatIssue>,
    pub suggestions: Vec<String>,
}