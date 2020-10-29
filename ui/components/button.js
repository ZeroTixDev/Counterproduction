'use strict';

const { mod, m } = require('../settings.js');
const defaults = require('../defaults.js')(
    {
        buttonPadding: () => 10,
    },
    true
);

module.exports = mod('button', (css, use, $, initial) => {
    css(require('./button.module.scss'));
    use(require('./bracketed.js'));
    const attrs = defaults(initial.attrs);
    return {
        view(vnode) {
            return $._.button(
                { style: { __buttonPadding: `${attrs.buttonPadding()}px` } },
                $.bracketed({ hover: true, bracketMargin: (bs) => (bs * 2) / 10 }, $._.interior(vnode.children))
            );
        },
    };
});
