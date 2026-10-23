// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) ibis inc.
// Additional terms under AGPL-3.0 section 7(b): see NOTICE.
//! The xlsx defects of the corpus report (Excel for the web's PDFs against
//! ibisOffice's), each as Excel shows it, read from the small files of
//! `test/corpus-report/xlsx` (made up, no corpus text). The ones that need
//! drawing or printing are scenarios (`ibis-office-app/src/xlsx/corpus_report.rs`).
//!
//! TODO: このファイルは、実文書の比較（コーパス）から見つかった崩れの試験の
//! 仮置き場で、ずっと残すものではない。試験が通るようになったら（崩れを直した
//! ら）、その試験を本番の試験に移すか、試験として要らなければ消し、最小文書
//! （`test/corpus-report/xlsx/`）も一緒に移すか消す。
//! 本番の移し先は `crates/ibis-pptx/tests/` の機能ごとの試験（例
//! `text_bullets_and_runs.rs`）。
//! 崩れが残っている試験（赤）はここに置いたままにする。

use ibis_doc_model::{CfKind, EMU_PER_POINT, EmuRect, Presentation, Sheet, ShapeKind, Slide, sheet_calc::sheet_of_slide};

fn path(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test/corpus-report/xlsx").join(name)
}

fn open(name: &str) -> Presentation {
    ibis_pptx::read(&std::fs::read(path(name)).unwrap_or_else(|e| panic!("{name}: {e}"))).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn first_sheet(p: &Presentation) -> (&Slide, &Sheet) {
    let slide = &p.slides[0];
    (slide, sheet_of_slide(slide).expect("a worksheet"))
}

fn points(emu: i64) -> f64 {
    emu as f64 / EMU_PER_POINT as f64
}

/// The cells from (`row`, `column`) to (`last_row`, `last_column`) (0-based,
/// inclusive) on the slide, in EMU.
fn cells_rect(slide: &Slide, (row, column): (u32, u32), (last_row, last_column): (u32, u32)) -> EmuRect {
    let frame = slide.shapes.iter().find(|s| matches!(s.kind, ShapeKind::Sheet(_))).expect("the sheet's frame").frame;
    let sheet = sheet_of_slide(slide).unwrap();
    let (xs, ys) = (sheet.column_offsets(), sheet.row_offsets());
    let (x0, y0) = (xs[column as usize], ys[row as usize]);
    let (x1, y1) = (xs[last_column as usize + 1], ys[last_row as usize + 1]);
    EmuRect::new(frame.x + x0, frame.y + y0, x1 - x0, y1 - y0)
}

fn overlaps(a: EmuRect, b: EmuRect) -> bool {
    a.width > 0 && a.height > 0 && a.x < b.x + b.width && b.x < a.x + a.width && a.y < b.y + b.height && b.y < a.y + a.height
}

/// Each conditional format's formulas as `xlsx::conditional::apply` takes
/// them: moved to a cell of the range (`ibis_calc::formula::shift`), then
/// worked out (`recalc::evaluate` parses it).
fn work_out_conditional_formulas(sheet: &Sheet) -> usize {
    let mut count = 0;
    for cf in &sheet.conditional {
        for rule in &cf.rules {
            if let CfKind::Expression(f) = &rule.kind {
                let moved = ibis_calc::formula::shift(f, 0, 0);
                let _ = ibis_calc::formula::parse(&moved);
                count += 1;
            }
        }
    }
    count
}

#[test]
fn corpus_report_xlsx_x1a_cf_ref_error_before_multibyte_text() {
    // X1a 条件付き書式の数式に `#REF!="はい"` のように `#` と多バイト文字が9バイト以内に並ぶと、
    // エラー値の判定のバイト切り出しで panic し、ファイルを開けない（exit 101）。
    // Excel は普通に開く（規則が成り立たないだけ）。ASCII の "yes" は今も落ちない（対照）。
    let ascii = open("cf-ref-error-ascii.xlsx");
    assert_eq!(work_out_conditional_formulas(first_sheet(&ascii).1), 1);
    let p = open("cf-ref-error.xlsx");
    assert_eq!(work_out_conditional_formulas(first_sheet(&p).1), 1, "#REF!=\"はい\" の規則が1つ読めて、落ちずに解析できる");
}

#[test]
fn corpus_report_xlsx_x1c_cf_multibyte_function_name() {
    // X1c 関数名に多バイト文字が混じる（`xあい(…)`）と、`_xlfn.` などの接頭辞を外す処理の
    // バイト切り出しで panic する。Excel なら知らない関数は #NAME? になるだけで開ける。
    let p = open("cf-function-name.xlsx");
    assert_eq!(work_out_conditional_formulas(first_sheet(&p).1), 1, "xあい(A1) の規則が1つ読めて、落ちずに解析できる");
}

#[test]
fn corpus_report_xlsx_x2a_header_footer_xstring_escape() {
    // X2a ヘッダー・フッターの `_xHHHH_` エスケープが元の文字に戻らず、フッターに `_x000D_` がそのまま印刷される。
    // ST_Xstring の `_x000D_` は CR、`_x005F_` は `_` そのもの（Excel は「SAMPLE LABEL」「_x0041_ end」と印刷する）。
    let p = open("xstring-escape.xlsx");
    let hf = &first_sheet(&p).1.print.header_footer;
    let header = hf.odd_header.as_deref().expect("a header");
    let footer = hf.odd_footer.as_deref().expect("a footer");
    assert!(!header.contains("_x000D_"), "ヘッダーに _x000D_ が残っている: {header:?}");
    assert!(!footer.contains("_x000D_"), "フッターに _x000D_ が残っている: {footer:?}");
    assert!(footer.ends_with("&R_x0041_ end"), "_x005F_x0041_ は _x0041_ に戻る: {footer:?}");
}

#[test]
fn corpus_report_xlsx_x3e_chart_axis_format_escaped_space() {
    // X3e グラフの軸の書式 `###\ ###` の空白から後ろを文字として扱い、目盛りの数値が「10000 ###」になる。
    // 再現ファイルは無いので、値軸の書式だけ `###\ ###` にしたグラフを作る（171 の chart4.xml と同じ書き方）。
    // Excel の目盛りは「10 000」「9 500」（171 p.5 の正解の文字）。
    let mut package = ibis_ooxml::Package::open(std::fs::read(path("x5b-all-zero-axis.xlsx")).unwrap()).unwrap();
    let xml = String::from_utf8(package.read("xl/charts/chart1.xml").unwrap()).unwrap();
    let from = "<c:numFmt formatCode=\"#,##0\" sourceLinked=\"0\"/>";
    assert!(xml.contains(from));
    package.set_bytes("xl/charts/chart1.xml", xml.replace(from, "<c:numFmt formatCode=\"###\\ ###\" sourceLinked=\"0\"/>").into_bytes());
    let p = ibis_pptx::read(&package.save(ibis_ooxml::SaveOptions::default()).unwrap()).unwrap();
    let chart = p.slides[0].shapes.iter().find_map(|s| if let ShapeKind::Chart(c) = &s.kind { Some(c) } else { None }).expect("a chart");
    let axis = chart.axes.iter().find(|a| a.kind == ibis_doc_model::AxisKind::Value).expect("a value axis");
    assert_eq!(axis.format, r"###\ ###");
    assert_eq!(ibis_doc_model::format_number(10_000.0, &axis.format), "10 000");
    assert_eq!(ibis_doc_model::format_number(9_500.0, &axis.format), "9 500");
}

#[test]
fn corpus_report_xlsx_x5f_rows_without_ht_use_default_row_height() {
    // X5f 高さの指定が無い行を、宣言された `defaultRowHeight`（12.75〜15pt）ではなく 18pt で数えるため、行が高くなる。
    // Arial 10・defaultRowHeight=12.75（customHeight なし）・ht の無い50行。Excel for the web は宣言どおり
    // 12.75pt で並べる（151 で (527−59)/34 行 ≒ 13.8pt と宣言値が合う）。許容差は 0.01pt（EMU の丸め）。
    let p = open("X6c-rows-without-ht-arial10.xlsx");
    let sheet = first_sheet(&p).1;
    for (i, &h) in sheet.rows.iter().take(50).enumerate() {
        assert!((points(h) - 12.75).abs() < 0.01, "行 {} の高さ {} pt（Excel は 12.75 pt）", i + 1, points(h));
    }
}

#[test]
fn corpus_report_xlsx_x5f_chart_anchored_to_rows_without_ht() {
    // X5f 高さの指定が無い行を 18pt で数えるため、行に固定したグラフも縦に 1.2〜1.4 倍に伸びる。
    // 行1〜20（twoCellAnchor の row 0〜20）に固定した縦棒。Excel なら 20 × 12.75 = 255pt
    // （ibisOffice は約 359pt）。許容差は 1pt（行の高さの丸め）。
    let p = open("x5f-default-row-height.xlsx");
    let chart = p.slides[0].shapes.iter().find(|s| matches!(s.kind, ShapeKind::Chart(_))).expect("a chart");
    let height = points(chart.frame.height);
    assert!((height - 255.0).abs() <= 1.0, "グラフの枠の高さ {height} pt（Excel は 255 pt）");
}

#[test]
fn corpus_report_xlsx_x6a_merged_wrapped_note_keeps_file_row_height() {
    // X6a 高さが自動の行で、結合セルの折り返し文字（ＭＳ／メイリオ系）の行の高さを先頭の1列の幅だけで測り直すため、
    // 行が数百 pt に伸びる。A:E を結合した注記4行（ＭＳ 明朝、折り返し、ht=13.5、customHeight なし）。
    // Excel は結合セルに合わせて行の高さを変えないので、44行 × 13.5 = 594pt（ibisOffice は 3682pt）。
    // 許容差は 0.01pt（EMU の丸め）。
    let p = open("X6a-merged-wrap-note-refit.xlsx");
    let sheet = first_sheet(&p).1;
    for row in 40..44 {
        let h = points(sheet.rows[row]);
        assert!((h - 13.5).abs() < 0.01, "注記の行 {} の高さ {h} pt（ファイルの ht は 13.5 pt）", row + 1);
    }
    let total: f64 = sheet.rows.iter().take(44).map(|&h| points(h)).sum();
    assert!((total - 594.0).abs() < 0.1, "印刷範囲 A1:E44 の高さ {total} pt（Excel は 594 pt）");
}

#[test]
fn corpus_report_xlsx_x6a_merged_wrapped_text_does_not_grow_rows() {
    // X6a 高さが自動の行で、結合セルの折り返し文字の行の高さを先頭の1列の幅だけで測り直すため、行が数百 pt に伸びる。
    // B:H を2行分結合し折り返す長文を入れた区画が8つ（ht なし、defaultRowHeight=15 customHeight=1、Calibri 11）。
    // Excel は結合セルに合わせて行の高さを変えないので、16行とも 15pt で1ページ（ibisOffice は8ページ）。
    // 文字の無い行が 18pt になるのは X5f（Calibri 11 の行を 18pt で数える）なので、ここでは 18pt までを許す。
    let p = open("merged-wrap-autofit.xlsx");
    let sheet = first_sheet(&p).1;
    let heights: Vec<f64> = sheet.rows.iter().take(16).map(|&h| points(h)).collect();
    assert!(heights.iter().all(|h| *h <= 18.0 + 0.01), "行の高さ {heights:?}（Excel はどれも 15 pt）");
}

#[test]
fn corpus_report_xlsx_x7a_column_width_in_windows_pixels() {
    // X7a 列の幅を Excel for Mac の「1画素＝1pt、数字の幅 6pt」で数えるため、どの列も Excel（Windows・web）より約9%広くなる。
    // Calibri 11（数字の幅 mdw = 7px）、width=12 の列。Excel は px = trunc((256*w + trunc(128/mdw))/256 * mdw) = 84px、
    // ×0.75 で 63pt（比べた9文書のページ割りがこの式と合う）。ibisOffice は 72pt。許容差は 1px（0.75pt）。
    let p = open("colwidth-7x12-a4.xlsx");
    let sheet = first_sheet(&p).1;
    for (i, &w) in sheet.columns.iter().take(7).enumerate() {
        let w = points(w);
        assert!((w - 63.0).abs() <= 0.75, "列 {} の幅 {w} pt（Excel は 63 pt）", (b'A' + i as u8) as char);
    }
}

#[test]
fn corpus_report_xlsx_x7c_unknown_paper_size_is_letter() {
    // X7c `paperSize` が知らない番号（257 など）のシートを A4 にするが、Excel for the web は Letter にする。
    // 151（paperSize="257"）の正解の用紙は 612×792pt。
    let p = open("paper-size-257.xlsx");
    let paper = first_sheet(&p).1.print.paper;
    assert_eq!((points(paper.0), points(paper.1)), (612.0, 792.0), "用紙（Excel は Letter 612×792pt）");
}

#[test]
fn corpus_report_xlsx_x8a_form_controls_are_shown() {
    // X8a チェックボックスとオプションボタン（フォームのコントロール）が描かれず、☑ や ○1〜○4 が出ない。
    // C3:H5 にチェックボックス2列とオプションボタン4列 × 3行（18個）。Excel はどれも描く。
    // 描かれることを、コントロールの置き場所に見えている図形があることで見る（DrawingML の影は隠れたまま）。
    let p = open("x8a-form-controls.xlsx");
    let slide = &p.slides[0];
    for row in 2..5 {
        for column in 2..8 {
            let cell = cells_rect(slide, (row, column), (row, column));
            let shown = slide.shown_shapes().filter(|s| !matches!(s.kind, ShapeKind::Sheet(_))).any(|s| overlaps(s.frame, cell));
            assert!(shown, "{}{} のコントロールが見えていない", (b'A' + column as u8) as char, row + 1);
        }
    }
}

#[test]
fn corpus_report_xlsx_x8b_ole_object_preview_is_shown() {
    // X8b 埋め込み OLE オブジェクトの表示用の絵（EMF）が描かれず、写真欄の枠と「写真貼付」が出ない。
    // B3:D9 に OLE オブジェクト（objectPr の表示用の PNG は枠と十字）。Excel は絵を重ねて描く。
    let p = open("x8b-ole-preview.xlsx");
    let slide = &p.slides[0];
    let area = cells_rect(slide, (2, 1), (8, 3));
    let shown = slide.shown_shapes().any(|s| matches!(s.kind, ShapeKind::Picture(_)) && overlaps(s.frame, area));
    assert!(shown, "B3:D9 に OLE の表示用の絵が無い（見えている図形：{:?}）", slide.shown_shapes().map(|s| (&s.kind, s.frame)).filter(|(k, _)| !matches!(k, ShapeKind::Sheet(_))).map(|(_, f)| f).collect::<Vec<_>>());
}
