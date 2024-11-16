use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::View;
#[cfg(mobile)]
use mobile::View;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the view APIs.
pub trait ViewExt<R: Runtime> {
  fn view(&self) -> &View<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ViewExt<R> for T {
  fn view(&self) -> &View<R> {
    self.state::<View<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("view")
    .invoke_handler(tauri::generate_handler![commands::view])
    .setup(|app, api| {
      #[cfg(mobile)]
      let view = mobile::init(app, api)?;
      #[cfg(desktop)]
      let view = desktop::init(app, api)?;
      app.manage(view);
      Ok(())
    })
    .build()
}
