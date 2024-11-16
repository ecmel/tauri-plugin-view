use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::ViewExt;

#[command]
pub(crate) async fn view<R: Runtime>(
  app: AppHandle<R>,
  payload: ViewRequest,
) -> Result<ViewResponse> {
  app.view().view(payload)
}
