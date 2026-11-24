
/// Um FlowLayout de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowLayout {
    pub id: ComponentId,
}

impl Handle for FlowLayout {
    fn id(&self) -> ComponentId {
        self.id
    }
}

/// Quem disparou um handler tipado (`sender: FlowLayout`): só um FlowLayout.
impl aylis::prelude::FromSender for FlowLayout {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {
        (ui.class_name(id) == Some("FlowLayout")).then_some(Self { id })
    }
}

impl FlowLayout {
    pub fn gap(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "gap").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_gap(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "gap", value)
    }
    pub fn line_gap(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "line_gap").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_line_gap(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "line_gap", value)
    }
    pub fn h_align(self, ui: &Ui) -> aylis::prelude::TextAlign {
        handles::read_enum::<aylis::prelude::TextAlign>(ui, self.id, "h_align")
    }
    pub fn set_h_align(self, ui: &mut Ui, value: aylis::prelude::TextAlign) {
        handles::write(ui, self.id, "h_align", PropValue::enum_value(value))
    }
    pub fn v_align(self, ui: &Ui) -> aylis::prelude::TextAlign {
        handles::read_enum::<aylis::prelude::TextAlign>(ui, self.id, "v_align")
    }
    pub fn set_v_align(self, ui: &mut Ui, value: aylis::prelude::TextAlign) {
        handles::write(ui, self.id, "v_align", PropValue::enum_value(value))
    }
    pub fn baseline(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "baseline").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_baseline(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "baseline", value)
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
    pub fn style(self, ui: &Ui) -> FlowLayoutStyle {
        handles::read_enum::<FlowLayoutStyle>(ui, self.id, "style")
    }
    pub fn set_style(self, ui: &mut Ui, value: FlowLayoutStyle) {
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
    pub fn font_weight(self, ui: &Ui) -> Option<aylis::prelude::FontWeight> {
        handles::read_enum::<Option<aylis::prelude::FontWeight>>(ui, self.id, "font_weight")
    }
    pub fn set_font_weight(self, ui: &mut Ui, value: Option<aylis::prelude::FontWeight>) {
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
}
