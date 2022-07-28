import * as wasm from "wasm";

// while (true) {
//     var input = window.prompt("Enter input: ");
//     wasm.greet(input);
// }

import {EditorState, EditorView, basicSetup} from "@codemirror/basic-setup"
import {javascript} from "@codemirror/lang-javascript"
import {oneDark, oneDarkTheme, oneDarkHighlightStyle} from "@codemirror/theme-one-dark"
import {Terminal} from "xterm";
import { FitAddon } from 'xterm-addon-fit';

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
        doc:`var i = 10;
var b = 2;
print a + b;

for (var i = 0; i < 10; i = i + 1) {
    print i;
}

var a = 0;
var e = 10;
while (a < e) {
    print a;
    a = a + 1;
}
`,
    }),
    parent: document.getElementById('source-code'),
})

const term = new Terminal();
const fit = new FitAddon();
term.loadAddon(fit);
term.open(document.getElementById('terminal'));
fit.fit();
term.write('\x1B[1;3;31m>>\x1B[0m ')


let runButton = document.getElementById('runButton');
runButton.addEventListener("click", runCode);

window.writeToTerm = (s) = term.writeln(s);

function runCode() {
    var input = editor.state.doc.toString();
    let output = wasm.evaluate(input);
    term.writeln(output);
}
