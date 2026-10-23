// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) ibis inc.
// Additional terms under AGPL-3.0 section 7(b): see NOTICE.
//! Layout rules checked without a window (so that they run on Windows, where there is no scenario runner): a small file of
//! `test/corpus-report/docx/` is laid out by a headless app as the export does it, and the lines' baselines and the pages are
//! read. The same rules are in `corpus_report.rs` for the scenario runner (macOS).

use std::sync::Arc;

use super::flow_layout::{Item, Page, Pagination};

fn laid_out(file: &str) -> Arc<Pagination> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test/corpus-report/docx").join(file);
    let mut presentation = ibis_pptx::read(&std::fs::read(path).expect("the file")).expect("a document");
    let (_platform, mut cx) = crate::export::headless_app();
    cx.allow_parking();
    let resources = cx.update(|app| {
        let mut resources = crate::render::shape_view::Resources::new(&presentation, app);
        resources.print = true;
        resources.exact = true;
        resources.preload.wait_all();
        resources
    });
    super::flow_layout::update_fields_on_open(&mut presentation, &resources);
    super::flow_layout::pages(presentation.flow.as_ref().expect("a flowing document"), &resources)
}

/// A character drawn on a page: its baseline and the glyph.
struct Glyph {
    c: char,
    y: f32,
}

fn glyphs(page: &Page) -> Vec<Glyph> {
    let mut out = Vec::new();
    for item in &page.items {
        if let Item::Text { y, text, .. } = item {
            out.extend(text.chars().map(|c| Glyph { c, y: *y }));
        }
    }
    out
}

/// The baseline of the first run of glyphs spelling `text`.
fn find(glyphs: &[Glyph], text: &str) -> f32 {
    let want: Vec<char> = text.chars().collect();
    (0..=glyphs.len().saturating_sub(want.len()))
        .find(|&i| glyphs[i..i + want.len()].iter().map(|g| g.c).eq(want.iter().copied()))
        .map(|i| glyphs[i].y)
        .unwrap_or_else(|| panic!("{text:?} is drawn"))
}

fn near(actual: f32, expected: f32, tolerance: f32) -> bool {
    (actual - expected).abs() <= tolerance
}

/// A10: text that cannot sit beside a floating table starts right under it (Word's trial files f2, f8: 25.0pt from the last row's
/// text to the next line's).
#[test]
fn a10_text_that_cannot_sit_beside_a_table_starts_under_it() {
    for file in ["ftab-f2-full-center.docx", "ftab-f8-slot-15pt.docx"] {
        let pages = laid_out(file);
        let gs = glyphs(&pages.pages[0]);
        let last_row = gs.iter().filter(|g| g.c == 'T').map(|g| g.y).fold(0.0f32, f32::max);
        let l00 = find(&gs, "L00");
        assert!(near(l00 - last_row, 25.0, 0.6), "{file}: L00 {:.1}pt under the last row's text; Word: 25.0pt", l00 - last_row);
    }
}

/// A10: the empty paragraphs go under the table (f5: eight of them; L00 to L01 is 9 lines).
#[test]
fn a10_empty_paragraphs_go_under_the_table() {
    let pages = laid_out("ftab-f5-full-empties.docx");
    let gs = glyphs(&pages.pages[0]);
    let (l00, l01, l02) = (find(&gs, "L00"), find(&gs, "L01"), find(&gs, "L02"));
    let rows = (l01 - l00) / (l02 - l01);
    assert!(near(rows, 9.0, 0.2), "L00 to L01 is {rows:.2} lines; Word: 9");
}

/// T7: a page break in a table's cell adds no line (the same place for the paragraph after, one page).
#[test]
fn t7_page_break_in_a_cell_adds_no_line() {
    let plain = laid_out("cellbreak-plain.docx");
    let after = find(&glyphs(&plain.pages[0]), "after");
    for file in ["cellbreak-start.docx", "cellbreak-both.docx"] {
        let pages = laid_out(file);
        let y = find(&glyphs(&pages.pages[0]), "after");
        assert!(pages.pages.len() == 1 && near(y, after, 0.1), "{file}: {} pages, `after` at {y:.1} (without the break: {after:.1})", pages.pages.len());
    }
}

/// F5: a paragraph moved on from the page of an obstacle is laid again without it (the floating table B after A and the empty
/// paragraphs comes at the top of page 2, not at the foot of an added page).
#[test]
fn f5_paragraph_moved_off_an_obstacle_page_is_laid_again() {
    for file in ["floats-a-b-1-empty.docx", "floats-a-b-3-empties.docx", "floats-a-b-5-empties.docx"] {
        let pages = laid_out(file);
        let on = pages.pages.iter().position(|p| glyphs(p).iter().any(|g| g.c == 'B')).expect("B");
        let y = find(&glyphs(&pages.pages[on]), "B");
        assert!(pages.pages.len() == 2 && on == 1 && y < 200.0, "{file}: {} pages, B on page {} at y {y:.0}", pages.pages.len(), on + 1);
    }
}

/// A floating table of one row whose text is taller than a page: the row is divided where the page ends and goes on at the top of
/// the next page (Word; 033's last page was blank and the end of the text was not drawn). Every item is drawn, on two pages.
#[test]
fn a_floating_row_taller_than_a_page_goes_on_to_the_next_page() {
    for (file, n) in [("floatrow-split-70.docx", 70), ("floatrow-split-120.docx", 120), ("floatrow-fits-30.docx", 30)] {
        let pages = laid_out(file);
        let drawn: Vec<String> = pages.pages.iter().map(|p| glyphs(p).iter().map(|g| g.c).collect::<String>()).collect();
        let missing: Vec<usize> = (0..n).filter(|i| !drawn.iter().any(|d| d.contains(&format!("ITEM{i:03}")))).collect();
        assert!(missing.is_empty(), "{file}: items {missing:?} are not drawn on any of the {} pages", pages.pages.len());
        // The items go on in order: the last item is on a page after the first's (when they do not all fit on one).
        let page_of = |i: usize| drawn.iter().position(|d| d.contains(&format!("ITEM{i:03}"))).unwrap();
        assert!(page_of(n - 1) >= page_of(0), "{file}: the last item before the first");
    }
}
