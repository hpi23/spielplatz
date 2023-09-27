<script lang="ts">
    import IconButton from '@smui/icon-button'
    import Button, { Label } from '@smui/button'
    import Dialog, { Title, Content, Actions } from '@smui/dialog'
    import Select, { Option } from '@smui/select'
  import LinearProgress from '@smui/linear-progress';
    import { onMount } from 'svelte'

    import Editor from './Editor.svelte'
    import type { CompileResult, RunResult } from './hpi'
    import HPIWorker from './hpi.worker?worker'

    import willkommenHPI from './scripts/willkommen.hpi?raw'
    import fibHPI from './scripts/fib.hpi?raw'
    import powHPI from './scripts/pow.hpi?raw'
    import approxPi from './scripts/approx_pi.hpi?raw'
    import primFaktoren from './scripts/primfaktoren.hpi?raw'
    import approxE from './scripts/approx_e.hpi?raw'
    import approxApery from './scripts/approx_apery.hpi?raw'

    const templates = {
        Willkommen: [willkommenHPI, 'Willkommen'],
        Fibonacci: [fibHPI, 'Fibonacci'],
        Pow: [powHPI, 'Exponentialrechnung'],
        ApproxPi: [approxPi, 'Pi mal Daumen'],
        primFaktoren: [primFaktoren, 'Primfaktorzerlegung'],
        ApproxE: [approxE, 'Die Eulersche Zahl'],
        ApproxApery: [approxApery, 'Apéry Konstante'],
    }

    const LOGO = "<span class='red'>" + ("\
ooooo   ooooo ooooooooo.   ooooo  <br>\
`888'   `888' `888   `Y88. `888'  <br>\
 888     888   888   .d88'  888   <br>".replaceAll(" ", "&nbsp;")) + "</span><span class='orange'>" +
(" 888ooooo888   888ooo88P'   888   <br>\
 888     888   888          888   <br>".replaceAll(" ", "&nbsp;")) + "</span><span class='yellow'>" +
(" 888     888   888          888   <br>\
o888o   o888o o888o        o888o  <br><br>".replaceAll(" ", "&nbsp;"))

    const backends = {
        tree: 'run tree-walking',
    }

    let currentBackend = Object.keys(backends)[0]

    let compileRes: CompileResult = undefined
    let showDiagnostics = true

    let code = ''
    $: if (loadedInitially) saveCode(code)

    let loadedInitially = false
    let loadedScript = ''
    let currentScript = Object.keys(templates)[0][0]

    let running = false
    let runRes: RunResult = undefined

    let _HPIWorker: Worker | null = null
    let resizer: HTMLDivElement | null = null

    let helpOpen = false

    function run() {
        running = true
        _HPIWorker = makeWorker(
            code,
            currentBackend,
        )
    }

    function update() {
        running = true
        if (!loadedInitially) return
        running = false
        if (!['tree'].includes(currentBackend)) run()
    }

    function cancel() {
        running = false
        _HPIWorker?.terminate()
        runRes = undefined
    }

    let terminal: HTMLDivElement

    function makeWorker(code: string, backend: string): Worker {
        let worker = new HPIWorker()

        worker.onmessage = function (event: { data: any[] }) {
            if (event.data[0] === 'print') {
                terminal.innerHTML += event.data[1]
                terminal.scrollTo(0, terminal.scrollHeight)
            }
            if (event.data[0] === 'ready') {
        terminal.innerText = ""
                worker.postMessage(['run', code, backend])
            }
            if (event.data[0] === 'finished') {
                running = false
                worker.terminate()
                    compileRes = undefined
                    runRes = JSON.parse(event.data[1])
            }
        }

        return worker
    }

    function saveCode(code: string) {
        window.localStorage.setItem('hpi-playground-code', code)
    }

    function saveBackend(backend: string) {
        window.localStorage.setItem('hpi-playground-backend', backend)
    }

    function saveScript(script: string) {
        window.localStorage.setItem('hpi-playground-script', script)
    }

    async function loadFromStorage(): Promise<string> {
        loadedScript = currentScript

        let storageScript = window.localStorage.getItem('hpi-playground-script')
        if (storageScript === null || storageScript === undefined || storageScript === "undefined") {
            if (currentScript === null || currentScript === undefined) {
                currentScript = Object.keys(templates)[0]
                console.log(`This page is visited for the first time, using default script: ${currentScript}`)
            }
            saveScript(currentScript)
        } else {
            loadedScript = storageScript
            currentScript = storageScript
        }

        let storageBackend = window.localStorage.getItem('hpi-playground-backend')
        if (storageBackend === null) {
            saveBackend(currentBackend)
        } else {
            currentBackend = storageBackend
        }

        let loaded_code = window.localStorage.getItem('hpi-playground-code')
        if (loaded_code === null) {
            code = templates[currentScript][0]
            saveCode(code)
            return code
        } else {
            return loaded_code
        }
    }

    function loadTemplate() {
        code = templates[currentScript][0]
        if (loadedScript !== currentScript) {
            saveScript(currentScript)
            loadedScript = currentScript
        }
    }

    onMount(async () => {
        code = await loadFromStorage()
        loadedInitially = true

        // the current position of mouse
        let mouseX = 0

        // width of editor
        let leftWidth = 0

        const editorDiv: HTMLElement = resizer.previousElementSibling as HTMLElement
        const outputDiv: HTMLElement = resizer.nextElementSibling as HTMLElement

        // Keybindings:
        // CTRL + S => force save the current code
        // F8 => run current code
        // F9 => cancel code execution
        document.addEventListener('keydown', (e: KeyboardEvent) => {
            if (e.ctrlKey && e.key === 's') {
                e.preventDefault()
                saveCode(code)
            } else if (e.key === 'F8' && !running) {
                run()
            } else if (e.key === 'F9' && running) {
                cancel()
            } else if (e.key === 'F10') {
                e.preventDefault()
                if (!running) {
                    loadTemplate()
                }
            }
        })

        // handle the mousedown event
        // that's triggered when user drags the resizer
        const mouseDownHandler = function (e: MouseEvent) {
            // Get the current mouse position
            mouseX = e.clientX
            leftWidth = editorDiv.getBoundingClientRect().width

            // Attach the listeners to `document`
            document.addEventListener('mousemove', mouseMoveHandler)
            document.addEventListener('mouseup', mouseUpHandler)
        }

        const mouseMoveHandler = function (e: MouseEvent) {
            // How far the mouse has been moved
            const dx = e.clientX - mouseX

            const newLeftWidth =
                ((leftWidth + dx) * 100) /
                (resizer.parentNode as HTMLElement).getBoundingClientRect().width
            editorDiv.style.width = `${newLeftWidth}%`

            editorDiv.style.userSelect = 'none'
            editorDiv.style.pointerEvents = 'none'

            outputDiv.style.userSelect = 'none'
            outputDiv.style.pointerEvents = 'none'
        }

        const mouseUpHandler = function () {
            editorDiv.style.removeProperty('user-select')
            editorDiv.style.removeProperty('pointer-events')

            outputDiv.style.removeProperty('user-select')
            outputDiv.style.removeProperty('pointer-events')

            // remove the handlers of `mousemove` and `mouseup`
            document.removeEventListener('mousemove', mouseMoveHandler)
            document.removeEventListener('mouseup', mouseUpHandler)
        }

        resizer.addEventListener('mousedown', mouseDownHandler)
    })

    $:console.log(running);
</script>

<main>
    <Dialog bind:open={helpOpen} aria-labelledby="help-title" aria-describedby="help-content">
        <Title id="help-title">Nutzung des Spielplatzes</Title>
        <Content id="help-content">
            <p>
                Der <a
                    class="highlight"
                    href="https://github.com/hpi23/hpi-playground"
                    target="_blank"
                    rel="noreferrer">HPI Spielplatz</a
                >
                ermöglicht es jedem, die wunderbare
                <a class="highlight" href="https://sprache.hpi.churc" target="_blank" rel="noreferrer"
                    >HPI Programmiersprache</a
                > zu verwenden.
                <br>
                Hierbei ist es egal, welches Betriebsystem Sie verwenden, nur Apple haben wir nicht getestet, da das sowieso schei*e ist.
            </p>

            <p>
            Alle von Ihnen durchgeführte Änderungen am aktuellen Dokument werden lokal, also auf ihrem Rechner abgelegt.
            Dies bedeutet, dass Sie die Seite jederzeit ohne Hemmungen neuladen können.
            Falls Sie doch ihre Änderungen überschreiben wollen, so nutzen Sie doch bitte den <code class="highlight">LADEN</code> Knopf in der oberen rechten Ecke.
            </p>

            <p>
            Dieser Knopf kann auch dazu verwendet werden, andere Quelltextvorlagen zu laden.
            Diese Vorlagen sind schon vorgefertigte Programme, die Sie als Inspiration nutzen dürfen.
            Um eine Vorlage zu laden, müssen Sie diese Zunächst in der Schaltfläche <code class="highlight">Vorlage auswählen</code> wählen.
            Anschließend nutzen Sie den bekannten <code class="highlight">LADEN</code> Knopf, um die gewählte Vorlage zu laden.
            </p>

            <p>
                Um das aktuelle Programm auszuführen, nutzen Sie bitte den Knopf zum Starten (<i
                    class="material-icons icon">play_arrow</i
                >)
                Falls das Programm nicht terminiert, obwohl es sollte, nutzen Sie den Knopf zum Abbrechen (<i class="material-icons icon">cancel</i>)
            </p>

            <p>
                Disclaimer: Diese Sprache darf nicht mit dem HPI (Hasso Plattner Institut) in Verbindung gebracht werden, da ihr Name und Logo ganz zufällig gewählt wurden.
            </p>

            <h4>Tastaturbindungen</h4>
            <ul>
                <li>
                    <code class="highlight">CTRL + S</code>: Speichern des aktuellen Dokumenteninhalts
                </li>
                <li>
                    <code class="highlight">F8</code>: Ausführen des aktuellen Programms
                </li>
                <li>
                    <code class="highlight">F9</code>: Terminieren des aktuellen Programms
                </li>
            </ul>
        </Content>
        <Actions>
            <Button on:click={() => (helpOpen = false)}>
                <Label>Schließen</Label>
            </Button>
        </Actions>
    </Dialog>

    <div class="main">
        <div class="main__editor">
            <Editor bind:code on:update={update} />
        </div>
        <div class="main__resizer" bind:this={resizer} />
        <div class="main__output">
            <div class="main__output__nav">
            <div class="main__output__nav__left">
                <div class="main__output__nav__left__top">
                    <Select bind:value={currentScript} label="Vorlage auswählen">
                        {#each Object.keys(templates) as template}
                            <Option value={template}>{templates[template][1]}</Option>
                        {/each}
                    </Select>
                </div>
                <div class="main__output__nav__left__bottom">
                    <Button
                        variant="raised"
                        on:click={loadTemplate}
                        disabled={(currentScript === loadedScript || running) &&
                            templates[currentScript][0] === code}><Label>Laden</Label></Button
                    >
                    <IconButton class="material-icons" on:click={run} disabled={running}
                        >play_arrow</IconButton
                    >
                    <IconButton
                        size="button"
                        class="material-icons"
                        on:click={() => (helpOpen = true)}>help</IconButton
                    >
                </div>
            </div>
            <div class="main__output__nav__right">
            <img src="/assets/logo.svg" alt="Das Logo des HPI">
            </div>
            </div>
            {#if running}
                    <LinearProgress indeterminate />
            {/if}
            <div class="main__output__terminal">
                <div bind:this={terminal}></div>
                {#if running}
                    Programm wird ausgeführt...
                {:else if runRes}
                    {#if runRes.runtimeError}
                        Laufzeitumgebung abgestürtzt:
                        <br />
                        {runRes.runtimeError.kind}: {runRes.runtimeError.message}
                    {:else if runRes.code !== null}
                        {#if runRes.diagnostics !== ''}
                            {@html runRes.diagnostics}
                            <br />
                            <br />
                        {/if}
                    {:else}
                        Kompilierung fehlgeschlagen:
                        <br />
                        {@html runRes.diagnostics}
                    {/if}
                {:else if compileRes}
                    {#if compileRes.diagnostics && ( showDiagnostics || compileRes.output === undefined)}
                        {@html compileRes.diagnostics}
                        <div class="main__output__terminal__sep" />
                    {/if}
                    {#if compileRes.error}
                        Compilation failed:
                        <br />
                        {@html compileRes.error}
                        <br />
                    {:else if compileRes.output !== undefined}
                        {@html compileRes.output}
                    {/if}
                {:else}
                    <span class="ahighlight">{@html LOGO}</span>
                    Betätigen Sie den dreieckigen <span class="highlight">Startknopf</span>, um die Programmausführung einzuleiten.
                {/if}
            </div>
        </div>
    </div>
</main>

<style lang="scss">
    @use 'mixins' as *;

    :global(body) {
        width: 100%;
        height: 100%;
        margin: 0;
    }

    .highlight {
        color: var(--clr-primary);
    }

    :global(.red) {
        color: #ef5350;
    }

    :global(.orange) {
        color: #ff7043;
    }

    :global(.yellow) {
        color: #fff176;
    }

    .icon {
        font-size: 1.2rem;
        color: var(--clr-primary);
        vertical-align: middle;
    }

    h4 {
        margin: 1rem 0;
    }

    ul {
        margin: 0;
        padding: 0 2rem;
    }

    .main {
        display: flex;
        height: 100vh;
        width: 100%;

        @include mobile {
            flex-direction: column;
            height: auto;
        }

        &__editor {
            width: calc(65% - 10px);
            overflow-x: auto;

            @include mobile {
                width: 100% !important;
            }
        }

        &__resizer {
            flex-shrink: 0;
            width: 4px;
            cursor: e-resize;

            @include mobile {
                display: none;
            }
        }

        &__output {
            flex: 1 0 0%;
            background-color: #222225;
            display: flex;
            flex-direction: column;

            &__nav {
                background-color: #323237;
                padding: 0.5rem 1rem;
                height: 8rem;
                display: flex;

                &__left {
                    width: 80%;

                display: flex;
                flex-direction: column;
                gap: 1rem;

                &__top,
                &__bottom {
                    display: flex;
                    align-items: center;
                    gap: 1rem;
                }
                }

                &__right {
                    flex-shrink: 1;
                    padding: 1rem;
                    box-sizing: border-box;

                    img {
                    aspect-ratio: 1;
                    height: auto;
                    max-height:85%;
                    }
                }

            }

            &__terminal {
                font-family: 'Jetbrains Mono NL', monospace;
                padding: 1rem 1.6rem;
                overflow: auto;

                &__sep {
                    height: 0.2rem;
                    background-color: #424242;
                    margin: 1.5rem 0;
                }
            }
        }
    }
</style>
