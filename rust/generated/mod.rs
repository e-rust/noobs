//! O que a IDE do Aylis refaz a cada "Gerar código" (não edite): o acesso tipado aos
//! componentes, a struct de cada form e o registro. O código dos forms é seu, em
//! `src/forms/`.

use aylis::prelude::Ui;

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

#[path = "../src/forms/settings/general.rs"]
pub mod settings_general;

#[path = "../src/forms/settings/onrecord.rs"]
pub mod settings_onrecord;

#[path = "../src/forms/settings/files.rs"]
pub mod settings_files;

#[path = "../src/forms/settings/video.rs"]
pub mod settings_video;

#[path = "../src/forms/settings/audio.rs"]
pub mod settings_audio;

#[path = "../src/forms/settings/replay.rs"]
pub mod settings_replay;

#[path = "../src/forms/settings/hotkeys.rs"]
pub mod settings_hotkeys;

#[path = "../src/forms/settings/devices.rs"]
pub mod settings_devices;

#[path = "../src/forms/settings/transcribe.rs"]
pub mod settings_transcribe;

#[path = "../src/forms/settings/updates.rs"]
pub mod settings_updates;

#[path = "../src/forms/settings/about.rs"]
pub mod settings_about;

#[path = "../src/forms/picker/window_card.rs"]
pub mod picker_window_card;

#[path = "../src/forms/picker/area_button.rs"]
pub mod picker_area_button;

#[path = "../src/forms/main/source_row.rs"]
pub mod main_source_row;

#[path = "../src/forms/main/legend_row.rs"]
pub mod main_legend_row;

#[path = "../src/forms/library/group.rs"]
pub mod library_group;

#[path = "../src/forms/library/rec_card.rs"]
pub mod library_rec_card;

#[path = "../src/forms/library/folder_card.rs"]
pub mod library_folder_card;

#[path = "../src/forms/player/info_section.rs"]
pub mod player_info_section;

#[path = "../src/forms/player/info_row.rs"]
pub mod player_info_row;

#[path = "../src/forms/player/view_option.rs"]
pub mod player_view_option;

#[path = "../src/forms/player/track_card.rs"]
pub mod player_track_card;

#[path = "../src/forms/player/turn.rs"]
pub mod player_turn;

#[path = "../src/forms/player/speaker_edit.rs"]
pub mod player_speaker_edit;

#[path = "../src/forms/settings/parts/hidden_folder.rs"]
pub mod settings_parts_hidden_folder;

#[path = "../src/forms/settings/parts/device_group.rs"]
pub mod settings_parts_device_group;

#[path = "../src/forms/settings/parts/device_row.rs"]
pub mod settings_parts_device_row;

#[path = "../src/forms/settings/parts/queue_row.rs"]
pub mod settings_parts_queue_row;

#[path = "../src/forms/settings/parts/about_item.rs"]
pub mod settings_parts_about_item;

#[path = "../src/forms/settings/parts/hotkey_card.rs"]
pub mod settings_parts_hotkey_card;

#[path = "../src/forms/settings/parts/vt_level.rs"]
pub mod settings_parts_vt_level;

#[path = "../src/forms/settings/parts/slider_tick.rs"]
pub mod settings_parts_slider_tick;

#[path = "../src/forms/export/pill.rs"]
pub mod export_pill;

#[path = "../src/forms/export/tick.rs"]
pub mod export_tick;

#[path = "../src/forms/export/part_label.rs"]
pub mod export_part_label;

#[path = "../src/forms/view/confirm.rs"]
pub mod view_confirm;

/// Registra as classes dos forms (cada form aberto ganha a instância dele), os handlers e
/// as classes dos pacotes de código.
pub fn register(ui: &mut Ui) {
    ui.register_form_class("frmMain", FrmMain::bind);
    ui.register_handler("frmMain.main_window_created", aylis::prelude::instances::form_handler(FrmMain::main_window_created));
    ui.register_handler("frmMain.close_query", aylis::prelude::instances::form_handler(FrmMain::close_query));
    ui.register_handler("frmMain.update_close", aylis::prelude::instances::form_handler(FrmMain::update_close));
    ui.register_handler("frmMain.update_open", aylis::prelude::instances::form_handler(FrmMain::update_open));
    ui.register_handler("frmMain.settings_click", aylis::prelude::instances::form_handler(FrmMain::settings_click));
    ui.register_handler("frmMain.record_click", aylis::prelude::instances::form_handler(FrmMain::record_click));
    ui.register_handler("frmMain.replay_click", aylis::prelude::instances::form_handler(FrmMain::replay_click));
    ui.register_handler("frmMain.pause_click", aylis::prelude::instances::form_handler(FrmMain::pause_click));
    ui.register_handler("frmMain.capture_click", aylis::prelude::instances::form_handler(FrmMain::capture_click));
    ui.register_handler("frmMain.replay_switch", aylis::prelude::instances::form_handler(FrmMain::replay_switch));
    ui.register_handler("frmMain.replay_save_click", aylis::prelude::instances::form_handler(FrmMain::replay_save_click));
    ui.register_handler("frmMain.display_change", aylis::prelude::instances::form_handler(FrmMain::display_change));
    ui.register_handler("frmMain.library_background", aylis::prelude::instances::form_handler(FrmMain::library_background));
    ui.register_handler("frmMain.library_resize", aylis::prelude::instances::form_handler(FrmMain::library_resize));
    ui.register_handler("frmMain.library_search", aylis::prelude::instances::form_handler(FrmMain::library_search));
    ui.register_handler("frmMain.library_open_dir", aylis::prelude::instances::form_handler(FrmMain::library_open_dir));
    ui.register_handler("frmMain.library_new_folder", aylis::prelude::instances::form_handler(FrmMain::library_new_folder));
    ui.register_handler("frmMain.library_merge", aylis::prelude::instances::form_handler(FrmMain::library_merge));
    ui.register_handler("frmMain.library_export", aylis::prelude::instances::form_handler(FrmMain::library_export));
    ui.register_handler("frmMain.library_delete", aylis::prelude::instances::form_handler(FrmMain::library_delete));
    ui.register_handler("frmMain.library_trash_drop", aylis::prelude::instances::form_handler(FrmMain::library_trash_drop));
    ui.register_handler("frmMain.library_crumb", aylis::prelude::instances::form_handler(FrmMain::library_crumb));
    ui.register_handler("frmMain.library_menu", aylis::prelude::instances::form_handler(FrmMain::library_menu));
    ui.register_handler("frmMain.main_ripple_frame", aylis::prelude::instances::form_handler(FrmMain::main_ripple_frame));
    ui.register_handler("frmMain.main_countdown_frame", aylis::prelude::instances::form_handler(FrmMain::main_countdown_frame));
    ui.register_form_class("frmSettings", FrmSettings::bind);
    ui.register_handler("frmSettings.settings_close", aylis::prelude::instances::form_handler(FrmSettings::settings_close));
    ui.register_handler("frmSettings.settings_tab", aylis::prelude::instances::form_handler(FrmSettings::settings_tab));
    ui.register_handler("frmSettings.settings_resize", aylis::prelude::instances::form_handler(FrmSettings::settings_resize));
    ui.register_form_class("frmPlayer", FrmPlayer::bind);
    ui.register_handler("frmPlayer.player_side_close", aylis::prelude::instances::form_handler(FrmPlayer::player_side_close));
    ui.register_handler("frmPlayer.player_tr_search", aylis::prelude::instances::form_handler(FrmPlayer::player_tr_search));
    ui.register_handler("frmPlayer.player_side_scroll", aylis::prelude::instances::form_handler(FrmPlayer::player_side_scroll));
    ui.register_handler("frmPlayer.player_stage_resize", aylis::prelude::instances::form_handler(FrmPlayer::player_stage_resize));
    ui.register_handler("frmPlayer.player_close", aylis::prelude::instances::form_handler(FrmPlayer::player_close));
    ui.register_handler("frmPlayer.player_play", aylis::prelude::instances::form_handler(FrmPlayer::player_play));
    ui.register_handler("frmPlayer.player_fullscreen", aylis::prelude::instances::form_handler(FrmPlayer::player_fullscreen));
    ui.register_handler("frmPlayer.player_info", aylis::prelude::instances::form_handler(FrmPlayer::player_info));
    ui.register_handler("frmPlayer.player_captions", aylis::prelude::instances::form_handler(FrmPlayer::player_captions));
    ui.register_handler("frmPlayer.player_speed", aylis::prelude::instances::form_handler(FrmPlayer::player_speed));
    ui.register_handler("frmPlayer.player_mute", aylis::prelude::instances::form_handler(FrmPlayer::player_mute));
    ui.register_handler("frmPlayer.player_seek", aylis::prelude::instances::form_handler(FrmPlayer::player_seek));
    ui.register_handler("frmPlayer.player_volume", aylis::prelude::instances::form_handler(FrmPlayer::player_volume));
    ui.register_handler("frmPlayer.player_transcript", aylis::prelude::instances::form_handler(FrmPlayer::player_transcript));
    ui.register_handler("frmPlayer.player_flash_frame", aylis::prelude::instances::form_handler(FrmPlayer::player_flash_frame));
    ui.register_form_class("frmExport", FrmExport::bind);
    ui.register_handler("frmExport.export_close", aylis::prelude::instances::form_handler(FrmExport::export_close));
    ui.register_handler("frmExport.export_resize", aylis::prelude::instances::form_handler(FrmExport::export_resize));
    ui.register_handler("frmExport.export_stage_down", aylis::prelude::instances::form_handler(FrmExport::export_stage_down));
    ui.register_handler("frmExport.export_stage_move", aylis::prelude::instances::form_handler(FrmExport::export_stage_move));
    ui.register_handler("frmExport.export_stage_up", aylis::prelude::instances::form_handler(FrmExport::export_stage_up));
    ui.register_handler("frmExport.export_stage_wheel", aylis::prelude::instances::form_handler(FrmExport::export_stage_wheel));
    ui.register_handler("frmExport.export_crop_change", aylis::prelude::instances::form_handler(FrmExport::export_crop_change));
    ui.register_handler("frmExport.export_crop_end", aylis::prelude::instances::form_handler(FrmExport::export_crop_end));
    ui.register_handler("frmExport.export_play", aylis::prelude::instances::form_handler(FrmExport::export_play));
    ui.register_handler("frmExport.export_crop_reset", aylis::prelude::instances::form_handler(FrmExport::export_crop_reset));
    ui.register_handler("frmExport.export_crop_zoom_out", aylis::prelude::instances::form_handler(FrmExport::export_crop_zoom_out));
    ui.register_handler("frmExport.export_crop_zoom_in", aylis::prelude::instances::form_handler(FrmExport::export_crop_zoom_in));
    ui.register_handler("frmExport.export_crop_zoom_reset", aylis::prelude::instances::form_handler(FrmExport::export_crop_zoom_reset));
    ui.register_handler("frmExport.export_mute", aylis::prelude::instances::form_handler(FrmExport::export_mute));
    ui.register_handler("frmExport.export_volume", aylis::prelude::instances::form_handler(FrmExport::export_volume));
    ui.register_handler("frmExport.export_cc", aylis::prelude::instances::form_handler(FrmExport::export_cc));
    ui.register_handler("frmExport.export_tl_down", aylis::prelude::instances::form_handler(FrmExport::export_tl_down));
    ui.register_handler("frmExport.export_tl_move", aylis::prelude::instances::form_handler(FrmExport::export_tl_move));
    ui.register_handler("frmExport.export_tl_up", aylis::prelude::instances::form_handler(FrmExport::export_tl_up));
    ui.register_handler("frmExport.export_tl_wheel", aylis::prelude::instances::form_handler(FrmExport::export_tl_wheel));
    ui.register_handler("frmExport.export_sb_down", aylis::prelude::instances::form_handler(FrmExport::export_sb_down));
    ui.register_handler("frmExport.export_sb_move", aylis::prelude::instances::form_handler(FrmExport::export_sb_move));
    ui.register_handler("frmExport.export_sb_up", aylis::prelude::instances::form_handler(FrmExport::export_sb_up));
    ui.register_handler("frmExport.export_cut", aylis::prelude::instances::form_handler(FrmExport::export_cut));
    ui.register_handler("frmExport.export_uncut", aylis::prelude::instances::form_handler(FrmExport::export_uncut));
    ui.register_handler("frmExport.export_toggle", aylis::prelude::instances::form_handler(FrmExport::export_toggle));
    ui.register_handler("frmExport.export_tl_zoom_out", aylis::prelude::instances::form_handler(FrmExport::export_tl_zoom_out));
    ui.register_handler("frmExport.export_tl_zoom_in", aylis::prelude::instances::form_handler(FrmExport::export_tl_zoom_in));
    ui.register_handler("frmExport.export_time_exit", aylis::prelude::instances::form_handler(FrmExport::export_time_exit));
    ui.register_handler("frmExport.export_time_key", aylis::prelude::instances::form_handler(FrmExport::export_time_key));
    ui.register_handler("frmExport.export_orig", aylis::prelude::instances::form_handler(FrmExport::export_orig));
    ui.register_handler("frmExport.export_mix", aylis::prelude::instances::form_handler(FrmExport::export_mix));
    ui.register_handler("frmExport.export_res", aylis::prelude::instances::form_handler(FrmExport::export_res));
    ui.register_handler("frmExport.export_enc", aylis::prelude::instances::form_handler(FrmExport::export_enc));
    ui.register_handler("frmExport.export_scale", aylis::prelude::instances::form_handler(FrmExport::export_scale));
    ui.register_handler("frmExport.export_speed_change", aylis::prelude::instances::form_handler(FrmExport::export_speed_change));
    ui.register_handler("frmExport.export_speed_exit", aylis::prelude::instances::form_handler(FrmExport::export_speed_exit));
    ui.register_handler("frmExport.export_speed_key", aylis::prelude::instances::form_handler(FrmExport::export_speed_key));
    ui.register_handler("frmExport.export_fps", aylis::prelude::instances::form_handler(FrmExport::export_fps));
    ui.register_handler("frmExport.export_quality", aylis::prelude::instances::form_handler(FrmExport::export_quality));
    ui.register_handler("frmExport.export_burn", aylis::prelude::instances::form_handler(FrmExport::export_burn));
    ui.register_handler("frmExport.export_cont", aylis::prelude::instances::form_handler(FrmExport::export_cont));
    ui.register_handler("frmExport.export_run", aylis::prelude::instances::form_handler(FrmExport::export_run));
    ui.register_handler("frmExport.export_preview", aylis::prelude::instances::form_handler(FrmExport::export_preview));
    ui.register_handler("frmExport.export_prevsec", aylis::prelude::instances::form_handler(FrmExport::export_prevsec));
    ui.register_form_class("frmPicker", FrmPicker::bind);
    ui.register_handler("frmPicker.picker_close", aylis::prelude::instances::form_handler(FrmPicker::picker_close));
    ui.register_handler("frmPicker.picker_refresh", aylis::prelude::instances::form_handler(FrmPicker::picker_refresh));
    ui.register_handler("frmPicker.picker_mode", aylis::prelude::instances::form_handler(FrmPicker::picker_mode));
    ui.register_handler("frmPicker.picker_audio", aylis::prelude::instances::form_handler(FrmPicker::picker_audio));
    ui.register_handler("frmPicker.picker_search", aylis::prelude::instances::form_handler(FrmPicker::picker_search));
    ui.register_handler("frmPicker.picker_timelapse", aylis::prelude::instances::form_handler(FrmPicker::picker_timelapse));
    ui.register_handler("frmPicker.picker_timelapse_cursor", aylis::prelude::instances::form_handler(FrmPicker::picker_timelapse_cursor));
    ui.register_handler("frmPicker.picker_select_area", aylis::prelude::instances::form_handler(FrmPicker::picker_select_area));
    ui.register_form_class("fraGeneral", FraGeneral::bind);
    ui.register_handler("fraGeneral.general_theme", aylis::prelude::instances::form_handler(FraGeneral::general_theme));
    ui.register_handler("fraGeneral.general_language", aylis::prelude::instances::form_handler(FraGeneral::general_language));
    ui.register_handler("fraGeneral.general_window_title", aylis::prelude::instances::form_handler(FraGeneral::general_window_title));
    ui.register_handler("fraGeneral.general_autostart", aylis::prelude::instances::form_handler(FraGeneral::general_autostart));
    ui.register_handler("fraGeneral.general_run_as_admin", aylis::prelude::instances::form_handler(FraGeneral::general_run_as_admin));
    ui.register_handler("fraGeneral.general_close_to_tray", aylis::prelude::instances::form_handler(FraGeneral::general_close_to_tray));
    ui.register_handler("fraGeneral.general_hibernate", aylis::prelude::instances::form_handler(FraGeneral::general_hibernate));
    ui.register_handler("fraGeneral.general_reset", aylis::prelude::instances::form_handler(FraGeneral::general_reset));
    ui.register_form_class("fraOnRecord", FraOnRecord::bind);
    ui.register_handler("fraOnRecord.onrecord_rec_indicator", aylis::prelude::instances::form_handler(FraOnRecord::onrecord_rec_indicator));
    ui.register_handler("fraOnRecord.onrecord_rec_clickable", aylis::prelude::instances::form_handler(FraOnRecord::onrecord_rec_clickable));
    ui.register_handler("fraOnRecord.onrecord_rec_corner", aylis::prelude::instances::form_handler(FraOnRecord::onrecord_rec_corner));
    ui.register_handler("fraOnRecord.onrecord_rec_opacity", aylis::prelude::instances::form_handler(FraOnRecord::onrecord_rec_opacity));
    ui.register_handler("fraOnRecord.onrecord_fullscreen", aylis::prelude::instances::form_handler(FraOnRecord::onrecord_fullscreen));
    ui.register_handler("fraOnRecord.onrecord_play_sound", aylis::prelude::instances::form_handler(FraOnRecord::onrecord_play_sound));
    ui.register_handler("fraOnRecord.onrecord_minimize", aylis::prelude::instances::form_handler(FraOnRecord::onrecord_minimize));
    ui.register_handler("fraOnRecord.onrecord_auto_record", aylis::prelude::instances::form_handler(FraOnRecord::onrecord_auto_record));
    ui.register_handler("fraOnRecord.onrecord_auto_apps", aylis::prelude::instances::form_handler(FraOnRecord::onrecord_auto_apps));
    ui.register_handler("fraOnRecord.onrecord_auto_except", aylis::prelude::instances::form_handler(FraOnRecord::onrecord_auto_except));
    ui.register_handler("fraOnRecord.onrecord_stop_on_lock", aylis::prelude::instances::form_handler(FraOnRecord::onrecord_stop_on_lock));
    ui.register_form_class("fraFiles", FraFiles::bind);
    ui.register_handler("fraFiles.files_browse", aylis::prelude::instances::form_handler(FraFiles::files_browse));
    ui.register_handler("fraFiles.files_record_dir", aylis::prelude::instances::form_handler(FraFiles::files_record_dir));
    ui.register_handler("fraFiles.files_filename_edit", aylis::prelude::instances::form_handler(FraFiles::files_filename_edit));
    ui.register_handler("fraFiles.files_filename", aylis::prelude::instances::form_handler(FraFiles::files_filename));
    ui.register_handler("fraFiles.files_library", aylis::prelude::instances::form_handler(FraFiles::files_library));
    ui.register_form_class("fraVideo", FraVideo::bind);
    ui.register_handler("fraVideo.video_codec", aylis::prelude::instances::form_handler(FraVideo::video_codec));
    ui.register_handler("fraVideo.video_quality", aylis::prelude::instances::form_handler(FraVideo::video_quality));
    ui.register_handler("fraVideo.video_test_run", aylis::prelude::instances::form_handler(FraVideo::video_test_run));
    ui.register_handler("fraVideo.video_test_cancel", aylis::prelude::instances::form_handler(FraVideo::video_test_cancel));
    ui.register_handler("fraVideo.video_test_swap", aylis::prelude::instances::form_handler(FraVideo::video_test_swap));
    ui.register_handler("fraVideo.video_test_zoom", aylis::prelude::instances::form_handler(FraVideo::video_test_zoom));
    ui.register_handler("fraVideo.video_stage_down", aylis::prelude::instances::form_handler(FraVideo::video_stage_down));
    ui.register_handler("fraVideo.video_stage_move", aylis::prelude::instances::form_handler(FraVideo::video_stage_move));
    ui.register_handler("fraVideo.video_stage_up", aylis::prelude::instances::form_handler(FraVideo::video_stage_up));
    ui.register_handler("fraVideo.video_test_use", aylis::prelude::instances::form_handler(FraVideo::video_test_use));
    ui.register_handler("fraVideo.video_fps", aylis::prelude::instances::form_handler(FraVideo::video_fps));
    ui.register_handler("fraVideo.video_keyframe", aylis::prelude::instances::form_handler(FraVideo::video_keyframe));
    ui.register_form_class("fraAudio", FraAudio::bind);
    ui.register_handler("fraAudio.audio_bitrate", aylis::prelude::instances::form_handler(FraAudio::audio_bitrate));
    ui.register_handler("fraAudio.audio_mute_when_muted", aylis::prelude::instances::form_handler(FraAudio::audio_mute_when_muted));
    ui.register_form_class("fraReplay", FraReplay::bind);
    ui.register_handler("fraReplay.replay_auto_start", aylis::prelude::instances::form_handler(FraReplay::replay_auto_start));
    ui.register_handler("fraReplay.replay_into_recording", aylis::prelude::instances::form_handler(FraReplay::replay_into_recording));
    ui.register_handler("fraReplay.replay_seconds", aylis::prelude::instances::form_handler(FraReplay::replay_seconds));
    ui.register_handler("fraReplay.replay_memory", aylis::prelude::instances::form_handler(FraReplay::replay_memory));
    ui.register_handler("fraReplay.replay_apps_edit", aylis::prelude::instances::form_handler(FraReplay::replay_apps_edit));
    ui.register_handler("fraReplay.replay_apps", aylis::prelude::instances::form_handler(FraReplay::replay_apps));
    ui.register_handler("fraReplay.replay_indicator", aylis::prelude::instances::form_handler(FraReplay::replay_indicator));
    ui.register_handler("fraReplay.replay_clickable", aylis::prelude::instances::form_handler(FraReplay::replay_clickable));
    ui.register_handler("fraReplay.replay_corner", aylis::prelude::instances::form_handler(FraReplay::replay_corner));
    ui.register_handler("fraReplay.replay_opacity", aylis::prelude::instances::form_handler(FraReplay::replay_opacity));
    ui.register_form_class("fraHotkeys", FraHotkeys::bind);
    ui.register_handler("fraHotkeys.hotkeys_add", aylis::prelude::instances::form_handler(FraHotkeys::hotkeys_add));
    ui.register_form_class("fraDevices", FraDevices::bind);
    ui.register_form_class("fraTranscribe", FraTranscribe::bind);
    ui.register_handler("fraTranscribe.transcribe_engine", aylis::prelude::instances::form_handler(FraTranscribe::transcribe_engine));
    ui.register_handler("fraTranscribe.transcribe_language", aylis::prelude::instances::form_handler(FraTranscribe::transcribe_language));
    ui.register_handler("fraTranscribe.transcribe_on_stop", aylis::prelude::instances::form_handler(FraTranscribe::transcribe_on_stop));
    ui.register_handler("fraTranscribe.transcribe_pending", aylis::prelude::instances::form_handler(FraTranscribe::transcribe_pending));
    ui.register_handler("fraTranscribe.transcribe_cancel", aylis::prelude::instances::form_handler(FraTranscribe::transcribe_cancel));
    ui.register_form_class("fraUpdates", FraUpdates::bind);
    ui.register_handler("fraUpdates.updates_auto", aylis::prelude::instances::form_handler(FraUpdates::updates_auto));
    ui.register_handler("fraUpdates.updates_check_now", aylis::prelude::instances::form_handler(FraUpdates::updates_check_now));
    ui.register_handler("fraUpdates.updates_open_page", aylis::prelude::instances::form_handler(FraUpdates::updates_open_page));
    ui.register_form_class("fraAbout", FraAbout::bind);
    ui.register_handler("fraAbout.about_repository", aylis::prelude::instances::form_handler(FraAbout::about_repository));
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
