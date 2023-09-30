use std::{
    fmt::{self, Display, Formatter},
    io::{Write, self}, collections::HashMap,
};

use crate::print_diagnostics;
use hpi_analyzer::Diagnostic;
use hpi_interpreter_tree::HPIHttpClient;
use serde::Serialize;
use wasm_bindgen::prelude::*;
// use wasmer::{imports, Function, Instance, InstantiationError, Module, Store};

#[wasm_bindgen(raw_module = "../../src/print.ts")]
extern "C" {
    pub fn print(message: String);
}

struct Console;
impl Write for Console {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let str = String::from_utf8(buf.to_vec()).expect("utf8 error");
        print(
            str.replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('\n', "<br>")
                .replace('\t', "&nbsp;&nbsp;&nbsp;&nbsp;")
                .replace(' ', "&nbsp;"),
        );

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Serialize)]
pub enum Backend {
    Tree,
}

impl From<String> for Backend {
    fn from(value: String) -> Self {
        match value.as_str() {
            "tree" => Self::Tree,
            other => panic!("Cannot convert invalid string `{}` to backend", other),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResult {
    pub code: Option<i64>,
    pub runtime_error: Option<WasmRuntimeError>,
    pub diagnostics: Option<String>,
}

impl RunResult {
    pub fn new_ok(code: i64, diagnostics: &[Diagnostic]) -> Self {
        Self {
            code: Some(code),
            runtime_error: None,
            diagnostics: Some(print_diagnostics(diagnostics)),
        }
    }

    pub fn new_error(message: String, diagnostics: &[Diagnostic]) -> Self {
        Self {
            code: None,
            runtime_error: Some(WasmRuntimeError {
                kind: WasmRuntimeErrorKind::Unknown.to_string(),
                message,
            }),
            diagnostics: Some(print_diagnostics(diagnostics)),
        }
    }
}

#[derive(Serialize)]
pub struct WasmRuntimeError {
    pub kind: String,
    pub message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WasmRuntimeErrorKind {
    StackOverflow,
    Arithmetic,
    OutOfMem,
    Unknown,
}

impl Display for WasmRuntimeErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            WasmRuntimeErrorKind::StackOverflow => "Stapelüberlauf",
            WasmRuntimeErrorKind::Arithmetic => "Rechenfehler",
            WasmRuntimeErrorKind::OutOfMem => "Kein Speicher mehr übrig",
            WasmRuntimeErrorKind::Unknown => "Unbekannter Fehler",
        })
    }
}

struct Client {}

impl HPIHttpClient for Client {
    fn request(&self, method: String, url: &str, body: String, headers: HashMap<String, String>) -> Result<(u16, String), String> {
        Err("HTTP Anfragen werden im Web momentan nicht unterstützt.".to_string())
    }
}

#[wasm_bindgen]
pub fn run(code: &str, backend: String) -> String {
    console_error_panic_hook::set_once();

    let (program, diagnostics) = match hpi_analyzer::analyze(code, "Spielplatz") {
        Ok(res) => res,
        Err(errs) => {
            let res = RunResult {
                code: None,
                runtime_error: None,
                diagnostics: Some(print_diagnostics(&errs)),
            };

            return serde_json::to_string(&res).expect("can always serialize this struct");
        }
    };

    let client = Client{};
    let res = match backend.into() {
        Backend::Tree => match hpi_interpreter_tree::Interpreter::new(Console, client).run(program) {
            Ok(code) => RunResult::new_ok(code, &diagnostics),
            Err(err) => RunResult::new_error(err.into_owned(), &diagnostics),
        },
    };

    serde_json::to_string(&res).expect("can always serialize this struct")
}

#[derive(Debug, Clone, Copy)]
struct ExitCode(i32);

impl Display for ExitCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn wasm_exit(code: i32) -> Result<(), ExitCode> {
    Err(ExitCode(code))
}
