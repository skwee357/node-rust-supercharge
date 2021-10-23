use neon::context::{Context, ModuleContext, FunctionContext};
use neon::types::JsNumber;
use neon::result::JsResult;
use neon::result::NeonResult;

fn fibonacci(n: i32) -> i32 {
	return match n {
		n if n < 1 => 0,
		n if n <= 2 => 1,
		_ => fibonacci(n - 1) + fibonacci(n - 2)
	}
}

fn fibonacci_api(mut cx: FunctionContext) -> JsResult<JsNumber> {
	let handle = cx.argument::<JsNumber>(0).unwrap();
	let res = fibonacci(handle.value(&mut cx) as i32);
	Ok(cx.number(res))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
	cx.export_function("fibonacci_rs", fibonacci_api)?;
	Ok(())
}
