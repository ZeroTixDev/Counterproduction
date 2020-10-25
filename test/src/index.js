'use strict';

require('./styles.sass');

const ui = require('autofactory-ui');
const { mod, m } = require('autofactory-ui/settings.js');
const root = mod('root', (css, use, $) => {
    use(ui.window);
    return {
        view() {
            return $.window({
                width: 100,
                height: 100,
                borderWidth: 10,
            });
        },
    };
});
m.mount(document.body, root.component);
