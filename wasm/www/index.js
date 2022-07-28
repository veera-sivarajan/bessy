import * as wasm from "wasm";

// while (true) {
//     var input = window.prompt("Enter input: ");
//     wasm.greet(input);
// }

import { CodeJar } from 'codejar';
import { withLineNumbers } from 'codejar/linenumbers';
import { gruvboxesque } from "./code-editor.js";

const codeEditor = document.querySelector('#code-editor');
const codeJar = CodeJar(codeEditor, withLineNumbers(gruvboxesque));
