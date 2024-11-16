package dev.ecmel.view

import android.app.Activity
import android.content.Intent
import androidx.core.content.FileProvider
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import java.io.File

@InvokeArg
class ViewArgs {
  var path: String? = null
}

@TauriPlugin
class ViewPlugin(private val activity: Activity) : Plugin(activity) {
  @Command
  fun view(invoke: Invoke) {
    val args = invoke.parseArgs(ViewArgs::class.java)
    val context = activity.applicationContext
    val path = args.path ?: ""

    val uri = FileProvider.getUriForFile(
      context,
      context.packageName + ".fileprovider",
      File(path)
    )

    context.grantUriPermission(
      context.packageName,
      uri,
      Intent.FLAG_GRANT_READ_URI_PERMISSION
    )

    val mime = context.contentResolver.getType(uri)

    try {
      val intent = Intent(Intent.ACTION_QUICK_VIEW)
        .setDataAndType(uri, mime)
        .addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
        .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
      context.startActivity(intent)
    } catch (e: Exception) {
      try {
        val intent = Intent(Intent.ACTION_VIEW)
          .setDataAndType(uri, mime)
          .addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
          .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        context.startActivity(intent)
      } catch (e: Exception) {
        val intent = Intent(Intent.ACTION_SEND)
          .setDataAndType(uri, mime)
          .addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
          .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        context.startActivity(intent)
      }
    }

    val ret = JSObject()
    ret.put("value", "OK")
    invoke.resolve(ret)
  }
}
