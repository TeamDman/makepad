pub use makepad_draw::makepad_platform;
pub use makepad_draw;

pub use makepad_derive_widget;
pub use makepad_draw::*;
pub use makepad_derive_widget::*;

pub mod button;
pub mod label;
pub mod image;
pub mod link_label;
pub mod drop_down;
pub mod popup_menu;
pub mod check_box;
pub mod radio_button;
pub mod text_input;
pub mod slider;
pub mod scroll_bar;
pub mod scroll_bars;
pub mod splitter;
pub mod fold_header;
pub mod fold_button;
pub mod hook_widget;
pub mod multi_window;
pub mod designer;
pub mod dock;
pub mod tab;
pub mod tab_bar;
pub mod tab_close_button;
pub mod list_view;
pub mod desktop_button;
pub mod desktop_window;
pub mod scroll_shadow;
pub mod rotated_image;
pub mod slide_panel;

pub mod file_tree;
pub mod slides_view;
pub mod color_picker;

#[macro_use]
pub mod window_menu;
pub mod debug_view;
pub mod nav_control;

pub mod view;
pub mod widget;

#[macro_use]
pub mod data_binding;

mod base;
mod theme_desktop_dark;
pub mod image_cache;

pub use crate::{
    data_binding::{DataBindingStore, DataBindingMap},
    button::*,
    view::*,
    image::*,
    label::*,
    slider::*,
    check_box::*,
    drop_down::*,
    radio_button::*,
    text_input::*,
    link_label::*,
    list_view::*,
    slide_panel::*,
    desktop_window::*,
    scroll_bars::{ScrollBars},
    scroll_shadow::{DrawScrollShadow},
    scroll_bar::{ScrollBar},
    slides_view::{SlidesView},
    widget::{
        WidgetSet,
        WidgetSetIterator,
        WidgetUid,
        WidgetDraw,
        WidgetDrawApi,
        CreateAt,
        WidgetActions,
        WidgetActionsApi,
        WidgetActionItem,
        WidgetRef,
        Widget,
        WidgetRegistry,
        WidgetFactory,
        WidgetAction,
        DrawStateWrap,
    }
};


pub fn live_design(cx: &mut Cx) {
    makepad_draw::live_design(cx);
    crate::debug_view::live_design(cx);
    crate::fold_header::live_design(cx);
    crate::splitter::live_design(cx);
    crate::base::live_design(cx);
    crate::theme_desktop_dark::live_design(cx);
    crate::slider::live_design(cx);
    crate::label::live_design(cx);
    crate::nav_control::live_design(cx);
    crate::image::live_design(cx);
    crate::rotated_image::live_design(cx);
    crate::view::live_design(cx);
    crate::fold_button::live_design(cx);
    crate::text_input::live_design(cx);
    crate::link_label::live_design(cx);
    crate::scroll_shadow::live_design(cx);
    crate::button::live_design(cx);
    crate::desktop_button::live_design(cx);
    crate::desktop_window::live_design(cx);
    crate::window_menu::live_design(cx);
    crate::scroll_bar::live_design(cx);
    crate::scroll_bars::live_design(cx);
    crate::check_box::live_design(cx);
    crate::radio_button::live_design(cx);
    crate::popup_menu::live_design(cx);
    crate::drop_down::live_design(cx);
    crate::multi_window::live_design(cx);
    crate::designer::live_design(cx);
    crate::hook_widget::live_design(cx);
    crate::list_view::live_design(cx);
    crate::slide_panel::live_design(cx);
    crate::tab::live_design(cx);
    crate::tab_bar::live_design(cx);
    crate::dock::live_design(cx);
    crate::color_picker::live_design(cx);
    crate::file_tree::live_design(cx);
    crate::slides_view::live_design(cx);
    crate::tab_close_button::live_design(cx);
}
