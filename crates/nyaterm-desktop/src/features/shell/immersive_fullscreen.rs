/// Temporary presentation state; panel visibility and sizes remain untouched.
#[derive(Default)]
pub(super) struct ImmersiveFullscreenState {
    entry: Option<FullscreenEntry>,
}

struct FullscreenEntry {
    was_fullscreen: bool,
    observed_fullscreen: bool,
}

impl ImmersiveFullscreenState {
    pub(super) fn is_active(&self) -> bool {
        self.entry.is_some()
    }

    /// Returns the native fullscreen state the caller should request.
    pub(super) fn toggle(&mut self, fullscreen: bool) -> bool {
        if let Some(previous) = self.leave() {
            return previous;
        }
        self.entry = Some(FullscreenEntry {
            was_fullscreen: fullscreen,
            observed_fullscreen: fullscreen,
        });
        true
    }

    pub(super) fn leave(&mut self) -> Option<bool> {
        self.entry.take().map(|entry| entry.was_fullscreen)
    }

    /// Native fullscreen changes can arrive asynchronously. Ignore windowed
    /// observations during entry, but restore chrome after a native exit.
    pub(super) fn observe_fullscreen(&mut self, fullscreen: bool) -> bool {
        let Some(entry) = self.entry.as_mut() else {
            return false;
        };
        if fullscreen {
            entry.observed_fullscreen = true;
            return false;
        }
        if entry.observed_fullscreen {
            self.entry = None;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::ImmersiveFullscreenState;

    #[test]
    fn windowed_entry_restores_windowed_state() {
        let mut state = ImmersiveFullscreenState::default();
        assert!(!state.is_active());
        assert!(state.toggle(false));
        assert!(state.is_active());
        assert!(!state.observe_fullscreen(true));
        assert!(!state.toggle(true));
        assert!(!state.is_active());
    }

    #[test]
    fn fullscreen_entry_restores_existing_native_fullscreen() {
        let mut state = ImmersiveFullscreenState::default();
        assert!(state.toggle(true));
        assert!(state.toggle(true));
        assert!(!state.is_active());
    }

    #[test]
    fn pending_native_entry_does_not_restore_chrome_prematurely() {
        let mut state = ImmersiveFullscreenState::default();
        state.toggle(false);
        assert!(!state.observe_fullscreen(false));
        assert!(state.is_active());
        assert!(!state.observe_fullscreen(true));
        assert!(state.is_active());
    }

    #[test]
    fn external_native_exit_restores_chrome_once() {
        for already_fullscreen in [false, true] {
            let mut state = ImmersiveFullscreenState::default();
            state.toggle(already_fullscreen);
            state.observe_fullscreen(true);
            assert!(state.observe_fullscreen(false));
            assert!(!state.is_active());
            assert!(!state.observe_fullscreen(false));
            assert_eq!(state.leave(), None);
        }
    }

    #[test]
    fn leaving_before_native_entry_completes_forgets_the_old_restore_state() {
        let mut state = ImmersiveFullscreenState::default();
        state.toggle(false);
        assert!(!state.toggle(false));
        assert!(!state.is_active());
        assert!(!state.observe_fullscreen(true));
        assert!(!state.is_active());
        state.toggle(true);
        assert_eq!(state.leave(), Some(true));
    }

    #[test]
    fn explicit_exit_is_idempotent() {
        for fullscreen in [false, true] {
            let mut state = ImmersiveFullscreenState::default();
            assert_eq!(state.leave(), None);
            state.toggle(fullscreen);
            assert_eq!(state.leave(), Some(fullscreen));
            assert_eq!(state.leave(), None);
            assert!(!state.is_active());
        }
    }
}
