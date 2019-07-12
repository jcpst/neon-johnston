use johnston::{gen_lattice, LatticeDimension};
use neon::prelude::*;
use neon::register_module;

// HACK: https://users.rust-lang.org/t/neon-electron-undefined-symbol-cxa-pure-virtual/21223/2
// Also: https://github.com/neon-bindings/neon/issues/394
#[no_mangle]
pub extern "C" fn __cxa_pure_virtual() {
    loop {}
}

fn result_to_js(mut cx: FunctionContext, dimensions: Vec<LatticeDimension>) -> Handle<JsArray> {
    let js_result = JsArray::new(&mut cx, dimensions.len() as u32);

    for (i, dimension) in dimensions.iter().enumerate() {
        let js_object = JsObject::new(&mut cx);
        let limit = cx.number(dimension.limit as f64);
        let otonal = JsArray::new(&mut cx, dimension.otonal.len() as u32);
        let utonal = JsArray::new(&mut cx, dimension.utonal.len() as u32);

        for (j, pitch) in dimension.otonal.iter().enumerate() {
            let js_pitch = JsObject::new(&mut cx);
            let cents = cx.number(pitch.cents as f64);
            let ratio = cx.string(pitch.ratio.to_string_radix(10));

            js_pitch.set(&mut cx, "cents", cents).unwrap();
            js_pitch.set(&mut cx, "ratio", ratio).unwrap();
            otonal.set(&mut cx, j as u32, js_pitch).unwrap();
        }

        for (j, pitch) in dimension.utonal.iter().enumerate() {
            let js_pitch = JsObject::new(&mut cx);
            let cents = cx.number(pitch.cents as f64);
            let ratio = cx.string(pitch.ratio.to_string_radix(10));

            js_pitch.set(&mut cx, "cents", cents).unwrap();
            js_pitch.set(&mut cx, "ratio", ratio).unwrap();
            utonal.set(&mut cx, j as u32, js_pitch).unwrap();
        }

        js_object.set(&mut cx, "limit", limit).unwrap();
        js_object.set(&mut cx, "otonal", otonal).unwrap();
        js_object.set(&mut cx, "utonal", utonal).unwrap();
        js_result.set(&mut cx, i as u32, js_object).unwrap();
    }

    js_result
}

fn generate_lattice(mut cx: FunctionContext) -> JsResult<JsArray> {
    // Get JS values into rust.
    let arg_1: Handle<JsArray> = cx.argument(0)?;
    let arg_2 = cx.argument::<JsNumber>(1)?.value();
    let vec: Vec<Handle<JsValue>> = arg_1.to_vec(&mut cx)?;
    let vec_of_usize = vec
        .iter()
        .map(|&x| x.downcast::<JsNumber>().unwrap().value() as usize)
        .collect::<Vec<_>>();

    // Make the call.
    let dimensions = vec_of_usize.as_slice();
    let times = arg_2 as usize;
    let result = gen_lattice(dimensions, times);

    // Get rust value back out to JS
    let js_result = result_to_js(cx, result);

    Ok(js_result)
}

register_module!(mut cx, {
    cx.export_function("generateLattice", generate_lattice)?;
    Ok(())
});
