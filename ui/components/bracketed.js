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
                const bs = Math.ceil(attrs.bracketSize(id) / 2) * 2;
                style.__bracketSize = `${bs}px`;
                style.__bracketEnterDistance = `${Math.round(attrs.bracketEnter(bs, id) / 2) * 2}px`;
                style.__bracketMargin = `${Math.round(attrs.bracketMargin(bs, id) / 2) * 2}px`;
            }
            return $._.bracketed[attrs.hover ? 'enterHover' : ''][enter ? 'enter' : ''](
                {
                    style,
                },
                interior,
                $._.bracket.top.lef(),
                $._.bracket.bot.lef(),
                $._.bracket.top.rig(),
                $._.bracket.bot.rig()
            );
        },
    };
});
