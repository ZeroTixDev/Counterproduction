'use strict';

require('./styles.scss');

const ui = require('autofactory-ui');
const { mod, m } = require('autofactory-ui/settings.js');
const root = mod('root', (css, use, $) => {
    use(ui.window);
    return {
        view() {
            return $.window({
                // borderWidth: 10,
                // width: 100,
                // height: 100,
                // boxSize: 20,
            });
        },
    };
});
m.mount(document.body, root.component);
