use std::{cell::Cell, fmt::Display, rc::Rc};

pub type ExtraPlaceId = u8;

pub const JIQUIA_ID: ExtraPlaceId = 1;
pub const JARDIM_BOTANICO_ID: ExtraPlaceId = 2;

#[derive(Clone)]
pub struct ExtraPlaceHolder {
    id_cell: Rc<Cell<ExtraPlaceId>>,
}

impl ExtraPlaceHolder {
    pub fn set(&self, id: ExtraPlaceId) {
        self.id_cell.set(id);
    }

    pub fn get(&self) -> ExtraPlaceId {
        self.id_cell.get()
    }

    pub fn is(&self, id: ExtraPlaceId) -> bool {
        self.id_cell.get() == id
    }

    pub fn points_to(&mut self, other: &ExtraPlaceHolder) {
        self.id_cell = Rc::clone(&other.id_cell);
    }
}

impl Default for ExtraPlaceHolder {
    fn default() -> Self {
        ExtraPlaceHolder {
            id_cell: Rc::new(Cell::new(JIQUIA_ID)),
        }
    }
}

impl Display for ExtraPlaceHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let place_id = self.id_cell.get();

        match place_id {
            JIQUIA_ID => write!(f, "Jiquia")?,
            JARDIM_BOTANICO_ID => write!(f, "Jardim Botânico")?,
            _ => write!(f, "Unknown Place With Id #{}", place_id)?,
        };

        Ok(())
    }
}
