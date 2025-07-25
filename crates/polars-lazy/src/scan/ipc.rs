use polars_core::prelude::*;
use polars_io::cloud::CloudOptions;
use polars_io::ipc::IpcScanOptions;
use polars_io::{HiveOptions, RowIndex};
use polars_utils::plpath::PlPath;
use polars_utils::slice_enum::Slice;

use crate::prelude::*;

#[derive(Clone)]
pub struct ScanArgsIpc {
    pub n_rows: Option<usize>,
    pub cache: bool,
    pub rechunk: bool,
    pub row_index: Option<RowIndex>,
    pub cloud_options: Option<CloudOptions>,
    pub hive_options: HiveOptions,
    pub include_file_paths: Option<PlSmallStr>,
}

impl Default for ScanArgsIpc {
    fn default() -> Self {
        Self {
            n_rows: None,
            cache: true,
            rechunk: false,
            row_index: None,
            cloud_options: Default::default(),
            hive_options: Default::default(),
            include_file_paths: None,
        }
    }
}

#[derive(Clone)]
struct LazyIpcReader {
    args: ScanArgsIpc,
    sources: ScanSources,
}

impl LazyIpcReader {
    fn new(args: ScanArgsIpc) -> Self {
        Self {
            args,
            sources: ScanSources::default(),
        }
    }
}

impl LazyFileListReader for LazyIpcReader {
    fn finish(self) -> PolarsResult<LazyFrame> {
        let args = self.args;

        let options = IpcScanOptions {};
        let pre_slice = args.n_rows.map(|len| Slice::Positive { offset: 0, len });

        let cloud_options = args.cloud_options;
        let hive_options = args.hive_options;
        let rechunk = args.rechunk;
        let cache = args.cache;
        let row_index = args.row_index;
        let include_file_paths = args.include_file_paths;

        let lf: LazyFrame = DslBuilder::scan_ipc(
            self.sources,
            options,
            UnifiedScanArgs {
                schema: None,
                cloud_options,
                hive_options,
                rechunk,
                cache,
                glob: true,
                projection: None,
                row_index,
                pre_slice,
                cast_columns_policy: CastColumnsPolicy::ERROR_ON_MISMATCH,
                missing_columns_policy: MissingColumnsPolicy::Raise,
                extra_columns_policy: ExtraColumnsPolicy::Raise,
                include_file_paths,
                column_mapping: None,
                deletion_files: None,
            },
        )?
        .build()
        .into();

        Ok(lf)
    }

    fn finish_no_glob(self) -> PolarsResult<LazyFrame> {
        unreachable!()
    }

    fn sources(&self) -> &ScanSources {
        &self.sources
    }

    fn with_sources(mut self, sources: ScanSources) -> Self {
        self.sources = sources;
        self
    }

    fn with_n_rows(mut self, n_rows: impl Into<Option<usize>>) -> Self {
        self.args.n_rows = n_rows.into();
        self
    }

    fn with_row_index(mut self, row_index: impl Into<Option<RowIndex>>) -> Self {
        self.args.row_index = row_index.into();
        self
    }

    fn rechunk(&self) -> bool {
        self.args.rechunk
    }

    fn with_rechunk(mut self, toggle: bool) -> Self {
        self.args.rechunk = toggle;
        self
    }

    fn n_rows(&self) -> Option<usize> {
        self.args.n_rows
    }

    fn row_index(&self) -> Option<&RowIndex> {
        self.args.row_index.as_ref()
    }

    /// [CloudOptions] used to list files.
    fn cloud_options(&self) -> Option<&CloudOptions> {
        self.args.cloud_options.as_ref()
    }
}

impl LazyFrame {
    /// Create a LazyFrame directly from a ipc scan.
    pub fn scan_ipc(path: PlPath, args: ScanArgsIpc) -> PolarsResult<Self> {
        Self::scan_ipc_sources(ScanSources::Paths([path].into()), args)
    }

    pub fn scan_ipc_files(paths: Arc<[PlPath]>, args: ScanArgsIpc) -> PolarsResult<Self> {
        Self::scan_ipc_sources(ScanSources::Paths(paths), args)
    }

    pub fn scan_ipc_sources(sources: ScanSources, args: ScanArgsIpc) -> PolarsResult<Self> {
        LazyIpcReader::new(args).with_sources(sources).finish()
    }
}
