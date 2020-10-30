'use strict';

const ui = require('autofactory-ui');
const { mod, m } = require('autofactory-ui/settings.js');
const root = mod('root', (css, use, $) => {
    css(require('./styles.scss'));
    use(ui.window);
    let toView = false;
    setTimeout(() => {
        toView = true;
        m.redraw();
    }, 1000);
    return {
        view() {
            if (toView) {
                return $.window(
                    $._.scene(
                        {
                            style: {
                                __cubeSize: '100px',
                                __centerDist: '120px',
                            },
                        },
                        $._.cube($._.front(), $._.right(), $._.back(), $._.left(), $._.top(), $._.bottom())
                    )
                );
            } else return $._();
        },
    };
});
m.mount(document.body, root.component);
