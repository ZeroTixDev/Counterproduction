/*
'use strict';

const { mod, m } = require('../settings.js');
const defaults = require('../defaults.js')({}, true);

module.exports = mod('_____', (css, use, $, initial) => {
    css(require('./_____.module.scss'));
    const attrs = defaults(initial.attrs);

    return {
        oncreate() {},
        view() {
            return $._();
        },
    };
});
*/
