'use strict';

const ui = require('counterproduction-ui');
const { mod, m } = require('counterproduction-ui/settings.js');
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
            const size = Math.floor(window.innerHeight * 0.01);
            if (toView) {
                return $.window(
                    $._.windowInterior(
                        {
                            style: {
                                __centerDist: `${Math.floor(13.5 * size)}px`,
                                __cubeSize: `${12 * size}px`,
                                __perspective: `${100 * size}px`,
                                __fontSize: `${23 * size}px`,
                                __startInto: `${16 * size}px`,
                                __endInto: `${10 * size}px`,
                                __disappearDistance: `${5 * size}px`,
                                __frontBorderSize: `${Math.ceil(window.innerHeight * 0.005)}px`,
                            },
                        },
                        $._.scene(
                            $._.cube(
                                $._.front(face()),
                                $._.right(face()),
                                $._.back(face()),
                                $._.left(face()),
                                $._.top(face()),
                                $._.bottom(face())
                            )
                        ),
                        $._.centered($._.logoText($._.partA('Counter'), $._.partB('Production')))
                    )
                );
            } else return $._();
        },
    };
});
m.mount(document.body, root.component);
