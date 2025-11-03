use {
    crate::{Route, TableId, TableInfo, TableWindow, Tables},
    serde::{Deserialize, Serialize},
    std::sync::atomic::{AtomicU8, Ordering},
    tauri_command_types::PopUpFeatures,
    yew::prelude::*,
    yew_router::prelude::*,
};

#[cfg(all(feature = "tauri", not(feature = "spa")))]
use tauri_command_types::CloseNotification;

#[derive(Clone, Properties, PartialEq)]
pub(crate) struct Properties {
    #[cfg(feature = "spa")]
    pub(crate) tables: Tables,

    #[cfg(feature = "spa")]
    pub(crate) show: bool,
}

impl Properties {
    fn class(&self) -> Option<&'static str> {
        #[cfg(not(feature = "spa"))]
        {
            None
        }

        #[cfg(feature = "spa")]
        {
            (!self.show).then_some("hide")
        }
    }
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub enum Msg {
    CreateWindow,
    CloseWindow(TableId),
}

pub(crate) struct Lobby {
    #[cfg(not(feature = "spa"))]
    tables: Tables,
    #[cfg(all(not(feature = "tauri"), not(feature = "spa")))]
    _child_listener: gloo_events::EventListener,
}

static TABLE_NUMBER: AtomicU8 = AtomicU8::new(0);

#[cfg(not(feature = "spa"))]
fn features(id: TableId) -> PopUpFeatures {
    use tauri_command_types::Size;

    let path = Route::Table { id }.to_path();

    #[cfg(all(feature = "tauri", not(feature = "spa")))]
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
        #[cfg(all(feature = "tauri", not(feature = "spa")))]
        close_notification,
    }
}

#[cfg(feature = "spa")]
fn features(id: TableId) -> PopUpFeatures {
    PopUpFeatures {
        path: Route::Table { id }.to_path(),
    }
}

fn table_id(id: TableId) -> Html {
    html! {
        <> { " Table " } { id } </>
    }
}

#[cfg(feature = "spa")]
fn linked_table_id(id: TableId) -> Html {
    html! {
        <Link<Route> to={Route::Table { id }}>
            { table_id(id) }
        </Link<Route>>
    }
}

impl Lobby {
    #[cfg(all(feature = "tauri", not(feature = "spa")))]
    fn new() -> Self {
        Self {
            #[cfg(not(feature = "spa"))]
            tables: Default::default(),
        }
    }

    #[cfg(all(not(feature = "tauri"), not(feature = "spa")))]
    fn new(_child_listener: gloo_events::EventListener) -> Self {
        Self {
            #[cfg(not(feature = "spa"))]
            tables: Default::default(),
            _child_listener,
        }
    }

    fn create_window(&mut self, ctx: &Context<Self>) -> bool {
        let link = ctx.link();
        let id = TABLE_NUMBER.fetch_add(1, Ordering::Relaxed);
        match TableWindow::new(features(id), false) {
            Ok(window) => {
                self.tables_mut(ctx).push(TableInfo::new(link, window, id));

                #[cfg(feature = "spa")]
                {
                    match ctx.link().navigator() {
                        None => log::error!("no navigator"),
                        Some(navigator) => navigator.replace(&Route::Table { id }),
                    }
                }
            }
            Err(e) => log::error!("new window failed: {e:?}"),
        }
        true
    }

    fn close_window(&mut self, id: TableId, ctx: &Context<Self>) -> bool {
        self.tables_mut(ctx).remove_by_id(id)
    }

    #[cfg(not(feature = "spa"))]
    fn tables(&self, _ctx: &Context<Self>) -> &Tables {
        &self.tables
    }

    #[cfg(feature = "spa")]
    fn tables<'a>(&self, ctx: &'a Context<Self>) -> &'a Tables {
        &ctx.props().tables
    }

    #[cfg(not(feature = "spa"))]
    fn tables_mut(&mut self, _ctx: &Context<Self>) -> &mut Tables {
        &mut self.tables
    }

    #[cfg(feature = "spa")]
    fn tables_mut<'a>(&self, ctx: &'a Context<Self>) -> &'a Tables {
        &ctx.props().tables
    }

    fn tables_view(&self, ctx: &Context<Self>) -> Html {
        #[cfg(feature = "spa")]
        use linked_table_id as table_id;

        html! {
            <ol class="tables-three-columns"> {
                self.tables(ctx).html(|TableInfo { id, close_callback, .. }| html! {
                    <li>
                        <span onclick={close_callback.clone()}>
                            { "🗑️" }
                        </span>
                        { table_id(*id) }
                    </li>
                })
            } </ol>
        }
    }
}

impl Component for Lobby {
    type Message = Msg;
    type Properties = Properties;

    #[cfg_attr(feature = "spa", expect(unused))]
    fn create(ctx: &Context<Self>) -> Self {
        #[cfg(feature = "spa")]
        {
            Self {}
        }

        #[cfg(not(feature = "spa"))]
        {
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
                let window = crate::Window::<Self>::current();
                Self::new(window.listener(link))
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            CreateWindow => self.create_window(ctx),
            CloseWindow(id) => self.close_window(id, ctx),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::CreateWindow);
        let class = ctx.props().class();
        html! {
            <div {class}>
                <button {onclick}>{"Create Window"}</button>
                { self.tables_view(ctx) }
            </div>
        }
    }
}
