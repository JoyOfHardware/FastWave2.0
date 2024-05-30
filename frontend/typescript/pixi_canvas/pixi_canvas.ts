import { Application, Text, Graphics, Container, TextStyle } from "pixi.js";

type Time = number;
type BitString = string;
type Timeline = Array<[Time, BitString | undefined]>;

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
    timeline: Timeline;
    app: Application;
    owner: Array<VarSignalRow>;
    index_in_owner: number;
    rows_container: Container;
    row_height: number;
    row_gap: number;
    row_height_with_gap: number;
    renderer_resize_callback = () => this.draw();
    // -- elements --
    row_container = new Container();
    signal_blocks_container = new Container();

    constructor(
        timeline: Timeline,
        app: Application,
        owner: Array<VarSignalRow>, 
        rows_container: Container,
        row_height: number,
        row_gap: number,
    ) {
        console.log("VarSignalRow timeline:", timeline);
        this.timeline = timeline;

        this.app = app;

        this.row_height = row_height;
        this.row_gap = row_gap;
        this.row_height_with_gap = row_height + row_gap;

        this.index_in_owner = owner.length;
        this.owner = owner;
        this.owner.push(this);

        this.rows_container = rows_container;
        this.create_element_tree();

        this.draw();
        this.app.renderer.on("resize", this.renderer_resize_callback);
    }

    create_element_tree() {
        // row_container
        this.row_container.y = this.index_in_owner * this.row_height_with_gap;
        this.rows_container.addChild(this.row_container);

        // signal_block_container
        this.row_container.addChild(this.signal_blocks_container);
    }

    draw() {
        if (this.timeline.length > 0) {
            const last_time = this.timeline[this.timeline.length - 1][0];
            // @TODO make formatter configurable
            const formatter: (signal_value: BitString) => string = signal_value => parseInt(signal_value, 2).toString(16); 
            // @TODO optimize - one pass, partly in Rust, partly outside of `draw()`, etc.
            const timeline: Array<[number, string | undefined]> = this.timeline.map(([time, value]) => {
                const x = time / last_time * this.app.screen.width;
                const formatted_value = typeof value === 'string' ? formatter(value) : undefined;
                return [x, formatted_value]
            });
            // @TODO optimize - don't recreate all on every draw
            this.signal_blocks_container.removeChildren();
            timeline.forEach(([x, value], index) => {
                if (typeof value === 'string') {
                    const block_width = timeline[index+1][0] - x;
                    const block_height = this.row_height;

                    // signal_block
                    const signal_block = new Container({x});
                    this.signal_blocks_container.addChild(signal_block);

                    // background
                    let background = new Graphics()
                        .roundRect(0, 0, block_width, block_height, 15)
                        .fill("SlateBlue");
                    signal_block.addChild(background);

                    // label
                    let style = new TextStyle({
                        align: "center",
                        fill: "White",
                        fontSize: 16,
                        fontFamily: 'system-ui, -apple-system, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji"',
                    });
                    // @TODO don't show when the label is wider/higher than the block
                    let label = new Text({ text: value, style });
                    label.x = (block_width - label.width) / 2;
                    label.y = (block_height - label.height) / 2;
                    signal_block.addChild(label);
                }
            })
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
