'use strict';

const { mod, m } = require('../settings.js');

module.exports = mod('bracketed', (css, use, $, initial) => {
    css(require('./bracketed.module.scss'));
    css(require('./positioned.scss'));
    let enter = initial.attrs?.enterOnHover ? 'enterHover' : '';
    let bs = Math.floor((initial.attrs?.bracketSize ?? 0) / 2) * 2;
    let interior;
    function computeBracketSize(interior) {
        console.log(interior.dom);
        return interior.dom == undefined ? 0 : Math.min(interior.dom.clientWidth, interior.dom.clientHeight);
    }
    return {
        oncreate(vnode) {
            if (enter !== '') return;
            setTimeout(() => {
                enter = 'enterNormal';
                m.redraw();
            }, vnode.attrs.enterDelay ?? 150); // undefined means instant.
        },
        onbeforeupdate(vnode, old) {
            if (vnode.attrs?.bracketSize === undefined) {
                bs = Math.floor(computeBracketSize(old.children[0]) / 2) * 2;
            }
            // Set the bracket styles.
        },
        onbeforeremove() {
            // Make brackets fade if they exist.
        },
        view(vnode) {
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
