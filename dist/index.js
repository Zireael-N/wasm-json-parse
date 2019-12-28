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

const sendRequest = async (e) => {
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

    try {
        let resp = await fetch(url.value);
        if (resp.status >= 400 && resp.status <= 599) {
            throw new Error(`Server returned ${resp.status}`);
        }

        const contentType = resp.headers.get('content-type');
        if (!contentType || !contentType.includes('application/json')) {
            throw new Error('Invalid Content-Type');
        }

        const blob = await resp.blob()

        const buffer = await new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onabort = reject;
            reader.onerror = reject;
            reader.onloadend = e => resolve(e.target.result);
            reader.readAsArrayBuffer(blob);
        });

        parseString(new TextDecoder('utf-8').decode(buffer));
        parseBuffer(buffer);
    } catch(e) {
        document.querySelectorAll('.table td:last-child').forEach(el => el.innerText = '-');
        throw e;
    } finally {
        url.disabled = false;
        button.disabled = false;
    }
    return false;
}

const parseString = str => {
    measure(() => JSON.parse(str), 'native');

    measure(() => parse_json(str), 'untyped');

    measure(() => parse_json_typed(str), 'typed');
}

const parseBuffer = buf => {
    let ptr;
    measure(() => ptr = allocate_buffer(buf.byteLength), 'alloc');

    measure(() => {
        new Uint8Array(memory.buffer, ptr, buf.byteLength).set(buf);
        return !0;
    }, 'move');

    measure(parse_json_move, 'untyped-move');

    measure(parse_json_move_typed, 'typed-move');
}

init().then(module => {
    memory = module.memory;
    sendRequest();

    const form = document.getElementById('form');
    if (form) {
        form.addEventListener('submit', sendRequest);
    }
});
