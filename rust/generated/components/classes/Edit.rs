
/// Um Edit de um form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edit {
    pub id: ComponentId,
}

impl Handle for Edit {
    fn id(&self) -> ComponentId {
        self.id
    }
}

/// Quem disparou um handler tipado (`sender: Edit`): só um Edit.
impl aylis::prelude::FromSender for Edit {
    fn from_sender(ui: &Ui, id: ComponentId) -> Option<Self> {
        (ui.class_name(id) == Some("Edit")).then_some(Self { id })
    }
}

impl Edit {
    pub fn text(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "text"))
    }
    pub fn set_text(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "text", Into::<String>::into(value))
    }
    pub fn placeholder(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "placeholder"))
    }
    pub fn set_placeholder(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "placeholder", Into::<String>::into(value))
    }
    pub fn border(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "border").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_border(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "border", value)
    }
    pub fn plain(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "plain").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_plain(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "plain", value)
    }
    pub fn max_length(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "max_length").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_max_length(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "max_length", value)
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
    pub fn char_map(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "char_map"))
    }
    pub fn set_char_map(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "char_map", Into::<String>::into(value))
    }
    pub fn char_case(self, ui: &Ui) -> aylis::prelude::CharCase {
        handles::read_enum::<aylis::prelude::CharCase>(ui, self.id, "char_case")
    }
    pub fn set_char_case(self, ui: &mut Ui, value: aylis::prelude::CharCase) {
        handles::write(ui, self.id, "char_case", PropValue::enum_value(value))
    }
    pub fn read_only(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "read_only").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_read_only(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "read_only", value)
    }
    pub fn password(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "password").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_password(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "password", value)
    }
    pub fn sensitive(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "sensitive").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_sensitive(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "sensitive", value)
    }
    pub fn auto_select(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "auto_select").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_auto_select(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "auto_select", value)
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
    pub fn wrap(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "wrap").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_wrap(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "wrap", value)
    }
    pub fn trimming(self, ui: &Ui) -> aylis::prelude::Trimming {
        handles::read_enum::<aylis::prelude::Trimming>(ui, self.id, "trimming")
    }
    pub fn set_trimming(self, ui: &mut Ui, value: aylis::prelude::Trimming) {
        handles::write(ui, self.id, "trimming", PropValue::enum_value(value))
    }
    pub fn icon(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "icon"))
    }
    pub fn set_icon(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "icon", Into::<String>::into(value))
    }
    pub fn clear_button(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "clear_button").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_clear_button(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "clear_button", value)
    }
    pub fn label(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "label"))
    }
    pub fn set_label(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "label", Into::<String>::into(value))
    }
    pub fn helper(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "helper"))
    }
    pub fn set_helper(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "helper", Into::<String>::into(value))
    }
    pub fn error(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "error"))
    }
    pub fn set_error(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "error", Into::<String>::into(value))
    }
    pub fn counter(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "counter").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_counter(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "counter", value)
    }
    pub fn prefix(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "prefix"))
    }
    pub fn set_prefix(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "prefix", Into::<String>::into(value))
    }
    pub fn suffix(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "suffix"))
    }
    pub fn set_suffix(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "suffix", Into::<String>::into(value))
    }
    pub fn mask(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "mask"))
    }
    pub fn set_mask(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "mask", Into::<String>::into(value))
    }
    pub fn rules(self, ui: &Ui) -> Vec<String> {
        handles::read_list(ui, self.id, "rules")
    }
    pub fn set_rules(self, ui: &mut Ui, value: &[impl AsRef<str>]) {
        handles::write(ui, self.id, "rules", handles::list_value(value))
    }
    pub fn spellcheck(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "spellcheck").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_spellcheck(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "spellcheck", value)
    }
    pub fn autocomplete(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "autocomplete").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_autocomplete(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "autocomplete", value)
    }
    pub fn language(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "language"))
    }
    pub fn set_language(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "language", Into::<String>::into(value))
    }
    pub fn keyboard_suggestions(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "keyboard_suggestions").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_keyboard_suggestions(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "keyboard_suggestions", value)
    }
    pub fn text_menu(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "text_menu").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_text_menu(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "text_menu", value)
    }
    pub fn caret_color(self, ui: &Ui) -> aylis::prelude::Color {
        handles::read(ui, self.id, "caret_color").and_then(|v| v.to_color().ok()).unwrap_or_default()
    }
    pub fn set_caret_color(self, ui: &mut Ui, value: impl Into<handles::ColorRef>) {
        handles::write(ui, self.id, "caret_color", Into::<handles::ColorRef>::into(value))
    }
    pub fn caret_width(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "caret_width").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_caret_width(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "caret_width", value)
    }
    pub fn caret_style(self, ui: &Ui) -> aylis::prelude::CaretStyle {
        handles::read_enum::<aylis::prelude::CaretStyle>(ui, self.id, "caret_style")
    }
    pub fn set_caret_style(self, ui: &mut Ui, value: aylis::prelude::CaretStyle) {
        handles::write(ui, self.id, "caret_style", PropValue::enum_value(value))
    }
    pub fn caret_blink(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "caret_blink").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_caret_blink(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "caret_blink", value)
    }
    pub fn on_change(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_change"))
    }
    pub fn set_on_change(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_change", Into::<String>::into(value))
    }
    pub fn on_caret_move(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_caret_move"))
    }
    pub fn set_on_caret_move(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_caret_move", Into::<String>::into(value))
    }
    pub fn on_paste(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "on_paste"))
    }
    pub fn set_on_paste(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "on_paste", Into::<String>::into(value))
    }
    pub fn sel_start(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "sel_start").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_sel_start(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "sel_start", value)
    }
    pub fn sel_length(self, ui: &Ui) -> i64 {
        handles::read(ui, self.id, "sel_length").and_then(|v| v.to_i64().ok()).unwrap_or_default()
    }
    pub fn set_sel_length(self, ui: &mut Ui, value: i64) {
        handles::write(ui, self.id, "sel_length", value)
    }
    pub fn sel_text(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "sel_text"))
    }
    pub fn set_sel_text(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "sel_text", Into::<String>::into(value))
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
    pub fn style(self, ui: &Ui) -> EditStyle {
        handles::read_enum::<EditStyle>(ui, self.id, "style")
    }
    pub fn set_style(self, ui: &mut Ui, value: EditStyle) {
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
    pub fn set_focus(self, ui: &mut Ui) -> bool {
        ui.set_focus(self.id)
    }
}
