use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_view);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<View<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("dev.ecmel.view", "ViewPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_view)?;
  Ok(View(handle))
}

/// Access to the view APIs.
pub struct View<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> View<R> {
  pub fn view(&self, payload: ViewRequest) -> crate::Result<ViewResponse> {
    self
      .0
      .run_mobile_plugin("view", payload)
      .map_err(Into::into)
  }
}
