'use strict';

require('./styles.scss');

const ui = require('autofactory-ui');
const { mod, m } = require('autofactory-ui/settings.js');
const root = mod('root', (css, use, $) => {
    use(ui.window);
    use(ui.bracketed);
    return {
        view() {
            return $.window($.bracketed($.div('Hiasdofiasudfoiasudfoiuo')));
        },
    };
});
m.mount(document.body, root.component);
