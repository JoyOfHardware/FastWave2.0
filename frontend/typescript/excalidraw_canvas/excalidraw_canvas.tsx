import { Excalidraw, MainMenu } from '@excalidraw/excalidraw'
import { ExcalidrawImperativeAPI } from '@excalidraw/excalidraw/types/types'
import { ExcalidrawElement } from '@excalidraw/excalidraw/types/element/types'
// @TODO doesn't work with Excalidraw 0.17.6
// import { ExcalidrawElementSkeleton, convertToExcalidrawElements } from '@excalidraw/excalidraw/types/data/transform'
import * as React from 'react'
import * as ReactDOM from 'react-dom/client'

export class ExcalidrawController {
    api: ExcalidrawImperativeAPI | undefined

    constructor() {}

    draw_diagram_element(excalidraw_element: ExcalidrawElement) {
      if (typeof this.api !== 'undefined') {
        const elements = this.api.getSceneElements()
        this.api.updateScene({
          elements: elements.concat(excalidraw_element)
        })
      }
    }

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
                excalidrawAPI={(api) => this.api = api}
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
  