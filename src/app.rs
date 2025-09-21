use {
    crate::{Lobby, Table, TableId},
    yew::prelude::*,
    yew_router::prelude::*,
};

#[derive(Clone, Debug, Eq, PartialEq, Routable)]
pub enum Route {
    #[at("/table/:id")]
    Table { id: TableId },

    #[at("/")]
    Index,
}

#[function_component(App)]
pub fn app() -> Html {
    fn switch(route: Route) -> Html {
        match route {
            Route::Index => html! { <Lobby /> },
            Route::Table { id } => html! { <Table {id} /> },
        }
    }
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
