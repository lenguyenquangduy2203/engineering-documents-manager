-- Enable foreign key support inside SQLite (Crucial!)
PRAGMA foreign_keys = ON;

-- 1. Core Component Entities
CREATE TABLE IF NOT EXISTS components (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL,                  -- "Schematic", "DesignSpec", etc.
    current_title TEXT NOT NULL
);

-- 2. Component Versions Data (The actual JSON state trait)
CREATE TABLE IF NOT EXISTS component_versions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    component_id INTEGER NOT NULL,
    version_number INTEGER NOT NULL,
    data TEXT NOT NULL,                  -- Validated as JSON by your Rust Serde layer
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (component_id) REFERENCES components(id) ON DELETE CASCADE
);

-- 3. Document Entities (Holding the Layout Array Component)
CREATE TABLE IF NOT EXISTS documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    is_completed INTEGER DEFAULT 0,
    published_at DATETIME
);

-- 4. Cleaned up Junction Table (Optional, but highly recommended)
CREATE TABLE IF NOT EXISTS document_layouts (
    document_id INTEGER NOT NULL,
    component_version_id INTEGER NOT NULL,
    position INTEGER NOT NULL, -- 0-indexed ordering track
    PRIMARY KEY (document_id, component_version_id),
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (component_version_id) REFERENCES component_versions(id) ON DELETE CASCADE,
    -- Enforce that a component can't occupy the same position in a doc twice
    UNIQUE(document_id, position)
);