import init, { analyze, run } from '../hpi-backend/pkg/hpi_backend'

///// LINTING /////

export interface Diagnostic {
    level: 'hint' | 'info' | 'warning' | 'error'
    error: 'none' | 'syntax' | 'type' | 'semantic' | 'reference'
    message: string
    span: Span
}

export interface Span {
    start: Location
    end: Location
}

export interface Location {
    line: number
    column: number
    charIdx: number
}

///// EXECUTION /////

export interface RunResult {
    // the exit-code of the program
    code: number
    // whether the VM has crashed during program execution
    runtimeError: RuntimeError
    // HTML representation of any diagnostcs
    diagnostics: string
}

export interface CompileResult {
    // specifies whether the compilation failed
    failed: boolean
    // HTML representation of any diagnostcs
    diagnostics: string
    // contains the output code
    output: string
    // other error
    error: string,
}

export interface RuntimeError {
    kind: 'stackOverflow' | 'arithmetic'
    message: string
}

export class Backend {
    constructor() {
        init()
    }

    lint(code: string): Diagnostic[] {
        let raw_res = analyze(code)
        return JSON.parse(raw_res)
    }

    run(code: string, backend: string): RunResult {
        let raw_res = run(code, backend)
        return JSON.parse(raw_res)
    }
}
