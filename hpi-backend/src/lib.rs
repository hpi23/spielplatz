pub mod lint;
pub mod run;

use hpi_analyzer::{Diagnostic, DiagnosticLevel};

fn print_diagnostics(errs: &[Diagnostic]) -> String {
    errs.iter()
        .map(display_err)
        .collect::<Vec<String>>()
        .join("<br><br>")
}

fn display_err(err: &Diagnostic) -> String {
    let code = err.source.to_string().replace([' ', '\t'], "&nbsp;");
    let lines: Vec<_> = code.split('\n').collect();

    let (raw_marker, raw_marker_single, color) = match err.level {
        DiagnosticLevel::Hint => ("~", "^", "#d472fe"), // magenta
        DiagnosticLevel::Info => ("~", "^", "#45bcf4"), // blue
        DiagnosticLevel::Warning => ("~", "^", "#ebc656"), // yellow
        DiagnosticLevel::Error(_) => ("^", "^", "#ff616e"), // red
    };

    let notes: String = err
        .notes
        .iter()
        .map(|note| format!("<br><b style='color: #4accd8;'> - Anmerkung: </b>{note}",))
        .collect();

    // take special action if the source code is empty or there is no useful span
    if err.source.is_empty() || err.span.is_empty() {
        return format!(
            "<b style='color: {color};'>{lvl}</b> in {path} <br> {msg}{notes}",
            color = color,
            lvl = err.level,
            path = err.span.start.path,
            msg = err.message,
        );
    }

    let line1 = match err.span.start.line > 1 {
        true => format!(
            "<br>&nbsp;&nbsp;<span style='color: #4f5666;'>{: >3} |</span>{}",
            err.span.start.line - 1,
            lines[err.span.start.line - 2],
        ),
        false => String::new(),
    };

    let line2 = format!(
        "&nbsp;&nbsp;<span style='color: #4f5666;'>{: >3} |</span> {}",
        err.span.start.line,
        lines[err.span.start.line - 1]
    );

    let line3 = match err.span.start.line < lines.len() {
        true => format!(
            "<br>&nbsp;&nbsp;<span style='color: #4f5666;'>{: >3} |</span> {}",
            err.span.start.line + 1,
            lines[err.span.start.line]
        ),
        false => String::new(),
    };

    let markers = match (
        err.span.start.line == err.span.end.line,
        err.span.start.column + 1 == err.span.end.column,
    ) {
        // same line, wide column difference
        (true, false) => raw_marker.repeat(err.span.end.column - err.span.start.column),
        // same line, just one column difference
        (true, true) => raw_marker_single.to_string(),
        // multiline span
        (_, _) => {
            format!(
                "{marker} ...<br>{space}<b style='color: #a0da71;'>+ {line_count} weitere Zeile{n}</b>",
                marker = raw_marker
                    .repeat(lines[err.span.start.line - 1].len() - err.span.start.column + 1),
                space = "&nbsp;".repeat(err.span.start.column + 7),
                line_count = err.span.end.line - err.span.start.line,
                n = match err.span.end.line - err.span.start.line == 1 {
                    true => "",
                    false => "n",
                },
            )
        }
    };

    let marker = format!(
        "{space}<b style='color: {color};'>{markers}</b>",
        space = "&nbsp;".repeat(err.span.start.column + 6),
    );

    format!(
            "<b style='color: {color};'>{lvl}</b> im Dokument {path}:{line}:{col}<br>{line1}<br>{line2}<br>{marker}{line3}<br><br> <b style='color: {color};'>{msg}</b>{notes}",
            lvl = err.level,
            path = err.span.start.path,
            line = err.span.start.line,
            col = err.span.start.column,
            msg = err.message,
        )
}
