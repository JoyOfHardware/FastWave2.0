import { Excalidraw } from '@excalidraw/excalidraw'
import * as React from 'react'
import * as ReactDOM from 'react-dom/client'

export class ExcalidrawController {
    constructor() {}

    async init(parent_element: HTMLElement) {
        const App = () => {
            return React.createElement(
              React.Fragment,
              null,
              React.createElement(
                "div",
                {
                  style: { height: "100%" },
                },
                React.createElement(Excalidraw),
              ),
            );
          };
        const root = ReactDOM.createRoot(parent_element);
        root.render(React.createElement(App));
    }
}
  