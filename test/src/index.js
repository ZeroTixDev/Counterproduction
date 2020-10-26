'use strict';

require('./styles.scss');

const ui = require('autofactory-ui');
const { mod, m } = require('autofactory-ui/settings.js');
const root = mod('root', (css, use, $) => {
    use(ui.window);
    use(ui.bracketed);
    return {
        view() {
            // console.log('Viewing');
            return $.bracketed($.div('Hiasdofiasudfoiasudfoiuo'));
        },
    };
});
m.mount(document.body, root.component);
