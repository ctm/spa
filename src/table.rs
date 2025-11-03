use {crate::TableId, yew::prelude::*};

#[cfg(feature = "spa")]
use crate::Route;

#[derive(Clone, Properties, PartialEq)]
pub(crate) struct Properties {
    pub(crate) id: TableId,
    #[cfg(feature = "spa")]
    pub(crate) show: bool,
    #[cfg(feature = "spa")]
    pub(crate) left: Option<Route>,
    #[cfg(feature = "spa")]
    pub(crate) right: Option<Route>,
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

pub(crate) struct Table {
    state: String, // This is just a proof of concept
    #[cfg(all(not(feature = "tauri"), not(feature = "spa")))]
    _unload_listener: gloo_events::EventListener,
}

#[cfg_attr(not(feature = "spa"), expect(unused_variables))]
fn nav_buttons(ctx: &Context<Table>) -> Option<Html> {
    #[cfg(not(feature = "spa"))]
    {
        None
    }

    #[cfg(feature = "spa")]
    {
        use {crate::Route, yew_router::prelude::*};
        fn button(label: &'static str, route: Option<Route>, n: &Navigator) -> Html {
            let disabled = route.is_none();
            let onclick: Option<Callback<MouseEvent>> = route.map(|r| {
                let n = n.clone();
                { move |_| n.replace(&r) }.into()
            });
            html! {
                <button {disabled} {onclick}> { label } </button>
            }
        }

        match ctx.link().navigator() {
            None => {
                log::error!("No navigator");
                None
            }
            Some(n) => {
                let left = button("⬅️", ctx.props().left, &n);
                let right = button("➡️️", ctx.props().right, &n);
                let goto_lobby: Callback<MouseEvent> = { move |_| n.replace(&Route::Index) }.into();
                Some(html! {
                    <div id="nav-overlay">
                        { left }
                        <button onclick={goto_lobby}> {"Lobby️"} </button>
                        { right }
                    </div>
                })
            }
        }
    }
}

impl Component for Table {
    type Message = ();
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {
        use crate::util::{timezone_from_browser_or_mountain, utc_now};
        let now = utc_now().with_timezone(&timezone_from_browser_or_mountain());

        let id = ctx.props().id;
        crate::Window::<Self>::current().set_title(format!("Table {id}"));

        #[cfg(all(not(feature = "tauri"), not(feature = "spa")))]
        let _unload_listener = {
            use crate::Window;

            let window = gloo_utils::window();
            gloo_events::EventListener::new(&window, "beforeunload", move |_| {
                if let Ok(parent) = Window::<crate::Lobby>::opener()
                    && let Err(e) = parent.send(&crate::lobby::Msg::CloseWindow(id))
                {
                    log::error!("Could not send CloseWindow({id}): {e}");
                }
            })
        };

        Table {
            state: format!("Created at {}", now.format("%H:%M:%S%.3f %Z")),

            #[cfg(all(not(feature = "tauri"), not(feature = "spa")))]
            _unload_listener,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = ctx.props().class();
        let nav_buttons = nav_buttons(ctx);
        html! {
            <div {class}>
                { &self.state }
                { nav_buttons }
            </div>
        }
    }
}
