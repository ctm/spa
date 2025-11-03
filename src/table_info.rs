use {
    crate::{Lobby, TableId, Window, lobby::Msg},
    yew::{html::Scope, prelude::*},
};

#[cfg(feature = "spa")]
use crate::Route;

pub(crate) type TableWindow = Window<Lobby>;

pub(crate) struct TableInfo {
    pub(crate) id: TableId,
    pub(crate) window: TableWindow,
    pub(crate) close_callback: Callback<MouseEvent>,
}

impl TableInfo {
    pub(crate) fn new(link: &Scope<Lobby>, window: TableWindow, id: TableId) -> Self {
        let link = link.clone();
        let close_callback = Callback::from(move |_| link.send_message(Msg::CloseWindow(id)));
        Self {
            id,
            window,
            close_callback,
        }
    }

    #[cfg(feature = "spa")]
    pub(crate) fn left_this_right(
        tables: &[TableInfo],
    ) -> impl Iterator<Item = (Option<Route>, TableId, Option<Route>)> {
        tables.iter().enumerate().map(|(i, info)| {
            let left = (i > 0).then(|| Route::Table {
                id: tables[i - 1].id,
            });
            let right = tables.get(i + 1).map(|info| Route::Table { id: info.id });
            (left, info.id, right)
        })
    }
}
#[cfg(feature = "spa")]
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

#[cfg(feature = "spa")]
#[derive(Clone, Default)]
pub struct Tables(Rc<RefCell<Vec<TableInfo>>>);

#[cfg(feature = "spa")]
impl PartialEq for Tables {
    fn eq(&self, other: &Tables) -> bool {
        let s = self.0.borrow();
        let o = other.0.borrow();
        s.len() == o.len() && s.iter().zip(o.iter()).all(|(t0, t1)| t0.id == t1.id)
    }
}

#[cfg(not(feature = "spa"))]
#[derive(Default)]
pub struct Tables(Vec<TableInfo>);

// This is how we make remove_by_id use &self for spa and &mut self for non-spa
macro_rules! remove_by_id {
    ($self:ident, $id:ident) => {{
        #[allow(unused_mut)]
        let mut tables = $self.tables_mut();

        match tables.iter().position(|e| e.id == $id) {
            Some(i) => {
                // ignores the possibility of failure.
                let _ = tables.remove(i).window.close();
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
    }};
}

impl Tables {
    #[cfg(feature = "spa")]
    pub(crate) fn remove_by_id(&self, id: TableId) -> bool {
        remove_by_id!(self, id)
    }

    #[cfg(not(feature = "spa"))]
    pub(crate) fn remove_by_id(&mut self, id: TableId) -> bool {
        remove_by_id!(self, id)
    }

    #[cfg(feature = "spa")]
    fn tables_mut(&self) -> RefMut<'_, Vec<TableInfo>> {
        self.0.borrow_mut()
    }

    #[cfg(not(feature = "spa"))]
    fn tables_mut(&mut self) -> &mut Vec<TableInfo> {
        &mut self.0
    }

    #[cfg(feature = "spa")]
    fn tables(&self) -> Ref<'_, Vec<TableInfo>> {
        self.0.borrow()
    }

    #[cfg(not(feature = "spa"))]
    fn tables(&self) -> &[TableInfo] {
        &self.0
    }

    #[cfg(feature = "spa")]
    pub(crate) fn push(&self, elem: TableInfo) {
        self.tables_mut().push(elem)
    }

    #[cfg(not(feature = "spa"))]
    pub(crate) fn push(&mut self, elem: TableInfo) {
        self.tables_mut().push(elem)
    }

    pub(crate) fn html(&self, f: impl Fn(&TableInfo) -> Html) -> Html {
        html! {
            for self.tables().iter().map(f)
        }
    }

    #[cfg(feature = "spa")]
    pub(crate) fn triple_html(
        &self,
        f: impl Fn((Option<Route>, TableId, Option<Route>)) -> Html,
    ) -> Html {
        html! {
            for TableInfo::left_this_right(&self.0.borrow()).map(f)
        }
    }
}
