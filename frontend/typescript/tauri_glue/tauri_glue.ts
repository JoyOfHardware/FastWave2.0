// @TODO use TS and Tauri bindgens to make this code properly typed

import { core } from '@tauri-apps/api'

const invoke = core.invoke;

type Filename = string;
type JavascriptCode = string;
type WellenHierarchy = unknown;
type Timeline = unknown;
type VarFormat = unknown;
type AddedDecodersCount = number;
type RemovedDecodersCount = number;
type DecoderPath = string;

export async function show_window(): Promise<void> {
    return await invoke("show_window");
}

export async function pick_and_load_waveform(): Promise<Filename | undefined> {
    return await invoke("pick_and_load_waveform");
}

export async function load_file_with_selected_vars(): Promise<JavascriptCode | undefined> {
    return await invoke("load_file_with_selected_vars");
}

export async function get_hierarchy(): Promise<WellenHierarchy> {
    return await invoke("get_hierarchy");
}

export async function load_signal_and_get_timeline(
    signal_ref_index: number, 
    timeline_zoom: number,
    timeline_viewport_width: number,
    timeline_viewport_x: number, 
    block_height: number,
    var_format: VarFormat,
): Promise<Timeline> {
    return await invoke("load_signal_and_get_timeline", { 
        signal_ref_index, 
        timeline_zoom, 
        timeline_viewport_width,
        timeline_viewport_x,
        block_height, 
        var_format 
    });
}

export async function unload_signal(signal_ref_index: number): Promise<void> {
    return await invoke("unload_signal", { signal_ref_index });
}

export async function add_decoders(decoder_paths: Array<DecoderPath>): Promise<AddedDecodersCount> {
    return await invoke("add_decoders", { decoder_paths });
}

export async function remove_all_decoders(): Promise<RemovedDecodersCount> {
    return await invoke("remove_all_decoders");
}
