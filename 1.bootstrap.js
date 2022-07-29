(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "../pkg/wasm.js":
/*!**********************!*\
  !*** ../pkg/wasm.js ***!
  \**********************/
/*! exports provided: evaluate */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_bg.wasm */ \"../pkg/wasm_bg.wasm\");\n/* harmony import */ var _wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./wasm_bg.js */ \"../pkg/wasm_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"evaluate\", function() { return _wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"evaluate\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/wasm.js?");

/***/ }),

/***/ "../pkg/wasm_bg.js":
/*!*************************!*\
  !*** ../pkg/wasm_bg.js ***!
  \*************************/
/*! exports provided: evaluate */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"evaluate\", function() { return evaluate; });\n/* harmony import */ var _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_bg.wasm */ \"../pkg/wasm_bg.wasm\");\n\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachedUint8Memory0 = new Uint8Array();\n\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachedInt32Memory0 = new Int32Array();\n\nfunction getInt32Memory0() {\n    if (cachedInt32Memory0.byteLength === 0) {\n        cachedInt32Memory0 = new Int32Array(_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachedInt32Memory0;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n* @param {string} input\n* @returns {string}\n*/\nfunction evaluate(input) {\n    try {\n        const retptr = _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n        const ptr0 = passStringToWasm0(input, _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        const len0 = WASM_VECTOR_LEN;\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"evaluate\"](retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n        _wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n    }\n}\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../web/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/wasm_bg.js?");

/***/ }),

/***/ "../pkg/wasm_bg.wasm":
/*!***************************!*\
  !*** ../pkg/wasm_bg.wasm ***!
  \***************************/
/*! exports provided: memory, evaluate, __wbindgen_add_to_stack_pointer, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/wasm_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var bessy_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! bessy-wasm */ \"../pkg/wasm.js\");\n/* harmony import */ var _codemirror_basic_setup__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! @codemirror/basic-setup */ \"./node_modules/@codemirror/basic-setup/dist/index.js\");\n/* harmony import */ var _codemirror_lang_javascript__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! @codemirror/lang-javascript */ \"./node_modules/@codemirror/lang-javascript/dist/index.js\");\n/* harmony import */ var _codemirror_theme_one_dark__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! @codemirror/theme-one-dark */ \"./node_modules/@codemirror/theme-one-dark/dist/index.js\");\n/* harmony import */ var xterm__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! xterm */ \"./node_modules/xterm/lib/xterm.js\");\n/* harmony import */ var xterm__WEBPACK_IMPORTED_MODULE_4___default = /*#__PURE__*/__webpack_require__.n(xterm__WEBPACK_IMPORTED_MODULE_4__);\n/* harmony import */ var xterm_addon_fit__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! xterm-addon-fit */ \"./node_modules/xterm-addon-fit/lib/xterm-addon-fit.js\");\n/* harmony import */ var xterm_addon_fit__WEBPACK_IMPORTED_MODULE_5___default = /*#__PURE__*/__webpack_require__.n(xterm_addon_fit__WEBPACK_IMPORTED_MODULE_5__);\n/* harmony import */ var _raw_loader_example_lox__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! raw-loader!./example.lox */ \"./node_modules/raw-loader/dist/cjs.js!./example.lox\");\n/* harmony import */ var xterm_css_xterm_css__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! xterm/css/xterm.css */ \"./node_modules/xterm/css/xterm.css\");\n\n\n\n\n\n\n\n\n\n\nlet timer;\nlet editor = new _codemirror_basic_setup__WEBPACK_IMPORTED_MODULE_1__[\"EditorView\"]({\n    state: _codemirror_basic_setup__WEBPACK_IMPORTED_MODULE_1__[\"EditorState\"].create({\n        extensions: [\n            _codemirror_basic_setup__WEBPACK_IMPORTED_MODULE_1__[\"basicSetup\"],\n            Object(_codemirror_lang_javascript__WEBPACK_IMPORTED_MODULE_2__[\"javascript\"])(), \n            _codemirror_theme_one_dark__WEBPACK_IMPORTED_MODULE_3__[\"oneDark\"],\n            _codemirror_theme_one_dark__WEBPACK_IMPORTED_MODULE_3__[\"oneDarkTheme\"],\n            _codemirror_theme_one_dark__WEBPACK_IMPORTED_MODULE_3__[\"oneDarkHighlightStyle\"].extension\n        ],\n        doc: _raw_loader_example_lox__WEBPACK_IMPORTED_MODULE_6__[\"default\"], \n    }),\n    parent: document.getElementById('source-code'),\n})\n\nconst term = new xterm__WEBPACK_IMPORTED_MODULE_4__[\"Terminal\"]({\n    convertEol: true,\n    theme: {\n        background: '#1d2026'\n    },\n    fontSize: 20,\n    enableBold: true\n});\n\nconst fit = new xterm_addon_fit__WEBPACK_IMPORTED_MODULE_5__[\"FitAddon\"]();\nterm.loadAddon(fit);\nlet terminal = document.getElementById('terminal');\nterm.open(terminal);\nfit.fit();\nterm.write('>> ')\n\nterminal.addEventListener('resize', fit.fit);\n\nlet runButton = document.getElementById('runButton');\nrunButton.addEventListener(\"click\", runCode);\n\nfunction runCode() {\n    var input = editor.state.doc.toString();\n    let output = bessy_wasm__WEBPACK_IMPORTED_MODULE_0__[\"evaluate\"](input);\n    console.log(output);\n    term.write(output);\n    term.write('>> ')\n}\n\nlet clearBtn = document.getElementById('clearBtn');\nclearBtn.addEventListener(\"click\", clearScreen);\nfunction clearScreen() {\n    term.write('\\x1bc');\n    term.write('>> ')\n}\n    \n\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/raw-loader/dist/cjs.js!./example.lox":
/*!***********************************************************!*\
  !*** ./node_modules/raw-loader/dist/cjs.js!./example.lox ***!
  \***********************************************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"print \\\"hello, wasm!\\\";\\n\\n// variable declaration\\nvar a = 10;\\nvar b = 2;\\nprint a + b;\\n\\n// control flow\\nif (2 > 1) {\\n    print true;\\n} else {\\n    print false;\\n}\\n\\n// for loop\\nfor (var i = 0; i < 10; i = i + 1) {\\n    print i;\\n}\\n\\n// while loop\\nvar a = 0;\\nvar e = 10;\\nwhile (a < e) {\\n    print a;\\n    a = a + 1;\\n}\\n\\n// NOTE: functions are yet to be implemented\\n// fun fact(num) {\\n//     if (num == 0) {\\n//         return 1;\\n//     } else {\\n//         return num * fact(num - 1);\\n//     }\\n// }\\n\");\n\n//# sourceURL=webpack:///./example.lox?./node_modules/raw-loader/dist/cjs.js");

/***/ })

}]);