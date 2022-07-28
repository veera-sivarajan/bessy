import * as wasm from "wasm";

// while (true) {
//     var input = window.prompt("Enter input: ");
//     wasm.greet(input);
// }

import {EditorState, EditorView, basicSetup} from "@codemirror/basic-setup"
import {javascript} from "@codemirror/lang-javascript"

let editor = new EditorView({
  state: EditorState.create({
    extensions: [basicSetup, javascript()]
  }),
  parent: document.body
})

