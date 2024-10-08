use zoon::*;

mod excalidraw_canvas;
use excalidraw_canvas::ExcalidrawCanvas;
pub use excalidraw_canvas::ExcalidrawController;

#[derive(Clone)]
pub struct DiagramPanel {
    canvas_controller: Mutable<Mutable<Option<SendWrapper<ExcalidrawController>>>>
}

impl DiagramPanel {
    pub fn new(
        canvas_controller: Mutable<Mutable<Option<SendWrapper<ExcalidrawController>>>>,
    ) -> impl Element {
        Self {
            canvas_controller
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
            .item(self.canvas())
    }

    fn canvas(&self) -> impl Element {
        let canvas_controller = self.canvas_controller.clone();
        ExcalidrawCanvas::new()
            .s(Align::new().top())
            .s(Width::fill())
            .s(Height::fill())
            .task_with_controller(move |controller| {
                canvas_controller.set(controller.clone());
                println!("hello from task_with_controller")
            })
    }
}
