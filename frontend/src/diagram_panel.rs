use zoon::*;

#[derive(Clone)]
pub struct DiagramPanel {
}

impl DiagramPanel {
    pub fn new(
    ) -> impl Element {
        Self {
        }
        .root()
    }

    fn root(&self) -> impl Element {
        Column::new()
            .s(Padding::all(20))
            .s(Scrollbars::y_and_clip_x())
            .s(Width::fill())
            .s(Height::fill())
            .s(Gap::new().y(20))
            .item("Diagram panel")
    }
}
