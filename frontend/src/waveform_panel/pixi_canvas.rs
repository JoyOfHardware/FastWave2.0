use crate::platform;
pub use js_bridge::PixiController;
use std::rc::Rc;
use zoon::*;

pub struct PixiCanvas {
    raw_el: RawHtmlEl<web_sys::HtmlElement>,
    controller: Mutable<Option<SendWrapper<js_bridge::PixiController>>>,
    #[allow(dead_code)]
    width: ReadOnlyMutable<u32>,
    #[allow(dead_code)]
    height: ReadOnlyMutable<u32>,
    task_with_controller: Mutable<Option<TaskHandle>>,
}

impl Element for PixiCanvas {}

impl RawElWrapper for PixiCanvas {
    type RawEl = RawHtmlEl<web_sys::HtmlElement>;
    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

impl Styleable<'_> for PixiCanvas {}
impl KeyboardEventAware for PixiCanvas {}
impl MouseEventAware for PixiCanvas {}
impl PointerEventAware for PixiCanvas {}
impl TouchEventAware for PixiCanvas {}
impl AddNearbyElement<'_> for PixiCanvas {}
impl HasIds for PixiCanvas {}

impl PixiCanvas {
    pub fn new(row_height: u32, row_gap: u32) -> Self {
        let controller: Mutable<Option<SendWrapper<js_bridge::PixiController>>> =
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
        // -- FastWave-specific --
        let timeline_getter = Rc::new(Closure::new(
            |signal_ref_index,
             timeline_zoom,
             timeline_viewport_width,
             timeline_viewport_x,
             row_height,
             var_format| {
                future_to_promise(async move {
                    let signal_ref = wellen::SignalRef::from_index(signal_ref_index).unwrap_throw();
                    let timeline = platform::load_signal_and_get_timeline(
                        signal_ref,
                        timeline_zoom,
                        timeline_viewport_width,
                        timeline_viewport_x,
                        row_height,
                        serde_wasm_bindgen::from_value(var_format).unwrap_throw(),
                    )
                    .await;
                    let timeline = serde_wasm_bindgen::to_value(&timeline).unwrap_throw();
                    Ok(timeline)
                })
            },
        ));
        // -- // --
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
                        let pixi_controller = SendWrapper::new(js_bridge::PixiController::new(
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
        f: impl FnOnce(Mutable<Option<SendWrapper<js_bridge::PixiController>>>) -> FUT,
    ) -> Self {
        self.task_with_controller
            .set(Some(Task::start_droppable(f(self.controller.clone()))));
        self
    }
}

mod js_bridge {
    use zoon::*;

    type TimelinePromise = js_sys::Promise;
    type SignalRefIndex = usize;
    type TimelineZoom = f64;
    type TimelineViewportWidth = u32;
    type TimelineViewportX = i32;
    type RowHeight = u32;
    type VarFormatJs = JsValue;
    type TimelineGetter = Closure<
        dyn FnMut(
            SignalRefIndex,
            TimelineZoom,
            TimelineViewportWidth,
            TimelineViewportX,
            RowHeight,
            VarFormatJs,
        ) -> TimelinePromise,
    >;

    // Note: Add all corresponding methods to `frontend/typescript/pixi_canvas/pixi_canvas.ts`
    #[wasm_bindgen(module = "/typescript/bundles/pixi_canvas.js")]
    extern "C" {
        #[derive(Clone)]
        pub type PixiController;

        // @TODO `row_height` and `row_gap` is FastWave-specific
        #[wasm_bindgen(constructor)]
        pub fn new(
            timeline_zoom: f64,
            timeline_viewport_width: u32,
            timeline_viewport_x: i32,
            row_height: u32,
            row_gap: u32,
            timeline_getter: &TimelineGetter,
        ) -> PixiController;

        #[wasm_bindgen(method)]
        pub async fn init(this: &PixiController, parent_element: &JsValue);

        #[wasm_bindgen(method)]
        pub async fn resize(this: &PixiController, width: u32, height: u32);

        #[wasm_bindgen(method)]
        pub fn destroy(this: &PixiController);

        // -- FastWave-specific --

        #[wasm_bindgen(method)]
        pub fn get_timeline_zoom(this: &PixiController) -> f64;

        #[wasm_bindgen(method)]
        pub fn get_timeline_viewport_width(this: &PixiController) -> u32;

        #[wasm_bindgen(method)]
        pub fn get_timeline_viewport_x(this: &PixiController) -> i32;

        #[wasm_bindgen(method)]
        pub fn set_var_format(this: &PixiController, index: usize, var_format: JsValue);

        #[wasm_bindgen(method)]
        pub fn zoom_or_pan(
            this: &PixiController,
            wheel_delta_y: f64,
            shift_key: bool,
            offset_x: u32,
        );

        #[wasm_bindgen(method)]
        pub fn remove_var(this: &PixiController, index: usize);

        #[wasm_bindgen(method)]
        pub fn push_var(
            this: &PixiController,
            signal_ref_index: usize,
            timeline: JsValue,
            var_format: JsValue,
        );

        #[wasm_bindgen(method)]
        pub fn pop_var(this: &PixiController);

        #[wasm_bindgen(method)]
        pub fn clear_vars(this: &PixiController);

        #[wasm_bindgen(method)]
        pub async fn redraw_all_rows(this: &PixiController);
    }
}
