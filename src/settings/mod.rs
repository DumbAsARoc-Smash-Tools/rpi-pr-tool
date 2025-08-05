use anyhow::anyhow;
use oauth2::basic::BasicTokenType;
use oauth2::{EmptyExtraTokenFields, StandardTokenResponse};

pub type SettingsArc = std::sync::Arc<std::sync::Mutex<RPIPRSettings>>;
type TokenType = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
pub const DEFAULT_SETTINGS_LOC: &str = "rpipr.settings";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RPIPRSettings {
    /// The path to which this struct should be saved
    /// on the filesystem
    #[serde(skip)]
    save_path: String,

    // StartGG Account Settings
    token: Option<TokenType>,
}

impl Default for RPIPRSettings {
    fn default() -> Self {
        Self {
            save_path: DEFAULT_SETTINGS_LOC.to_string(),
            token: None,
        }
    }
}

impl RPIPRSettings {
    /// Loads a settings file from the filesystem,
    /// either at the path provided as `path`, or
    /// the default path specified above with
    /// DEFAULT_SETTINGS_LOC. If a file isn't
    /// at the path specified, this method returns
    /// a new settings instance whose save path is
    /// the same as the passed in path (or default path).
    /// Any other errors are returned as anyhow!()
    /// calls for GUI error handling purposes.
    pub fn load_from_file<P>(path: Option<P>) -> anyhow::Result<SettingsArc>
    where
        P: ToString,
    {
        let fs_path = match path {
            Some(path) => path.to_string(),
            None => DEFAULT_SETTINGS_LOC.to_string(),
        };
        let settings_file = match std::fs::File::open(&fs_path) {
            Ok(f) => f,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    let settings = Self {
                        save_path: fs_path.clone(),
                        ..Default::default()
                    };
                    settings.save_to_file()?;
                    return Ok(std::sync::Arc::new(std::sync::Mutex::new(settings)));
                }
                return Err(anyhow!("Error opening settings file: {}", e));
            }
        };

        let mut settings: Self = match serde_json::from_reader(settings_file) {
            Ok(set) => set,
            Err(e) => return Err(anyhow!("Error deserializing settings: {}", e)),
        };
        settings.save_path = fs_path.clone();

        Ok(std::sync::Arc::new(std::sync::Mutex::new(settings)))
    }

    /// Save the contents of the RPIPRSettings object
    /// to the file dictated in self.save_path. Will
    /// return an error on any failures.
    pub fn save_to_file(&self) -> anyhow::Result<()> {
        use std::io::Write;

        let serialized = serde_json::to_string_pretty(self)?;
        let mut file = match std::fs::File::create(&self.save_path) {
            Ok(f) => f,
            Err(e) => return Err(anyhow!("Failed to write to settings file: {}", e)),
        };
        write!(file, "{}", serialized)?;

        Ok(())
    }

    pub fn get_token(&self) -> Option<TokenType> {
        self.token.clone()
    }

    pub fn set_token(&self, token: Option<TokenType>) -> Self {
        Self {
            token,

            save_path: self.save_path.clone(),
        }
    }
}
