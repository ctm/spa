use serde::{Deserialize, Serialize};

#[cfg(not(feature = "spa"))]
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Deserialize, Serialize)]
pub struct Size {
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub top: u32,
    pub left: u32,
}

#[cfg(not(feature = "spa"))]
#[derive(Debug, Deserialize, Serialize)]
pub struct PopUpFeatures {
    pub url: String,
    pub target: String,
    pub size: Option<Size>,
    pub position: Option<Position>,
    #[cfg(all(feature = "tauri", not(feature = "spa")))]
    pub close_notification: Option<CloseNotification>,
}

#[cfg(feature = "spa")]
#[derive(Debug, Deserialize, Serialize)]
pub struct PopUpFeatures {
    pub path: String,
}

#[cfg(not(feature = "spa"))]
impl Display for PopUpFeatures {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let need_comma;
        if let Some(Size { width, height }) = &self.size {
            write!(f, "innerWidth={width},innerHeight={height}")?;
            need_comma = true;
        } else {
            need_comma = false;
        }
        if let Some(Position { top, left }) = &self.position {
            if need_comma {
                write!(f, ",")?;
            }
            write!(f, "top={top},left={left}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Close {
    pub label: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetTitle {
    pub label: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CloseNotification {
    pub receiver_label: String,
    pub id: u64, // Compromise
}

pub static CLOSED_EVENT: &str = "closed";
