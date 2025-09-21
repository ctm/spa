use {
    crate::human, derive_more::Display, tauri_command_types::PopUpFeatures, wasm_bindgen::JsValue,
};

#[derive(thiserror::Error, Debug, Display)]
pub(crate) enum Error {
    Failure(String),

    #[cfg(not(feature = "tauri"))]
    NoWindow,
}

impl From<JsValue> for Error {
    fn from(jv: JsValue) -> Self {
        Self::Failure(human(jv))
    }
}

#[cfg(not(feature = "tauri"))]
#[derive(thiserror::Error, Debug, Display)]
pub(crate) enum OpenerError {
    Failure(String),

    #[cfg(not(feature = "tauri"))]
    NoWindow,

    #[cfg(not(feature = "tauri"))]
    #[display("wrong type: {_0:?}")]
    WrongType(JsValue),

    #[cfg(feature = "tauri")]
    BroadcastChannelFailure(String),
}

#[cfg(not(feature = "tauri"))]
impl From<JsValue> for OpenerError {
    fn from(jv: JsValue) -> Self {
        Self::Failure(human(jv))
    }
}

#[cfg(not(feature = "tauri"))]
#[derive(thiserror::Error, Debug, Display)]
pub(crate) enum SendError {
    #[cfg(feature = "tauri")]
    NoChannel,

    #[cfg(feature = "tauri")]
    PostMessageFailed(String),

    CantSerialize(serde_wasm_bindgen::Error),

    #[cfg(not(feature = "tauri"))]
    CustomEventNewFailed(String),

    #[cfg(not(feature = "tauri"))]
    DispatchEventFailed(String),
}

#[cfg(not(feature = "tauri"))]
impl From<serde_wasm_bindgen::Error> for SendError {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Self::CantSerialize(e)
    }
}

#[cfg(feature = "tauri")]
pub(crate) use tauri::Window;

#[cfg(not(feature = "tauri"))]
pub(crate) use web_sys::Window;

#[cfg(feature = "tauri")]
mod tauri {
    use {
        super::{Error, PopUpFeatures, human},
        log::warn,
        serde::{Deserialize, Serialize},
        std::marker::PhantomData,
        web_sys::BroadcastChannel,
        yew::Component,
    };

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(crate) struct Window<C: Component> {
        label: String,
        message_type: PhantomData<C>,
        channel: Option<BroadcastChannel>,
    }

    impl<C: Component> Window<C>
    where
        C::Message: Serialize + for<'a> Deserialize<'a>,
    {
        // NOTE: currently this code will always return Ok, even if tauri
        // fails to open a new window.
        pub(crate) fn new(features: PopUpFeatures, need_channel: bool) -> Result<Self, Error> {
            let label = features.target.clone();
            let channel = if need_channel {
                match BroadcastChannel::new(&label) {
                    Err(e) => {
                        warn!("BroadcastChannel::new({label}) failed: {}", human(e));
                        None
                    }
                    Ok(channel) => Some(channel),
                }
            } else {
                None
            };
            yew::platform::spawn_local(async move {
                let _ = tauri_sys::core::invoke_result::<bool, String>("open_window", &features)
                    .await
                    // Ugh: although we log the error to the console, we
                    // don't send a message to the calling component
                    // saying that the open failed.
                    .inspect_err(|e| log::error!("{e:?}"));
            });
            Ok(Self {
                label,
                message_type: PhantomData,
                channel,
            })
        }

        fn current_label() -> String {
            gloo_utils::window()
                .location()
                .pathname()
                .unwrap_or_else(|e| {
                    log::error!("Can't get location's href: {e:?}");
                    "BROKEN".to_string()
                })
        }

        pub(crate) fn current() -> Self {
            Self {
                label: Self::current_label(),
                message_type: PhantomData,
                channel: None,
            }
        }

        pub(crate) fn close(&self) -> Result<(), Error> {
            let close = tauri_command_types::Close {
                label: self.label.clone(),
            };
            yew::platform::spawn_local(async move {
                if let Err(e) =
                    tauri_sys::core::invoke_result::<bool, String>("close_window", &close).await
                {
                    log::error!("Could not inoke close_window: {e}");
                }
            });
            Ok(())
        }

        pub(crate) fn set_title(&self, title: String) {
            let set_title = tauri_command_types::SetTitle {
                label: self.label.clone(),
                title,
            };
            yew::platform::spawn_local(async move {
                if let Err(e) =
                    tauri_sys::core::invoke_result::<bool, String>("set_title", &set_title).await
                {
                    log::error!("could not invoke set_title: {e}");
                }
            });
        }
    }
}

#[cfg(not(feature = "tauri"))]
mod web_sys {
    use {
        super::{Error, OpenerError, PopUpFeatures, SendError},
        gloo_events::EventListener,
        serde::{Deserialize, Serialize},
        std::marker::PhantomData,
        wasm_bindgen::JsCast,
        web_sys::{CustomEvent, CustomEventInit},
        yew::{Component, html::Scope},
    };

    const CUSTOM_EVENT_NAME: &str = "mb2-undocked-chat";

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(crate) struct Window<C: Component> {
        window: web_sys::Window,
        message_type: PhantomData<C>,
    }

    impl<C: Component> Window<C>
    where
        C::Message: Serialize + for<'a> Deserialize<'a>,
    {
        pub(crate) fn current() -> Self {
            Self {
                window: gloo_utils::window(),
                message_type: PhantomData,
            }
        }

        pub(crate) fn opener() -> Result<Self, OpenerError> {
            match gloo_utils::window().opener()? {
                v if v.is_object() => Ok(Self {
                    window: v.unchecked_into::<web_sys::Window>(),
                    message_type: PhantomData,
                }),
                v if v.is_null() => Err(OpenerError::NoWindow),
                v => Err(OpenerError::WrongType(v)),
            }
        }

        pub(crate) fn new(features: PopUpFeatures, _need_channel: bool) -> Result<Self, Error> {
            gloo_utils::window()
                .open_with_url_and_target_and_features(
                    &features.url,
                    &features.target,
                    &features.to_string(),
                )?
                .ok_or(Error::NoWindow)
                .map(|window| Self {
                    window,
                    message_type: PhantomData,
                })
        }

        pub(crate) fn set_title(&self, title: String) {
            if let Some(document) = self.window.document() {
                document.set_title(&title);
            }
        }

        pub(crate) fn close(&self) -> Result<(), Error> {
            self.window.close().map_err(Into::into)
        }

        pub(crate) fn send(&self, message: &C::Message) -> Result<(), SendError> {
            use super::{SendError::*, human};

            let request = serde_wasm_bindgen::to_value(message)?;
            let init_dict = CustomEventInit::new();
            init_dict.set_detail(&request);
            let event = CustomEvent::new_with_event_init_dict(CUSTOM_EVENT_NAME, &init_dict)
                .map_err(|e| CustomEventNewFailed(human(e)))?;
            self.window
                .dispatch_event(&event)
                .map_err(|e| DispatchEventFailed(human(e)))
                .map(|_| ())
        }

        pub(crate) fn listener(&self, link: Scope<C>) -> EventListener {
            EventListener::new(&self.window, CUSTOM_EVENT_NAME, move |e| {
                // dyn_ref fails here, but unchecked_ref succeeds
                let r = e.unchecked_ref::<CustomEvent>();
                match serde_wasm_bindgen::from_value::<C::Message>(r.detail()) {
                    Err(e) => log::warn!("Could not deserialize: {:?}", e),
                    Ok(r) => link.send_message(r),
                }
            })
        }
    }
}
