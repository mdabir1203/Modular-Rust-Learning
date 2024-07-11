import { acceleratedValues } from '../../animation/animators/utils/accelerated-values.mjs';
import { camelToDash } from '../../render/dom/utils/camel-to-dash.mjs';
import { transformProps } from '../../render/html/utils/transform.mjs';

function getWillChangeName(name) {
    if (transformProps.has(name)) {
        return "transform";
    }
    else if (acceleratedValues.has(name) ||
        // Manually check for backgroundColor as accelerated animations
        // are currently disabled for this value (see `acceleratedValues`)
        // but can still be put on the compositor.
        name === "backgroundColor") {
        return camelToDash(name);
    }
}

export { getWillChangeName };
