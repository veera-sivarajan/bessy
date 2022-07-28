import * as wasm from "wasm";

// while (true) {
//     var input = window.prompt("Enter input: ");
//     wasm.greet(input);
// }

import {EditorState, EditorView, basicSetup} from "@codemirror/basic-setup"
import {javascript} from "@codemirror/lang-javascript"
import {oneDark, oneDarkTheme, oneDarkHighlightStyle} from "@codemirror/theme-one-dark"
import {Terminal} from "xterm";

let output = document.getElementById('output');
let timer;
let editor = new EditorView({
    state: EditorState.create({
        extensions: [
            basicSetup,
            javascript(), 
            oneDark,
            oneDarkTheme,
            oneDarkHighlightStyle.extension,
            EditorView.updateListener.of((v)=> {
                if(v.docChanged) {
                    if(timer) clearTimeout(timer);
                    timer = setTimeout(() => {
                        var y = document.createTextNode("This just got added");
                        output.appendChild(y);
                    }, 500 );
                }
            })
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

var term = new Terminal();
term.open(document.getElementById('terminal'));
term.write('Hello from \x1B[1;3;31mxterm.js\x1B[0m $ ')


let runButton = document.getElementById('runButton');
runButton.addEventListener("click", runCode);

function runCode() {
        // var input = window.prompt("Enter input: ");
    var input = editor.state.doc.toString();
    // console.log(input);
    wasm.greet(input);
}
