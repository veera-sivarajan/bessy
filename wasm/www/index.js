import * as wasm from "wasm";

while (true) {
    var input = window.prompt("Enter input: ");
    wasm.greet(input);
}
