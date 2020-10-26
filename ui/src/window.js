'use strict';

const { mod, m } = require('../settings.js');
module.exports = mod('window', (css, use, $, vnode) => {
    css(require('./window.module.scss'));
    let drawInner = false;
    return {
        oninit() {
            setTimeout(() => {
                drawInner = true;
                m.redraw();
            }, 600); // Boxes appear at this point; the inner content is also added at this point.
        },
        view() {
            return $.div.window(
                {
                    style: {
                        __halfBorder: `${Math.ceil((vnode.attrs?.borderWidth ?? 2) / 2)}px`,
                        __width: `${Math.round((vnode.attrs?.width ?? window.innerHeight * 1.5) / 2) * 2}px`,
                        __height: `${Math.round((vnode.attrs?.height ?? window.innerHeight * 0.85) / 2) * 2}px`,
                        __halfBoxSize: `${Math.round((vnode.attrs?.boxSize ?? 10) / 2)}px`,
                    },
                },
                $.div.container[vnode.attrs?.openType ?? 'slow']($.div.interior(drawInner ? vnode.children : []))
            );
        },
    };
});
