'use strict';

const { mod, m } = require('../settings.js');
const defaults = require('../defaults.js')(() => ({
    width: () => window.innerHeight * 1.5,
    height: () => window.innerHeight * 0.85,
    openType: 'slow',
}));
module.exports = mod('window', (css, use, $, initial) => {
    css(require('./window.module.scss'));
    use(require('./bracketed.js'));
    css(require('./positioned.scss'));
    const attrs = defaults(initial.attrs);
    let drawInner = false;
    setTimeout(() => {
        drawInner = true;
        m.redraw();
    }, 390); // Boxes appear at this point; the inner content is also added at this point.
    return {
        view(vnode) {
            return $._.window(
                {
                    style: {
                        __halfBorder: `1px`,
                        __width: `${Math.round(attrs.width() / 2) * 2}px`,
                        __height: `${Math.round(attrs.height() / 2) * 2}px`,
                        __halfBoxSize: `7px`,
                    },
                },
                $.bracketed(
                    {
                        bracketSize: 100,
                        control: (a) => setTimeout(() => a(true), 1500),
                        enterTime: '0.8s',
                    },
                    $._.container[attrs.openType][drawInner ? 'boxReady' : ''](
                        $._.interior(drawInner ? vnode.children : []),
                        $._.box.top.lef(),
                        $._.box.bot.lef(),
                        $._.box.top.rig(),
                        $._.box.bot.rig()
                    )
                )
            );
        },
    };
});
