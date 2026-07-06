//! Session persistence — save/restore workspaces, layouts, and working directories.
//!
//! Resume snapshots are host-specific, so they are stored per-hostname under
//! `~/.config/herdr/hosts/<hostname>/session.json` (with pane screen history
//! alongside in `session-history.json`). This keeps a synced config dir from
//! sharing one host's session with another; the pre-per-host shared
//! `~/.config/herdr/session.json` is still read once as a migration fallback.
//! Installed plugins are persisted separately (shared) at `plugins.json`.

mod io;
pub mod plugin_registry;
mod restore;
mod snapshot;

pub use self::io::{clear, clear_history, load, load_history, save};
pub use self::restore::restore;
#[cfg(unix)]
pub use self::restore::{handoff_pane_aliases, restore_handoff};
pub use self::snapshot::{
    capture, capture_history, DirectionSnapshot, LayoutSnapshot, SessionHistorySnapshot,
    SessionSnapshot, TabSnapshot, WorkspaceSnapshot,
};
