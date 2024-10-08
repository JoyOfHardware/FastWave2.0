import { Excalidraw, MainMenu } from '@excalidraw/excalidraw'
import * as React from 'react'
import * as ReactDOM from 'react-dom/client'

export class ExcalidrawController {
    constructor() {}

    async init(parent_element: HTMLElement) {
        const App = () => {
            return (
              <>
                <div style={{ height: "100%" }}>
                  <Excalidraw 
                    theme='dark'
                    gridModeEnabled={true}
                    UIOptions={{canvasActions: {toggleTheme: true}}}
                    initialData={{appState: {
                      // Canvas background: 3. default, blue
                      viewBackgroundColor: "#f5faff",
                      // Sloppiness: Artist
                      currentItemRoughness: 0,
                      // Font family: Code
                      currentItemFontFamily: 3,
                    }}}
                  >
                    <MainMenu>
                      <MainMenu.DefaultItems.LoadScene />
                      <MainMenu.DefaultItems.SaveToActiveFile />
                      <MainMenu.DefaultItems.Export />
                      <MainMenu.DefaultItems.SaveAsImage />
                      <MainMenu.DefaultItems.ClearCanvas />
                      <MainMenu.DefaultItems.ToggleTheme />
                      <MainMenu.DefaultItems.ChangeCanvasBackground />
                    </MainMenu>
                  </Excalidraw>
                </div>
              </>
            );
          };
        const root = ReactDOM.createRoot(parent_element);
        root.render(React.createElement(App));
    }
}
  