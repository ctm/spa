use {
    tauri::{Emitter, EventTarget, Manager, WebviewWindow, WindowEvent},
    tauri_command_types::{CLOSED_EVENT, CloseNotification, Position, Size},
};

#[tauri::command(rename_all = "snake_case")]
async fn open_window(
    app_handle: tauri::AppHandle,
    url: String,
    target: String,
    size: Option<Size>,
    position: Option<Position>,
    close_notification: Option<CloseNotification>,
) -> Result<bool, String> {
    use tauri::WebviewUrl::*;

    let url = if cfg!(debug_assertions) {
        External(url.parse().map_err(|e| format!("Invalid url {url}: {e}"))?)
    } else {
        // The old code looked explicitly for the prefix
        // tauri://localhost/ but under Windows, we get
        // http://tauri.localhost/ so now we simply look for the third
        // slash.
        let url = url
            .match_indices('/')
            .nth(2)
            .map(|(n, _)| &url[(n + 1)..])
            .ok_or_else(|| format!("Expected three slashes url: {url}"))?;

        App(std::path::PathBuf::from(url))
    };
    let mut builder = tauri::WebviewWindowBuilder::new(&app_handle, target, url);

    #[cfg(desktop)]
    {
        builder = builder.resizable(true);
        if let Some(Size { width, height }) = size {
            builder = builder.inner_size(width.into(), height.into());
        }
        if let Some(Position { top, left }) = position {
            builder = builder.position(left.into(), top.into());
        }
    }
    let window = builder
        .build()
        .map_err(|e| format!("Could not build window: {e}"))?;

    if let Some(CloseNotification { receiver_label, id }) = close_notification {
        let target = EventTarget::Webview {
            label: receiver_label,
        };

        window.on_window_event(move |e| {
            if matches!(e, &WindowEvent::Destroyed)
                && let Err(err) = app_handle.emit_to(target.clone(), CLOSED_EVENT, id)
            {
                log::error!("Could not send close: {err:?}");
            }
        });
    }

    #[cfg(desktop)]
    let _ = window.set_focus(); // Perhaps we want to log failure

    Ok(true)
}

fn window_apply(
    app_handle: tauri::AppHandle,
    label: &str,
    f: impl FnOnce(WebviewWindow) -> Result<bool, String>,
) -> Result<bool, String> {
    app_handle
        .get_webview_window(label)
        .ok_or_else(|| format!("Could not get window {label}"))
        .and_then(f)
}

#[tauri::command]
async fn set_title(
    app_handle: tauri::AppHandle,
    label: String,
    title: String,
) -> Result<bool, String> {
    window_apply(app_handle, &label, |w| {
        #[cfg(desktop)]
        {
            w.set_title(&title)
                .map_err(|e| format!("Could not set window {label} title to {title}: {e:?}"))
                .map(|_| true)
        }
        #[cfg(mobile)]
        {
            Ok(false)
        }
    })
}

#[tauri::command]
async fn close_window(app_handle: tauri::AppHandle, label: String) -> Result<bool, String> {
    window_apply(app_handle, &label, |w| {
        #[cfg(desktop)]
        {
            w.close()
                .map_err(|e| format!("Could not close window {label}: {e:?}"))
                .map(|_| true)
        }
        #[cfg(mobile)]
        {
            Ok(false)
        }
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            open_window,
            set_title,
            close_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
