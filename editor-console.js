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
        files: folder('Folder', true, folder('Interior', false), file('A', false), file('B', true)),
        text: `Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.`,
        // TODO: ALLOW SUPPORT FOR MD VIEWER AND WRAPPED TEXT
        lineNumberStart: 1,
        currentLine: 1,
        lineNumPaddingLeft: 3,
        lineNumPaddingRight: 3,
        editorCorner: [0, 0],
        sidebarSize: 30,
        sidebarPadding: 3,
        menuPadding: 2,
        folderIndentation: 2,
        folderSpacing: 1,
        folderCorner: [0, 0],
        tabs: ['foo', 'bar', 'some-really-long-file-name'],
        tabActiveIndex: 0,
        tabHoverIndex: 2,
        tabSize: 22,
        tabPadding: 2,
        tabsSaved: [false, false, true],
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
        const fLines = `FOLDERS

v Autofactory

  > .github

  > core

  > node_modules

  > prototype

  > prototype2

  > target

  > test

  > ui

    .eslintrc.js

    .gitignore

    .prettierrc.js

    .stylelintrc.js
`
            .split('\n')
            .map((a) => snip(' '.repeat(s.sidebarPadding) + a));

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
