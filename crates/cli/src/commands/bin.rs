use crate::{
    helpers::enable_logging_with_level,
    tools::{create_tool, ToolType},
};
use proto_core::{detect_version_from_environment, ProtoError};

pub async fn bin(
    tool_type: ToolType,
    forced_version: Option<String>,
    use_shim: bool,
) -> Result<(), ProtoError> {
    enable_logging_with_level("warn");

    let mut tool = create_tool(&tool_type)?;
    let version = detect_version_from_environment(&tool, forced_version).await?;

    tool.resolve_version(&version).await?;
    tool.find_bin_path().await?;

    if use_shim {
        tool.create_shims().await?;

        if let Some(shim_path) = tool.get_shim_path() {
            println!("{}", shim_path.to_string_lossy());

            return Ok(());
        }
    }

    println!("{}", tool.get_bin_path()?.to_string_lossy());

    Ok(())
}
