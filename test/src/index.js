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
    function face(...args) {
        return $._.face($._.faceInterior(...args));
    }
    return {
        view() {
            if (toView) {
                return $.window(
                    $._.scene(
                        $._.cube(
                            $._.front(face('AF')),
                            $._.right(face()),
                            $._.back(face()),
                            $._.left(face()),
                            $._.top(face()),
                            $._.bottom(face())
                        )
                    )
                );
            } else return $._();
        },
    };
});
m.mount(document.body, root.component);
