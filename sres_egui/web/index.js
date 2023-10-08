async function run() {
    let res = await import('../target/wasm-pack/sres.js');
    res.start_app("the_canvas_id");
}

run();
