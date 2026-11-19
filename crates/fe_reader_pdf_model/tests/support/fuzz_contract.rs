//! Read-only PDF parsing and diagnostic invariants on synthetic inputs.
use fe_reader_pdf_model::{
    extract_text_spans_bytes, inspect_lab_bytes, inspect_timeline_bytes, sniff_pdf_bytes,
};

pub fn check(data: &[u8]) {
    if data.len() > 65_536 {
        return;
    }
    let sniff = sniff_pdf_bytes(data);
    let lab = inspect_lab_bytes(data);
    let text = extract_text_spans_bytes(data);
    let timeline = inspect_timeline_bytes(data);
    if !data.starts_with(b"%PDF-") {
        assert!(sniff.is_err() && lab.is_err() && text.is_err() && timeline.is_err());
        return;
    }
    let sniff = sniff.expect("PDF-header boundary is shared");
    let lab = lab.expect("PDF-header boundary is shared");
    let text = text.expect("PDF-header boundary is shared");
    let timeline = timeline.expect("PDF-header boundary is shared");
    assert_eq!(sniff.fingerprint.sha256_hex, lab.document_sha256);
    assert_eq!(sniff.fingerprint.sha256_hex, timeline.document_sha256);
    assert!(lab.stream_count <= lab.object_count);
    assert!(lab.pages.len() <= lab.object_count);
    assert!(
        text.spans
            .windows(2)
            .all(|pair| pair[0].page_index <= pair[1].page_index)
    );
    assert!(
        timeline
            .revisions
            .windows(2)
            .all(|pair| pair[0].startxref_offset < pair[1].startxref_offset)
    );
    assert_eq!(
        timeline,
        inspect_timeline_bytes(data).expect("timeline is repeatable")
    );
}
