'use strict';

const { mod, m } = require('../settings.js');
const defaults = require('../defaults.js')(
    {
        bracketSize: (interior) => Math.min(interior.clientWidth, interior.clientHeight) / 2,
        bracketEnter: (bs) => bs, // Functions take in `interior` (the dom) as well.
        bracketMargin: (bs) => (bs * 3) / 10,
        control: () => {},
        enterTime: '0.3s',
        hover: false,
    },
    true
);

module.exports = mod('bracketed', (css, use, $, initial) => {
    css(require('./bracketed.module.scss'));
    css(require('./positioned.scss'));
    const attrs = defaults(initial.attrs);
    let enter = false;
    let interior;
    let id;
    return {
        oncreate() {
            id = interior.dom;
            m.redraw();
            attrs.control((a) => {
                enter = a;
                console.log(`Setting: ${enter}`);
                m.redraw();
            });
        },
        onbeforeremove() {
            // Make brackets fade if they exist.
        },
        view(vnode) {
            interior = $.div.interior(vnode.children);
            id = interior.dom ?? id;
            const style = {
                __bracketSize: '0px',
                __bracketEnterDistance: '0px',
                __bracketMargin: '0px',
                __enterTime: attrs.enterTime,
            };
            if (id !== undefined) {
                const bs = attrs.bracketSize(id);
                style.__bracketSize = `${bs}px`;
                style.__bracketEnterDistance = `${attrs.bracketEnter(bs, id)}px`;
                style.__bracketMargin = `${attrs.bracketMargin(bs, id)}px`;
            }
            return $.div.bracketed[attrs.hover ? 'enterHover' : ''][enter ? 'enter' : ''](
                {
                    style,
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
