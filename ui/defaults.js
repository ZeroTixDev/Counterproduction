'use strict';

module.exports = (def, af) =>
    function withDefaults(obj) {
        return new Proxy(obj, {
            get(_, prop) {
                const val = obj[prop];
                const defval = def[prop];
                if (val === undefined) {
                    return defval;
                } else if (typeof val === 'object') {
                    return withDefaults(val, defval);
                } else if (af && typeof defval === 'function' && typeof val !== 'function') {
                    return () => val;
                } else {
                    return val;
                }
            },
        });
    };
