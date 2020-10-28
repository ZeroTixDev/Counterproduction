'use strict';

const ui = require('autofactory-ui');
const { mod, m } = require('autofactory-ui/settings.js');
const root = mod('root', (css, use, $) => {
    css(require('./styles.scss'));
    use(ui.window);
    use(ui.bracketed);
    return {
        view() {
            return $.window(
                $.bracketed({ hover: true }, $.div.interior('Bracketed Text (Opens on hover)')),
                $.bracketed(
                    {
                        control: (set) => {
                            console.log('Controlled');
                            let enter = false;
                            setInterval(() => {
                                enter = !enter;
                                set(enter);
                            }, 1000);
                        },
                    },
                    $.div.interior('Bracketed Text (Changes every second)')
                )
            );
        },
    };
});
m.mount(document.body, root.component);
