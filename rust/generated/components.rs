//! Os componentes dos forms com as propriedades tipadas e a struct de cada form, com os
//! componentes resolvidos ao abrir (`self.edtNome.set_text(ui, ...)`).
//! Gerado pela IDE do Aylis a cada "Gerar código": não edite.

#![allow(non_snake_case, dead_code, unused_imports)]

use aylis::prelude::handles::{self, Handle};
use aylis::prelude::{ComponentId, PropValue, Ui};

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ActivityIndicatorStyle {
    #[default]
    Default,
    Audio,
    Bars,
    Dots,
    Grid,
    Hourglass,
    Ios,
    Orbit,
    Puff,
    Tail,
}

impl aylis::prelude::PropEnum for ActivityIndicatorStyle {
    const NAMES: &'static [&'static str] = &["", "audio", "bars", "dots", "grid", "hourglass", "ios", "orbit", "puff", "tail"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Audio, Self::Bars, Self::Dots, Self::Grid, Self::Hourglass, Self::Ios, Self::Orbit, Self::Puff, Self::Tail].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum BreadcrumbStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for BreadcrumbStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ButtonStyle {
    #[default]
    Default,
    Accent,
    Danger,
    Elevated,
    Fab,
    FabExtended,
    Flat,
    Ghost,
    Glass,
    GlassProminent,
    Glossy,
    Icon,
    IconFilled,
    IconOutlined,
    IconTonal,
    Info,
    Link,
    Negative,
    Outline,
    Pill,
    Positive,
    Push,
    Reveal,
    Round,
    Rounded,
    Secondary,
    Success,
    Text,
    Tonal,
    Unelevated,
    Warning,
    CfBtn,
    CfDanger,
    Ex,
    ExCc,
    ExCcOn,
    ExPrimary,
    ExSm,
    ExZoom,
    Pl,
    PlCheck,
    PlClock,
    PlClose,
    PlDim,
    PlItem,
    PlItemOn,
    PlSideX,
    PlSpeed,
    PlTmute,
    PlTmuteOff,
    PlTrick,
    PlVicon,
    PlViconOn,
    S,
    SBack,
    SDanger,
    SOn,
    SPrimary,
    SSeg,
    SSegOn,
    SX,
    Tool,
}

impl aylis::prelude::PropEnum for ButtonStyle {
    const NAMES: &'static [&'static str] = &["", "accent", "danger", "elevated", "fab", "fab_extended", "flat", "ghost", "glass", "glass_prominent", "glossy", "icon", "icon_filled", "icon_outlined", "icon_tonal", "info", "link", "negative", "outline", "pill", "positive", "push", "reveal", "round", "rounded", "secondary", "success", "text", "tonal", "unelevated", "warning", "cf_btn", "cf_danger", "ex", "ex_cc", "ex_cc_on", "ex_primary", "ex_sm", "ex_zoom", "pl", "pl_check", "pl_clock", "pl_close", "pl_dim", "pl_item", "pl_item_on", "pl_side_x", "pl_speed", "pl_tmute", "pl_tmute_off", "pl_trick", "pl_vicon", "pl_vicon_on", "s", "s_back", "s_danger", "s_on", "s_primary", "s_seg", "s_seg_on", "s_x", "tool"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Accent, Self::Danger, Self::Elevated, Self::Fab, Self::FabExtended, Self::Flat, Self::Ghost, Self::Glass, Self::GlassProminent, Self::Glossy, Self::Icon, Self::IconFilled, Self::IconOutlined, Self::IconTonal, Self::Info, Self::Link, Self::Negative, Self::Outline, Self::Pill, Self::Positive, Self::Push, Self::Reveal, Self::Round, Self::Rounded, Self::Secondary, Self::Success, Self::Text, Self::Tonal, Self::Unelevated, Self::Warning, Self::CfBtn, Self::CfDanger, Self::Ex, Self::ExCc, Self::ExCcOn, Self::ExPrimary, Self::ExSm, Self::ExZoom, Self::Pl, Self::PlCheck, Self::PlClock, Self::PlClose, Self::PlDim, Self::PlItem, Self::PlItemOn, Self::PlSideX, Self::PlSpeed, Self::PlTmute, Self::PlTmuteOff, Self::PlTrick, Self::PlVicon, Self::PlViconOn, Self::S, Self::SBack, Self::SDanger, Self::SOn, Self::SPrimary, Self::SSeg, Self::SSegOn, Self::SX, Self::Tool].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CheckBoxStyle {
    #[default]
    Default,
    Round,
    Mod,
    Web,
}

impl aylis::prelude::PropEnum for CheckBoxStyle {
    const NAMES: &'static [&'static str] = &["", "round", "mod", "web"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Round, Self::Mod, Self::Web].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ChipStyle {
    #[default]
    Default,
    Danger,
    Filter,
    Input,
    Outline,
    Success,
    Warning,
}

impl aylis::prelude::PropEnum for ChipStyle {
    const NAMES: &'static [&'static str] = &["", "danger", "filter", "input", "outline", "success", "warning"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Danger, Self::Filter, Self::Input, Self::Outline, Self::Success, Self::Warning].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CircleStyle {
    #[default]
    Default,
    SDot,
}

impl aylis::prelude::PropEnum for CircleStyle {
    const NAMES: &'static [&'static str] = &["", "s_dot"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::SDot].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ComboBoxStyle {
    #[default]
    Default,
    Outlined,
    Ex,
}

impl aylis::prelude::PropEnum for ComboBoxStyle {
    const NAMES: &'static [&'static str] = &["", "outlined", "ex"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Outlined, Self::Ex].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CropBoxStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for CropBoxStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum EditStyle {
    #[default]
    Default,
    Filled,
    Flat,
    Outlined,
    Search,
    Ex,
    PlSearch,
    PlSpkEdit,
}

impl aylis::prelude::PropEnum for EditStyle {
    const NAMES: &'static [&'static str] = &["", "filled", "flat", "outlined", "search", "ex", "pl_search", "pl_spk_edit"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Filled, Self::Flat, Self::Outlined, Self::Search, Self::Ex, Self::PlSearch, Self::PlSpkEdit].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FlowLayoutStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for FlowLayoutStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FormName {
    #[default]
    Default,
    FrmMain,
    FrmSettings,
    FrmPlayer,
    FrmViewer,
    FrmExport,
    FrmPicker,
    FraGeneral,
    FraOnRecord,
    FraFiles,
    FraVideo,
    FraAudio,
    FraReplay,
    FraHotkeys,
    FraDevices,
    FraTranscribe,
    FraUpdates,
    FraAbout,
    FraWindowCard,
    FraAreaButton,
    FraSourceRow,
    FraLegendRow,
    FraLibGroup,
    FraRecCard,
    FraFolderCard,
    FraInfoSection,
    FraInfoRow,
    FraViewOption,
    FraTrackCard,
    FraTurn,
    FraSpeakerEdit,
    FraHiddenFolder,
    FraDeviceGroup,
    FraDeviceRow,
    FraQueueRow,
    FraAboutItem,
    FraHotkeyCard,
    FraVtLevel,
    FraSliderTick,
    FraExPill,
    FraExTick,
    FraExPartLabel,
    FraConfirm,
    FrmPrint,
}

impl aylis::prelude::PropEnum for FormName {
    const NAMES: &'static [&'static str] = &["", "frmMain", "frmSettings", "frmPlayer", "frmViewer", "frmExport", "frmPicker", "fraGeneral", "fraOnRecord", "fraFiles", "fraVideo", "fraAudio", "fraReplay", "fraHotkeys", "fraDevices", "fraTranscribe", "fraUpdates", "fraAbout", "fraWindowCard", "fraAreaButton", "fraSourceRow", "fraLegendRow", "fraLibGroup", "fraRecCard", "fraFolderCard", "fraInfoSection", "fraInfoRow", "fraViewOption", "fraTrackCard", "fraTurn", "fraSpeakerEdit", "fraHiddenFolder", "fraDeviceGroup", "fraDeviceRow", "fraQueueRow", "fraAboutItem", "fraHotkeyCard", "fraVtLevel", "fraSliderTick", "fraExPill", "fraExTick", "fraExPartLabel", "fraConfirm", "frmPrint"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::FrmMain, Self::FrmSettings, Self::FrmPlayer, Self::FrmViewer, Self::FrmExport, Self::FrmPicker, Self::FraGeneral, Self::FraOnRecord, Self::FraFiles, Self::FraVideo, Self::FraAudio, Self::FraReplay, Self::FraHotkeys, Self::FraDevices, Self::FraTranscribe, Self::FraUpdates, Self::FraAbout, Self::FraWindowCard, Self::FraAreaButton, Self::FraSourceRow, Self::FraLegendRow, Self::FraLibGroup, Self::FraRecCard, Self::FraFolderCard, Self::FraInfoSection, Self::FraInfoRow, Self::FraViewOption, Self::FraTrackCard, Self::FraTurn, Self::FraSpeakerEdit, Self::FraHiddenFolder, Self::FraDeviceGroup, Self::FraDeviceRow, Self::FraQueueRow, Self::FraAboutItem, Self::FraHotkeyCard, Self::FraVtLevel, Self::FraSliderTick, Self::FraExPill, Self::FraExTick, Self::FraExPartLabel, Self::FraConfirm, Self::FrmPrint].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FormStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for FormStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FrameStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for FrameStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FrameViewStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for FrameViewStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum IconStyle {
    #[default]
    Default,
    Muted,
}

impl aylis::prelude::PropEnum for IconStyle {
    const NAMES: &'static [&'static str] = &["", "muted"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Muted].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ImageStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for ImageStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LabelStyle {
    #[default]
    Default,
    Caption,
    Error,
    Heading,
    Kbd,
    Link,
    Mono,
    Muted,
    Title,
    Action,
    Body,
    CfMsg,
    CfTitle,
    Count,
    ExBadge,
    ExChk,
    ExCol,
    ExHint,
    ExLabel,
    ExTick,
    ExTitle,
    ExVal,
    ExVal2,
    ExX,
    H2,
    Key,
    Legend,
    LegendTrack,
    Meta,
    PlBadge,
    PlCaption,
    PlClock,
    PlEmpty,
    PlIkey,
    PlIval,
    PlKey,
    PlKeyOn,
    PlLoading,
    PlOpt,
    PlOptOn,
    PlPct,
    PlSect,
    PlSideTitle,
    PlSpk,
    PlTime,
    PlTitle,
    PlTrack,
    PlTtext,
    PlTtime,
    PlVlabel,
    PlVol,
    PlZoom,
    Row,
    SAboutL,
    SAboutV,
    SBig,
    SDesc,
    SDetail,
    SEnd,
    SErr,
    SGroup,
    SHint,
    SInfo,
    SInline,
    SInline3,
    SKnob,
    SLabel,
    SLink,
    SList,
    SName,
    SName2,
    SOk,
    SPill,
    SPlus,
    SQ,
    SQDot,
    SQDur,
    SQNow,
    SQNow12,
    SQOn,
    SStatus,
    SStatusFail,
    SStatusOk,
    STab,
    STabOn,
    STag,
    STagText,
    STick,
    STitle,
    SWarn,
    Section,
    Size,
    Status,
    Sub,
    Timer,
    TimerMs,
    Val,
}

impl aylis::prelude::PropEnum for LabelStyle {
    const NAMES: &'static [&'static str] = &["", "caption", "error", "heading", "kbd", "link", "mono", "muted", "title", "action", "body", "cf_msg", "cf_title", "count", "ex_badge", "ex_chk", "ex_col", "ex_hint", "ex_label", "ex_tick", "ex_title", "ex_val", "ex_val2", "ex_x", "h2", "key", "legend", "legend_track", "meta", "pl_badge", "pl_caption", "pl_clock", "pl_empty", "pl_ikey", "pl_ival", "pl_key", "pl_key_on", "pl_loading", "pl_opt", "pl_opt_on", "pl_pct", "pl_sect", "pl_side_title", "pl_spk", "pl_time", "pl_title", "pl_track", "pl_ttext", "pl_ttime", "pl_vlabel", "pl_vol", "pl_zoom", "row", "s_about_l", "s_about_v", "s_big", "s_desc", "s_detail", "s_end", "s_err", "s_group", "s_hint", "s_info", "s_inline", "s_inline3", "s_knob", "s_label", "s_link", "s_list", "s_name", "s_name2", "s_ok", "s_pill", "s_plus", "s_q", "s_q_dot", "s_q_dur", "s_q_now", "s_q_now12", "s_q_on", "s_status", "s_status_fail", "s_status_ok", "s_tab", "s_tab_on", "s_tag", "s_tag_text", "s_tick", "s_title", "s_warn", "section", "size", "status", "sub", "timer", "timer_ms", "val"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Caption, Self::Error, Self::Heading, Self::Kbd, Self::Link, Self::Mono, Self::Muted, Self::Title, Self::Action, Self::Body, Self::CfMsg, Self::CfTitle, Self::Count, Self::ExBadge, Self::ExChk, Self::ExCol, Self::ExHint, Self::ExLabel, Self::ExTick, Self::ExTitle, Self::ExVal, Self::ExVal2, Self::ExX, Self::H2, Self::Key, Self::Legend, Self::LegendTrack, Self::Meta, Self::PlBadge, Self::PlCaption, Self::PlClock, Self::PlEmpty, Self::PlIkey, Self::PlIval, Self::PlKey, Self::PlKeyOn, Self::PlLoading, Self::PlOpt, Self::PlOptOn, Self::PlPct, Self::PlSect, Self::PlSideTitle, Self::PlSpk, Self::PlTime, Self::PlTitle, Self::PlTrack, Self::PlTtext, Self::PlTtime, Self::PlVlabel, Self::PlVol, Self::PlZoom, Self::Row, Self::SAboutL, Self::SAboutV, Self::SBig, Self::SDesc, Self::SDetail, Self::SEnd, Self::SErr, Self::SGroup, Self::SHint, Self::SInfo, Self::SInline, Self::SInline3, Self::SKnob, Self::SLabel, Self::SLink, Self::SList, Self::SName, Self::SName2, Self::SOk, Self::SPill, Self::SPlus, Self::SQ, Self::SQDot, Self::SQDur, Self::SQNow, Self::SQNow12, Self::SQOn, Self::SStatus, Self::SStatusFail, Self::SStatusOk, Self::STab, Self::STabOn, Self::STag, Self::STagText, Self::STick, Self::STitle, Self::SWarn, Self::Section, Self::Size, Self::Status, Self::Sub, Self::Timer, Self::TimerMs, Self::Val].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LayoutStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for LayoutStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LevelMeterStyle {
    #[default]
    Default,
    Stereo,
    Vertical,
}

impl aylis::prelude::PropEnum for LevelMeterStyle {
    const NAMES: &'static [&'static str] = &["", "stereo", "vertical"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Stereo, Self::Vertical].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum MediaCardStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for MediaCardStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum MemoStyle {
    #[default]
    Default,
    Flat,
}

impl aylis::prelude::PropEnum for MemoStyle {
    const NAMES: &'static [&'static str] = &["", "flat"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Flat].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PanelStyle {
    #[default]
    Default,
    Appbar,
    Card,
    Filled,
    Flat,
    Glass,
    Outlined,
    Overlay,
    Reveal,
    Sidebar,
    Box,
    CfCard,
    Column,
    Dialog,
    ExChk,
    ExChkOn,
    ExHead,
    ExStage,
    ExStageCrop,
    ExTl,
    Pane,
    PlBadge,
    PlBox,
    PlBoxOn,
    PlCard,
    PlFlash,
    PlHead,
    PlLoading,
    PlOpt,
    PlPop,
    PlRadio,
    PlRadioOn,
    PlTurn,
    PlTurnOn,
    PlView,
    PlViewOn,
    Player,
    Plside,
    Row,
    RowActive,
    SBar,
    SBarFill,
    SBarOn,
    SBox,
    SDev,
    SFrame,
    SMod,
    SModOn,
    SPill,
    SQCur,
    SQRow,
    SSegBox,
    SSide,
    SStage,
    STab,
    STabOn,
    STag,
    SVt,
    SVtA,
    SVtB,
    Scrim,
    Wcard,
}

impl aylis::prelude::PropEnum for PanelStyle {
    const NAMES: &'static [&'static str] = &["", "appbar", "card", "filled", "flat", "glass", "outlined", "overlay", "reveal", "sidebar", "box", "cf_card", "column", "dialog", "ex_chk", "ex_chk_on", "ex_head", "ex_stage", "ex_stage_crop", "ex_tl", "pane", "pl_badge", "pl_box", "pl_box_on", "pl_card", "pl_flash", "pl_head", "pl_loading", "pl_opt", "pl_pop", "pl_radio", "pl_radio_on", "pl_turn", "pl_turn_on", "pl_view", "pl_view_on", "player", "plside", "row", "row_active", "s_bar", "s_bar_fill", "s_bar_on", "s_box", "s_dev", "s_frame", "s_mod", "s_mod_on", "s_pill", "s_q_cur", "s_q_row", "s_seg_box", "s_side", "s_stage", "s_tab", "s_tab_on", "s_tag", "s_vt", "s_vt_a", "s_vt_b", "scrim", "wcard"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Appbar, Self::Card, Self::Filled, Self::Flat, Self::Glass, Self::Outlined, Self::Overlay, Self::Reveal, Self::Sidebar, Self::Box, Self::CfCard, Self::Column, Self::Dialog, Self::ExChk, Self::ExChkOn, Self::ExHead, Self::ExStage, Self::ExStageCrop, Self::ExTl, Self::Pane, Self::PlBadge, Self::PlBox, Self::PlBoxOn, Self::PlCard, Self::PlFlash, Self::PlHead, Self::PlLoading, Self::PlOpt, Self::PlPop, Self::PlRadio, Self::PlRadioOn, Self::PlTurn, Self::PlTurnOn, Self::PlView, Self::PlViewOn, Self::Player, Self::Plside, Self::Row, Self::RowActive, Self::SBar, Self::SBarFill, Self::SBarOn, Self::SBox, Self::SDev, Self::SFrame, Self::SMod, Self::SModOn, Self::SPill, Self::SQCur, Self::SQRow, Self::SSegBox, Self::SSide, Self::SStage, Self::STab, Self::STabOn, Self::STag, Self::SVt, Self::SVtA, Self::SVtB, Self::Scrim, Self::Wcard].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PathStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for PathStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ProgressBarStyle {
    #[default]
    Default,
    Danger,
    Success,
    Thin,
    Ex,
}

impl aylis::prelude::PropEnum for ProgressBarStyle {
    const NAMES: &'static [&'static str] = &["", "danger", "success", "thin", "ex"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Danger, Self::Success, Self::Thin, Self::Ex].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum RecordButtonStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for RecordButtonStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum RectangleStyle {
    #[default]
    Default,
    Edge,
    Edge2,
    ExRuler,
    ExSb,
    ExTick,
    PlLine,
    PlLine2,
    PlTurnMark,
    Sep,
    Stripe,
}

impl aylis::prelude::PropEnum for RectangleStyle {
    const NAMES: &'static [&'static str] = &["", "edge", "edge2", "ex_ruler", "ex_sb", "ex_tick", "pl_line", "pl_line2", "pl_turn_mark", "sep", "stripe"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Edge, Self::Edge2, Self::ExRuler, Self::ExSb, Self::ExTick, Self::PlLine, Self::PlLine2, Self::PlTurnMark, Self::Sep, Self::Stripe].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ScreenLayoutStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for ScreenLayoutStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ScrollBoxStyle {
    #[default]
    Default,
    Horizontal,
    Refresh,
}

impl aylis::prelude::PropEnum for ScrollBoxStyle {
    const NAMES: &'static [&'static str] = &["", "horizontal", "refresh"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Horizontal, Self::Refresh].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SegmentedControlStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for SegmentedControlStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SwitchStyle {
    #[default]
    Default,
    Success,
}

impl aylis::prelude::PropEnum for SwitchStyle {
    const NAMES: &'static [&'static str] = &["", "success"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Success].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TabControlStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for TabControlStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TabItemStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for TabItemStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ThemeName {
    #[default]
    Default,
    AylisLight,
    AylisDark,
    Material3Light,
    Material3Dark,
    Material2Light,
    Material2Dark,
    Android7Light,
    Android7Dark,
    IOS26Light,
    IOS26Dark,
    IOS13Light,
    IOS13Dark,
    IOS7Light,
    WinUI3Light,
    WinUI3Dark,
    Windows11Light,
    Windows11Dark,
    Windows10Light,
    Windows10Dark,
    Win32,
    Windows7,
    WindowsPhoneDark,
    WindowsPhoneLight,
    Vuetify3Light,
    Vuetify3Dark,
    Bootstrap5Light,
    Bootstrap5Dark,
    ShadcnUiLight,
    ShadcnUiDark,
    AntDesign5Light,
    AntDesign5Dark,
    ChakraUILight,
    ChakraUIDark,
    Quasar2Light,
    Quasar2Dark,
    NoOBSDark,
    NoOBSLight,
}

impl aylis::prelude::PropEnum for ThemeName {
    const NAMES: &'static [&'static str] = &["", "Aylis Light", "Aylis Dark", "Material 3 Light", "Material 3 Dark", "Material 2 Light", "Material 2 Dark", "Android 7 Light", "Android 7 Dark", "iOS 26 Light", "iOS 26 Dark", "iOS 13 Light", "iOS 13 Dark", "iOS 7 Light", "WinUI 3 Light", "WinUI 3 Dark", "Windows 11 Light", "Windows 11 Dark", "Windows 10 Light", "Windows 10 Dark", "Win32", "Windows 7", "Windows Phone Dark", "Windows Phone Light", "Vuetify 3 Light", "Vuetify 3 Dark", "Bootstrap 5 Light", "Bootstrap 5 Dark", "shadcn/ui Light", "shadcn/ui Dark", "Ant Design 5 Light", "Ant Design 5 Dark", "Chakra UI Light", "Chakra UI Dark", "Quasar 2 Light", "Quasar 2 Dark", "NoOBS Dark", "NoOBS Light"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::AylisLight, Self::AylisDark, Self::Material3Light, Self::Material3Dark, Self::Material2Light, Self::Material2Dark, Self::Android7Light, Self::Android7Dark, Self::IOS26Light, Self::IOS26Dark, Self::IOS13Light, Self::IOS13Dark, Self::IOS7Light, Self::WinUI3Light, Self::WinUI3Dark, Self::Windows11Light, Self::Windows11Dark, Self::Windows10Light, Self::Windows10Dark, Self::Win32, Self::Windows7, Self::WindowsPhoneDark, Self::WindowsPhoneLight, Self::Vuetify3Light, Self::Vuetify3Dark, Self::Bootstrap5Light, Self::Bootstrap5Dark, Self::ShadcnUiLight, Self::ShadcnUiDark, Self::AntDesign5Light, Self::AntDesign5Dark, Self::ChakraUILight, Self::ChakraUIDark, Self::Quasar2Light, Self::Quasar2Dark, Self::NoOBSDark, Self::NoOBSLight].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TrackBarStyle {
    #[default]
    Default,
    Ex,
    PlTrack,
    PlVol,
}

impl aylis::prelude::PropEnum for TrackBarStyle {
    const NAMES: &'static [&'static str] = &["", "ex", "pl_track", "pl_vol"];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default, Self::Ex, Self::PlTrack, Self::PlVol].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Os valores de um enum do acesso tipado.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum WaveformStyle {
    #[default]
    Default,
}

impl aylis::prelude::PropEnum for WaveformStyle {
    const NAMES: &'static [&'static str] = &[""];
    fn from_index(index: usize) -> Option<Self> {
        [Self::Default].get(index).copied()
    }
    fn index(self) -> usize {
        self as usize
    }
}

/// Um registro do acesso tipado (no fio, um objeto JSON com os mesmos campos).
#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxToken {
    pub name: String,
    pub words: Vec<String>,
    pub prefix: String,
    pub color: Option<aylis::prelude::Color>,
}

impl SyntaxToken {
    /// O registro lido do JSON.
    pub fn from_json(v: &aylis::json::serde_json::Value) -> Self {
        Self {
            name: v.get("name").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
            words: v.get("words").and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default(),
            prefix: v.get("prefix").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
            color: v.get("color").and_then(|x| x.as_str()).and_then(aylis::prelude::Color::from_hex),
        }
    }

    /// O registro em JSON.
    pub fn to_json(&self) -> aylis::json::serde_json::Value {
        let mut m = aylis::json::serde_json::Map::new();
        m.insert("name".into(), aylis::json::serde_json::Value::from(self.name.clone()));
        m.insert("words".into(), aylis::json::serde_json::Value::from(self.words.clone()));
        m.insert("prefix".into(), aylis::json::serde_json::Value::from(self.prefix.clone()));
        if let Some(x) = self.color {
            m.insert("color".into(), aylis::json::serde_json::Value::from(x.to_hex()));
        }
        aylis::json::serde_json::Value::Object(m)
    }

    /// A lista em JSON, como a tela grava.
    pub fn list_text(items: &[Self]) -> String {
        aylis::json::serde_json::Value::Array(items.iter().map(Self::to_json).collect()).to_string()
    }

    /// A lista do texto com o JSON (vazia se não é uma lista).
    pub fn list_from_text(text: &str) -> Vec<Self> {
        aylis::json::serde_json::from_str::<aylis::json::serde_json::Value>(text).ok().and_then(|j| j.as_array().map(|a| a.iter().map(Self::from_json).collect())).unwrap_or_default()
    }
}

impl Default for SyntaxToken {
    fn default() -> Self {
        Self::from_json(&aylis::json::serde_json::Value::Null)
    }
}
include!("components/classes/ActivityIndicator.rs");
include!("components/classes/Breadcrumb.rs");
include!("components/classes/Button.rs");
include!("components/classes/CheckBox.rs");
include!("components/classes/Chip.rs");
include!("components/classes/Circle.rs");
include!("components/classes/ComboBox.rs");
include!("components/classes/CropBox.rs");
include!("components/classes/Edit.rs");
include!("components/classes/FlowLayout.rs");
include!("components/classes/Form.rs");
include!("components/classes/Frame.rs");
include!("components/classes/FrameTimer.rs");
include!("components/classes/FrameView.rs");
include!("components/classes/Icon.rs");
include!("components/classes/Image.rs");
include!("components/classes/Label.rs");
include!("components/classes/Layout.rs");
include!("components/classes/LevelMeter.rs");
include!("components/classes/MediaCard.rs");
include!("components/classes/Memo.rs");
include!("components/classes/MenuItem.rs");
include!("components/classes/Panel.rs");
include!("components/classes/Path.rs");
include!("components/classes/PopupMenu.rs");
include!("components/classes/ProgressBar.rs");
include!("components/classes/RecordButton.rs");
include!("components/classes/Rectangle.rs");
include!("components/classes/ScreenLayout.rs");
include!("components/classes/ScrollBox.rs");
include!("components/classes/SegmentedControl.rs");
include!("components/classes/Switch.rs");
include!("components/classes/TabControl.rs");
include!("components/classes/TabItem.rs");
include!("components/classes/TrackBar.rs");
include!("components/classes/Waveform.rs");
include!("components/classes/WindowSettings.rs");
include!("components/forms/main.rs");
include!("components/forms/settings.rs");
include!("components/forms/player.rs");
include!("components/forms/viewer.rs");
include!("components/forms/export.rs");
include!("components/forms/picker.rs");
include!("components/forms/settings_general.rs");
include!("components/forms/settings_onrecord.rs");
include!("components/forms/settings_files.rs");
include!("components/forms/settings_video.rs");
include!("components/forms/settings_audio.rs");
include!("components/forms/settings_replay.rs");
include!("components/forms/settings_hotkeys.rs");
include!("components/forms/settings_devices.rs");
include!("components/forms/settings_transcribe.rs");
include!("components/forms/settings_updates.rs");
include!("components/forms/settings_about.rs");
include!("components/forms/picker_window_card.rs");
include!("components/forms/picker_area_button.rs");
include!("components/forms/main_source_row.rs");
include!("components/forms/main_legend_row.rs");
include!("components/forms/library_group.rs");
include!("components/forms/library_rec_card.rs");
include!("components/forms/library_folder_card.rs");
include!("components/forms/player_info_section.rs");
include!("components/forms/player_info_row.rs");
include!("components/forms/player_view_option.rs");
include!("components/forms/player_track_card.rs");
include!("components/forms/player_turn.rs");
include!("components/forms/player_speaker_edit.rs");
include!("components/forms/settings_parts_hidden_folder.rs");
include!("components/forms/settings_parts_device_group.rs");
include!("components/forms/settings_parts_device_row.rs");
include!("components/forms/settings_parts_queue_row.rs");
include!("components/forms/settings_parts_about_item.rs");
include!("components/forms/settings_parts_hotkey_card.rs");
include!("components/forms/settings_parts_vt_level.rs");
include!("components/forms/settings_parts_slider_tick.rs");
include!("components/forms/export_pill.rs");
include!("components/forms/export_tick.rs");
include!("components/forms/export_part_label.rs");
include!("components/forms/view_confirm.rs");
include!("components/forms/print.rs");
