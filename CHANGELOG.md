# Changelog

## plotters-dixous 0.2.1

- add the complete list of mouse event on the component *plotters*

## plotters-dioxus 0.2.0 (2024-02-22)

### Added

- callback to handle the on_wheel and on_click action.

### Improved

- Change the rendering system because of performance issue: 
  - does not use svg anymore
  - Draw directly in a bitmap
  - render the bitmap in the html with a base64 encoding.
- Rename the user interface *on_drawing* into *init*

### Removed

- The *dioxus* backend which generate *LazyNodes*

## plotters-dioxus 0.1.0 (2024-02-18)

### Added

- A *dioxus* backend implementing the trait *DrawingBackend* from *plotter-rs* to generate *LazyNodes of svg items*
- A *dioxus* component named **Plotter**, using the *dioxus* backend to render the plots.
- A *callback* on_drawing for the component **Plotter** as user interface to define plots. 
