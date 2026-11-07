//! O que a IDE do Iron refaz a cada "Gerar código" (não edite): o acesso tipado aos
//! componentes, a struct de cada form e o registro. O código dos forms é seu, em
//! `src/forms/`.

use iron::prelude::Ui;
use iron::prelude::instances::form_handler;

pub mod components;
pub use components::*;

#[path = "../src/forms/main.rs"]
pub mod main;

#[path = "../src/forms/settings.rs"]
pub mod settings;

#[path = "../src/forms/player.rs"]
pub mod player;

#[path = "../src/forms/export.rs"]
pub mod export;

#[path = "../src/forms/picker.rs"]
pub mod picker;

#[path = "../src/forms/general.rs"]
pub mod general;

#[path = "../src/forms/onrecord.rs"]
pub mod onrecord;

#[path = "../src/forms/files.rs"]
pub mod files;

#[path = "../src/forms/video.rs"]
pub mod video;

#[path = "../src/forms/audio.rs"]
pub mod audio;

#[path = "../src/forms/replay.rs"]
pub mod replay;

#[path = "../src/forms/hotkeys.rs"]
pub mod hotkeys;

#[path = "../src/forms/devices.rs"]
pub mod devices;

#[path = "../src/forms/transcribe.rs"]
pub mod transcribe;

#[path = "../src/forms/updates.rs"]
pub mod updates;

#[path = "../src/forms/about.rs"]
pub mod about;

#[path = "../src/forms/window_card.rs"]
pub mod window_card;

#[path = "../src/forms/area_button.rs"]
pub mod area_button;

#[path = "../src/forms/source_row.rs"]
pub mod source_row;

#[path = "../src/forms/legend_row.rs"]
pub mod legend_row;

#[path = "../src/forms/group.rs"]
pub mod group;

#[path = "../src/forms/rec_card.rs"]
pub mod rec_card;

#[path = "../src/forms/folder_card.rs"]
pub mod folder_card;

#[path = "../src/forms/info_section.rs"]
pub mod info_section;

#[path = "../src/forms/info_row.rs"]
pub mod info_row;

#[path = "../src/forms/view_option.rs"]
pub mod view_option;

#[path = "../src/forms/track_card.rs"]
pub mod track_card;

#[path = "../src/forms/turn.rs"]
pub mod turn;

#[path = "../src/forms/speaker_edit.rs"]
pub mod speaker_edit;

#[path = "../src/forms/hidden_folder.rs"]
pub mod hidden_folder;

#[path = "../src/forms/device_group.rs"]
pub mod device_group;

#[path = "../src/forms/device_row.rs"]
pub mod device_row;

#[path = "../src/forms/queue_row.rs"]
pub mod queue_row;

#[path = "../src/forms/about_item.rs"]
pub mod about_item;

#[path = "../src/forms/hotkey_card.rs"]
pub mod hotkey_card;

#[path = "../src/forms/vt_level.rs"]
pub mod vt_level;

#[path = "../src/forms/slider_tick.rs"]
pub mod slider_tick;

#[path = "../src/forms/pill.rs"]
pub mod pill;

#[path = "../src/forms/tick.rs"]
pub mod tick;

#[path = "../src/forms/part_label.rs"]
pub mod part_label;

#[path = "../src/forms/confirm.rs"]
pub mod confirm;

/// Registra as classes dos forms (cada form aberto ganha a instância dele), os handlers e
/// as classes dos pacotes de código.
pub fn register(ui: &mut Ui) {
    ui.register_form_class("frmMain", FrmMain::bind);
    ui.register_handler("frmMain.close_query", form_handler(FrmMain::close_query));
    ui.register_handler("frmMain.update_close", form_handler(FrmMain::update_close));
    ui.register_handler("frmMain.update_open", form_handler(FrmMain::update_open));
    ui.register_handler("frmMain.settings_click", form_handler(FrmMain::settings_click));
    ui.register_handler("frmMain.record_click", form_handler(FrmMain::record_click));
    ui.register_handler("frmMain.replay_click", form_handler(FrmMain::replay_click));
    ui.register_handler("frmMain.pause_click", form_handler(FrmMain::pause_click));
    ui.register_handler("frmMain.capture_click", form_handler(FrmMain::capture_click));
    ui.register_handler("frmMain.replay_switch", form_handler(FrmMain::replay_switch));
    ui.register_handler("frmMain.replay_save_click", form_handler(FrmMain::replay_save_click));
    ui.register_handler("frmMain.display_change", form_handler(FrmMain::display_change));
    ui.register_handler("frmMain.library_resize", form_handler(FrmMain::library_resize));
    ui.register_handler("frmMain.library_background", form_handler(FrmMain::library_background));
    ui.register_handler("frmMain.library_search", form_handler(FrmMain::library_search));
    ui.register_handler("frmMain.library_open_dir", form_handler(FrmMain::library_open_dir));
    ui.register_handler("frmMain.library_new_folder", form_handler(FrmMain::library_new_folder));
    ui.register_handler("frmMain.library_merge", form_handler(FrmMain::library_merge));
    ui.register_handler("frmMain.library_export", form_handler(FrmMain::library_export));
    ui.register_handler("frmMain.library_delete", form_handler(FrmMain::library_delete));
    ui.register_handler("frmMain.library_trash_drop", form_handler(FrmMain::library_trash_drop));
    ui.register_handler("frmMain.library_crumb", form_handler(FrmMain::library_crumb));
    ui.register_handler("frmMain.library_menu", form_handler(FrmMain::library_menu));
    ui.register_form_class("frmSettings", FrmSettings::bind);
    ui.register_handler("frmSettings.settings_close", form_handler(FrmSettings::settings_close));
    ui.register_handler("frmSettings.settings_tab", form_handler(FrmSettings::settings_tab));
    ui.register_handler("frmSettings.settings_resize", form_handler(FrmSettings::settings_resize));
    ui.register_form_class("frmPlayer", FrmPlayer::bind);
    ui.register_handler("frmPlayer.player_side_close", form_handler(FrmPlayer::player_side_close));
    ui.register_handler("frmPlayer.player_tr_search", form_handler(FrmPlayer::player_tr_search));
    ui.register_handler("frmPlayer.player_side_scroll", form_handler(FrmPlayer::player_side_scroll));
    ui.register_handler("frmPlayer.player_stage_resize", form_handler(FrmPlayer::player_stage_resize));
    ui.register_handler("frmPlayer.player_close", form_handler(FrmPlayer::player_close));
    ui.register_handler("frmPlayer.player_play", form_handler(FrmPlayer::player_play));
    ui.register_handler("frmPlayer.player_fullscreen", form_handler(FrmPlayer::player_fullscreen));
    ui.register_handler("frmPlayer.player_info", form_handler(FrmPlayer::player_info));
    ui.register_handler("frmPlayer.player_captions", form_handler(FrmPlayer::player_captions));
    ui.register_handler("frmPlayer.player_speed", form_handler(FrmPlayer::player_speed));
    ui.register_handler("frmPlayer.player_mute", form_handler(FrmPlayer::player_mute));
    ui.register_handler("frmPlayer.player_seek", form_handler(FrmPlayer::player_seek));
    ui.register_handler("frmPlayer.player_volume", form_handler(FrmPlayer::player_volume));
    ui.register_handler("frmPlayer.player_transcript", form_handler(FrmPlayer::player_transcript));
    ui.register_form_class("frmExport", FrmExport::bind);
    ui.register_handler("frmExport.export_close", form_handler(FrmExport::export_close));
    ui.register_handler("frmExport.export_resize", form_handler(FrmExport::export_resize));
    ui.register_handler("frmExport.export_stage_down", form_handler(FrmExport::export_stage_down));
    ui.register_handler("frmExport.export_stage_move", form_handler(FrmExport::export_stage_move));
    ui.register_handler("frmExport.export_stage_up", form_handler(FrmExport::export_stage_up));
    ui.register_handler("frmExport.export_stage_wheel", form_handler(FrmExport::export_stage_wheel));
    ui.register_handler("frmExport.export_crop_change", form_handler(FrmExport::export_crop_change));
    ui.register_handler("frmExport.export_crop_end", form_handler(FrmExport::export_crop_end));
    ui.register_handler("frmExport.export_play", form_handler(FrmExport::export_play));
    ui.register_handler("frmExport.export_crop_reset", form_handler(FrmExport::export_crop_reset));
    ui.register_handler("frmExport.export_crop_zoom_out", form_handler(FrmExport::export_crop_zoom_out));
    ui.register_handler("frmExport.export_crop_zoom_in", form_handler(FrmExport::export_crop_zoom_in));
    ui.register_handler("frmExport.export_crop_zoom_reset", form_handler(FrmExport::export_crop_zoom_reset));
    ui.register_handler("frmExport.export_mute", form_handler(FrmExport::export_mute));
    ui.register_handler("frmExport.export_volume", form_handler(FrmExport::export_volume));
    ui.register_handler("frmExport.export_cc", form_handler(FrmExport::export_cc));
    ui.register_handler("frmExport.export_tl_down", form_handler(FrmExport::export_tl_down));
    ui.register_handler("frmExport.export_tl_move", form_handler(FrmExport::export_tl_move));
    ui.register_handler("frmExport.export_tl_up", form_handler(FrmExport::export_tl_up));
    ui.register_handler("frmExport.export_tl_wheel", form_handler(FrmExport::export_tl_wheel));
    ui.register_handler("frmExport.export_sb_down", form_handler(FrmExport::export_sb_down));
    ui.register_handler("frmExport.export_sb_move", form_handler(FrmExport::export_sb_move));
    ui.register_handler("frmExport.export_sb_up", form_handler(FrmExport::export_sb_up));
    ui.register_handler("frmExport.export_cut", form_handler(FrmExport::export_cut));
    ui.register_handler("frmExport.export_uncut", form_handler(FrmExport::export_uncut));
    ui.register_handler("frmExport.export_toggle", form_handler(FrmExport::export_toggle));
    ui.register_handler("frmExport.export_tl_zoom_out", form_handler(FrmExport::export_tl_zoom_out));
    ui.register_handler("frmExport.export_tl_zoom_in", form_handler(FrmExport::export_tl_zoom_in));
    ui.register_handler("frmExport.export_time_exit", form_handler(FrmExport::export_time_exit));
    ui.register_handler("frmExport.export_time_key", form_handler(FrmExport::export_time_key));
    ui.register_handler("frmExport.export_orig", form_handler(FrmExport::export_orig));
    ui.register_handler("frmExport.export_mix", form_handler(FrmExport::export_mix));
    ui.register_handler("frmExport.export_res", form_handler(FrmExport::export_res));
    ui.register_handler("frmExport.export_enc", form_handler(FrmExport::export_enc));
    ui.register_handler("frmExport.export_scale", form_handler(FrmExport::export_scale));
    ui.register_handler("frmExport.export_speed_change", form_handler(FrmExport::export_speed_change));
    ui.register_handler("frmExport.export_speed_exit", form_handler(FrmExport::export_speed_exit));
    ui.register_handler("frmExport.export_speed_key", form_handler(FrmExport::export_speed_key));
    ui.register_handler("frmExport.export_fps", form_handler(FrmExport::export_fps));
    ui.register_handler("frmExport.export_quality", form_handler(FrmExport::export_quality));
    ui.register_handler("frmExport.export_burn", form_handler(FrmExport::export_burn));
    ui.register_handler("frmExport.export_cont", form_handler(FrmExport::export_cont));
    ui.register_handler("frmExport.export_run", form_handler(FrmExport::export_run));
    ui.register_handler("frmExport.export_preview", form_handler(FrmExport::export_preview));
    ui.register_handler("frmExport.export_prevsec", form_handler(FrmExport::export_prevsec));
    ui.register_form_class("frmPicker", FrmPicker::bind);
    ui.register_handler("frmPicker.picker_close", form_handler(FrmPicker::picker_close));
    ui.register_handler("frmPicker.picker_refresh", form_handler(FrmPicker::picker_refresh));
    ui.register_handler("frmPicker.picker_mode", form_handler(FrmPicker::picker_mode));
    ui.register_handler("frmPicker.picker_audio", form_handler(FrmPicker::picker_audio));
    ui.register_handler("frmPicker.picker_search", form_handler(FrmPicker::picker_search));
    ui.register_handler("frmPicker.picker_timelapse", form_handler(FrmPicker::picker_timelapse));
    ui.register_handler("frmPicker.picker_timelapse_cursor", form_handler(FrmPicker::picker_timelapse_cursor));
    ui.register_handler("frmPicker.picker_select_area", form_handler(FrmPicker::picker_select_area));
    ui.register_form_class("fraGeneral", FraGeneral::bind);
    ui.register_handler("fraGeneral.general_theme", form_handler(FraGeneral::general_theme));
    ui.register_handler("fraGeneral.general_language", form_handler(FraGeneral::general_language));
    ui.register_handler("fraGeneral.general_window_title", form_handler(FraGeneral::general_window_title));
    ui.register_handler("fraGeneral.general_autostart", form_handler(FraGeneral::general_autostart));
    ui.register_handler("fraGeneral.general_run_as_admin", form_handler(FraGeneral::general_run_as_admin));
    ui.register_handler("fraGeneral.general_close_to_tray", form_handler(FraGeneral::general_close_to_tray));
    ui.register_handler("fraGeneral.general_hibernate", form_handler(FraGeneral::general_hibernate));
    ui.register_handler("fraGeneral.general_reset", form_handler(FraGeneral::general_reset));
    ui.register_form_class("fraOnRecord", FraOnRecord::bind);
    ui.register_handler("fraOnRecord.onrecord_rec_indicator", form_handler(FraOnRecord::onrecord_rec_indicator));
    ui.register_handler("fraOnRecord.onrecord_rec_clickable", form_handler(FraOnRecord::onrecord_rec_clickable));
    ui.register_handler("fraOnRecord.onrecord_rec_corner", form_handler(FraOnRecord::onrecord_rec_corner));
    ui.register_handler("fraOnRecord.onrecord_rec_opacity", form_handler(FraOnRecord::onrecord_rec_opacity));
    ui.register_handler("fraOnRecord.onrecord_fullscreen", form_handler(FraOnRecord::onrecord_fullscreen));
    ui.register_handler("fraOnRecord.onrecord_play_sound", form_handler(FraOnRecord::onrecord_play_sound));
    ui.register_handler("fraOnRecord.onrecord_minimize", form_handler(FraOnRecord::onrecord_minimize));
    ui.register_handler("fraOnRecord.onrecord_auto_record", form_handler(FraOnRecord::onrecord_auto_record));
    ui.register_handler("fraOnRecord.onrecord_auto_apps", form_handler(FraOnRecord::onrecord_auto_apps));
    ui.register_handler("fraOnRecord.onrecord_auto_except", form_handler(FraOnRecord::onrecord_auto_except));
    ui.register_handler("fraOnRecord.onrecord_stop_on_lock", form_handler(FraOnRecord::onrecord_stop_on_lock));
    ui.register_form_class("fraFiles", FraFiles::bind);
    ui.register_handler("fraFiles.files_browse", form_handler(FraFiles::files_browse));
    ui.register_handler("fraFiles.files_record_dir", form_handler(FraFiles::files_record_dir));
    ui.register_handler("fraFiles.files_filename_edit", form_handler(FraFiles::files_filename_edit));
    ui.register_handler("fraFiles.files_filename", form_handler(FraFiles::files_filename));
    ui.register_handler("fraFiles.files_library", form_handler(FraFiles::files_library));
    ui.register_form_class("fraVideo", FraVideo::bind);
    ui.register_handler("fraVideo.video_codec", form_handler(FraVideo::video_codec));
    ui.register_handler("fraVideo.video_quality", form_handler(FraVideo::video_quality));
    ui.register_handler("fraVideo.video_test_run", form_handler(FraVideo::video_test_run));
    ui.register_handler("fraVideo.video_test_cancel", form_handler(FraVideo::video_test_cancel));
    ui.register_handler("fraVideo.video_test_swap", form_handler(FraVideo::video_test_swap));
    ui.register_handler("fraVideo.video_test_zoom", form_handler(FraVideo::video_test_zoom));
    ui.register_handler("fraVideo.video_stage_down", form_handler(FraVideo::video_stage_down));
    ui.register_handler("fraVideo.video_stage_move", form_handler(FraVideo::video_stage_move));
    ui.register_handler("fraVideo.video_stage_up", form_handler(FraVideo::video_stage_up));
    ui.register_handler("fraVideo.video_test_use", form_handler(FraVideo::video_test_use));
    ui.register_handler("fraVideo.video_fps", form_handler(FraVideo::video_fps));
    ui.register_handler("fraVideo.video_keyframe", form_handler(FraVideo::video_keyframe));
    ui.register_form_class("fraAudio", FraAudio::bind);
    ui.register_handler("fraAudio.audio_bitrate", form_handler(FraAudio::audio_bitrate));
    ui.register_handler("fraAudio.audio_mute_when_muted", form_handler(FraAudio::audio_mute_when_muted));
    ui.register_form_class("fraReplay", FraReplay::bind);
    ui.register_handler("fraReplay.replay_auto_start", form_handler(FraReplay::replay_auto_start));
    ui.register_handler("fraReplay.replay_into_recording", form_handler(FraReplay::replay_into_recording));
    ui.register_handler("fraReplay.replay_seconds", form_handler(FraReplay::replay_seconds));
    ui.register_handler("fraReplay.replay_memory", form_handler(FraReplay::replay_memory));
    ui.register_handler("fraReplay.replay_apps_edit", form_handler(FraReplay::replay_apps_edit));
    ui.register_handler("fraReplay.replay_apps", form_handler(FraReplay::replay_apps));
    ui.register_handler("fraReplay.replay_indicator", form_handler(FraReplay::replay_indicator));
    ui.register_handler("fraReplay.replay_clickable", form_handler(FraReplay::replay_clickable));
    ui.register_handler("fraReplay.replay_corner", form_handler(FraReplay::replay_corner));
    ui.register_handler("fraReplay.replay_opacity", form_handler(FraReplay::replay_opacity));
    ui.register_form_class("fraHotkeys", FraHotkeys::bind);
    ui.register_handler("fraHotkeys.hotkeys_add", form_handler(FraHotkeys::hotkeys_add));
    ui.register_form_class("fraDevices", FraDevices::bind);
    ui.register_form_class("fraTranscribe", FraTranscribe::bind);
    ui.register_handler("fraTranscribe.transcribe_engine", form_handler(FraTranscribe::transcribe_engine));
    ui.register_handler("fraTranscribe.transcribe_language", form_handler(FraTranscribe::transcribe_language));
    ui.register_handler("fraTranscribe.transcribe_on_stop", form_handler(FraTranscribe::transcribe_on_stop));
    ui.register_handler("fraTranscribe.transcribe_pending", form_handler(FraTranscribe::transcribe_pending));
    ui.register_handler("fraTranscribe.transcribe_cancel", form_handler(FraTranscribe::transcribe_cancel));
    ui.register_form_class("fraUpdates", FraUpdates::bind);
    ui.register_handler("fraUpdates.updates_auto", form_handler(FraUpdates::updates_auto));
    ui.register_handler("fraUpdates.updates_check_now", form_handler(FraUpdates::updates_check_now));
    ui.register_handler("fraUpdates.updates_open_page", form_handler(FraUpdates::updates_open_page));
    ui.register_form_class("fraAbout", FraAbout::bind);
    ui.register_handler("fraAbout.about_repository", form_handler(FraAbout::about_repository));
    ui.register_form_class("fraWindowCard", FraWindowCard::bind);
    ui.register_form_class("fraAreaButton", FraAreaButton::bind);
    ui.register_form_class("fraSourceRow", FraSourceRow::bind);
    ui.register_form_class("fraLegendRow", FraLegendRow::bind);
    ui.register_form_class("fraLibGroup", FraLibGroup::bind);
    ui.register_form_class("fraRecCard", FraRecCard::bind);
    ui.register_form_class("fraFolderCard", FraFolderCard::bind);
    ui.register_form_class("fraInfoSection", FraInfoSection::bind);
    ui.register_form_class("fraInfoRow", FraInfoRow::bind);
    ui.register_form_class("fraViewOption", FraViewOption::bind);
    ui.register_form_class("fraTrackCard", FraTrackCard::bind);
    ui.register_form_class("fraTurn", FraTurn::bind);
    ui.register_form_class("fraSpeakerEdit", FraSpeakerEdit::bind);
    ui.register_form_class("fraHiddenFolder", FraHiddenFolder::bind);
    ui.register_form_class("fraDeviceGroup", FraDeviceGroup::bind);
    ui.register_form_class("fraDeviceRow", FraDeviceRow::bind);
    ui.register_form_class("fraQueueRow", FraQueueRow::bind);
    ui.register_form_class("fraAboutItem", FraAboutItem::bind);
    ui.register_form_class("fraHotkeyCard", FraHotkeyCard::bind);
    ui.register_form_class("fraVtLevel", FraVtLevel::bind);
    ui.register_form_class("fraSliderTick", FraSliderTick::bind);
    ui.register_form_class("fraExPill", FraExPill::bind);
    ui.register_form_class("fraExTick", FraExTick::bind);
    ui.register_form_class("fraExPartLabel", FraExPartLabel::bind);
    ui.register_form_class("fraConfirm", FraConfirm::bind);
}
