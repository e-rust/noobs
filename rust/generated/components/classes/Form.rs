
/// Um Form de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Form {
    pub id: ComponentId,
}

impl Handle for Form {
    fn id(&self) -> ComponentId {
        self.id
    }
}

/// Quem disparou um handler tipado (`sender: Form`): só um Form.
impl aylis::prelude::FromSender for Form {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {
        (ui.class_name(id) == Some("Form")).then_some(Self { id })
    }
}

impl Form {
    pub fn caption(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "caption"))
    }
    pub fn set_caption(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "caption", Into::<String>::into(value))
    }
    pub fn position(self, ui: &Ui) -> aylis::prelude::FormPosition {
        handles::read_enum::<aylis::prelude::FormPosition>(ui, self.id, "position")
    }
    pub fn set_position(self, ui: &mut Ui, value: aylis::prelude::FormPosition) {
        handles::write(ui, self.id, "position", PropValue::enum_value(value))
    }
    pub fn border_style(self, ui: &Ui) -> aylis::prelude::BorderStyle {
        handles::read_enum::<aylis::prelude::BorderStyle>(ui, self.id, "border_style")
    }
    pub fn set_border_style(self, ui: &mut Ui, value: aylis::prelude::BorderStyle) {
        handles::write(ui, self.id, "border_style", PropValue::enum_value(value))
    }
    pub fn window_corners(self, ui: &Ui) -> aylis::prelude::WindowCorners {
        handles::read_enum::<aylis::prelude::WindowCorners>(ui, self.id, "window_corners")
    }
    pub fn set_window_corners(self, ui: &mut Ui, value: aylis::prelude::WindowCorners) {
        handles::write(ui, self.id, "window_corners", PropValue::enum_value(value))
    }
    pub fn transparent(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "transparent").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_transparent(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "transparent", value)
    }
    pub fn window_shape(self, ui: &Ui) -> aylis::prelude::WindowShape {
        handles::read_enum::<aylis::prelude::WindowShape>(ui, self.id, "window_shape")
    }
    pub fn set_window_shape(self, ui: &mut Ui, value: aylis::prelude::WindowShape) {
        handles::write(ui, self.id, "window_shape", PropValue::enum_value(value))
    }
    pub fn shape_radius(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "shape_radius").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_shape_radius(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "shape_radius", value)
    }
    pub fn shape_radii(self, ui: &Ui) -> aylis::prelude::Bounds {
        handles::read(ui, self.id, "shape_radii").and_then(|v| v.to_bounds().ok()).unwrap_or_default()
    }
    pub fn set_shape_radii(self, ui: &mut Ui, value: aylis::prelude::Bounds) {
        handles::write(ui, self.id, "shape_radii", value)
    }
    pub fn shape_path(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "shape_path"))
    }
    pub fn set_shape_path(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "shape_path", Into::<String>::into(value))
    }
    pub fn window_state(self, ui: &Ui) -> aylis::prelude::WindowState {
        handles::read_enum::<aylis::prelude::WindowState>(ui, self.id, "window_state")
    }
    pub fn set_window_state(self, ui: &mut Ui, value: aylis::prelude::WindowState) {
        handles::write(ui, self.id, "window_state", PropValue::enum_value(value))
    }
    pub fn transition(self, ui: &Ui) -> aylis::prelude::Transition {
        handles::read_enum::<aylis::prelude::Transition>(ui, self.id, "transition")
    }
    pub fn set_transition(self, ui: &mut Ui, value: aylis::prelude::Transition) {
        handles::write(ui, self.id, "transition", PropValue::enum_value(value))
    }
    pub fn layer_cache(self, ui: &Ui) -> aylis::prelude::LayerCaching {
        handles::read_enum::<aylis::prelude::LayerCaching>(ui, self.id, "layer_cache")
    }
    pub fn set_layer_cache(self, ui: &mut Ui, value: aylis::prelude::LayerCaching) {
        handles::write(ui, self.id, "layer_cache", PropValue::enum_value(value))
    }
    pub fn frame_reuse_kb(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "frame_reuse_kb").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_frame_reuse_kb(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "frame_reuse_kb", value)
    }
    pub fn topmost(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "topmost").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_topmost(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "topmost", value)
    }
    pub fn bottommost(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "bottommost").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_bottommost(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "bottommost", value)
    }
    pub fn keep_awake(self, ui: &Ui) -> aylis::prelude::KeepAwake {
        handles::read_enum::<aylis::prelude::KeepAwake>(ui, self.id, "keep_awake")
    }
    pub fn set_keep_awake(self, ui: &mut Ui, value: aylis::prelude::KeepAwake) {
        handles::write(ui, self.id, "keep_awake", PropValue::enum_value(value))
    }
    pub fn confine_cursor(self, ui: &Ui) -> aylis::prelude::CursorConfine {
        handles::read_enum::<aylis::prelude::CursorConfine>(ui, self.id, "confine_cursor")
    }
    pub fn set_confine_cursor(self, ui: &mut Ui, value: aylis::prelude::CursorConfine) {
        handles::write(ui, self.id, "confine_cursor", PropValue::enum_value(value))
    }
    pub fn window_opacity(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "window_opacity").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_window_opacity(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "window_opacity", value)
    }
    pub fn full_screen_mode(self, ui: &Ui) -> aylis::prelude::FullScreenMode {
        handles::read_enum::<aylis::prelude::FullScreenMode>(ui, self.id, "full_screen_mode")
    }
    pub fn set_full_screen_mode(self, ui: &mut Ui, value: aylis::prelude::FullScreenMode) {
        handles::write(ui, self.id, "full_screen_mode", PropValue::enum_value(value))
    }
    pub fn video_width(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "video_width").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_video_width(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "video_width", value)
    }
    pub fn video_height(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "video_height").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_video_height(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "video_height", value)
    }
    pub fn video_refresh(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "video_refresh").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_video_refresh(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "video_refresh", value)
    }
    pub fn show_in_taskbar(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "show_in_taskbar").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_show_in_taskbar(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "show_in_taskbar", value)
    }
    pub fn active_control(self, ui: &Ui) -> Option<ComponentId> {
        handles::read_ref(ui, self.id, "active_control")
    }
    pub fn set_active_control(self, ui: &mut Ui, value: impl Into<handles::Ref>) {
        handles::write(ui, self.id, "active_control", Into::<handles::Ref>::into(value))
    }
    pub fn tab_order(self, ui: &Ui) -> Vec<String> {
        handles::read_list(ui, self.id, "tab_order")
    }
    pub fn set_tab_order(self, ui: &mut Ui, value: &[impl AsRef<str>]) {
        handles::write(ui, self.id, "tab_order", handles::list_value(value))
    }
    pub fn tab_keys(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "tab_keys"))
    }
    pub fn set_tab_keys(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "tab_keys", Into::<String>::into(value))
    }
    pub fn back_tab_keys(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "back_tab_keys"))
    }
    pub fn set_back_tab_keys(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "back_tab_keys", Into::<String>::into(value))
    }
    pub fn allowed_chars(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "allowed_chars"))
    }
    pub fn set_allowed_chars(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "allowed_chars", Into::<String>::into(value))
    }
    pub fn blocked_chars(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "blocked_chars"))
    }
    pub fn set_blocked_chars(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "blocked_chars", Into::<String>::into(value))
    }
    pub fn only_allowed(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "only_allowed").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_only_allowed(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "only_allowed", value)
    }
    pub fn chars_override(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "chars_override").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_chars_override(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "chars_override", value)
    }
    pub fn char_map(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "char_map"))
    }
    pub fn set_char_map(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "char_map", Into::<String>::into(value))
    }
    pub fn key_preview(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "key_preview").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_key_preview(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "key_preview", value)
    }
    pub fn zoom(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "zoom").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_zoom(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "zoom", value)
    }
    pub fn monitor(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "monitor").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_monitor(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "monitor", value)
    }
    pub fn remember_placement(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "remember_placement").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_remember_placement(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "remember_placement", value)
    }
    pub fn gpu(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "gpu"))
    }
    pub fn set_gpu(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "gpu", Into::<String>::into(value))
    }
    pub fn present(self, ui: &Ui) -> aylis::prelude::FormPresent {
        handles::read_enum::<aylis::prelude::FormPresent>(ui, self.id, "present")
    }
    pub fn set_present(self, ui: &mut Ui, value: aylis::prelude::FormPresent) {
        handles::write(ui, self.id, "present", PropValue::enum_value(value))
    }
    pub fn span_monitors(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "span_monitors").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_span_monitors(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "span_monitors", value)
    }
    pub fn physical_pixels(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "physical_pixels").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_physical_pixels(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "physical_pixels", value)
    }
    pub fn activate_on_show(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "activate_on_show").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_activate_on_show(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "activate_on_show", value)
    }
    pub fn force_foreground(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "force_foreground").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_force_foreground(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "force_foreground", value)
    }
    pub fn close_action(self, ui: &Ui) -> aylis::prelude::CloseAction {
        handles::read_enum::<aylis::prelude::CloseAction>(ui, self.id, "close_action")
    }
    pub fn set_close_action(self, ui: &mut Ui, value: aylis::prelude::CloseAction) {
        handles::write(ui, self.id, "close_action", PropValue::enum_value(value))
    }
    pub fn frame_rate(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "frame_rate").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_frame_rate(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "frame_rate", value)
    }
    pub fn orientation(self, ui: &Ui) -> aylis::prelude::FormOrientation {
        handles::read_enum::<aylis::prelude::FormOrientation>(ui, self.id, "orientation")
    }
    pub fn set_orientation(self, ui: &mut Ui, value: aylis::prelude::FormOrientation) {
        handles::write(ui, self.id, "orientation", PropValue::enum_value(value))
    }
    pub fn host(self, ui: &Ui) -> Option<ComponentId> {
        handles::read_ref(ui, self.id, "host")
    }
    pub fn set_host(self, ui: &mut Ui, value: impl Into<handles::Ref>) {
        handles::write(ui, self.id, "host", Into::<handles::Ref>::into(value))
    }
    pub fn dockable(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "dockable").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_dockable(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "dockable", value)
    }
    pub fn on_show(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_show"))
    }
    pub fn set_on_show(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_show", Into::<String>::into(value))
    }
    pub fn on_window_created(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_window_created"))
    }
    pub fn set_on_window_created(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_window_created", Into::<String>::into(value))
    }
    pub fn on_close(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_close"))
    }
    pub fn set_on_close(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_close", Into::<String>::into(value))
    }
    pub fn on_activate(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_activate"))
    }
    pub fn set_on_activate(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_activate", Into::<String>::into(value))
    }
    pub fn on_deactivate(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_deactivate"))
    }
    pub fn set_on_deactivate(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_deactivate", Into::<String>::into(value))
    }
    pub fn on_minimize(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_minimize"))
    }
    pub fn set_on_minimize(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_minimize", Into::<String>::into(value))
    }
    pub fn on_restore(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_restore"))
    }
    pub fn set_on_restore(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_restore", Into::<String>::into(value))
    }
    pub fn on_navigate(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_navigate"))
    }
    pub fn set_on_navigate(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_navigate", Into::<String>::into(value))
    }
    pub fn on_gpu_change(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_gpu_change"))
    }
    pub fn set_on_gpu_change(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_gpu_change", Into::<String>::into(value))
    }
    pub fn on_dock(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_dock"))
    }
    pub fn set_on_dock(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_dock", Into::<String>::into(value))
    }
    pub fn on_undock(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_undock"))
    }
    pub fn set_on_undock(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_undock", Into::<String>::into(value))
    }
    pub fn client_width(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "client_width").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_client_width(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "client_width", value)
    }
    pub fn client_height(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "client_height").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_client_height(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "client_height", value)
    }
    pub fn left(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "left").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_left(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "left", value)
    }
    pub fn top(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "top").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_top(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "top", value)
    }
    pub fn width(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "width").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_width(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "width", value)
    }
    pub fn height(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "height").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_height(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "height", value)
    }
    pub fn align(self, ui: &Ui) -> aylis::prelude::Align {
        handles::read_enum::<aylis::prelude::Align>(ui, self.id, "align")
    }
    pub fn set_align(self, ui: &mut Ui, value: aylis::prelude::Align) {
        handles::write(ui, self.id, "align", PropValue::enum_value(value))
    }
    pub fn anchors(self, ui: &Ui) -> aylis::prelude::Anchors {
        handles::read(ui, self.id, "anchors").and_then(|v| v.to_anchors().ok()).unwrap_or_default()
    }
    pub fn set_anchors(self, ui: &mut Ui, value: aylis::prelude::Anchors) {
        handles::write(ui, self.id, "anchors", PropValue::from(value))
    }
    pub fn margins(self, ui: &Ui) -> aylis::prelude::Bounds {
        handles::read(ui, self.id, "margins").and_then(|v| v.to_bounds().ok()).unwrap_or_default()
    }
    pub fn set_margins(self, ui: &mut Ui, value: aylis::prelude::Bounds) {
        handles::write(ui, self.id, "margins", value)
    }
    pub fn padding(self, ui: &Ui) -> aylis::prelude::Bounds {
        handles::read(ui, self.id, "padding").and_then(|v| v.to_bounds().ok()).unwrap_or_default()
    }
    pub fn set_padding(self, ui: &mut Ui, value: aylis::prelude::Bounds) {
        handles::write(ui, self.id, "padding", value)
    }
    pub fn visible(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "visible").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_visible(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "visible", value)
    }
    pub fn enabled(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "enabled").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_enabled(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "enabled", value)
    }
    pub fn hit_test(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "hit_test").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_hit_test(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "hit_test", value)
    }
    pub fn clip_children(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "clip_children").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_clip_children(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "clip_children", value)
    }
    pub fn inherit_state(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "inherit_state").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_inherit_state(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "inherit_state", value)
    }
    pub fn translate(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "translate").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_translate(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "translate", value)
    }
    pub fn captures_wheel(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "captures_wheel").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_captures_wheel(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "captures_wheel", value)
    }
    pub fn captures_gesture(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "captures_gesture").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_captures_gesture(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "captures_gesture", value)
    }
    pub fn can_focus(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "can_focus").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_can_focus(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "can_focus", value)
    }
    pub fn draggable(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "draggable").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_draggable(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "draggable", value)
    }
    pub fn accepts_drops(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "accepts_drops").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_accepts_drops(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "accepts_drops", value)
    }
    pub fn drag_data(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "drag_data"))
    }
    pub fn set_drag_data(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "drag_data", Into::<String>::into(value))
    }
    pub fn drag_label(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "drag_label"))
    }
    pub fn set_drag_label(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "drag_label", Into::<String>::into(value))
    }
    pub fn drag_image(self, ui: &Ui) -> Option<ComponentId> {
        handles::read_ref(ui, self.id, "drag_image")
    }
    pub fn set_drag_image(self, ui: &mut Ui, value: impl Into<handles::Ref>) {
        handles::write(ui, self.id, "drag_image", Into::<handles::Ref>::into(value))
    }
    pub fn drop_types(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "drop_types"))
    }
    pub fn set_drop_types(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "drop_types", Into::<String>::into(value))
    }
    pub fn tab_stop(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "tab_stop").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_tab_stop(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "tab_stop", value)
    }
    pub fn cursor(self, ui: &Ui) -> aylis::prelude::CursorShape {
        handles::read_enum::<aylis::prelude::CursorShape>(ui, self.id, "cursor")
    }
    pub fn set_cursor(self, ui: &mut Ui, value: aylis::prelude::CursorShape) {
        handles::write(ui, self.id, "cursor", PropValue::enum_value(value))
    }
    pub fn opacity(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "opacity").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_opacity(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "opacity", value)
    }
    pub fn blend_mode(self, ui: &Ui) -> aylis::prelude::BlendMode {
        handles::read_enum::<aylis::prelude::BlendMode>(ui, self.id, "blend_mode")
    }
    pub fn set_blend_mode(self, ui: &mut Ui, value: aylis::prelude::BlendMode) {
        handles::write(ui, self.id, "blend_mode", PropValue::enum_value(value))
    }
    pub fn cache_layer(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "cache_layer").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_cache_layer(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "cache_layer", value)
    }
    pub fn rotation(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "rotation").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_rotation(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "rotation", value)
    }
    pub fn scale(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "scale").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_scale(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "scale", value)
    }
    pub fn translate_x(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "translate_x").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_translate_x(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "translate_x", value)
    }
    pub fn translate_y(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "translate_y").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_translate_y(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "translate_y", value)
    }
    pub fn aspect_ratio(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "aspect_ratio").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_aspect_ratio(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "aspect_ratio", value)
    }
    pub fn max_width(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "max_width").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_max_width(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "max_width", value)
    }
    pub fn min_width(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "min_width").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_min_width(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "min_width", value)
    }
    pub fn min_height(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "min_height").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_min_height(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "min_height", value)
    }
    pub fn max_height(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "max_height").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_max_height(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "max_height", value)
    }
    pub fn self_align(self, ui: &Ui) -> aylis::prelude::SelfAlign {
        handles::read_enum::<aylis::prelude::SelfAlign>(ui, self.id, "self_align")
    }
    pub fn set_self_align(self, ui: &mut Ui, value: aylis::prelude::SelfAlign) {
        handles::write(ui, self.id, "self_align", PropValue::enum_value(value))
    }
    pub fn auto_size(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "auto_size").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_auto_size(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "auto_size", value)
    }
    pub fn column_span(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "column_span").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_column_span(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "column_span", value)
    }
    pub fn grid_row(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "grid_row").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_grid_row(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "grid_row", value)
    }
    pub fn grid_column(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "grid_column").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_grid_column(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "grid_column", value)
    }
    pub fn row_span(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "row_span").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_row_span(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "row_span", value)
    }
    pub fn grow(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "grow").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_grow(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "grow", value)
    }
    pub fn shadow(self, ui: &Ui) -> Option<aylis::prelude::Shadow> {
        handles::read_shadow(ui, self.id, "shadow")
    }
    pub fn set_shadow(self, ui: &mut Ui, value: Option<aylis::prelude::Shadow>) {
        handles::write(ui, self.id, "shadow", handles::shadow_value(value))
    }
    pub fn style(self, ui: &Ui) -> FormStyle {
        handles::read_enum::<FormStyle>(ui, self.id, "style")
    }
    pub fn set_style(self, ui: &mut Ui, value: FormStyle) {
        handles::write(ui, self.id, "style", PropValue::enum_value(value))
    }
    pub fn theme(self, ui: &Ui) -> ThemeName {
        handles::read_enum::<ThemeName>(ui, self.id, "theme")
    }
    pub fn set_theme(self, ui: &mut Ui, value: ThemeName) {
        handles::write(ui, self.id, "theme", PropValue::enum_value(value))
    }
    pub fn fill(self, ui: &Ui) -> aylis::prelude::Color {
        handles::read(ui, self.id, "fill").and_then(|v| v.to_color().ok()).unwrap_or_default()
    }
    pub fn set_fill(self, ui: &mut Ui, value: impl Into<handles::ColorRef>) {
        handles::write(ui, self.id, "fill", Into::<handles::ColorRef>::into(value))
    }
    pub fn border_color(self, ui: &Ui) -> aylis::prelude::Color {
        handles::read(ui, self.id, "border_color").and_then(|v| v.to_color().ok()).unwrap_or_default()
    }
    pub fn set_border_color(self, ui: &mut Ui, value: impl Into<handles::ColorRef>) {
        handles::write(ui, self.id, "border_color", Into::<handles::ColorRef>::into(value))
    }
    pub fn text_color(self, ui: &Ui) -> aylis::prelude::Color {
        handles::read(ui, self.id, "text_color").and_then(|v| v.to_color().ok()).unwrap_or_default()
    }
    pub fn set_text_color(self, ui: &mut Ui, value: impl Into<handles::ColorRef>) {
        handles::write(ui, self.id, "text_color", Into::<handles::ColorRef>::into(value))
    }
    pub fn accent_color(self, ui: &Ui) -> aylis::prelude::Color {
        handles::read(ui, self.id, "accent_color").and_then(|v| v.to_color().ok()).unwrap_or_default()
    }
    pub fn set_accent_color(self, ui: &mut Ui, value: impl Into<handles::ColorRef>) {
        handles::write(ui, self.id, "accent_color", Into::<handles::ColorRef>::into(value))
    }
    pub fn success_color(self, ui: &Ui) -> aylis::prelude::Color {
        handles::read(ui, self.id, "success_color").and_then(|v| v.to_color().ok()).unwrap_or_default()
    }
    pub fn set_success_color(self, ui: &mut Ui, value: impl Into<handles::ColorRef>) {
        handles::write(ui, self.id, "success_color", Into::<handles::ColorRef>::into(value))
    }
    pub fn warning_color(self, ui: &Ui) -> aylis::prelude::Color {
        handles::read(ui, self.id, "warning_color").and_then(|v| v.to_color().ok()).unwrap_or_default()
    }
    pub fn set_warning_color(self, ui: &mut Ui, value: impl Into<handles::ColorRef>) {
        handles::write(ui, self.id, "warning_color", Into::<handles::ColorRef>::into(value))
    }
    pub fn danger_color(self, ui: &Ui) -> aylis::prelude::Color {
        handles::read(ui, self.id, "danger_color").and_then(|v| v.to_color().ok()).unwrap_or_default()
    }
    pub fn set_danger_color(self, ui: &mut Ui, value: impl Into<handles::ColorRef>) {
        handles::write(ui, self.id, "danger_color", Into::<handles::ColorRef>::into(value))
    }
    pub fn font_family(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "font_family"))
    }
    pub fn set_font_family(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "font_family", Into::<String>::into(value))
    }
    pub fn font_size(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "font_size").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_font_size(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "font_size", value)
    }
    pub fn font_weight(self, ui: &Ui) -> FontWeight {
        handles::read_enum::<FontWeight>(ui, self.id, "font_weight")
    }
    pub fn set_font_weight(self, ui: &mut Ui, value: FontWeight) {
        handles::write(ui, self.id, "font_weight", PropValue::enum_value(value))
    }
    pub fn font_italic(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "font_italic").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_font_italic(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "font_italic", value)
    }
    pub fn font_underline(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "font_underline").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_font_underline(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "font_underline", value)
    }
    pub fn font_strikeout(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "font_strikeout").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_font_strikeout(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "font_strikeout", value)
    }
    pub fn hint(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "hint"))
    }
    pub fn set_hint(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "hint", Into::<String>::into(value))
    }
    pub fn tag(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "tag").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_tag(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "tag", value)
    }
    pub fn tag_string(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "tag_string"))
    }
    pub fn set_tag_string(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "tag_string", Into::<String>::into(value))
    }
    pub fn accessible_name(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "accessible_name"))
    }
    pub fn set_accessible_name(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "accessible_name", Into::<String>::into(value))
    }
    pub fn popup_menu(self, ui: &Ui) -> Option<ComponentId> {
        handles::read_ref(ui, self.id, "popup_menu")
    }
    pub fn set_popup_menu(self, ui: &mut Ui, value: impl Into<handles::Ref>) {
        handles::write(ui, self.id, "popup_menu", Into::<handles::Ref>::into(value))
    }
    pub fn on_click(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_click"))
    }
    pub fn set_on_click(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_click", Into::<String>::into(value))
    }
    pub fn on_dbl_click(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_dbl_click"))
    }
    pub fn set_on_dbl_click(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_dbl_click", Into::<String>::into(value))
    }
    pub fn on_long_press(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_long_press"))
    }
    pub fn set_on_long_press(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_long_press", Into::<String>::into(value))
    }
    pub fn on_pointer_down(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_pointer_down"))
    }
    pub fn set_on_pointer_down(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_pointer_down", Into::<String>::into(value))
    }
    pub fn on_pointer_up(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_pointer_up"))
    }
    pub fn set_on_pointer_up(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_pointer_up", Into::<String>::into(value))
    }
    pub fn on_pointer_move(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_pointer_move"))
    }
    pub fn set_on_pointer_move(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_pointer_move", Into::<String>::into(value))
    }
    pub fn on_pointer_enter(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_pointer_enter"))
    }
    pub fn set_on_pointer_enter(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_pointer_enter", Into::<String>::into(value))
    }
    pub fn on_pointer_leave(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_pointer_leave"))
    }
    pub fn set_on_pointer_leave(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_pointer_leave", Into::<String>::into(value))
    }
    pub fn on_wheel(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_wheel"))
    }
    pub fn set_on_wheel(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_wheel", Into::<String>::into(value))
    }
    pub fn on_gesture(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_gesture"))
    }
    pub fn set_on_gesture(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_gesture", Into::<String>::into(value))
    }
    pub fn on_swipe(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_swipe"))
    }
    pub fn set_on_swipe(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_swipe", Into::<String>::into(value))
    }
    pub fn on_key_up(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_key_up"))
    }
    pub fn set_on_key_up(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_key_up", Into::<String>::into(value))
    }
    pub fn on_enter(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_enter"))
    }
    pub fn set_on_enter(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_enter", Into::<String>::into(value))
    }
    pub fn on_exit(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_exit"))
    }
    pub fn set_on_exit(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_exit", Into::<String>::into(value))
    }
    pub fn on_resize(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_resize"))
    }
    pub fn set_on_resize(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_resize", Into::<String>::into(value))
    }
    pub fn on_drag_move(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_drag_move"))
    }
    pub fn set_on_drag_move(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_drag_move", Into::<String>::into(value))
    }
    pub fn on_drag_leave(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_drag_leave"))
    }
    pub fn set_on_drag_leave(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_drag_leave", Into::<String>::into(value))
    }
    pub fn on_drag_drop(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_drag_drop"))
    }
    pub fn set_on_drag_drop(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_drag_drop", Into::<String>::into(value))
    }
    pub fn on_drag_end(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_drag_end"))
    }
    pub fn set_on_drag_end(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_drag_end", Into::<String>::into(value))
    }
    pub fn move_to_monitor(self, ui: &mut Ui, monitor: i64) -> Result<(), String> {
        handles::try_call(ui, self.id, "move_to_monitor", &[monitor.into()]).map(|_| ())
    }
    pub fn request_attention(self, ui: &mut Ui, critical: bool) -> Result<(), String> {
        handles::try_call(ui, self.id, "request_attention", &[critical.into()]).map(|_| ())
    }
    pub fn gpu_adapter(self, ui: &mut Ui) -> Result<String, String> {
        let v = handles::try_call(ui, self.id, "gpu_adapter", &[])?;
        Ok(handles::text(ui, Some(v)))
    }
    pub fn gpu_adapters(self, ui: &mut Ui) -> Result<Vec<String>, String> {
        let v = handles::try_call(ui, self.id, "gpu_adapters", &[])?;
        Ok(v.to_str().unwrap_or_default().split(';').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect())
    }
    pub fn render_rgba(self, ui: &mut Ui, x: f64, y: f64, width: f64, height: f64, scale: f64) -> Result<String, String> {
        let v = handles::try_call(ui, self.id, "render_rgba", &[x.into(), y.into(), width.into(), height.into(), scale.into()])?;
        Ok(handles::text(ui, Some(v)))
    }
    pub fn show(self, ui: &mut Ui) {
        let _ = ui.show(self.id);
    }
    pub fn close(self, ui: &mut Ui) -> bool {
        ui.close(self.id)
    }
    /// O nome do controle com foco (vazio sem foco).
    pub fn focused(self, ui: &Ui) -> String {
        ui.focused(ui.root_of(self.id)).and_then(|f| ui.name(f)).unwrap_or_default().to_string()
    }
    pub fn focus_next(self, ui: &mut Ui) -> bool {
        let root = ui.root_of(self.id);
        ui.focus_next(root, true)
    }
    pub fn focus_prev(self, ui: &mut Ui) -> bool {
        let root = ui.root_of(self.id);
        ui.focus_next(root, false)
    }
}
