import { Excalidraw, MainMenu } from '@excalidraw/excalidraw'
import { ExcalidrawImperativeAPI } from '@excalidraw/excalidraw/types/types'
import { ExcalidrawElement } from '@excalidraw/excalidraw/types/element/types'
// @TODO doesn't work with Excalidraw 0.17.6
// import { ExcalidrawElementSkeleton, convertToExcalidrawElements } from '@excalidraw/excalidraw/types/data/transform'
import * as React from 'react'
import * as ReactDOM from 'react-dom/client'

export class ExcalidrawController {
    api: Promise<ExcalidrawImperativeAPI>
    resolve_api: (api: ExcalidrawImperativeAPI) => void

    constructor() {
      this.resolve_api = (api) => {};
      this.api = new Promise(resolve => {
        this.resolve_api = (api) => resolve(api)
      });
    }

    draw_diagram_element(excalidraw_element: ExcalidrawElement) {
      this.api.then(api => {
        const elements = api.getSceneElements()
        api.updateScene({
          elements: elements.concat(excalidraw_element)
        })
      });
    }

    listen_for_component_text_changes(id: string, on_change: (text: string) => void) {
      this.api.then(api => {
        let old_text: string | null = null; 
        api.onChange((elements: readonly ExcalidrawElement[]) => {
          const element = elements.find(element => element.id === id);
          if (typeof element !== 'undefined') {
            if (element.type === 'text') {
              if (old_text === null) {
                old_text = element.text;
                on_change(old_text);
              } else {
                if (old_text !== element.text) {
                  old_text = element.text;
                  on_change(old_text);
                }
              }
            }
          }
        })
      })
    }

    set_component_text(id: string, text: string) {
      this.api.then(api => {
        let element_found = false;
        const elements = api.getSceneElements().map(element => {
          if (element.id === id) {
            element_found = true;
            return { ...element, text: text, originalText: text }
          } else {
            return element
          }
        });
        if (element_found) {
            api.updateScene({
              elements
            })
        }
      })
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
                excalidrawAPI={(api) => this.resolve_api(api)}
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
  