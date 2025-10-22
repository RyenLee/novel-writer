use similar::{TextDiff, ChangeTag};

pub struct DiffUtils;

impl DiffUtils {
    pub fn calculate_diff(old_text: &str, new_text: &str) -> String {
        let diff = TextDiff::from_chars(old_text, new_text);
        
        let mut result = String::new();
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => {
                    result.push_str(&format!("\x1b[31m[-{}]\x1b[0m", change.value()));
                }
                ChangeTag::Insert => {
                    result.push_str(&format!("\x1b[32m[+{}]\x1b[0m", change.value()));
                }
                ChangeTag::Equal => {
                    result.push_str(change.value());
                }
            }
        }
        
        result
    }
    
    pub fn get_change_statistics(old_text: &str, new_text: &str) -> ChangeStats {
        let diff = TextDiff::from_chars(old_text, new_text);
        
        let mut stats = ChangeStats::default();
        
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => stats.deletions += change.value().chars().count(),
                ChangeTag::Insert => stats.insertions += change.value().chars().count(),
                ChangeTag::Equal => stats.unchanged += change.value().chars().count(),
            }
        }
        
        stats.total_changes = stats.insertions + stats.deletions;
        stats
    }
    
    pub fn find_similar_chunks(text1: &str, text2: &str, min_length: usize) -> Vec<SimilarChunk> {
        let _diff = TextDiff::from_chars(text1, text2);
        let mut similar_chunks = Vec::new();
        
        // Simplified implementation: compare chunks directly
        let lines1: Vec<&str> = text1.lines().collect();
        let lines2: Vec<&str> = text2.lines().collect();
        
        for (i, line1) in lines1.iter().enumerate() {
            for (j, line2) in lines2.iter().enumerate() {
                if line1.len() >= min_length && line2.len() >= min_length {
                    let similarity = Self::calculate_similarity(line1, line2);
                    
                    if similarity > 0.7 { // 70% similarity threshold
                        similar_chunks.push(SimilarChunk {
                            old_text: line1.to_string(),
                            new_text: line2.to_string(),
                            similarity,
                            old_start: i,
                            new_start: j,
                        });
                    }
                }
            }
        }
        
        similar_chunks
    }
    
    fn calculate_similarity(text1: &str, text2: &str) -> f64 {
        let diff = TextDiff::from_chars(text1, text2);
        let total_chars = text1.chars().count().max(text1.chars().count());
        
        if total_chars == 0 {
            return 1.0;
        }
        
        let changes = diff.iter_all_changes().filter(|c| c.tag() != ChangeTag::Equal).count();
        1.0 - (changes as f64 / total_chars as f64)
    }
    
    pub fn create_patch(old_text: &str, new_text: &str) -> String {
        let diff = TextDiff::from_lines(old_text, new_text);
        diff.unified_diff()
            .context_radius(3)
            .header("old", "new")
            .to_string()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ChangeStats {
    pub insertions: usize,
    pub deletions: usize,
    pub unchanged: usize,
    pub total_changes: usize,
}

#[derive(Debug, Clone)]
pub struct SimilarChunk {
    pub old_text: String,
    pub new_text: String,
    pub similarity: f64,
    pub old_start: usize,
    pub new_start: usize,
}