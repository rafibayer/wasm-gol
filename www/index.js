// not used, but we could add functions here to call JS code from our Rust

register_plugin = function (importObject) {
    // importObject.env.log_js= function (js_object) {
    //     console.log(js_object)
    // }
}

// miniquad_add_plugin receive an object with two fields: register_plugin and on_init. Both are functions, both are optional.
miniquad_add_plugin({register_plugin});