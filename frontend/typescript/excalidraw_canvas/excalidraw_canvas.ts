import { Excalidraw } from '@excalidraw/excalidraw'
import * as React from 'react'
import * as ReactDOM from 'react-dom/client'

export function attach_to_canvas() {
    const App = () => {
        return React.createElement(
          React.Fragment,
          null,
          React.createElement(
            "div",
            {
              style: { height: "500px" },
            },
            React.createElement(Excalidraw),
          ),
        );
      };
      
    const excalidrawWrapper = document.getElementById("excalidraw_canvas");
    const root = ReactDOM.createRoot(excalidrawWrapper!);
}

export class ExcalidrawController {
    // app: Application
    // // -- FastWave-specific --
    // var_signal_rows: Array<VarSignalRow> = [];
    // var_signal_rows_container = new Container();
    // // @TODO reset `timeline_*` on file unload?
    // timeline_zoom: number;
    // timeline_viewport_width: number; 
    // timeline_viewport_x: number;
    // row_height: number;
    // row_gap: number;
    // timeline_getter: TimelineGetter;

    constructor(
        // timeline_zoom: number,
        // timeline_viewport_width: number,
        // timeline_viewport_x: number,
        // row_height: number, 
        // row_gap: number, 
        // timeline_getter: TimelineGetter
    ) {
        this.hello()
        // this.app = new Application();
        // // -- FastWave-specific --
        // this.timeline_zoom = timeline_zoom;
        // this.timeline_viewport_width = timeline_viewport_width;
        // this.timeline_viewport_x = timeline_viewport_x;
        // this.row_height = row_height;
        // this.row_gap = row_gap;
        // this.app.stage.addChild(this.var_signal_rows_container);
        // this.timeline_getter = timeline_getter;
    }

    async init(parent_element: HTMLElement) {
        // await this.app.init({ background: color_dark_slate_blue, antialias: true, resizeTo: parent_element });
        // parent_element.appendChild(this.app.canvas);
    }

    async resize(width: number, _height: number) {
        // // -- FastWave-specific --
        // this.timeline_viewport_width = width;
        // await this.redraw_all_rows();
        // // -- // --
        // this.app.queueResize();
    }

    destroy() {
        // const rendererDestroyOptions = {
        //     removeView: true
        // }
        // const options = {
        //     children: true,
        //     texture: true,
        //     textureSource: true,
        //     context: true,
        // }
        // this.app.destroy(rendererDestroyOptions, options);
    }

    hello() {
        console.log("Hello from excalidraw_canvas TS")
    }
}
  