'use strict';

const { mod } = require('../settings.js');
module.exports = mod('test', (css, use, $) => {
    css(require('./test.module.sass'));
    return {
        view() {
            return $.div.hello('Hello World');
        },
    };
});
