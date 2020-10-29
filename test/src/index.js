'use strict';

const ui = require('autofactory-ui');
const { mod, m } = require('autofactory-ui/settings.js');
const root = mod('root', (css, use, $) => {
    css(require('./styles.scss'));
    css(require('autofactory-ui/global-styles.scss'));
    use(ui.window);
    use(ui.bracketed);
    use(ui.button);
    return {
        view() {
            return $.window(
                $._.windowInterior(
                    $.bracketed({ hover: true }, $.div.interior('Bracketed Text (Opens on hover)')),
                    $.bracketed(
                        {
                            control: (set) => {
                                let enter = false;
                                setInterval(() => {
                                    enter = !enter;
                                    set(enter);
                                }, 1000);
                            },
                        },
                        $.div.interior('Bracketed Text (Changes every second)')
                    ),
                    $.button('CLICK ME!')
                )
            );
        },
    };
});
m.mount(document.body, root.component);
