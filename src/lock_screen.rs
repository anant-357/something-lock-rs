use smithay_client_toolkit::shell::wlr_layer::LayerSurface;

pub struct Layer {
    pub layer_surface: LayerSurface,
    pub width: u32,
    pub height: u32,
}
