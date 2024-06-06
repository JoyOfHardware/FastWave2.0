import { Application, Text, Graphics, Container, TextStyle, ContainerChild } from "pixi.js";

// @TODO sync with Rust and `tauri_glue.ts`
type Timeline = {
    blocks: Array<TimelineBlock>
}
type TimelineBlock = {
    x: number,
    width: number,
    height: number,
    label: TimeLineBlockLabel | undefined,
}
type TimeLineBlockLabel = {
    text: string,
    x: number,
    y: number,
}

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

    screen_width() {
        return this.app.screen.width;
    }

    // -- FastWave-specific --

    remove_var(index: number) {
        if (typeof this.var_signal_rows[index] !== 'undefined') {
            this.var_signal_rows[index].destroy();
        }
    }

    push_var(timeline: Timeline) {
        console.log("Timline in Typescript:", timeline);
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
    owner: Array<VarSignalRow>;
    index_in_owner: number;
    rows_container: Container;
    row_height: number;
    row_gap: number;
    row_height_with_gap: number;
    renderer_resize_callback = () => this.draw();
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

        this.row_height = row_height;
        this.row_gap = row_gap;
        this.row_height_with_gap = row_height + row_gap;

        this.index_in_owner = owner.length;
        this.owner = owner;
        this.owner.push(this);

        this.rows_container = rows_container;

        // row_container
        this.row_container.y = this.index_in_owner * this.row_height_with_gap;
        this.rows_container.addChild(this.row_container);

        // signal_blocks_container
        this.row_container.addChild(this.signal_blocks_container);

        this.draw();
        // this.app.renderer.on("resize", (width, height) => { 
        //     // @TODO only on `width` change
        //     // @TODO inline `renderer_resize_callback`?
        //     this.draw();
        // });
    }

    draw() {
        this.signal_blocks_container.removeChildren();
        this.timeline.blocks.forEach(timeline_block => {
            // signal_block
            const signal_block = new Container();
            signal_block.x = timeline_block.x;
            this.signal_blocks_container.addChild(signal_block);

            // background
            const background = new Graphics()
                .roundRect(0, 0, timeline_block.width, timeline_block.height, 15)
                .fill("SlateBlue");
            signal_block.addChild(background);

            // label
            if (timeline_block.label !== undefined) {
                const label = new Text({ text: timeline_block.label.text, style: this.label_style });
                label.x = timeline_block.label.x;
                label.y = timeline_block.label.y;
                signal_block.addChild(label);
            }
        });
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
