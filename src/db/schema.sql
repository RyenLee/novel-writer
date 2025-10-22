-- 小说写作工具数据库Schema

-- 小说表
CREATE TABLE IF NOT EXISTS novels (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT DEFAULT '',
    description TEXT DEFAULT '',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    word_count INTEGER DEFAULT 0,
    status TEXT DEFAULT 'draft' CHECK(status IN ('draft', 'writing', 'completed', 'abandoned')),
    is_pinned BOOLEAN DEFAULT 0,
    pinned_order INTEGER DEFAULT NULL
);

-- 章节表（使用路径枚举法实现树状结构）
CREATE TABLE IF NOT EXISTS chapters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    novel_id INTEGER NOT NULL,
    parent_id INTEGER,
    title TEXT NOT NULL,
    content TEXT DEFAULT '',
    sort_path TEXT NOT NULL,  -- 排序路径，用于实现树状结构
    word_count INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    chapter_type TEXT DEFAULT 'chapter' CHECK(chapter_type IN ('volume', 'chapter', 'scene')),
    is_archived BOOLEAN DEFAULT 0,
    
    FOREIGN KEY (novel_id) REFERENCES novels(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES chapters(id) ON DELETE SET NULL
);

-- 已移除思维导图相关表定义

-- 章节版本表（版本控制系统）
CREATE TABLE IF NOT EXISTS chapter_versions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    chapter_id INTEGER NOT NULL,
    parent_version_id INTEGER,
    version_type TEXT DEFAULT 'diff' CHECK(version_type IN ('snapshot', 'diff')),
    content TEXT NOT NULL,
    diff_data TEXT,  -- 差异数据，用于diff版本
    word_count INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    commit_message TEXT DEFAULT '',
    is_auto_save BOOLEAN DEFAULT 0,
    
    FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_version_id) REFERENCES chapter_versions(id) ON DELETE SET NULL
);

-- 写作统计表
CREATE TABLE IF NOT EXISTS writing_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    novel_id INTEGER NOT NULL,
    date TEXT NOT NULL,  -- YYYY-MM-DD格式
    word_count INTEGER DEFAULT 0,
    writing_time INTEGER DEFAULT 0,  -- 写作时间（秒）
    session_count INTEGER DEFAULT 1,
    
    FOREIGN KEY (novel_id) REFERENCES novels(id) ON DELETE CASCADE,
    UNIQUE(novel_id, date)
);

-- 创建索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_chapters_novel_id ON chapters(novel_id);
CREATE INDEX IF NOT EXISTS idx_chapters_parent_id ON chapters(parent_id);
CREATE INDEX IF NOT EXISTS idx_chapters_sort_path ON chapters(sort_path);
-- 已移除思维导图相关索引
CREATE INDEX IF NOT EXISTS idx_chapter_versions_chapter ON chapter_versions(chapter_id);
CREATE INDEX IF NOT EXISTS idx_chapter_versions_created ON chapter_versions(created_at);
CREATE INDEX IF NOT EXISTS idx_writing_stats_novel_date ON writing_stats(novel_id, date);

-- 创建触发器以自动更新时间戳
CREATE TRIGGER IF NOT EXISTS update_novel_timestamp 
AFTER UPDATE ON novels
BEGIN
    UPDATE novels SET updated_at = datetime('now') WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_chapter_timestamp
AFTER UPDATE ON chapters
BEGIN
    UPDATE chapters SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- 已移除思维导图相关触发器

-- 灵感功能相关表
CREATE TABLE IF NOT EXISTS inspirations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    novel_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    is_pinned BOOLEAN DEFAULT 0,
    
    FOREIGN KEY (novel_id) REFERENCES novels(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS inspiration_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    inspiration_id INTEGER NOT NULL,
    tag TEXT NOT NULL,
    
    FOREIGN KEY (inspiration_id) REFERENCES inspirations(id) ON DELETE CASCADE,
    UNIQUE(inspiration_id, tag)
);

CREATE TABLE IF NOT EXISTS inspiration_chapter_links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    inspiration_id INTEGER NOT NULL,
    chapter_id INTEGER NOT NULL,
    
    FOREIGN KEY (inspiration_id) REFERENCES inspirations(id) ON DELETE CASCADE,
    FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE CASCADE,
    UNIQUE(inspiration_id, chapter_id)
);

-- 创建触发器以更新小说字数统计
CREATE TRIGGER IF NOT EXISTS update_novel_word_count 
AFTER UPDATE ON chapters
BEGIN
    UPDATE novels 
    SET word_count = (
        SELECT COALESCE(SUM(word_count), 0) 
        FROM chapters 
        WHERE novel_id = NEW.novel_id AND is_archived = 0
    )
    WHERE id = NEW.novel_id;
END;

CREATE TRIGGER IF NOT EXISTS insert_novel_word_count 
AFTER INSERT ON chapters
BEGIN
    UPDATE novels 
    SET word_count = (
        SELECT COALESCE(SUM(word_count), 0) 
        FROM chapters 
        WHERE novel_id = NEW.novel_id AND is_archived = 0
    )
    WHERE id = NEW.novel_id;
END;