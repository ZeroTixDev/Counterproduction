'use strict';

const { mod, m } = require('../settings.js');
const defaults = require('../defaults.js')({
    continous: true, // Whether to always measure or only measure once.

}, true);

module.exports = mod('measure', (css, use, $, initial) => {
    const attrs = defaults(initial.attrs);

    return {
        oncreate() {

        },
        view(vnode) {
            return $._.measurer(vnode.children);
        },
    };
});
