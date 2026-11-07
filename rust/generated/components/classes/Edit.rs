
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
    pub fn char_case(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "char_case"))
    }
    pub fn set_char_case(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "char_case", Into::<String>::into(value))
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
    pub fn h_align(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "h_align"))
    }
    pub fn set_h_align(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "h_align", Into::<String>::into(value))
    }
    pub fn v_align(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "v_align"))
    }
    pub fn set_v_align(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "v_align", Into::<String>::into(value))
    }
    pub fn wrap(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "wrap").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_wrap(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "wrap", value)
    }
    pub fn trimming(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "trimming"))
    }
    pub fn set_trimming(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "trimming", Into::<String>::into(value))
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
    pub fn rules(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "rules"))
    }
    pub fn set_rules(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "rules", Into::<String>::into(value))
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
    pub fn caret_color(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "caret_color"))
    }
    pub fn set_caret_color(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "caret_color", Into::<String>::into(value))
    }
    pub fn caret_width(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "caret_width").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_caret_width(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "caret_width", value)
    }
    pub fn caret_style(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "caret_style"))
    }
    pub fn set_caret_style(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "caret_style", Into::<String>::into(value))
    }
    pub fn caret_blink(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "caret_blink").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_caret_blink(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "caret_blink", value)
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
    pub fn align(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "align"))
    }
    pub fn set_align(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "align", Into::<String>::into(value))
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
    pub fn captures_wheel(self, ui: &Ui) -> bool {
        handles::read(ui, self.id, "captures_wheel").and_then(|v| v.to_bool().ok()).unwrap_or_default()
    }
    pub fn set_captures_wheel(self, ui: &mut Ui, value: bool) {
        handles::write(ui, self.id, "captures_wheel", value)
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
    pub fn drag_image(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "drag_image"))
    }
    pub fn set_drag_image(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "drag_image", Into::<String>::into(value))
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
    pub fn cursor(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "cursor"))
    }
    pub fn set_cursor(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "cursor", Into::<String>::into(value))
    }
    pub fn opacity(self, ui: &Ui) -> f64 {
        handles::read(ui, self.id, "opacity").and_then(|v| v.to_f64().ok()).unwrap_or_default()
    }
    pub fn set_opacity(self, ui: &mut Ui, value: f64) {
        handles::write(ui, self.id, "opacity", value)
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
    pub fn self_align(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "self_align"))
    }
    pub fn set_self_align(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "self_align", Into::<String>::into(value))
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
    pub fn shadow(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "shadow"))
    }
    pub fn set_shadow(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "shadow", Into::<String>::into(value))
    }
    pub fn style(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "style"))
    }
    pub fn set_style(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "style", Into::<String>::into(value))
    }
    pub fn theme(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "theme"))
    }
    pub fn set_theme(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "theme", Into::<String>::into(value))
    }
    pub fn fill(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "fill"))
    }
    pub fn set_fill(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "fill", Into::<String>::into(value))
    }
    pub fn border_color(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "border_color"))
    }
    pub fn set_border_color(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "border_color", Into::<String>::into(value))
    }
    pub fn text_color(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "text_color"))
    }
    pub fn set_text_color(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "text_color", Into::<String>::into(value))
    }
    pub fn accent_color(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "accent_color"))
    }
    pub fn set_accent_color(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "accent_color", Into::<String>::into(value))
    }
    pub fn success_color(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "success_color"))
    }
    pub fn set_success_color(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "success_color", Into::<String>::into(value))
    }
    pub fn warning_color(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "warning_color"))
    }
    pub fn set_warning_color(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "warning_color", Into::<String>::into(value))
    }
    pub fn danger_color(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "danger_color"))
    }
    pub fn set_danger_color(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "danger_color", Into::<String>::into(value))
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
    pub fn font_weight(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "font_weight"))
    }
    pub fn set_font_weight(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "font_weight", Into::<String>::into(value))
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
    pub fn popup_menu(self, ui: &Ui) -> String {
        handles::text(ui, handles::read(ui, self.id, "popup_menu"))
    }
    pub fn set_popup_menu(self, ui: &mut Ui, value: impl Into<String>) {
        handles::write(ui, self.id, "popup_menu", Into::<String>::into(value))
    }
    pub fn set_focus(self, ui: &mut Ui) -> bool {
        ui.set_focus(self.id)
    }
}
