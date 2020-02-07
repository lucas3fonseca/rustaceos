const wasmFile = import('./pkg/fill_web');

export const process = async (_wasm) => {
  // wasm.greet("Test")
  // wasm.run()
  console.info('Running WASM...')
}

wasmFile.catch(console.error);
