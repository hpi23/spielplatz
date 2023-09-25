import init, {run} from '../hpi-backend/pkg/hpi_backend';


async function main() {
    await init();

    postMessage(['ready'])
    onmessage = function(event) {
            let res = run(event.data[1], event.data[2])
            postMessage(['finished', res])
    }
}

export function print(message: string) {
    postMessage(['print', message])
}

setTimeout(main, 10)
