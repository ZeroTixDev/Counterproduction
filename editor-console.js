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
            "'use strict';\r\n\r\nfunction folder(name, open = false, ...contents) {\r\n    return {\r\n        name,\r\n        open,\r\n        folder: true,\r\n        contents,\r\n    };\r\n}\r\nfunction file(name, open = false) {\r\n    return {\r\n        name,\r\n        open,\r\n        folder: false,\r\n    };\r\n}\r\n\r\nfunction consoleRenderTarget(\r\n    globalCSS = {\r\n        'font-family': `'Fira Code'`,\r\n        'font-size': '12px',\r\n        'line-height': '15px',\r\n    }\r\n) {\r\n    return function render(strings, ...colors) {\r\n        const outStr = strings.map((a) => a.replace('%', '%%')).join('%c');\r\n        const acc = { arr: [], last: globalCSS };\r\n        colors.forEach((c) => {\r\n            Object.assign(acc.last, c);\r\n            acc.arr.push(\r\n                Object.entries(acc.last)\r\n                    .map(([a, b]) => `${a}:${b};`)\r\n                    .join('')\r\n            );\r\n        });\r\n        console.log(outStr, ...acc.arr);\r\n    };\r\n}\r\n\r\nfunction color(color, chain = {}) {\r\n    chain.color = color;\r\n    return chain;\r\n}\r\nconst clr = color;\r\nfunction background(background, chain = {}) {\r\n    chain['background-color'] = background;\r\n    return chain;\r\n}\r\nconst bg = background;\r\n\r\nfunction snippet(strings, ...colors) {\r\n    if (typeof strings === 'string') {\r\n        console.assert(colors.length === 0);\r\n        return {\r\n            strings: [strings],\r\n            colors: [],\r\n            length: strings.length,\r\n        };\r\n    }\r\n    return {\r\n        strings,\r\n        colors,\r\n        length: strings.reduce((a, b) => a + b.length, 0),\r\n    };\r\n}\r\nconst snip = snippet;\r\n\r\nfunction apply(str, color) {\r\n    return {\r\n        strings: ['', str],\r\n        colors: [color],\r\n        length: str.length,\r\n    };\r\n}\r\nconst app = apply;\r\n\r\nfunction snippetCombine(...snippets) {\r\n    return snippets.reduce((a, b) => ({\r\n        strings: a.strings.slice(0, -1).concat(a.strings[a.strings.length - 1] + b.strings[0], ...b.strings.slice(1)),\r\n        colors: a.colors.concat(b.colors),\r\n        length: a.length + b.length,\r\n    }));\r\n}\r\nconst combine = snippetCombine;\r\nfunction combineInPlace(root, ...snippets) {\r\n    const comb = snippetCombine(root, ...snippets);\r\n    Object.assign(root, comb);\r\n    return root;\r\n}\r\nconst cip = combineInPlace;\r\n// TODO: MAKE THIS DECENTLY EFFICIENT\r\nfunction cut(snippet, start, total) {\r\n    const strings = [];\r\n    let position = 0;\r\n    snippet.strings.forEach((str) => {\r\n        strings.push(str.substring(start - position, start - position + total));\r\n        position += str.length;\r\n    });\r\n    if (start > position) {\r\n        strings[strings.length - 1] += ' '.repeat(total);\r\n    } else if (total + start > position) {\r\n        strings[strings.length - 1] += ' '.repeat(total + start - position);\r\n    }\r\n    return {\r\n        strings,\r\n        colors: snippet.colors,\r\n        length: total,\r\n    };\r\n}\r\n\r\nfunction ws(n) {\r\n    return snippet(' '.repeat(n));\r\n}\r\n\r\nfunction extend(a, b, start) {\r\n    b.forEach((l, i) => cip(a[start + i], l));\r\n}\r\n\r\nfunction render(settings = {}) {\r\n    // Default settings\r\n    const s = {\r\n        files: folder(\r\n            'Autofactory',\r\n            true,\r\n            folder('.github'),\r\n            folder('core'),\r\n            folder('node_modules'),\r\n            folder('prototype'),\r\n            folder('prototype2', true, folder('.cargo'), folder('src'), file('build.rs'), file('Cargo.toml')),\r\n            folder('target'),\r\n            folder('test'),\r\n            folder('ui'),\r\n            file('.eslintrc.js'),\r\n            file('.gitignore'),\r\n            file('.prettierrc.js'),\r\n            file('.stylelintrc.js'),\r\n            file('.tern-project'),\r\n            file('banner.png'),\r\n            file('Cargo.lock'),\r\n            file('Cargo.toml'),\r\n            file('CODE_OF_CONDUCT.md'),\r\n            file('CREDITS.md'),\r\n            file('editor-console.js', true),\r\n            file('LICENSE.md'),\r\n            file('logo.png'),\r\n            file('namesforgames.md'),\r\n            file('package.json'),\r\n            file('pnpm-lock.yaml'),\r\n            file('pnpm-workspace.yaml'),\r\n            file('proto.zip'),\r\n            file('README.md'),\r\n            file('rustfmt.toml'),\r\n            file('text-colors.txt')\r\n        ),\r\n        text:\r\n            \"\",\r\n        // TODO: ALLOW SUPPORT FOR MD VIEWER AND WRAPPED TEXT\r\n        lineNumberStart: 1,\r\n        currentLine: 1,\r\n        lineNumPaddingLeft: 3,\r\n        lineNumPaddingRight: 3,\r\n        editorCorner: [0, 339],\r\n        sidebarSize: 30,\r\n        sidebarPadding: 3,\r\n        menuPadding: 2,\r\n        folderIndentation: 2,\r\n        folderSpacing: 1,\r\n        folderCorner: [0, 0],\r\n        tabs: ['editor-console.js', 'text-colors.txt', 'main.rs'],\r\n        tabActiveIndex: 0,\r\n        tabHoverIndex: 2,\r\n        tabSize: 22,\r\n        tabPadding: 2,\r\n        tabsSaved: [false, true, true],\r\n        totalSize: [220, 60], // width, height\r\n        renderTarget: consoleRenderTarget(),\r\n        colors: {\r\n            editorBg: '#343d46',\r\n            sidebarBg: '#2c333d',\r\n            scrollBg: '#37474f',\r\n            text: '#d8dee9',\r\n            currentLine: '#4e5a65',\r\n            tabHoverBg: '#2f3841',\r\n            tabHighlight: '#80ccc4',\r\n            inactive: '#546e7a',\r\n            activeFile: '#85bb48',\r\n            inactiveFolder: '#afbdc4',\r\n            lineNumber: '#868e98',\r\n        },\r\n        symbols: {\r\n            openFolder: '\u25BC',\r\n            closedFolder: '\u25B6',\r\n            file: ' ',\r\n            fileActive: '\u258E',\r\n            tabLeft: '<',\r\n            tabRight: '>', // Has spaces to readjust.\r\n            tabClose: '\u00D7',\r\n            tabUnsaved: '/',\r\n            tabActiveBottom: '\u2581',\r\n            tabSelect: '\u2630',\r\n            tabTooLong: '\u2026',\r\n            bottomScroll: '\u2585',\r\n            // Figure out what to do for right scrollbars.\r\n        },\r\n        // Add cursor?\r\n    };\r\n    Object.assign(s, settings);\r\n\r\n    function scroll(lines, eBox, left, bgColor) {\r\n        const width = Math.max(0, ...lines.map((l) => l.length));\r\n        const height = lines.length;\r\n        const leftWidth = Math.max(0, ...left.map((l) => l.length));\r\n        const res = Array(eBox.height)\r\n            .fill(null)\r\n            .map(() => snip``);\r\n\r\n        const iBox = {\r\n            left: eBox.left,\r\n            top: eBox.top,\r\n            width: eBox.width,\r\n            height: eBox.height,\r\n        };\r\n\r\n        iBox.width -= leftWidth;\r\n\r\n        if (iBox.left < 0 || iBox.top < 0) {\r\n            throw new Error('Invalid Scrolling Box.');\r\n        }\r\n\r\n        const hasVerticalScrollbar = iBox.height < height;\r\n        if (hasVerticalScrollbar) {\r\n            iBox.width--;\r\n        }\r\n        const hasHorizontalScrollbar = iBox.width < width;\r\n        if (hasHorizontalScrollbar) {\r\n            iBox.height--;\r\n        }\r\n        function computeScrollbar(total, box, boxTop) {\r\n            const size = Math.round((box * box) / total);\r\n            const top = Math.round((box * boxTop) / total);\r\n            return {\r\n                color(i) {\r\n                    // Confirm this works.\r\n                    if (i >= top && i - top < size) {\r\n                        return s.colors.scrollBg;\r\n                    } else {\r\n                        return s.colors.sidebarBg;\r\n                    }\r\n                },\r\n            };\r\n        }\r\n        const vScrollbar = computeScrollbar(height, iBox.height, iBox.top);\r\n        const hScrollbar = computeScrollbar(width, iBox.width, iBox.left);\r\n        res.forEach((l, i) => {\r\n            if (i >= iBox.height) return;\r\n            const loc = iBox.top + i;\r\n            cip(\r\n                l,\r\n                left[loc] ?? app(' '.repeat(leftWidth), bg(bgColor)),\r\n                lines[loc] === undefined ? snip`` : cut(lines[loc], iBox.left, iBox.width)\r\n            );\r\n        });\r\n        if (hasHorizontalScrollbar) {\r\n            const l = res[iBox.height];\r\n            cip(l, app('', bg(bgColor)));\r\n            for (let i = 0; i < iBox.width; i++) {\r\n                cip(l, app(s.symbols.bottomScroll, clr(hScrollbar.color(i))));\r\n            }\r\n        }\r\n        if (hasVerticalScrollbar) {\r\n            res.forEach((l, i) => {\r\n                if (i >= iBox.height) return;\r\n                cip(l, app(' ', bg(vScrollbar.color(i))));\r\n            });\r\n        }\r\n        if (hasVerticalScrollbar && hasHorizontalScrollbar) {\r\n            cip(res[iBox.height], app(' ', bg(s.colors.sidebarBg)));\r\n        }\r\n        return res;\r\n    }\r\n\r\n    const lines = Array(s.totalSize[1])\r\n        .fill(null)\r\n        .map(() => snip``);\r\n    // Sidebar\r\n\r\n    {\r\n        // TODO: ACTUALLY ADD FOLDERS\r\n        const fLines = [snip(' '.repeat(s.sidebarPadding) + 'FOLDERS')];\r\n\r\n        const descend = function descend(leftIndent, file) {\r\n            fLines.push(\r\n                ...Array(s.folderSpacing)\r\n                    .fill(null)\r\n                    .map(() => snip``)\r\n            );\r\n            if (file.folder) {\r\n                const l = ws(leftIndent);\r\n                if (file.open) {\r\n                    cip(l, app(s.symbols.openFolder, clr(s.colors.tabHighlight)));\r\n                } else {\r\n                    cip(l, app(s.symbols.closedFolder, clr(s.colors.inactive)), app('', clr(s.colors.inactiveFolder)));\r\n                }\r\n                cip(l, ws(1), snip(file.name));\r\n                fLines.push(l);\r\n                if (file.open) {\r\n                    file.contents.forEach((a) => descend(leftIndent + s.folderIndentation, a));\r\n                }\r\n            } else {\r\n                const l = snip``;\r\n                if (file.open) {\r\n                    cip(\r\n                        l,\r\n                        app(s.symbols.fileActive, clr(s.colors.activeFile)),\r\n                        app(' '.repeat(leftIndent + 1), clr(s.colors.text))\r\n                    );\r\n                } else {\r\n                    cip(l, app(' '.repeat(leftIndent + 2), clr(s.colors.inactive)));\r\n                }\r\n                cip(l, snip(file.name));\r\n                fLines.push(l);\r\n            }\r\n        };\r\n\r\n        descend(s.sidebarPadding, s.files);\r\n\r\n        const box = {\r\n            left: s.folderCorner[0],\r\n            top: s.folderCorner[1],\r\n            width: s.sidebarSize,\r\n            height: s.totalSize[1] - 1,\r\n        };\r\n        extend(\r\n            lines,\r\n            scroll(\r\n                fLines,\r\n                box,\r\n                fLines.map(() => app('', bg(s.colors.sidebarBg, clr(s.colors.text)))),\r\n                s.colors.sidebarBg\r\n            ),\r\n            1\r\n        );\r\n        lines.forEach((l) => cip(l, app(' '.repeat(s.sidebarSize - l.length), bg(s.colors.sidebarBg))));\r\n    }\r\n    // Tabs\r\n    {\r\n        // TODO: Make tabs automatically shrink to fit.\r\n        cip(lines[1], ws(2), app(s.symbols.tabLeft, clr(s.colors.inactive)), ws(2), snip(s.symbols.tabRight), ws(2));\r\n        [0, 2].forEach((l) => cip(lines[l], ws(8)));\r\n        cip(lines[2], app('', clr(s.colors.tabHighlight)));\r\n        const nameSpace = s.tabSize - 3 * s.tabPadding - 1;\r\n        s.tabs.forEach((tab, i) => {\r\n            const active = i === s.tabActiveIndex;\r\n            const hover = i === s.tabHoverIndex;\r\n            const saved = s.tabsSaved[i];\r\n            const bgColor = hover ? s.colors.tabHoverBg : s.colors.sidebarBg;\r\n            const textColor = active ? s.colors.text : s.colors.inactive;\r\n            const buttonColor = active || !saved ? s.colors.tabHighlight : s.colors.inactive;\r\n            const bottom = active ? s.symbols.tabActiveBottom : ' ';\r\n            const saveSymbol = saved ? s.symbols.tabClose : s.symbols.tabUnsaved;\r\n            [0, 1, 2].forEach((l) => cip(lines[l], app('', bg(bgColor))));\r\n            cip(lines[0], ws(s.tabSize));\r\n            cip(\r\n                lines[1],\r\n                ws(s.tabPadding),\r\n                app(\r\n                    tab.length > nameSpace\r\n                        ? tab.substr(0, nameSpace - 1) + s.symbols.tabTooLong\r\n                        : tab + ' '.repeat(nameSpace - tab.length),\r\n                    clr(textColor)\r\n                ),\r\n                ws(s.tabPadding),\r\n                app(saveSymbol, clr(buttonColor)),\r\n                ws(s.tabPadding)\r\n            );\r\n            cip(lines[2], snip(bottom.repeat(s.tabSize)));\r\n        });\r\n        [0, 1, 2].forEach((l) =>\r\n            cip(lines[l], app(' '.repeat(s.totalSize[0] - s.menuPadding - 1 - lines[l].length), bg(s.colors.sidebarBg)))\r\n        );\r\n        cip(lines[1], app(s.symbols.tabSelect, clr(s.colors.inactive)), ws(s.menuPadding));\r\n        [0, 2].forEach((l) => cip(lines[l], ws(s.menuPadding + 1)));\r\n    }\r\n    // Editor\r\n    {\r\n        // TODO: MAKE SURE THAT IT DOES THE CURRENT LINE HIGHLIGHT\r\n        const editorStart = 3;\r\n        const editorSpace = [s.totalSize[0] - s.sidebarSize, s.totalSize[1] - editorStart];\r\n        const textLines = s.text.split('\\n').map((a) => app(a, bg(s.colors.editorBg, clr(s.colors.text))));\r\n        // What if s.text is ''?\r\n        const maxLineLength = `${textLines.length}`.length;\r\n        console.log(maxLineLength);\r\n        const leftLines = textLines.map((_, i) => {\r\n            const lineNum = `${i + 1}`;\r\n            return combine(\r\n                app('', bg(i === s.currentLine ? s.colors.currentLine : s.colors.editorBg)),\r\n                ws(s.lineNumPaddingLeft),\r\n                app(lineNum, clr(s.colors.lineNumber)),\r\n                ws(maxLineLength - lineNum.length),\r\n                ws(s.lineNumPaddingRight)\r\n            );\r\n        });\r\n        textLines.push(\r\n            ...Array(editorSpace[1] - 1)\r\n                .fill(null)\r\n                .map(() => snip``)\r\n        );\r\n        leftLines.push(\r\n            ...Array(editorSpace[1] - 1)\r\n                .fill(null)\r\n                .map(() =>\r\n                    app(' '.repeat(s.lineNumPaddingLeft + maxLineLength + s.lineNumPaddingRight), bg(s.colors.editorBg))\r\n                )\r\n        );\r\n        const box = {\r\n            left: s.editorCorner[0],\r\n            top: s.editorCorner[1],\r\n            width: editorSpace[0],\r\n            height: editorSpace[1],\r\n        };\r\n        extend(lines, scroll(textLines, box, leftLines, s.colors.editorBg), editorStart);\r\n        // Right Scrollbar\r\n    }\r\n    const entireText = combine(...lines.map((l) => cip(snip`\\n`, l)));\r\n    s.renderTarget(entireText.strings, ...entireText.colors);\r\n}\r\nrender();\r\n",
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
        renderTarget: consoleRenderTarget(),
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
        // TODO: ACTUALLY ADD FOLDERS
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
            cip(lines[l], app(' '.repeat(s.totalSize[0] - s.menuPadding - 2 - lines[l].length), bg(s.colors.sidebarBg)))
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
        console.log(maxLineLength);
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
    const entireText = combine(...lines.map((l) => cip(snip`\n`, l)));
    s.renderTarget(entireText.strings, ...entireText.colors);
}
render();
