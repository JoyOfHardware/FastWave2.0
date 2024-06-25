How to create and build the Python component:

1. `pip install componentize-py`
2. Create the `python_decoder` folder
3. `cd python_decoder`
4. Create `.gitignore` with content `__pycache__`
5. Create the `src` folder with the file `app.py`
6. Create the `wit` folder with the file `world.wit`
7. Update code as needed
8. `componentize-py --wit-path wit/world.wit bindings src/bindings`
9. `componentize-py --wit-path wit/world.wit componentize src.app --output python_decoder.wasm`
