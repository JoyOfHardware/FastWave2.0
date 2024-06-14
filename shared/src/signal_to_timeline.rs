use crate::*;

pub fn signal_to_timeline(
    signal: &wellen::Signal,
    time_table: &[wellen::Time],
    timeline_zoom: f64,
    timeline_viewport_width: u32,
    timeline_viewport_x: i32,
    block_height: u32,
    var_format: VarFormat,
) -> Timeline {
    const MIN_BLOCK_WIDTH: u32 = 3;
    // Courier New, 16px, sync with `label_style` in `pixi_canvas.rs`
    const LETTER_WIDTH: f64 = 9.61;
    const LETTER_HEIGHT: u32 = 18;
    const LABEL_X_PADDING: u32 = 10;

    let Some(last_time) = time_table.last().copied() else {
        return Timeline::default();
    };

    let last_time = last_time as f64;
    let timeline_viewport_x = timeline_viewport_x as f64;
    let timeline_width = timeline_viewport_width as f64 * timeline_zoom;

    let mut x_value_pairs = signal
        .iter_changes()
        .map(|(index, value)| {
            let index = index as usize;
            let time = time_table[index] as f64;
            let x = time / last_time * timeline_width - timeline_viewport_x;
            (x, value)
        })
        .peekable();

    // @TODO parallelize?
    let mut blocks = Vec::new();
    while let Some((block_x, value)) = x_value_pairs.next() {
        if block_x >= (timeline_viewport_width as f64) {
            break;
        }

        let next_block_x = if let Some((next_block_x, _)) = x_value_pairs.peek() {
            *next_block_x
        } else {
            timeline_width - timeline_viewport_x
        };

        let block_width = (next_block_x - block_x) as u32;
        if block_width < MIN_BLOCK_WIDTH {
            continue;
        }
        if block_x + (block_width as f64) <= 0. {
            continue;
        }

        // @TODO cache?
        let value = var_format.format(value);

        let value_width = (value.chars().count() as f64 * LETTER_WIDTH) as u32;
        // @TODO Ellipsis instead of hiding?
        let label = if (value_width + (2 * LABEL_X_PADDING)) <= block_width {
            Some(TimeLineBlockLabel {
                text: value,
                x: (block_width - value_width) / 2,
                y: (block_height - LETTER_HEIGHT) / 2,
            })
        } else {
            None
        };

        let block = TimelineBlock {
            x: block_x as i32,
            width: block_width,
            height: block_height,
            label,
        };
        blocks.push(block);
    }

    Timeline { blocks }
}
