// @TODO use TS and Tauri bindgens to make this code properly typed

import { core } from '@tauri-apps/api'

const invoke = core.invoke;

type Filename = string;
type WellenHierarchy = unknown;
type WellenTimeTable = unknown;
type WellenSignal = unknown;
type Timeline = unknown;
type VarFormat = unknown;

export async function show_window(): Promise<void> {
    return await invoke("show_window");
}

export async function pick_and_load_waveform(): Promise<Filename | undefined> {
    return await invoke("pick_and_load_waveform");
}

export async function get_hierarchy(): Promise<WellenHierarchy> {
    return await invoke("get_hierarchy");
}

export async function load_signal_and_get_timeline(
    signal_ref_index: number, 
    screen_width: number, 
    block_height: number,
    var_format: VarFormat,
): Promise<Timeline> {
    return await invoke("load_signal_and_get_timeline", { signal_ref_index, screen_width, block_height, var_format });
}

export async function unload_signal(signal_ref_index: number): Promise<void> {
    return await invoke("unload_signal", { signal_ref_index });
}
