use hpi_analyzer::{Diagnostic, DiagnosticLevel, ErrorKind};
use hpi_parser::{Location, Span};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn analyze(code: &str) -> String {
    console_error_panic_hook::set_once();

    let diagnostics = match hpi_analyzer::analyze(code, "playground") {
        Ok((_, diagnostics)) => diagnostics,
        Err(diagnostics) => diagnostics,
    };

    let new_diagnostics = diagnostics
        .into_iter()
        .map(WasmDiagnostic::from)
        .collect::<Vec<WasmDiagnostic>>();

    serde_json::to_string(&new_diagnostics).expect("can always serialize diagnostics")
}

#[derive(Serialize)]
pub struct WasmDiagnostic {
    pub level: WasmDiagnosticLevel,
    pub error: Option<WasmErrorKind>,
    pub message: String,
    pub span: WasmSpan,
}

impl<'src> From<Diagnostic<'src>> for WasmDiagnostic {
    fn from(src: Diagnostic) -> Self {
        Self {
            error: match src.level {
                DiagnosticLevel::Error(ref kind) => Some(WasmErrorKind::from(kind.clone())),
                _ => None,
            },
            level: WasmDiagnosticLevel::from(src.level),
            message: src.message.to_string(),
            span: WasmSpan::from(src.span),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WasmErrorKind {
    Syntax,
    Type,
    Semantic,
    Reference,
}

impl From<ErrorKind> for WasmErrorKind {
    fn from(src: ErrorKind) -> Self {
        match src {
            ErrorKind::Syntax => Self::Syntax,
            ErrorKind::Type => Self::Type,
            ErrorKind::Semantic => Self::Semantic,
            ErrorKind::Reference => Self::Reference,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WasmDiagnosticLevel {
    Hint,
    Info,
    Warning,
    Error,
}

impl From<DiagnosticLevel> for WasmDiagnosticLevel {
    fn from(src: DiagnosticLevel) -> Self {
        match src {
            DiagnosticLevel::Hint => Self::Hint,
            DiagnosticLevel::Info => Self::Info,
            DiagnosticLevel::Warning => Self::Warning,
            DiagnosticLevel::Error(_) => Self::Error,
        }
    }
}

#[derive(Serialize)]
pub struct WasmSpan {
    pub start: WasmLocation,
    pub end: WasmLocation,
}

impl<'src> From<Span<'src>> for WasmSpan {
    fn from(src: Span<'src>) -> Self {
        Self {
            start: WasmLocation::from(src.start),
            end: WasmLocation::from(src.end),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WasmLocation {
    pub line: usize,
    pub column: usize,
    pub char_idx: usize,
}

impl<'src> From<Location<'src>> for WasmLocation {
    fn from(src: Location<'src>) -> Self {
        Self {
            line: src.line,
            column: src.column,
            char_idx: src.char_idx,
        }
    }
}
