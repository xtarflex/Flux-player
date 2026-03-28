use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg(mobile)]
pub const DB_NAME: &str = "sqlite:flux.db";
#[cfg(not(mobile))]
pub const DB_NAME: &str = "sqlite:flux.db";

pub fn get_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            description: "create media table",
            sql: "
                CREATE TABLE IF NOT EXISTS media (
                    path TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    year INTEGER,
                    artist TEXT,
                    album TEXT,
                    poster_path TEXT,
                    backdrop_path TEXT,
                    album_art_path TEXT,
                    duration INTEGER,
                    media_type TEXT CHECK(media_type IN ('video', 'audio', 'mixed')),
                    last_played INTEGER DEFAULT 0,
                    added_at INTEGER NOT NULL
                );
            ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "create playlists table",
            sql: "
                CREATE TABLE IF NOT EXISTS playlists (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    description TEXT,
                    type TEXT CHECK(type IN ('audio', 'video', 'mixed')),
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL,
                    is_smart BOOLEAN DEFAULT 0,
                    smart_criteria TEXT,
                    cover_image TEXT
                );
            ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "create playlist_items table",
            sql: "
                CREATE TABLE IF NOT EXISTS playlist_items (
                    playlist_id TEXT NOT NULL,
                    media_path TEXT NOT NULL,
                    position INTEGER NOT NULL,
                    added_at INTEGER NOT NULL,
                    FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
                    FOREIGN KEY (media_path) REFERENCES media(path) ON DELETE CASCADE,
                    PRIMARY KEY (playlist_id, media_path)
                );
            ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 4,
            description: "create settings table",
            sql: "
                CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                );
            ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 5,
            description: "add synopsis column",
            sql: "ALTER TABLE media ADD COLUMN synopsis TEXT;",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 6,
            description: "add rating column",
            sql: "ALTER TABLE media ADD COLUMN rating REAL;",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 7,
            description: "add genres column",
            sql: "ALTER TABLE media ADD COLUMN genres TEXT;",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 8,
            description: "add director column",
            sql: "ALTER TABLE media ADD COLUMN director TEXT;",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 9,
            description: "add starring column",
            sql: "ALTER TABLE media ADD COLUMN starring TEXT;",
            kind: MigrationKind::Up,
        },
    ]
}
