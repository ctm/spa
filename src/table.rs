use {crate::TableId, yew::prelude::*};

#[derive(Clone, Properties, PartialEq)]
pub(crate) struct Properties {
    pub(crate) id: TableId,
}

pub(crate) struct Table {
    state: String, // This is just a proof of concept
    #[cfg(not(feature = "tauri"))]
    _unload_listener: gloo_events::EventListener,
}

impl Component for Table {
    type Message = ();
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {
        use crate::util::{timezone_from_browser_or_mountain, utc_now};
        let now = utc_now().with_timezone(&timezone_from_browser_or_mountain());

        let id = ctx.props().id;
        crate::Window::<Self>::current().set_title(format!("Table {id}"));

        #[cfg(not(feature = "tauri"))]
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

            #[cfg(not(feature = "tauri"))]
            _unload_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            { &self.state }
        }
    }
}
