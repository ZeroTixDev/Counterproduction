'use strict';

const { mod, m } = require('../settings.js');

module.exports = mod('bracketed', (css, use, $, vnode) => {
    css(require('./bracketed.module.scss'));
    css(require('./positioned.scss'));
    let enter = vnode.attrs?.enterOnHover ? 'enterHover' : '';
    let bs = Math.floor((vnode.attrs?.bracketSize ?? 0) / 2) * 2;
    let interior;
    function computeBracketSize(interior) {
        console.log(interior.dom);
        return interior.dom == undefined ? 0 : Math.min(interior.dom.clientWidth, interior.dom.clientHeight);
    }
    return {
        oncreate() {
            if (enter !== '') return;
            setTimeout(() => {
                enter = 'enterNormal';
                m.redraw();
            }, vnode.attrs.enterDelay ?? 150); // undefined means instant.
        },
        onbeforeupdate(_, old) {
            if (vnode.attrs?.bracketSize === undefined) {
                bs = Math.floor(computeBracketSize(old.children[0]) / 2) * 2;
            }
            // Set the bracket styles.
        },
        onbeforeremove() {
            // Make brackets fade if they exist.
        },
        view() {
            console.log('Viewing');
            console.log(enter);
            interior = $.div.interior(vnode.children);
            return $.div.bracketed[enter](
                {
                    style: {
                        __bracketSize: `${bs}px`,
                        __bracketEnterDistance: `${bs}px`,
                        __bracketMargin: `${Math.floor(vnode.attrs?.bracketMargin ?? (bs * 3) / 10)}px`,
                    },
                },
                interior,
                $.div.bracket.top.lef(),
                $.div.bracket.bot.lef(),
                $.div.bracket.top.rig(),
                $.div.bracket.bot.rig()
            );
        },
    };
});