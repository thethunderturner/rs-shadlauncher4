use crate::config::storage::user_directory;
use crate::library::title::Title;
use rusqlite::{Connection, params};
use std::io::Cursor;
use std::path::Path;

pub struct Db {
    conn: Connection,
}

#[derive(Default, Debug)]
pub struct LoadedTitle {
    pub title: Title,
    pub icon: Option<std::sync::Arc<[u8]>>, // Only used when loading the icon at the launcher
    pub background: Option<std::sync::Arc<[u8]>>, // Only used when you click on a row
    pub compatibility: Option<String>,
    pub playtime: Option<String>,
    pub has_custom_config: bool,
}

#[derive(Default, Debug)]
pub struct LoadedTitles {
    pub loaded_titles: Vec<LoadedTitle>,
    pub load_ms: u128,
}

impl LoadedTitles {
    pub fn load() -> Self {
        Self::try_load().unwrap_or_else(|error| {
            eprintln!("Failed to load cached titles: {error}");
            Self::default()
        })
    }

    pub fn try_load() -> std::io::Result<Self> {
        let start = std::time::Instant::now();
        let db = Db::connect()?;
        db.create_tables().map_err(std::io::Error::other)?;
        let titles = db.load_titles().map_err(std::io::Error::other)?;

        Ok(Self {
            loaded_titles: titles,
            load_ms: start.elapsed().as_millis(),
        })
    }
}

impl Db {
    pub fn connect() -> std::io::Result<Self> {
        let path = user_directory()?.join("cache.db");
        let connection = Connection::open(path).map_err(std::io::Error::other)?;

        Ok(Self { conn: connection })
    }

    pub fn load_titles(&self) -> rusqlite::Result<Vec<LoadedTitle>> {
        let mut statement = self.conn.prepare(
            "SELECT titles.serial, parent_dir, name, publisher_id, version, fw, app_type, category, app_path, patch_path, icon, supports_vr, supports_neo, COALESCE(NULLIF(c.status, ''), NULLIF(titles.compatibility, ''), 'Unknown'), playtime, NULLIF(c.milestone, '')
                 FROM titles LEFT JOIN compatibility c ON titles.serial = c.serial
                 ORDER BY name COLLATE NOCASE, titles.serial",
        )?;
        statement
            .query_map([], |row| {
                let custom_config_dir = user_directory()
                    .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?
                    .join("custom_configs");
                Ok(LoadedTitle {
                    title: Title {
                        serial: row.get(0)?,
                        parent_dir: row.get::<_, String>(1)?.into(),
                        name: row.get(2)?,
                        publisher_id: row.get(3)?,
                        version: row.get(4)?,
                        fw: row.get(5)?,
                        app_type: row.get(6)?,
                        category: row.get(7)?,
                        app_path: row.get::<_, Option<String>>(8)?.map(Into::into),
                        patch_path: row.get::<_, Option<String>>(9)?.map(Into::into),
                        supports_vr: row.get(11)?,
                        supports_neo: row.get(12)?,
                        milestone: row.get(15)?,
                        ..Title::default()
                    },
                    icon: row.get::<_, Option<Vec<u8>>>(10)?.map(Into::into),
                    compatibility: row.get(13)?,
                    playtime: row.get(14)?,
                    has_custom_config: custom_config_dir
                        .join(format!("{}.json", row.get::<_, String>(0)?))
                        .is_file(),
                    ..LoadedTitle::default()
                })
            })?
            .collect()
    }

    pub fn save_titles(&mut self, titles: &[Title]) -> rusqlite::Result<()> {
        self.save_titles_with_progress(titles, |_, _| {})
    }

    pub fn save_titles_with_progress(
        &mut self,
        titles: &[Title],
        mut progress: impl FnMut(usize, usize),
    ) -> rusqlite::Result<()> {
        progress(0, titles.len());
        let transaction = self.conn.transaction()?;
        {
            let mut statement = transaction.prepare(
                "INSERT INTO Titles(serial, parent_dir, NAME, publisher_id, VERSION, fw, app_type, category, app_path, patch_path, icon, background, supports_vr, supports_neo)
                     VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
                     ON Conflict(serial) do UPDATE SET
                      parent_dir = excluded.parent_dir,
                      NAME = excluded.NAME,
                      publisher_id = excluded.publisher_id,
                      VERSION = excluded.VERSION,
                      fw = excluded.fw,
                      app_type = excluded.app_type,
                      category = excluded.category,
                      app_path = excluded.app_path,
                      patch_path = excluded.patch_path,
                      icon = excluded.icon,
                      background = excluded.background,
                      supports_vr = excluded.supports_vr,
                      supports_neo = excluded.supports_neo",
            )?;

            for (index, title) in titles.iter().enumerate() {
                let icon = read_artwork(title.icon_path.as_deref(), 128, 128);
                let background = read_artwork(title.background_path.as_deref(), 1280, 720);
                statement.execute(params![
                    title.serial,
                    title.parent_dir.to_string_lossy(),
                    title.name,
                    title.publisher_id,
                    title.version,
                    title.fw,
                    title.app_type,
                    title.category,
                    title.app_path.as_deref().map(|path| path.to_string_lossy()),
                    title.patch_path.as_deref().map(|path| path.to_string_lossy()),
                    icon,
                    background,
                    title.supports_vr,
                    title.supports_neo,
                ])?;
                progress(index + 1, titles.len());
            }
        }

        transaction.commit()
    }

    pub fn save_compatibility(&mut self, reports: &super::compatibility::Reports) -> rusqlite::Result<()> {
        let transaction = self.conn.transaction()?;
        transaction.execute("DELETE FROM compatibility", [])?;
        // Remove legacy statuses so missing reports stay unknown after restarting.
        transaction.execute("UPDATE titles SET compatibility = NULL", [])?;
        {
            let mut statement = transaction
                .prepare("INSERT INTO compatibility (serial, status, milestone) VALUES (?1, ?2, ?3)")?;
            for (serial, report) in reports {
                statement.execute(params![serial, report.status, report.milestone])?;
            }
        }
        transaction.commit()
    }

    pub fn create_tables(&self) -> rusqlite::Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS titles
                  (
                     serial          TEXT PRIMARY KEY,
                     parent_dir      TEXT NOT NULL,
                     name            TEXT NOT NULL,
                     publisher_id    TEXT NOT NULL,
                     version         TEXT NOT NULL,
                     fw              INTEGER NOT NULL,
                     app_type        INTEGER,
                     category        TEXT NOT NULL,
                     app_path        TEXT,
                     patch_path      TEXT,
                     icon            BLOB,
                     background      BLOB,
                     supports_vr     INTEGER NOT NULL,
                     supports_neo    INTEGER NOT NULL,
                     compatibility   TEXT DEFAULT '',
                     playtime        TEXT DEFAULT ''
                  );
             CREATE TABLE IF NOT EXISTS compatibility (
                 serial TEXT PRIMARY KEY,
                 status TEXT NOT NULL,
                 milestone TEXT
             );",
        )
    }

    // We load the background dynamically from the cache. Backgrounds are quite large, so loading everything in memory would take around 300MB of RAM
    // Furtehrmore, backgrounds are loaded once at a time, when you click on a row. Icons in contrast are loaded all at once in the table
    pub fn load_background(&self, serial: &str) -> rusqlite::Result<Option<Vec<u8>>> {
        self.conn
            .query_row("SELECT background FROM titles WHERE serial = ?1", params![serial], |row| row.get(0))
    }
}

fn read_artwork(path: Option<&Path>, width: u32, height: u32) -> Option<Vec<u8>> {
    let path = path?;

    let result = (|| -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let bytes = std::fs::read(path)?;
        let image = image::load_from_memory(&bytes)?;
        let resized = image.thumbnail(width, height);

        let mut output = Cursor::new(Vec::new());
        resized.write_to(&mut output, image::ImageFormat::Png)?;
        Ok(output.into_inner())
    })();

    match result {
        Ok(bytes) => Some(bytes),
        Err(error) => {
            eprintln!("Failed to cache artwork {}: {error}", path.display());
            None
        }
    }
}
