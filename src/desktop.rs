use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<View<R>> {
  Ok(View(app.clone()))
}

/// Access to the view APIs.
pub struct View<R: Runtime>(AppHandle<R>);

impl<R: Runtime> View<R> {
  pub fn view(&self, payload: ViewRequest) -> crate::Result<ViewResponse> {
    Ok(ViewResponse {
      value: payload.path,
    })
  }
}
