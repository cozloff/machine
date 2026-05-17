use crate::services::CommandResult;
use std::path::Path;

pub fn load_dotenv() -> CommandResult {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    dotenvy::from_path_override(path).ok();
    Ok(())
}
