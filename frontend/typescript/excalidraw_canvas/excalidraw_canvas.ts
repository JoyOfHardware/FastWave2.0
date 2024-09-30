import { Excalidraw } from '@excalidraw/excalidraw'
import * as React from 'react'
import * as ReactDOM from 'react-dom/client'

export function hello(): String {
    return "Hello from excalidraw_canvas"
}

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

  