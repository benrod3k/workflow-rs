//!
//! Builder for application shortcuts.
//!
//! # Synopsis
//! ```rust
//! use workflow_nw::prelude::*;
//! use workflow_nw::result::Result;
//! use workflow_dom::utils::window;
//!
//! # fn test()->Result<()>{
//!
//! let shortcut = ShortcutBuilder::new()
//!     .key("Ctrl+Shift+Q")
//!     .active(|_|{
//!         window().alert_with_message("Ctrl+Shift+Q pressed, App will close")?;
//!         //nw_sys::app::quit();
//!         nw_sys::app::close_all_windows();
//!         Ok(())
//!     })
//!     .build()?;
//!     
//! nw_sys::app::register_global_hot_key(&shortcut);
//!
//! # Ok(())
//! # }
//! ```
//!

use crate::application::app;
use crate::result::Result;
use nw_sys::prelude::*;
use wasm_bindgen::prelude::*;
use workflow_wasm::prelude::*;

/// Shortcut Info Object returned by [`ShortcutBuilder.finalize`](ShortcutBuilder#method.finalize) method
pub struct ShortcutInfo {
    pub shortcut: nw_sys::Shortcut,
    pub active_callback: Option<Callback<CallbackClosure<JsValue>>>,
    pub failed_callback: Option<Callback<CallbackClosure<JsValue>>>,
}

/// Provides a builder pattern for building application
/// keyboard shortcuts.
///
/// For usage example please refer to [Examples](self)
pub struct ShortcutBuilder {
    pub options: nw_sys::shortcut::Options,
    pub active_callback: Option<Callback<CallbackClosure<JsValue>>>,
    pub failed_callback: Option<Callback<CallbackClosure<JsValue>>>,
}

impl Default for ShortcutBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ShortcutBuilder {
    pub fn new() -> Self {
        Self {
            options: nw_sys::shortcut::Options::new(),
            active_callback: None,
            failed_callback: None,
        }
    }

    fn set(mut self, key: &str, value: JsValue) -> Self {
        self.options = self.options.set(key, value);
        self
    }

    /// Set the `key` of a `Shortcut`.
    /// It is a string to specify the shortcut key, like "Ctrl+Alt+A".
    /// The key is consisted of zero or more modifiers and a key on your keyboard.
    /// Only one key code is supported. Key code is case insensitive.
    ///
    /// ### List of supported modifiers:
    ///
    /// - Ctrl
    /// - Alt
    /// - Shift
    /// - Command: Command modifier maps to Apple key (⌘) on Mac,
    ///   and maps to the Windows key on Windows and Linux.
    ///
    /// ### List of supported keys:
    ///
    /// - Alphabet: `A`-`Z`
    /// - Digits: `0`-`9`
    /// - Function Keys: `F1`-`F24`
    /// - Home / End / PageUp / PageDown / Insert / Delete
    /// - Up / Down / Left / Right
    /// - MediaNextTrack / MediaPlayPause / MediaPrevTrack / MediaStop
    /// - Comma or `,`
    /// - Period or `.`
    /// - Tab or `\t`
    /// - Backquote or `` ` ``
    /// - Enter or `\n`
    /// - Minus or `-`
    /// - Equal or `=`
    /// - Backslash or `\`
    /// - Semicolon or `;`
    /// - Quote or `'`
    /// - BracketLeft or `[`
    /// - BracketRight or `]`
    /// - Escape
    ///
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutkey)
    pub fn key(self, key: &str) -> Self {
        self.set("key", JsValue::from(key))
    }

    /// Set the active callback of a Shortcut.
    /// It will be called when user presses the shortcut.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutactive)
    pub fn active<F>(mut self, callback: F) -> Self
    where
        F: FnMut(JsValue) -> std::result::Result<(), JsValue> + 'static,
    {
        let callback = Callback::new(callback);
        self = self.set("active", callback.clone().into());
        self.active_callback = Some(callback);

        self
    }

    /// Set the failed callback of a Shortcut.
    /// It will be called when application passes an invalid key,
    /// or failed to register the key.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutfailed)
    pub fn failed<F>(mut self, callback: F) -> Self
    where
        F: FnMut(JsValue) -> std::result::Result<(), JsValue> + 'static,
    {
        let callback = Callback::new(callback);
        self = self.set("failed", callback.clone().into());
        self.failed_callback = Some(callback);

        self
    }

    /// create [nw_sys::Shortcut](nw_sys::Shortcut) and
    /// return it
    ///
    pub fn build(self) -> Result<nw_sys::Shortcut> {
        if let Some(callback) = self.active_callback {
            let app = match app() {
                Some(app) => app,
                None => return Err("app is not initialized".to_string().into()),
            };
            app.callbacks.retain(callback)?;
        }
        if let Some(callback) = self.failed_callback {
            let app = match app() {
                Some(app) => app,
                None => return Err("app is not initialized".to_string().into()),
            };
            app.callbacks.retain(callback)?;
        }

        let shortcut = nw_sys::Shortcut::new(&self.options);
        Ok(shortcut)
    }

    /// create [nw_sys::Shortcut](nw_sys::Shortcut) and
    /// return it with
    /// [active_callback](Self#structfield.active_callback),
    /// [failed_callback](Self#structfield.failed_callback) handlers
    ///
    pub fn finalize(self) -> Result<ShortcutInfo> {
        let shortcut = nw_sys::Shortcut::new(&self.options);
        Ok(ShortcutInfo {
            shortcut,
            active_callback: self.active_callback,
            failed_callback: self.failed_callback,
        })
    }
}
