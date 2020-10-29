'use strict';

const { mod, m } = require('../settings.js');
const defaults = require('../defaults.js')(
    {
        padding: () => 8,
        margin: () => 8,
        bracketShiftDistance: (interior) => Math.min(interior.clientWidth, interior.clientHeight) / 20,
    },
    true
);

module.exports = mod('button', (css, use, $, initial) => {
    css(require('./button.module.scss'));
    use(require('./bracketed.js'));
    const attrs = defaults(initial.attrs);
    let interior;
    let bracketSize;
    let bracketShiftDistance;
    return {
        oncreate() {
            bracketSize = Math.min(interior.clientWidth, interior.clientHeight) / 2;
            bracketShiftDistance = attrs.bracketShiftDistance(interior.dom);
            m.redraw();
        },
        view(vnode) {
            interior = $._.interior(vnode.children);
            return $._.button(
                {
                    style: {
                        __padding: `${Math.round(attrs.padding())}px`,
                        __margin: `${Math.round(attrs.margin())}px`,
                        __bracketShiftDistance: `${Math.ceil(bracketShiftDistance ?? 1)}px`,
                    },
                },
                $.bracketed(
                    {
                        hover: true,
                        bracketMargin: (bs) => (bs * 2) / 10,
                        bracketSize,
                    },
                    $._.container(interior)
                )
            );
        },
    };
});
