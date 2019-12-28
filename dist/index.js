import init, {
    allocate_buffer,
    parse_json,
    parse_json_typed,
    parse_json_move,
    parse_json_move_typed,
} from './wasm_json_parse.js';

let memory;

const measure = (cb, id) => {
    const el = document.getElementById(id);
    if (!el) {
        throw new Error(`Couldn't find element #${id}`);
    }

    let start = performance.now();
    try {
        if (cb()) {
            el.innerText = (performance.now() - start).toFixed(4);
            return;
        }
    } catch (e) { /* don't care about errors */ }

    el.innerText = '-';
};

const sendRequest = (e) => {
    if (e && e.preventDefault) {
        e.preventDefault();
    }

    const url = document.getElementById('url');
    const button = document.getElementById('submit');

    if (!url || !button) {
        return;
    }

    url.disabled = true;
    button.disabled = true;

    fetch(url.value)
        .then(r => {
            if (r.status >= 400 && r.status <= 599) {
                throw new Error(`Server returned ${r.status}`);
            }

            const contentType = r.headers.get('content-type');
            if (!contentType || contentType !== 'application/json') {
                throw new Error('Invalid Content-Type');
            }

            return r.text();
        })
        .then(r => {
            measure(() => JSON.parse(r), 'native');

            measure(() => parse_json(r), 'untyped');

            measure(() => parse_json_typed(r), 'typed');

            let ptr;
            measure(() => ptr = allocate_buffer(r.length), 'alloc');

            measure(() => {
                const buffer = new Uint8Array(memory.buffer, ptr, r.length);
                const encoder = new TextEncoder();
                return encoder.encodeInto(r, buffer);
            }, 'move');

            measure(parse_json_move, 'untyped-move');

            measure(parse_json_move_typed, 'typed-move');
        })
        .catch((e) => {
            document.querySelectorAll('.table td:last-child').forEach(el => el.innerText = '-');
            throw e;
        })
        .finally(() => {
            url.disabled = false;
            button.disabled = false;
        })

    return false;
}

init().then(module => {
    memory = module.memory;
    sendRequest();

    const form = document.getElementById('form');
    if (form) {
        form.addEventListener('submit', sendRequest);
    }
});
