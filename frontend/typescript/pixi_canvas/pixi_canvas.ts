import { Application, Text, Graphics, Container, TextStyle, Sprite, Texture } from "pixi.js";

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

type TimelineGetter = (signal_ref_index: number, screen_width: number, row_height: number) => Promise<Timeline>;

export class PixiController {
    app: Application
    // -- FastWave-specific --
    var_signal_rows: Array<VarSignalRow> = [];
    var_signal_rows_container = new Container();
    row_height: number;
    row_gap: number;
    previous_parent_width: number | null;
    timeline_getter: TimelineGetter

    constructor(row_height: number, row_gap: number, timeline_getter: TimelineGetter) {
        this.app = new Application();
        // -- FastWave-specific --
        this.row_height = row_height;
        this.row_gap = row_gap;
        this.app.stage.addChild(this.var_signal_rows_container);
        this.previous_parent_width = null;
        this.timeline_getter = timeline_getter;
    }

    async init(parent_element: HTMLElement) {
        await this.app.init({ background: 'DarkSlateBlue', antialias: true, resizeTo: parent_element });
        parent_element.appendChild(this.app.canvas);
    }

    // Default automatic Pixi resizing according to the parent is not reliable 
    // and the `app.renderer`'s `resize` event is fired on every browser window size change 
    async resize(width: number, height: number) {
        this.app.resize();
        // -- FastWave-specific --
        const width_changed = width !== this.previous_parent_width;
        this.previous_parent_width = width;
        if (width_changed) {
            await this.redraw_rows();
        }
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

    async redraw_rows() {
        await Promise.all(this.var_signal_rows.map(async row => { 
            const timeline = await this.timeline_getter(row.signal_ref_index, this.app.screen.width, this.row_height);
            row.redraw(timeline);
        }))
    }

    remove_var(index: number) {
        if (typeof this.var_signal_rows[index] !== 'undefined') {
            this.var_signal_rows[index].destroy();
        }
    }

    push_var(signal_ref_index: number, timeline: Timeline) {
        new VarSignalRow(
            signal_ref_index,
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
    signal_ref_index: number;
    timeline: Timeline;
    app: Application;
    owner: Array<VarSignalRow>;
    index_in_owner: number;
    rows_container: Container;
    row_height: number;
    row_gap: number;
    row_height_with_gap: number;
    row_container = new Container();
    row_container_background: Sprite;
    signal_blocks_container = new Container();
    label_style = new TextStyle({
        align: "center",
        fill: "White",
        fontSize: 16,
        fontFamily: 'system-ui, -apple-system, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji"',
    });    

    constructor(
        signal_ref_index: number,
        timeline: Timeline,
        app: Application,
        owner: Array<VarSignalRow>, 
        rows_container: Container,
        row_height: number,
        row_gap: number,
    ) {
        this.signal_ref_index = signal_ref_index;
        this.timeline = timeline;
        this.app = app;

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

        // row background
        this.row_container_background = new Sprite();
        this.row_container_background.texture = Texture.WHITE;
        this.row_container_background.tint = '0x550099';
        this.row_container_background.height = this.row_height;
        this.row_container.addChild(this.row_container_background);

        // signal_blocks_container
        this.row_container.addChild(this.signal_blocks_container);

        this.draw();
    }

    redraw(timeline: Timeline) {
        this.timeline = timeline;
        this.draw();
    }

    draw() {
        // Screen can be null when we are, for instance, switching between miller column and tree layout
        // and then the canvas has to be recreated
        if (this.app.screen === null) {
            return;
        }

        this.row_container_background.width = this.app.screen.width;

        this.signal_blocks_container.removeChildren();
        this.timeline.blocks.forEach(timeline_block => {
            // signal_block
            const signal_block = new Container();
            signal_block.x = timeline_block.x;
            this.signal_blocks_container.addChild(signal_block);

            // background
            const gap_between_blocks = 2;
            const background = new Graphics()
                .rect(gap_between_blocks / 2, 0, timeline_block.width - gap_between_blocks, timeline_block.height)
                .fill('SlateBlue');
            signal_block.addChild(background);

            // label
            if (timeline_block.label !== undefined) {
                const label = new Text();
                label.text = timeline_block.label.text;
                label.style = this.label_style;
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
        this.owner.splice(this.index_in_owner, 1);
        this.rows_container.removeChildAt(this.index_in_owner);
        this.row_container.destroy(true);
        this.owner.slice(this.index_in_owner).forEach(row => row.decrement_index());
    }
}
