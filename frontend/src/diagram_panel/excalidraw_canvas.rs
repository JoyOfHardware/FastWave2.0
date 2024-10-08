use crate::platform;
pub use js_bridge::ExcalidrawController;
use std::rc::Rc;
use zoon::*;

pub struct ExcalidrawCanvas {
    raw_el: RawHtmlEl<web_sys::HtmlElement>,
    controller: Mutable<Option<SendWrapper<js_bridge::ExcalidrawController>>>,
    #[allow(dead_code)]
    width: ReadOnlyMutable<u32>,
    #[allow(dead_code)]
    height: ReadOnlyMutable<u32>,
    task_with_controller: Mutable<Option<TaskHandle>>,
}

impl Element for ExcalidrawCanvas {}

impl RawElWrapper for ExcalidrawCanvas {
    type RawEl = RawHtmlEl<web_sys::HtmlElement>;
    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

impl Styleable<'_> for ExcalidrawCanvas {}
impl KeyboardEventAware for ExcalidrawCanvas {}
impl MouseEventAware for ExcalidrawCanvas {}
impl PointerEventAware for ExcalidrawCanvas {}
impl TouchEventAware for ExcalidrawCanvas {}
impl AddNearbyElement<'_> for ExcalidrawCanvas {}
impl HasIds for ExcalidrawCanvas {}

impl ExcalidrawCanvas {
    pub fn new() -> Self {
        let controller: Mutable<Option<SendWrapper<js_bridge::ExcalidrawController>>> =
            Mutable::new(None);
        let width = Mutable::new(0);
        let height = Mutable::new(0);
        let resize_task = Task::start_droppable(
            map_ref! {
                let width = width.signal(),
                let height = height.signal() => (*width, *height)
            }
            .dedupe()
            .throttle(|| Timer::sleep(50))
            .for_each(
                clone!((controller) move |(width, height)| clone!((controller) async move {
                    if let Some(controller) = controller.lock_ref().as_ref() {
                        controller.resize(width, height).await
                    }
                })),
            ),
        );
        let task_with_controller = Mutable::new(None);
        Self {
            controller: controller.clone(),
            width: width.read_only(),
            height: height.read_only(),
            task_with_controller: task_with_controller.clone(),
            raw_el: El::new()
                .s(Clip::both())
                .on_viewport_size_change(clone!((width, height) move |new_width, new_height| {
                    width.set_neq(new_width);
                    height.set_neq(new_height);
                }))
                .update_raw_el(|raw_el| {
                    // @TODO rewrite to a native Zoon API
                    raw_el.event_handler_with_options(
                        EventOptions::new().preventable(),
                        clone!((controller) move |event: events_extra::WheelEvent| {
                            event.prevent_default();
                            if let Some(controller) = controller.lock_ref().as_ref() {
                                controller.zoom_or_pan(
                                    event.delta_y(),
                                    event.shift_key(),
                                    event.offset_x() as u32,
                                );
                            }
                        }),
                    )
                })
                .after_insert(clone!((controller, timeline_getter) move |element| {
                    Task::start(async move {
                        let pixi_controller = SendWrapper::new(js_bridge::ExcalidrawController::new(
                            1.,
                            width.get(),
                            0,
                            row_height,
                            row_gap,
                            &timeline_getter
                        ));
                        pixi_controller.init(&element).await;
                        controller.set(Some(pixi_controller));
                    });
                }))
                .after_remove(move |_| {
                    drop(timeline_getter);
                    drop(resize_task);
                    drop(task_with_controller);
                    if let Some(controller) = controller.take() {
                        controller.destroy();
                    }
                })
                .into_raw_el(),
        }
    }

    pub fn task_with_controller<FUT: Future<Output = ()> + 'static>(
        self,
        f: impl FnOnce(Mutable<Option<SendWrapper<js_bridge::ExcalidrawController>>>) -> FUT,
    ) -> Self {
        self.task_with_controller
            .set(Some(Task::start_droppable(f(self.controller.clone()))));
        self
    }
}

mod js_bridge {
    use zoon::*;

    // Note: Add all corresponding methods to `frontend/typescript/excalidraw_canvas/excalidraw_canvas.ts`
    #[wasm_bindgen(module = "/typescript/bundles/excalidraw_canvas.js")]
    extern "C" {
        #[derive(Clone)]
        pub type ExcalidrawController;

        #[wasm_bindgen(method)]
        pub async fn init(this: &ExcalidrawController, parent_element: &JsValue);

        #[wasm_bindgen(method)]
        pub async fn resize(this: &ExcalidrawController, width: u32, height: u32);

        #[wasm_bindgen(method)]
        pub fn destroy(this: &ExcalidrawController);

        // -- FastWave-specific --

        #[wasm_bindgen(method)]
        pub fn get_timeline_zoom(this: &ExcalidrawController) -> f64;
    }
}
