use gpui::actions;

use crate::TitleBar;

actions!(
    collab,
    [
        /// Toggles screen sharing on or off.
        ToggleScreenSharing,
        /// Toggles microphone mute.
        ToggleMute,
        /// Toggles deafen mode (mute both microphone and speakers).
        ToggleDeafen
    ]
);

impl TitleBar {
}
