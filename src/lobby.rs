use {
    crate::{Route, TableId, Window},
    serde::{Deserialize, Serialize},
    std::sync::atomic::{AtomicU8, Ordering},
    tauri_command_types::{CloseNotification, PopUpFeatures, Size},
    yew::{html::Scope, prelude::*},
    yew_router::prelude::*,
};

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub enum Msg {
    CreateWindow,
    CloseWindow(TableId),
}

type TableWindow = Window<Lobby>;

struct TableInfo {
    id: TableId,
    window: TableWindow,
    close_callback: Callback<MouseEvent>,
}

impl TableInfo {
    fn new(link: &Scope<Lobby>, window: TableWindow, id: TableId) -> Self {
        let link = link.clone();
        let close_callback = Callback::from(move |_| link.send_message(Msg::CloseWindow(id)));
        Self {
            id,
            window,
            close_callback,
        }
    }
}

pub struct Lobby {
    tables: Vec<TableInfo>,
    #[cfg(not(feature = "tauri"))]
    _child_listener: gloo_events::EventListener,
}

static TABLE_NUMBER: AtomicU8 = AtomicU8::new(0);

fn features(id: TableId) -> PopUpFeatures {
    let path = Route::Table { id }.to_path();
    let close_notification = Some(CloseNotification {
        receiver_label: "main".to_string(),
        id: id.into(),
    });
    let location = gloo_utils::window().location();
    let host = location.host().expect("Could not get host");
    let protocol = location.protocol().expect("Could not get protocol");
    let url = format!("{protocol}//{host}{path}");
    PopUpFeatures {
        url,
        target: path,
        size: Some(Size {
            height: 200,
            width: 300,
        }),
        position: None,
        close_notification,
    }
}

impl Lobby {
    #[cfg(feature = "tauri")]
    fn new() -> Self {
        Self {
            tables: Default::default(),
        }
    }

    #[cfg(not(feature = "tauri"))]
    fn new(_child_listener: gloo_events::EventListener) -> Self {
        Self {
            tables: Default::default(),
            _child_listener,
        }
    }

    fn create_window(&mut self, link: &Scope<Self>) -> bool {
        let id = TABLE_NUMBER.fetch_add(1, Ordering::Relaxed);
        match TableWindow::new(features(id), false) {
            Ok(window) => {
                self.tables.push(TableInfo::new(link, window, id));
            }
            Err(e) => log::error!("new window failed: {e:?}"),
        }
        true
    }

    fn close_window(&mut self, id: TableId) -> bool {
        match self.tables.iter().position(|e| e.id == id) {
            Some(i) => {
                // ignores the possibility of failure.
                let _ = self.tables.remove(i).window.close();
                true
            }
            None => {
                // If we've closed a window via the trash-basket, then
                // we'll also get a destroyed message after we've done
                // the removal, which means we won't find the table
                // and we'll get here.
                false
            }
        }
    }

    fn tables(&self) -> Html {
        html! {
            <ol class="tables-three-columns"> {
                for self.tables.iter().map(|TableInfo { id, close_callback, .. }| html! {
                    <li> <span onclick={close_callback.clone()}> { "🗑️" } </span> { " Table " } { id } </li>
                })
            } </ol>
        }
    }
}

impl Component for Lobby {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();

        #[cfg(feature = "tauri")]
        {
            use {
                futures::StreamExt,
                tauri_command_types::CLOSED_EVENT,
                tauri_sys::event::{EventTarget, listen_to},
            };

            yew::platform::spawn_local(async move {
                match listen_to::<u64>(CLOSED_EVENT, EventTarget::Any).await {
                    Err(e) => log::error!("Can't listen_to(CLOSED_EVENT, ...): {e:?}"),
                    Ok(mut events) => {
                        while let Some(event) = events.next().await {
                            link.send_message(Msg::CloseWindow(event.payload as u8));
                        }
                    }
                }
            });
            Self::new()
        }

        #[cfg(not(feature = "tauri"))]
        {
            let window = Window::<Self>::current();
            Self::new(window.listener(link))
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            CreateWindow => self.create_window(ctx.link()),
            CloseWindow(id) => self.close_window(id),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::CreateWindow);
        html! {
            <main class="container">
                <button {onclick}>{"Create Window"}</button>
                { self.tables() }
            </main>
        }
    }
}
