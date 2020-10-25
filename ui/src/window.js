'use strict';

const { mod, m } = require('../settings.js');
module.exports = mod('window', (css, use, $, vnode) => {
    css(require('./window.module.sass'));
    let drawInner = false;
    setTimeout(() => {
        drawInner = true;
        m.redraw();
    }, 600); // Boxes appear at this point; the inner content is also added at this point.
    return {
        view() {
            return $.div.window(
                {
                    style: {
                        __halfBorder: `${Math.ceil((vnode.attrs.borderWidth ?? 2) / 2)}px`,
                    },
                },
                $.div.container($.div.interior(drawInner ? vnode.children : []))
            );
        },
    };
});
