// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) ibis inc.
// Additional terms under AGPL-3.0 section 7(b): see NOTICE.
//! The workbook's right pane (todo-fix #79): no tabs; it shows the attributes of what is selected,
//! the cells' format (セルの書式設定), the chart's (グラフ エディター) or the shape's or text
//! box's (サイズと位置・塗りつぶし・枠線). データの入力規則 is opened from the Data menu and stays
//! until what is selected changes to another kind.
use gpui_kit::*;

use crate::shell::editor::SlideEditor;

/// What the right pane is for: the selection's kind (a chart or a shape by its id).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RightKey {
    Cells,
    Chart(ibis_doc_model::ShapeId),
    Shape(ibis_doc_model::ShapeId),
}

impl SlideEditor {
    /// The right pane's button / 表示 > サイドバー: show or hide the pane. Shown, it is for what is
    /// selected (the render's `sync_right_pane`).
    pub(crate) fn toggle_sidebar(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.sidebar_open = !self.sidebar_open;
        if let Some(s) = self.sheet_state_mut() {
            s.right_key = None;
        }
        cx.notify();
        window.refresh();
    }

    /// What the right pane is for now.
    fn right_key(&self) -> RightKey {
        if let Some((id, _)) = self.sheet_chart() {
            RightKey::Chart(id)
        } else if let Some(id) = self.selected_sheet_shape() {
            RightKey::Shape(id)
        } else {
            RightKey::Cells
        }
    }

    /// The pane follows the selection: with nothing of the attribute panels open it shows the
    /// selection's, and when the selection goes to another kind (or another chart, shape) the pane
    /// goes with it. `open` is whether another panel (the name manager, a search, ...) is
    /// already on show, which stays while the selection stays.
    pub(crate) fn sync_right_pane(&mut self, shown: bool, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if !self.sidebar_open || !self.is_workbook() {
            return false;
        }
        // The query editor is not a panel of the chain (it fills the window): the pane leaves it be (it was closed here,
        // as a panel no attribute pane stands for: xlsx::query001_editor).
        if self.sheet_state().is_some_and(|s| s.panels.query_editor.is_some()) {
            return false;
        }
        let key = self.right_key();
        let before = self.sheet_state().and_then(|s| s.right_key);
        // Only the attribute panels follow the selection; another panel (the name manager, a search,
        // the selection pane, ...) stays until it is closed.
        let attribute_panel = self.sheet_state().is_some_and(|s| s.custom_format.is_some() || s.chart_edit.is_some() || s.validation.is_some() || s.panels.frame.is_some());
        if shown && (!attribute_panel || before.is_none() || before == Some(key)) {
            if let Some(s) = self.sheet_state_mut() {
                s.right_key = Some(key);
            }
            self.refresh_shape_pane(key, window, cx);
            return false;
        }
        if let Some(s) = self.sheet_state_mut() {
            s.right_key = Some(key);
            s.custom_format = None;
            s.validation = None;
            s.chart_edit = None;
            s.names = None;
            s.find = None;
            s.link = None;
            s.page_setup = false;
            s.panels.frame = None;
        }
        match key {
            RightKey::Cells => self.open_custom_format_unfocused(window, cx),
            RightKey::Chart(_) => self.open_sheet_chart_panel(window, cx),
            RightKey::Shape(_) => self.open_sheet_frame_panel(window, cx),
        }
        true
    }

    /// A shape's panel shows its place and size as they are now (after a drag, an undo).
    fn refresh_shape_pane(&mut self, key: RightKey, window: &mut Window, cx: &mut Context<Self>) {
        if let RightKey::Shape(id) = key {
            self.refresh_sheet_frame_panel(id, window, cx);
        }
    }
}
