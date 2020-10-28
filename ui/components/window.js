'use strict';

const { mod, m } = require('../settings.js');
const defaults = require('../defaults.js')(() => ({
    borderWidth: 2,
    width: window.innerHeight * 1.5,
    height: window.innerHeight * 0.85,
    boxSize: 14,
    openType: 'slow',
}));
module.exports = mod('window', (css, use, $, initial) => {
    use(require('./bracketed.js'));
    css(require('./window.module.scss'));
    css(require('./positioned.scss'));
    const attrs = defaults(initial.attrs);
    let drawInner = false;
    setTimeout(() => {
        drawInner = true;
        m.redraw();
    }, 800); // Boxes appear at this point; the inner content is also added at this point.
    return {
        view(vnode) {
            return $.div.window(
                {
                    style: {
                        __halfBorder: `${Math.ceil(attrs.borderWidth / 2)}px`,
                        __width: `${Math.round(attrs.width / 2) * 2}px`,
                        __height: `${Math.round(attrs.height / 2) * 2}px`,
                        __halfBoxSize: `${Math.round(attrs.boxSize / 2)}px`,
                    },
                },
                $.bracketed(
                    {
                        bracketSize: 100,
                        control: (a) => setTimeout(() => a(true), 1500),
                        enterTime: '0.8s',
                    },
                    $.div.container[attrs.openType][drawInner ? 'boxReady' : ''](
                        $.div.interior(drawInner ? vnode.children : []),
                        $.div.box.top.lef(),
                        $.div.box.bot.lef(),
                        $.div.box.top.rig(),
                        $.div.box.bot.rig()
                    )
                )
            );
        },
    };
});
