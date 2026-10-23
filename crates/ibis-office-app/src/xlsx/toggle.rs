// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) ibis inc.
// Additional terms under AGPL-3.0 section 7(b): see NOTICE.
//! A check box row for the side panels: the mark, then the caption, which
//! wraps over several lines when it is long (a button's label is cut with
//! "…" instead).
use gpui_kit::component::h_flex;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use crate::shell::palette::rgb;

pub(crate) fn toggle_row(id: impl Into<ElementId>, on: bool, label: impl Into<SharedString>, on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .items_center()
        .gap_2()
        .px(px(6.))
        .py(px(3.))
        .rounded_md()
        .cursor_pointer()
        .hover(|s| s.bg(crate::shell::palette::shade(0.0, 0.05)))
        .on_click(on_click)
        .child(crate::shell::chrome::checkbox_box(on))
        .child(div().flex_1().min_w_0().text_size(px(13.)).child(label.into()))
}

/// Left-aligned radio option row without button borders, using primary blue (`PRIMARY_BUTTON_BG`)
/// for the inner filled circle of the selected double-circle indicator.
pub(crate) fn radio_row(
    id: impl Into<ElementId>,
    on: bool,
    disabled: bool,
    label: impl Into<SharedString>,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let primary = rgb(crate::shell::chrome::PRIMARY_BUTTON_BG);
    let ring = div()
        .w(px(14.))
        .h(px(14.))
        .rounded(px(7.))
        .border_1()
        .border_color(if on { primary } else { rgb(0x888888) })
        .flex()
        .items_center()
        .justify_center()
        .flex_shrink_0()
        .when(on, |d| {
            d.child(
                div()
                    .w(px(8.))
                    .h(px(8.))
                    .rounded(px(4.))
                    .bg(primary),
            )
        });
    h_flex()
        .id(id)
        .w_full()
        .items_center()
        .gap_2()
        .py(px(3.))
        .px_1()
        .rounded(px(3.))
        .when(disabled, |d| d.opacity(0.45))
        .when(!disabled, |d| {
            d.cursor_pointer()
                .hover(|s| s.bg(rgb(0xedf2f7)))
                .on_click(on_click)
        })
        .child(ring)
        .child(div().flex_1().min_w_0().text_size(px(12.)).child(label.into()))
}
