use {
    crate::{Lobby, Table, TableId},
    yew::prelude::*,
    yew_router::prelude::*,
};

#[cfg(feature = "spa")]
use crate::Tables;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Routable)]
pub enum Route {
    #[at("/table/:id")]
    Table { id: TableId },

    #[at("/")]
    Index,
}

pub(crate) struct App {
    #[cfg(feature = "spa")]
    tables: Tables,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            #[cfg(feature = "spa")]
            tables: Default::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        #[cfg(not(feature = "spa"))]
        fn switch(route: Route) -> Html {
            match route {
                Route::Index => html! { <Lobby /> },
                Route::Table { id } => html! { <Table {id} /> },
            }
        }

        #[cfg(feature = "spa")]
        let switch = {
            let tables = self.tables.clone();
            let tables_too = tables.clone();

            move |route: Route| {
                let id = match route {
                    Route::Index => None,
                    Route::Table { id } => Some(id),
                };
                let tables = tables.clone();
                html! {
                    <>
                        <Lobby {tables} show={id.is_none()} />
                        {
                            tables_too.triple_html(|(left, this, right)| {
                                html! {
                                    <Table key={this} id={this} show={id == Some(this)} {left} {right} />
                                }
                            })
                        }
                    </>
                }
            }
        };

        html! {
            <BrowserRouter>
                <main class="container">
                    <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
        }
    }
}
