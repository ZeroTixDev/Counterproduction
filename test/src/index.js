'use strict';

const ui = require('autofactory-ui');
const { mod, m } = require('autofactory-ui/settings.js');
const root = mod('root', (css, use, $) => {
    css(require('./styles.scss'));
    use(ui.window);
    use(ui.bracketed);
    return {
        view() {
            return $.window($.bracketed($.div.interior('Hiasdofiasudfoiasudfoiuo')));
        },
    };
});
m.mount(document.body, root.component);
