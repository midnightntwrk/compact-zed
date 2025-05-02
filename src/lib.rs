use zed_extension_api::{self as zed, LanguageServerId, Result};

struct CompactExtension;

impl CompactExtension {
    fn language_server_binary_path(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        worktree
            .which("compact-language-server")
            .ok_or("compact-language-server not found".to_string())
    }
}

impl zed::Extension for CompactExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(CompactExtension);
