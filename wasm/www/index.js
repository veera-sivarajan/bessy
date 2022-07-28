import * as wasm from "wasm";

import {EditorState, EditorView, basicSetup} from "@codemirror/basic-setup"
import {javascript} from "@codemirror/lang-javascript"
import {oneDark, oneDarkTheme, oneDarkHighlightStyle} from "@codemirror/theme-one-dark"
import {Terminal} from "xterm";
import { FitAddon } from 'xterm-addon-fit';
import exampleFile from '!raw-loader!./example.lox'

let timer;
let editor = new EditorView({
    state: EditorState.create({
        extensions: [
            basicSetup,
            javascript(), 
            oneDark,
            oneDarkTheme,
            oneDarkHighlightStyle.extension
        ],
        doc: exampleFile,
    }),
    parent: document.getElementById('source-code'),
})

const term = new Terminal({
    convertEol: true,
    theme: {
        background: '#1d2026'
    },
    fontSize: 20,
    enableBold: true
});

const fit = new FitAddon();
term.loadAddon(fit);
term.open(document.getElementById('terminal'));
fit.fit();
term.write('>> ')


let runButton = document.getElementById('runButton');
runButton.addEventListener("click", runCode);

function runCode() {
    var input = editor.state.doc.toString();
    let output = wasm.evaluate(input);
    console.log(output);
    term.write(output);
    term.write('>> ')
}

let clearBtn = document.getElementById('clearBtn');
clearBtn.addEventListener("click", clearScreen);
function clearScreen() {
    term.write('\x1bc');
    term.write('>> ')
}
    

