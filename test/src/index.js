'use strict';

const ui = require('autofactory-ui');
const { mod, m } = require('autofactory-ui/settings.js');
const root = mod('root', (css, use, $) => {
    css(require('./styles.scss'));
    use(ui.window);
    use(ui.bracketed);
    use(ui.button);
    let toView = false;
    setTimeout(() => {
        toView = true;
        m.redraw();
    }, 1000);
    return {
        view() {
            if (toView) {
                return $.window(
                    $._.windowInterior(
                        $._.wrapper(
                            $.bracketed({ hover: true }, $._.interior('Bracketed Text (Opens on hover)')),
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
                                $._.interior('Bracketed Text (Changes every second)')
                            ),
                            $.button('CLICK ME!'),
                            $.button({ margin: 30 }, $._.bigButtonInterior('This is a big button.'))
                        )
                    )
                );
            } else return $._();
        },
    };
});
m.mount(document.body, root.component);
