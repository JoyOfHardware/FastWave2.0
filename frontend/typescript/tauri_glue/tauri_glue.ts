// @TODO use TS and Tauri bindgens to make this code properly typed

import { core } from '@tauri-apps/api'

const invoke = core.invoke;

type WellenHierarchy = unknown;
type WellenTimeTable = unknown;
type WellenSignal = unknown;

export async function show_window(): Promise<void> {
    return await invoke("show_window");
}

export async function load_waveform(test_file_name: string): Promise<void> {
    return await invoke("load_waveform", { test_file_name });
}

export async function get_hierarchy(): Promise<WellenHierarchy> {
    return await invoke("get_hierarchy");
}

export async function get_time_table(): Promise<WellenTimeTable> {
    return await invoke("get_time_table");
}

export async function load_and_get_signal(signal_ref_index: number): Promise<WellenSignal> {
    return await invoke("load_and_get_signal", { signal_ref_index });
}

export async function unload_signal(signal_ref_index: number): Promise<void> {
    return await invoke("unload_signal", { signal_ref_index });
}
