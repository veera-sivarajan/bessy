const colors = {
    'red': '#cc241d',
    'green': '#b8bb26',
    'yellow': '#d79921',
    'blue': '#458588',
    'purple': '#b16286',
    'aqua': '#8ec07c',
    'gray': '#a89984',
    'darkgray': '#282828'
}
const gruvboxesque = editor => {
    let code = editor.textContent;
    var highlights = new Array(
        { search: /(\d+?)/g, replace: `<font color=${colors['purple']}>$1</font>` },
        { search: /(\/\/.*\n)/g, replace: `<font color=${colors['gray']}>$1</font>` },
        { search: /\b(if|else|for|while)\b/g, replace: `<font color=${colors['red']}>$1</font>` },
        { search: /\b(var)\b/g, replace: `<font color=${colors['blue']}>$1</font>` },
        { search: /\b(fun|λ)\b/g, replace: `<font color=${colors['aqua']}>$1</font>` },
        { search: /(\".*?\")/g, replace: `<font color=${colors['green']}>$1</font>` },
        { search: /\b(true|false)\b/g, replace: `<font color=${colors['purple']}>$1</font>` },
        { search: /\b(return)\b/g, replace: `<font color=${colors['red']}>$1</font>` },
        { search: /(\w+?)(\()/g, replace: `<font color=${colors['green']}>$1</font>$2` },
    );
    for (let i = 0; i < highlights.length; i++) {
        code = code.replace(highlights[i].search, highlights[i].replace)
    }
    editor.innerHTML = code;
};

// // ------- Rainbox lolcat --------
const rainbow = (ch, i) => {
    let red = Math.round(Math.sin(0.01 * i + 0) * 127 + 128);
    let green = Math.round(Math.sin(0.01 * i + 2 * Math.PI / 3) * 127 + 128);
    let blue = Math.round(Math.sin(0.01 * i + 4 * Math.PI / 3) * 127 + 128);
    return `<span style="color: rgb(${red}, ${green}, ${blue})">${ch}</span>`;
};

const lolcat = editor => {
    const code = editor.textContent
        .split('')
        .map(rainbow)
        .join('');
    editor.innerHTML = code;
};


export { gruvboxesque };
