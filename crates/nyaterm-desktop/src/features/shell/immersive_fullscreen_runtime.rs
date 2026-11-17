use gpui::{Context, Window};

use crate::features::NyaTermApp;

impl NyaTermApp {
    pub(in crate::features) fn toggle_immersive_fullscreen(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let browser_was_visible = self.transfer_browser_is_visible();
        let fullscreen = self
            .shell
            .toggle_immersive_fullscreen(window.is_fullscreen());
        if self.shell.immersive_fullscreen() {
            self.shell.close_root_menus();
            self.shell.chrome.activity_bar_context_menu = None;
            self.shell.chrome.header_status_menu_open = false;
            self.shell.panels.clear_floating_outside_press();
            self.transfer.close_transfer_job_menu();
            self.transfer.close_browser_favorites_menu();
            self.transfer.close_browser_path_menu();
            self.transfer.close_browser_upload_menu();
            self.ai.close_transient_menus();
            self.remote_ops.close_docker_menus();
            self.defer_transfer_panel_snapshot_flush(cx);
            self.defer_ai_panel_snapshot_flush(cx);
            self.defer_remote_panel_snapshot_flush(cx);
        }
        if window.is_fullscreen() != fullscreen {
            window.toggle_fullscreen();
        }
        self.immersive_fullscreen_changed(browser_was_visible, window, cx);
    }

    /// Explicitly opening a tool should make that tool visible and focusable.
    pub(in crate::features) fn exit_immersive_fullscreen(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let browser_was_visible = self.transfer_browser_is_visible();
        if let Some(fullscreen) = self.shell.leave_immersive_fullscreen() {
            if window.is_fullscreen() != fullscreen {
                window.toggle_fullscreen();
            }
            self.immersive_fullscreen_changed(browser_was_visible, window, cx);
        }
    }

    pub(in crate::features) fn toggle_native_fullscreen(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let browser_was_visible = self.transfer_browser_is_visible();
        if self.shell.leave_immersive_fullscreen().is_some() {
            // F11 always leaves immersive mode for the ordinary windowed UI.
            if window.is_fullscreen() {
                window.toggle_fullscreen();
            }
            self.immersive_fullscreen_changed(browser_was_visible, window, cx);
        } else {
            window.toggle_fullscreen();
            cx.notify();
        }
    }

    pub(in crate::features) fn reconcile_immersive_fullscreen(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let browser_was_visible = self.transfer_browser_is_visible();
        if self
            .shell
            .observe_immersive_fullscreen(window.is_fullscreen())
        {
            self.immersive_fullscreen_changed(browser_was_visible, window, cx);
        }
    }

    fn immersive_fullscreen_changed(
        &mut self,
        browser_was_visible: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.transfer_browser_visibility_changed(browser_was_visible, cx);
        self.sync_terminal_frame_snapshot_priority();
        self.focus_active_workspace_surface(window, cx);
        cx.notify();
    }
}
