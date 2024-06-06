import { Application, Text, Graphics, Container, TextStyle, ContainerChild } from "pixi.js";

type Time = number;
type BitString = string;
type Timeline = Array<[Time, BitString]>;

type X = number;
type TimelineForUI = Array<[X, string]>;

const MIN_BLOCK_WIDTH = 1;

export class PixiController {
    app: Application
    // -- FastWave-specific --
    var_signal_rows: Array<VarSignalRow> = [];
    var_signal_rows_container = new Container();
    row_height: number;
    row_gap: number;

    constructor(row_height: number, row_gap: number) {
        this.app = new Application();
        // -- FastWave-specific --
        this.row_height = row_height;
        this.row_gap = row_gap;
        this.app.stage.addChild(this.var_signal_rows_container);
    }

    async init(parent_element: HTMLElement) {
        await this.app.init({ background: 'DarkSlateBlue', antialias: true, resizeTo: parent_element });
        parent_element.appendChild(this.app.canvas);
    }

    // Default automatic Pixi resizing is not reliable
    queue_resize() {
        this.app.queueResize();
    }

    destroy() {
        const rendererDestroyOptions = {
            removeView: true
        }
        const options = {
            children: true,
            texture: true,
            textureSource: true,
            context: true,
        }
        this.app.destroy(rendererDestroyOptions, options);
    }

    // -- FastWave-specific --

    remove_var(index: number) {
        if (typeof this.var_signal_rows[index] !== 'undefined') {
            this.var_signal_rows[index].destroy();
        }
    }

    push_var(timeline: Timeline) {
        new VarSignalRow(
            timeline,
            this.app,
            this.var_signal_rows,
            this.var_signal_rows_container,
            this.row_height,
            this.row_gap,
        )
    }

    pop_var() {
        this.remove_var(this.var_signal_rows.length - 1);
    }

    clear_vars() {
        this.var_signal_rows.slice().reverse().forEach(row => row.destroy());
    }
}

class VarSignalRow {
    app: Application;
    timeline: Timeline;
    last_time: Time;
    formatter: (signal_value: BitString) => string;
    timeline_for_ui: TimelineForUI;
    owner: Array<VarSignalRow>;
    index_in_owner: number;
    rows_container: Container;
    row_height: number;
    row_gap: number;
    row_height_with_gap: number;
    renderer_resize_callback = () => this.redraw_on_canvas_resize();
    // -- elements --
    row_container = new Container();
    signal_blocks_container = new Container();
    label_style = new TextStyle({
        align: "center",
        fill: "White",
        fontSize: 16,
        fontFamily: 'system-ui, -apple-system, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji"',
    });    

    constructor(
        timeline: Timeline,
        app: Application,
        owner: Array<VarSignalRow>, 
        rows_container: Container,
        row_height: number,
        row_gap: number,
    ) {
        this.app = app;

        this.timeline = timeline;
        this.last_time = timeline[timeline.length - 1][0];
        this.formatter = signal_value => parseInt(signal_value, 2).toString(16);

        this.timeline_for_ui = this.timeline.map(([time, value]) => {
            const x = time / this.last_time * this.app.screen.width;
            return [x, this.formatter(value)]
        });

        this.row_height = row_height;
        this.row_gap = row_gap;
        this.row_height_with_gap = row_height + row_gap;

        this.index_in_owner = owner.length;
        this.owner = owner;
        this.owner.push(this);

        this.rows_container = rows_container;

        this.draw();
        this.app.renderer.on("resize", this.renderer_resize_callback);
    }

    async draw() {
        // row_container
        this.row_container.y = this.index_in_owner * this.row_height_with_gap;
        this.rows_container.addChild(this.row_container);

        // signal_block_container
        this.row_container.addChild(this.signal_blocks_container);

        for (let index = 0; index < this.timeline_for_ui.length; index++) {
            if (index == this.timeline_for_ui.length - 1) {
                return;
            }
            await new Promise(resolve => setTimeout(resolve, 0));
            const [x, value] = this.timeline_for_ui[index];

            // signal_block
            const block_width = this.timeline_for_ui[index+1][0] - x;
            const block_height = this.row_height;
            if (block_width < MIN_BLOCK_WIDTH) {
                return;
            }
            const signal_block = new Container();
            signal_block.x = x;
            this.signal_blocks_container.addChild(signal_block);

            // background
            const background = new Graphics()
                .roundRect(0, 0, block_width, block_height, 15)
                .fill("SlateBlue");
            background.label = "background";
            signal_block.addChild(background);

            // label
            const label = new Text({ text: value, style: this.label_style });
            label.x = (block_width - label.width) / 2;
            label.y = (block_height - label.height) / 2;
            label.visible = label.width < block_width;
            label.label = "label";
            signal_block.addChild(label);
        }
    }

    async redraw_on_canvas_resize() {
        for (let index = 0; index < this.timeline_for_ui.length; index++) {
            const x = this.timeline[index][0] / this.last_time * this.app.screen.width;
            this.timeline_for_ui[index][0] = x;
        }
        for (let index = 0; index < this.timeline_for_ui.length; index++) {
            if (index == this.timeline_for_ui.length - 1) {
                return;
            }
            await new Promise(resolve => setTimeout(resolve, 0));
            const [x, value] = this.timeline_for_ui[index];
            
            // signal_block
            const block_width = this.timeline_for_ui[index+1][0] - x;
            const block_height = this.row_height;
            const block_visible = block_width >= MIN_BLOCK_WIDTH;

            let signal_block: ContainerChild | undefined = this.signal_blocks_container.children[index];
            if (signal_block === undefined && !block_visible) {
                return;
            }
            if (signal_block !== undefined && !block_visible) {
                signal_block.visible = false;
                return;
            }
            if (signal_block === undefined && block_visible) {
                signal_block = new Container();
                signal_block.x = x;
                this.signal_blocks_container.addChild(signal_block);
            } else if (signal_block !== undefined && block_visible) {
                signal_block.visible = true;
                signal_block.x = x;
            }

            // background
            let background = signal_block.getChildByLabel("background");
            if (background === null) {
                background = new Graphics()
                    .roundRect(0, 0, block_width, block_height, 15)
                    .fill("SlateBlue");
                background.label = "background";
                signal_block.addChild(background);
            } else {
                background.width = block_width;
            }

            // label
            const label = signal_block.getChildByLabel("label");
            if (label === null ) {
                const label = new Text({ text: value, style: this.label_style });
                label.x = (block_width - label.width) / 2;
                label.y = (block_height - label.height) / 2;
                label.visible = label.width < block_width;
                label.label = "label";
                signal_block.addChild(label);
            } else {
                label.x = (block_width - label.width) / 2;
                label.y = (block_height - label.height) / 2;
                label.visible = label.width < block_width;
            }
        }
    }

    decrement_index() {
        this.index_in_owner--;
        this.row_container.y -= this.row_height_with_gap;
    }

    destroy() {
        this.app.renderer.off("resize", this.renderer_resize_callback);
        this.owner.splice(this.index_in_owner, 1);
        this.rows_container.removeChildAt(this.index_in_owner);
        this.row_container.destroy(true);
        this.owner.slice(this.index_in_owner).forEach(row => row.decrement_index());
    }
}
