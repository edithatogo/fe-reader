use fe_reader_perf_benches::{
    metadata_scrub, page_tree_inspect, parser_sniff, template_planning, text_extraction,
};
use iai_callgrind::{
    Dhat, LibraryBenchmarkConfig, library_benchmark, library_benchmark_group, main,
};

#[library_benchmark]
fn parser() {
    let _ = parser_sniff();
}

#[library_benchmark]
fn page_tree() {
    let _ = page_tree_inspect();
}

#[library_benchmark]
fn text_extract() {
    let _ = text_extraction();
}

#[library_benchmark]
fn template_plan() {
    let _ = template_planning();
}

#[library_benchmark]
fn metadata_diff() {
    metadata_diff();
}

#[library_benchmark]
fn metadata_scrub_clean_share() {
    let _ = metadata_scrub(fe_reader_metadata::MetadataScrubMode::CleanShare);
}

#[library_benchmark]
fn metadata_scrub_aggressive() {
    let _ = metadata_scrub(fe_reader_metadata::MetadataScrubMode::Aggressive);
}

library_benchmark_group!(
    name = dhat_group;
    benchmarks = parser, page_tree, text_extract, template_plan, metadata_diff, metadata_scrub_clean_share, metadata_scrub_aggressive
);

fn main() {
    main!(
        config = LibraryBenchmarkConfig::default().tool(Dhat::default());
        library_benchmark_groups = dhat_group
    );
}
