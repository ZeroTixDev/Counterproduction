'use strict';

function folder(name, open = false, ...contents) {
    return {
        name,
        open,
        folder: true,
        contents,
    };
}
function file(name, open = false) {
    return {
        name,
        open,
        folder: false,
    };
}
// eslint-disable-next-line no-unused-vars
function consoleRenderTarget(
    globalCSS = {
        'font-family': `'Fira Code'`,
        'font-size': '12px',
        'line-height': '15px',
    }
) {
    return function render(strings, ...colors) {
        const outStr = strings.map((a) => a.replace('%', '%%')).join('%c');
        const acc = { arr: [], last: globalCSS };
        colors.forEach((c) => {
            Object.assign(acc.last, c);
            acc.arr.push(
                Object.entries(acc.last)
                    .map(([a, b]) => `${a}:${b};`)
                    .join('')
            );
        });
        console.log(outStr, ...acc.arr);
    };
}
// eslint-disable-next-line no-unused-vars
function terminalRenderTarget() {
    function hexToRgb(hex) {
        const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
        return {
            r: parseInt(result[1], 16),
            g: parseInt(result[2], 16),
            b: parseInt(result[3], 16),
        };
    }

    function renderColor(colors) {
        let str = '';
        if ('color' in colors) {
            const crgb = hexToRgb(colors.color);
            str += `\x1B[38;2;${crgb.r};${crgb.g};${crgb.b}m`;
        }
        if ('background-color' in colors) {
            const brgb = hexToRgb(colors['background-color']);
            str += `\x1B[48;2;${brgb.r};${brgb.g};${brgb.b}m`;
        }
        return str;
    }
    return function render(strings, ...colors) {
        let out = '';
        colors.forEach((c, i) => {
            out += strings[i] + renderColor(c);
        });
        out += strings[strings.length - 1];
        console.log(out);
    };
}

function color(color, chain = {}) {
    chain.color = color;
    return chain;
}
const clr = color;
function background(background, chain = {}) {
    chain['background-color'] = background;
    return chain;
}
const bg = background;

function snippet(strings, ...colors) {
    if (typeof strings === 'string') {
        console.assert(colors.length === 0);
        return {
            strings: [strings],
            colors: [],
            length: strings.length,
        };
    }
    return {
        strings,
        colors,
        length: strings.reduce((a, b) => a + b.length, 0),
    };
}
const snip = snippet;

function apply(str, color) {
    return {
        strings: ['', str],
        colors: [color],
        length: str.length,
    };
}
const app = apply;

function snippetCombine(...snippets) {
    return snippets.reduce((a, b) => ({
        strings: a.strings.slice(0, -1).concat(a.strings[a.strings.length - 1] + b.strings[0], ...b.strings.slice(1)),
        colors: a.colors.concat(b.colors),
        length: a.length + b.length,
    }));
}
const combine = snippetCombine;
function combineInPlace(root, ...snippets) {
    const comb = snippetCombine(root, ...snippets);
    Object.assign(root, comb);
    return root;
}
const cip = combineInPlace;
// TODO: MAKE THIS DECENTLY EFFICIENT
function cut(snippet, start, total) {
    const strings = [];
    let position = 0;
    snippet.strings.forEach((str) => {
        strings.push(str.substring(start - position, start - position + total));
        position += str.length;
    });
    if (start > position) {
        strings[strings.length - 1] += ' '.repeat(total);
    } else if (total + start > position) {
        strings[strings.length - 1] += ' '.repeat(total + start - position);
    }
    return {
        strings,
        colors: snippet.colors,
        length: total,
    };
}

function ws(n) {
    return snippet(' '.repeat(n));
}

function extend(a, b, start) {
    b.forEach((l, i) => cip(a[start + i], l));
}

function render(settings = {}) {
    // Default settings
    const s = {
        files: folder(
            'Autofactory',
            true,
            folder('.github'),
            folder('core'),
            folder('node_modules'),
            folder('prototype'),
            folder('prototype2', true, folder('.cargo'), folder('src'), file('build.rs'), file('Cargo.toml')),
            folder('target'),
            folder('test'),
            folder('ui'),
            file('.eslintrc.js'),
            file('.gitignore'),
            file('.prettierrc.js'),
            file('.stylelintrc.js'),
            file('.tern-project'),
            file('banner.png'),
            file('Cargo.lock'),
            file('Cargo.toml'),
            file('CODE_OF_CONDUCT.md'),
            file('CREDITS.md'),
            file('editor-console.js', true),
            file('LICENSE.md'),
            file('logo.png'),
            file('namesforgames.md'),
            file('package.json'),
            file('pnpm-lock.yaml'),
            file('pnpm-workspace.yaml'),
            file('proto.zip'),
            file('README.md'),
            file('rustfmt.toml'),
            file('text-colors.txt')
        ),
        text:
            "'use strict';\n\nfunction folder(name, open = false, ...contents) {\n    return {\n        name,\n        open,\n        folder: true,\n        contents,\n    };\n}\nfunction file(name, open = false) {\n    return {\n        name,\n        open,\n        folder: false,\n    };\n}\n// eslint-disable-next-line no-unused-vars\nfunction consoleRenderTarget(\n    globalCSS = {\n        'font-family': `'Fira Code'`,\n        'font-size': '12px',\n        'line-height': '15px',\n    }\n) {\n    return function render(strings, ...colors) {\n        const outStr = strings.map((a) => a.replace('%', '%%')).join('%c');\n        const acc = { arr: [], last: globalCSS };\n        colors.forEach((c) => {\n            Object.assign(acc.last, c);\n            acc.arr.push(\n                Object.entries(acc.last)\n                    .map(([a, b]) => `${a}:${b};`)\n                    .join('')\n            );\n        });\n        console.log(outStr, ...acc.arr);\n    };\n}\n// eslint-disable-next-line no-unused-vars\nfunction terminalRenderTarget() {\n    function hexToRgb(hex) {\n        const result = /^#?([a-f\\d]{2})([a-f\\d]{2})([a-f\\d]{2})$/i.exec(hex);\n        return {\n            r: parseInt(result[1], 16),\n            g: parseInt(result[2], 16),\n            b: parseInt(result[3], 16),\n        };\n    }\n\n    function renderColor(colors) {\n        let str = '';\n        if ('color' in colors) {\n            const crgb = hexToRgb(colors.color);\n            str += `\\x1B[38;2;${crgb.r};${crgb.g};${crgb.b}m`;\n        }\n        if ('background-color' in colors) {\n            const brgb = hexToRgb(colors['background-color']);\n            str += `\\x1B[48;2;${brgb.r};${brgb.g};${brgb.b}m`;\n        }\n        return str;\n    }\n    return function render(strings, ...colors) {\n        let out = '';\n        colors.forEach((c, i) => {\n            out += strings[i] + renderColor(c);\n        });\n        out += strings[strings.length - 1];\n        console.log(out);\n    };\n}\n\nfunction color(color, chain = {}) {\n    chain.color = color;\n    return chain;\n}\nconst clr = color;\nfunction background(background, chain = {}) {\n    chain['background-color'] = background;\n    return chain;\n}\nconst bg = background;\n\nfunction snippet(strings, ...colors) {\n    if (typeof strings === 'string') {\n        console.assert(colors.length === 0);\n        return {\n            strings: [strings],\n            colors: [],\n            length: strings.length,\n        };\n    }\n    return {\n        strings,\n        colors,\n        length: strings.reduce((a, b) => a + b.length, 0),\n    };\n}\nconst snip = snippet;\n\nfunction apply(str, color) {\n    return {\n        strings: ['', str],\n        colors: [color],\n        length: str.length,\n    };\n}\nconst app = apply;\n\nfunction snippetCombine(...snippets) {\n    return snippets.reduce((a, b) => ({\n        strings: a.strings.slice(0, -1).concat(a.strings[a.strings.length - 1] + b.strings[0], ...b.strings.slice(1)),\n        colors: a.colors.concat(b.colors),\n        length: a.length + b.length,\n    }));\n}\nconst combine = snippetCombine;\nfunction combineInPlace(root, ...snippets) {\n    const comb = snippetCombine(root, ...snippets);\n    Object.assign(root, comb);\n    return root;\n}\nconst cip = combineInPlace;\n// TODO: MAKE THIS DECENTLY EFFICIENT\nfunction cut(snippet, start, total) {\n    const strings = [];\n    let position = 0;\n    snippet.strings.forEach((str) => {\n        strings.push(str.substring(start - position, start - position + total));\n        position += str.length;\n    });\n    if (start > position) {\n        strings[strings.length - 1] += ' '.repeat(total);\n    } else if (total + start > position) {\n        strings[strings.length - 1] += ' '.repeat(total + start - position);\n    }\n    return {\n        strings,\n        colors: snippet.colors,\n        length: total,\n    };\n}\n\nfunction ws(n) {\n    return snippet(' '.repeat(n));\n}\n\nfunction extend(a, b, start) {\n    b.forEach((l, i) => cip(a[start + i], l));\n}\n\nfunction render(settings = {}) {\n    // Default settings\n    const s = {\n        files: folder(\n            'Autofactory',\n            true,\n            folder('.github'),\n            folder('core'),\n            folder('node_modules'),\n            folder('prototype'),\n            folder('prototype2', true, folder('.cargo'), folder('src'), file('build.rs'), file('Cargo.toml')),\n            folder('target'),\n            folder('test'),\n            folder('ui'),\n            file('.eslintrc.js'),\n            file('.gitignore'),\n            file('.prettierrc.js'),\n            file('.stylelintrc.js'),\n            file('.tern-project'),\n            file('banner.png'),\n            file('Cargo.lock'),\n            file('Cargo.toml'),\n            file('CODE_OF_CONDUCT.md'),\n            file('CREDITS.md'),\n            file('editor-console.js', true),\n            file('LICENSE.md'),\n            file('logo.png'),\n            file('namesforgames.md'),\n            file('package.json'),\n            file('pnpm-lock.yaml'),\n            file('pnpm-workspace.yaml'),\n            file('proto.zip'),\n            file('README.md'),\n            file('rustfmt.toml'),\n            file('text-colors.txt')\n        ),\n        text:\n            \"\",\n        // TODO: ALLOW SUPPORT FOR MD VIEWER AND WRAPPED TEXT\n        lineNumberStart: 1,\n        currentLine: 348,\n        lineNumPaddingLeft: 3,\n        lineNumPaddingRight: 3,\n        editorCorner: [0, 339],\n        sidebarSize: 30,\n        sidebarPadding: 3,\n        menuPadding: 2,\n        folderIndentation: 2,\n        folderSpacing: 1,\n        folderCorner: [0, 0],\n        tabs: ['editor-console.js', 'text-colors.txt', 'main.rs'],\n        tabActiveIndex: 0,\n        tabHoverIndex: 2,\n        tabSize: 22,\n        tabPadding: 2,\n        tabsSaved: [false, true, true],\n        totalSize: [220, 60], // width, height\n        renderTarget: terminalRenderTarget(),\n        colors: {\n            editorBg: '#343d46',\n            sidebarBg: '#2c333d',\n            scrollBg: '#37474f',\n            text: '#d8dee9',\n            currentLine: '#4e5a65',\n            tabHoverBg: '#2f3841',\n            tabHighlight: '#80ccc4',\n            inactive: '#546e7a',\n            activeFile: '#85bb48',\n            inactiveFolder: '#afbdc4',\n            lineNumber: '#868e98',\n        },\n        symbols: {\n            openFolder: '\u25BC',\n            closedFolder: '\u25B6',\n            file: ' ',\n            fileActive: '\u258E',\n            tabLeft: '<',\n            tabRight: '>', // Has spaces to readjust.\n            tabClose: '\u00D7',\n            tabUnsaved: '/',\n            tabActiveBottom: '\u2581',\n            tabSelect: '\u2630',\n            tabTooLong: '\u2026',\n            bottomScroll: '\u2585',\n            // Figure out what to do for right scrollbars.\n        },\n        // Add cursor?\n    };\n    Object.assign(s, settings);\n\n    function scroll(lines, eBox, left, bgColor) {\n        const width = Math.max(0, ...lines.map((l) => l.length));\n        const height = lines.length;\n        const leftWidth = Math.max(0, ...left.map((l) => l.length));\n        const res = Array(eBox.height)\n            .fill(null)\n            .map(() => snip``);\n\n        const iBox = {\n            left: eBox.left,\n            top: eBox.top,\n            width: eBox.width,\n            height: eBox.height,\n        };\n\n        iBox.width -= leftWidth;\n\n        if (iBox.left < 0 || iBox.top < 0) {\n            throw new Error('Invalid Scrolling Box.');\n        }\n\n        const hasVerticalScrollbar = iBox.height < height;\n        if (hasVerticalScrollbar) {\n            iBox.width--;\n        }\n        const hasHorizontalScrollbar = iBox.width < width;\n        if (hasHorizontalScrollbar) {\n            iBox.height--;\n        }\n        function computeScrollbar(total, box, boxTop) {\n            const size = Math.round((box * box) / total);\n            const top = Math.round((box * boxTop) / total);\n            return {\n                color(i) {\n                    // Confirm this works.\n                    if (i >= top && i - top < size) {\n                        return s.colors.scrollBg;\n                    } else {\n                        return s.colors.sidebarBg;\n                    }\n                },\n            };\n        }\n        const vScrollbar = computeScrollbar(height, iBox.height, iBox.top);\n        const hScrollbar = computeScrollbar(width, iBox.width, iBox.left);\n        res.forEach((l, i) => {\n            if (i >= iBox.height) return;\n            const loc = iBox.top + i;\n            cip(\n                l,\n                left[loc] ?? app(' '.repeat(leftWidth), bg(bgColor)),\n                lines[loc] === undefined ? snip`` : cut(lines[loc], iBox.left, iBox.width)\n            );\n        });\n        if (hasHorizontalScrollbar) {\n            const l = res[iBox.height];\n            cip(l, app('', bg(bgColor)));\n            for (let i = 0; i < iBox.width; i++) {\n                cip(l, app(s.symbols.bottomScroll, clr(hScrollbar.color(i))));\n            }\n        }\n        if (hasVerticalScrollbar) {\n            res.forEach((l, i) => {\n                if (i >= iBox.height) return;\n                cip(l, app(' ', bg(vScrollbar.color(i))));\n            });\n        }\n        if (hasVerticalScrollbar && hasHorizontalScrollbar) {\n            cip(res[iBox.height], app(' ', bg(s.colors.sidebarBg)));\n        }\n        return res;\n    }\n\n    const lines = Array(s.totalSize[1])\n        .fill(null)\n        .map(() => snip``);\n    // Sidebar\n    {\n        const fLines = [snip(' '.repeat(s.sidebarPadding) + 'FOLDERS')];\n\n        const descend = function descend(leftIndent, file) {\n            fLines.push(\n                ...Array(s.folderSpacing)\n                    .fill(null)\n                    .map(() => snip``)\n            );\n            if (file.folder) {\n                const l = ws(leftIndent);\n                if (file.open) {\n                    cip(l, app(s.symbols.openFolder, clr(s.colors.tabHighlight)));\n                } else {\n                    cip(l, app(s.symbols.closedFolder, clr(s.colors.inactive)), app('', clr(s.colors.inactiveFolder)));\n                }\n                cip(l, ws(1), snip(file.name));\n                fLines.push(l);\n                if (file.open) {\n                    file.contents.forEach((a) => descend(leftIndent + s.folderIndentation, a));\n                }\n            } else {\n                const l = snip``;\n                if (file.open) {\n                    cip(\n                        l,\n                        app(s.symbols.fileActive, clr(s.colors.activeFile)),\n                        app(' '.repeat(leftIndent + 1), clr(s.colors.text))\n                    );\n                } else {\n                    cip(l, app(' '.repeat(leftIndent + 2), clr(s.colors.inactive)));\n                }\n                cip(l, snip(file.name));\n                fLines.push(l);\n            }\n        };\n\n        descend(s.sidebarPadding, s.files);\n\n        const box = {\n            left: s.folderCorner[0],\n            top: s.folderCorner[1],\n            width: s.sidebarSize,\n            height: s.totalSize[1] - 1,\n        };\n        extend(\n            lines,\n            scroll(\n                fLines,\n                box,\n                fLines.map(() => app('', bg(s.colors.sidebarBg, clr(s.colors.text)))),\n                s.colors.sidebarBg\n            ),\n            1\n        );\n        lines.forEach((l) => cip(l, app(' '.repeat(s.sidebarSize - l.length), bg(s.colors.sidebarBg))));\n    }\n    // Tabs\n    {\n        // TODO: Make tabs automatically shrink to fit.\n        cip(lines[1], ws(2), app(s.symbols.tabLeft, clr(s.colors.inactive)), ws(2), snip(s.symbols.tabRight), ws(2));\n        [0, 2].forEach((l) => cip(lines[l], ws(8)));\n        cip(lines[2], app('', clr(s.colors.tabHighlight)));\n        const nameSpace = s.tabSize - 3 * s.tabPadding - 1;\n        s.tabs.forEach((tab, i) => {\n            const active = i === s.tabActiveIndex;\n            const hover = i === s.tabHoverIndex;\n            const saved = s.tabsSaved[i];\n            const bgColor = hover ? s.colors.tabHoverBg : s.colors.sidebarBg;\n            const textColor = active ? s.colors.text : s.colors.inactive;\n            const buttonColor = active || !saved ? s.colors.tabHighlight : s.colors.inactive;\n            const bottom = active ? s.symbols.tabActiveBottom : ' ';\n            const saveSymbol = saved ? s.symbols.tabClose : s.symbols.tabUnsaved;\n            [0, 1, 2].forEach((l) => cip(lines[l], app('', bg(bgColor))));\n            cip(lines[0], ws(s.tabSize));\n            cip(\n                lines[1],\n                ws(s.tabPadding),\n                app(\n                    tab.length > nameSpace\n                        ? tab.substr(0, nameSpace - 1) + s.symbols.tabTooLong\n                        : tab + ' '.repeat(nameSpace - tab.length),\n                    clr(textColor)\n                ),\n                ws(s.tabPadding),\n                app(saveSymbol, clr(buttonColor)),\n                ws(s.tabPadding)\n            );\n            cip(lines[2], snip(bottom.repeat(s.tabSize)));\n        });\n        [0, 1, 2].forEach((l) =>\n            cip(lines[l], app(' '.repeat(s.totalSize[0] - s.menuPadding - 1 - lines[l].length), bg(s.colors.sidebarBg)))\n        );\n        cip(lines[1], app(s.symbols.tabSelect, clr(s.colors.inactive)), ws(s.menuPadding));\n        [0, 2].forEach((l) => cip(lines[l], ws(s.menuPadding + 1)));\n    }\n    // Editor\n    {\n        // TODO: MAKE SURE THAT IT DOES THE CURRENT LINE HIGHLIGHT\n        const editorStart = 3;\n        const editorSpace = [s.totalSize[0] - s.sidebarSize, s.totalSize[1] - editorStart];\n        const textLines = s.text.split('\\n').map((a) => app(a, bg(s.colors.editorBg, clr(s.colors.text))));\n        // What if s.text is ''?\n        const maxLineLength = `${textLines.length}`.length;\n        const leftLines = textLines.map((_, i) => {\n            const lineNum = `${i + 1}`;\n            return combine(\n                app('', bg(i === s.currentLine ? s.colors.currentLine : s.colors.editorBg)),\n                ws(s.lineNumPaddingLeft),\n                app(lineNum, clr(s.colors.lineNumber)),\n                ws(maxLineLength - lineNum.length),\n                ws(s.lineNumPaddingRight)\n            );\n        });\n        textLines.push(\n            ...Array(editorSpace[1] - 1)\n                .fill(null)\n                .map(() => snip``)\n        );\n        leftLines.push(\n            ...Array(editorSpace[1] - 1)\n                .fill(null)\n                .map(() =>\n                    app(' '.repeat(s.lineNumPaddingLeft + maxLineLength + s.lineNumPaddingRight), bg(s.colors.editorBg))\n                )\n        );\n        const box = {\n            left: s.editorCorner[0],\n            top: s.editorCorner[1],\n            width: editorSpace[0],\n            height: editorSpace[1],\n        };\n        extend(lines, scroll(textLines, box, leftLines, s.colors.editorBg), editorStart);\n        // Right Scrollbar\n    }\n    const entireText = combine(...lines.map((l) => combine(app('\\n', bg('#000000')), l)));\n    s.renderTarget(entireText.strings, ...entireText.colors);\n}\nrender();\n",
        // TODO: ALLOW SUPPORT FOR MD VIEWER AND WRAPPED TEXT
        lineNumberStart: 1,
        currentLine: 348,
        lineNumPaddingLeft: 3,
        lineNumPaddingRight: 3,
        editorCorner: [0, 339],
        sidebarSize: 30,
        sidebarPadding: 3,
        menuPadding: 2,
        folderIndentation: 2,
        folderSpacing: 1,
        folderCorner: [0, 0],
        tabs: ['editor-console.js', 'text-colors.txt', 'main.rs'],
        tabActiveIndex: 0,
        tabHoverIndex: 2,
        tabSize: 22,
        tabPadding: 2,
        tabsSaved: [false, true, true],
        totalSize: [220, 60], // width, height
        renderTarget: terminalRenderTarget(),
        colors: {
            editorBg: '#343d46',
            sidebarBg: '#2c333d',
            scrollBg: '#37474f',
            text: '#d8dee9',
            currentLine: '#4e5a65',
            tabHoverBg: '#2f3841',
            tabHighlight: '#80ccc4',
            inactive: '#546e7a',
            activeFile: '#85bb48',
            inactiveFolder: '#afbdc4',
            lineNumber: '#868e98',
        },
        symbols: {
            openFolder: '▼',
            closedFolder: '▶',
            file: ' ',
            fileActive: '▎',
            tabLeft: '<',
            tabRight: '>', // Has spaces to readjust.
            tabClose: '×',
            tabUnsaved: '/',
            tabActiveBottom: '▁',
            tabSelect: '☰',
            tabTooLong: '…',
            bottomScroll: '▅',
            // Figure out what to do for right scrollbars.
        },
        // Add cursor?
    };
    Object.assign(s, settings);

    function scroll(lines, eBox, left, bgColor) {
        const width = Math.max(0, ...lines.map((l) => l.length));
        const height = lines.length;
        const leftWidth = Math.max(0, ...left.map((l) => l.length));
        const res = Array(eBox.height)
            .fill(null)
            .map(() => snip``);

        const iBox = {
            left: eBox.left,
            top: eBox.top,
            width: eBox.width,
            height: eBox.height,
        };

        iBox.width -= leftWidth;

        if (iBox.left < 0 || iBox.top < 0) {
            throw new Error('Invalid Scrolling Box.');
        }

        const hasVerticalScrollbar = iBox.height < height;
        if (hasVerticalScrollbar) {
            iBox.width--;
        }
        const hasHorizontalScrollbar = iBox.width < width;
        if (hasHorizontalScrollbar) {
            iBox.height--;
        }
        function computeScrollbar(total, box, boxTop) {
            const size = Math.round((box * box) / total);
            const top = Math.round((box * boxTop) / total);
            return {
                color(i) {
                    // Confirm this works.
                    if (i >= top && i - top < size) {
                        return s.colors.scrollBg;
                    } else {
                        return s.colors.sidebarBg;
                    }
                },
            };
        }
        const vScrollbar = computeScrollbar(height, iBox.height, iBox.top);
        const hScrollbar = computeScrollbar(width, iBox.width, iBox.left);
        res.forEach((l, i) => {
            if (i >= iBox.height) return;
            const loc = iBox.top + i;
            cip(
                l,
                left[loc] ?? app(' '.repeat(leftWidth), bg(bgColor)),
                lines[loc] === undefined ? snip`` : cut(lines[loc], iBox.left, iBox.width)
            );
        });
        if (hasHorizontalScrollbar) {
            const l = res[iBox.height];
            cip(l, app('', bg(bgColor)));
            for (let i = 0; i < iBox.width; i++) {
                cip(l, app(s.symbols.bottomScroll, clr(hScrollbar.color(i))));
            }
        }
        if (hasVerticalScrollbar) {
            res.forEach((l, i) => {
                if (i >= iBox.height) return;
                cip(l, app(' ', bg(vScrollbar.color(i))));
            });
        }
        if (hasVerticalScrollbar && hasHorizontalScrollbar) {
            cip(res[iBox.height], app(' ', bg(s.colors.sidebarBg)));
        }
        return res;
    }

    const lines = Array(s.totalSize[1])
        .fill(null)
        .map(() => snip``);
    // Sidebar
    {
        const fLines = [snip(' '.repeat(s.sidebarPadding) + 'FOLDERS')];

        const descend = function descend(leftIndent, file) {
            fLines.push(
                ...Array(s.folderSpacing)
                    .fill(null)
                    .map(() => snip``)
            );
            if (file.folder) {
                const l = ws(leftIndent);
                if (file.open) {
                    cip(l, app(s.symbols.openFolder, clr(s.colors.tabHighlight)));
                } else {
                    cip(l, app(s.symbols.closedFolder, clr(s.colors.inactive)), app('', clr(s.colors.inactiveFolder)));
                }
                cip(l, ws(1), snip(file.name));
                fLines.push(l);
                if (file.open) {
                    file.contents.forEach((a) => descend(leftIndent + s.folderIndentation, a));
                }
            } else {
                const l = snip``;
                if (file.open) {
                    cip(
                        l,
                        app(s.symbols.fileActive, clr(s.colors.activeFile)),
                        app(' '.repeat(leftIndent + 1), clr(s.colors.text))
                    );
                } else {
                    cip(l, app(' '.repeat(leftIndent + 2), clr(s.colors.inactive)));
                }
                cip(l, snip(file.name));
                fLines.push(l);
            }
        };

        descend(s.sidebarPadding, s.files);

        const box = {
            left: s.folderCorner[0],
            top: s.folderCorner[1],
            width: s.sidebarSize,
            height: s.totalSize[1] - 1,
        };
        extend(
            lines,
            scroll(
                fLines,
                box,
                fLines.map(() => app('', bg(s.colors.sidebarBg, clr(s.colors.text)))),
                s.colors.sidebarBg
            ),
            1
        );
        lines.forEach((l) => cip(l, app(' '.repeat(s.sidebarSize - l.length), bg(s.colors.sidebarBg))));
    }
    // Tabs
    {
        // TODO: Make tabs automatically shrink to fit.
        cip(lines[1], ws(2), app(s.symbols.tabLeft, clr(s.colors.inactive)), ws(2), snip(s.symbols.tabRight), ws(2));
        [0, 2].forEach((l) => cip(lines[l], ws(8)));
        cip(lines[2], app('', clr(s.colors.tabHighlight)));
        const nameSpace = s.tabSize - 3 * s.tabPadding - 1;
        s.tabs.forEach((tab, i) => {
            const active = i === s.tabActiveIndex;
            const hover = i === s.tabHoverIndex;
            const saved = s.tabsSaved[i];
            const bgColor = hover ? s.colors.tabHoverBg : s.colors.sidebarBg;
            const textColor = active ? s.colors.text : s.colors.inactive;
            const buttonColor = active || !saved ? s.colors.tabHighlight : s.colors.inactive;
            const bottom = active ? s.symbols.tabActiveBottom : ' ';
            const saveSymbol = saved ? s.symbols.tabClose : s.symbols.tabUnsaved;
            [0, 1, 2].forEach((l) => cip(lines[l], app('', bg(bgColor))));
            cip(lines[0], ws(s.tabSize));
            cip(
                lines[1],
                ws(s.tabPadding),
                app(
                    tab.length > nameSpace
                        ? tab.substr(0, nameSpace - 1) + s.symbols.tabTooLong
                        : tab + ' '.repeat(nameSpace - tab.length),
                    clr(textColor)
                ),
                ws(s.tabPadding),
                app(saveSymbol, clr(buttonColor)),
                ws(s.tabPadding)
            );
            cip(lines[2], snip(bottom.repeat(s.tabSize)));
        });
        [0, 1, 2].forEach((l) =>
            cip(lines[l], app(' '.repeat(s.totalSize[0] - s.menuPadding - 1 - lines[l].length), bg(s.colors.sidebarBg)))
        );
        cip(lines[1], app(s.symbols.tabSelect, clr(s.colors.inactive)), ws(s.menuPadding));
        [0, 2].forEach((l) => cip(lines[l], ws(s.menuPadding + 1)));
    }
    // Editor
    {
        // TODO: MAKE SURE THAT IT DOES THE CURRENT LINE HIGHLIGHT
        const editorStart = 3;
        const editorSpace = [s.totalSize[0] - s.sidebarSize, s.totalSize[1] - editorStart];
        const textLines = s.text.split('\n').map((a) => app(a, bg(s.colors.editorBg, clr(s.colors.text))));
        // What if s.text is ''?
        const maxLineLength = `${textLines.length}`.length;
        const leftLines = textLines.map((_, i) => {
            const lineNum = `${i + 1}`;
            return combine(
                app('', bg(i === s.currentLine ? s.colors.currentLine : s.colors.editorBg)),
                ws(s.lineNumPaddingLeft),
                app(lineNum, clr(s.colors.lineNumber)),
                ws(maxLineLength - lineNum.length),
                ws(s.lineNumPaddingRight)
            );
        });
        textLines.push(
            ...Array(editorSpace[1] - 1)
                .fill(null)
                .map(() => snip``)
        );
        leftLines.push(
            ...Array(editorSpace[1] - 1)
                .fill(null)
                .map(() =>
                    app(' '.repeat(s.lineNumPaddingLeft + maxLineLength + s.lineNumPaddingRight), bg(s.colors.editorBg))
                )
        );
        const box = {
            left: s.editorCorner[0],
            top: s.editorCorner[1],
            width: editorSpace[0],
            height: editorSpace[1],
        };
        extend(lines, scroll(textLines, box, leftLines, s.colors.editorBg), editorStart);
        // Right Scrollbar
    }
    const entireText = combine(...lines.map((l) => combine(app('\n', bg('#000000')), l)));
    s.renderTarget(entireText.strings, ...entireText.colors);
}
render();
