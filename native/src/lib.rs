#[macro_use]
extern crate neon;
extern crate johnston;

use johnston::gen_lattice;
use neon::prelude::*;

fn generate_lattice(mut cx: FunctionContext) -> JsResult<JsArray> {
    let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
    let v: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
    let b = cx.argument::<JsNumber>(1)?.value() as f64;
    let c = v
        .iter()
        .map(|&x| x.downcast::<JsNumber>().unwrap().value())
        .collect::<Vec<_>>();
    let result = gen_lattice(
        c.iter().map(|&x| x as usize).collect::<Vec<_>>().as_slice(),
        b as usize,
    );
    let js_result = JsArray::new(&mut cx, result.len() as u32);

    for (i, dimension) in result.iter().enumerate() {
        let js_object = JsObject::new(&mut cx);
        let limit = cx.number(dimension.limit as f64);
        let otonal = JsArray::new(&mut cx, dimension.otonal.len() as u32);
        let utonal = JsArray::new(&mut cx, dimension.otonal.len() as u32);

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

    Ok(js_result)
}

register_module!(mut cx, {
    cx.export_function("generateLattice", generate_lattice)?;
    Ok(())
});
