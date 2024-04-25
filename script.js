import init, { FileHolder } from "./pkg/wasm_blur.js";

await init();
let file = new FileHolder();

let form = document.querySelector("form");

form.addEventListener("submit", async function (e) {
  e.preventDefault();

  const meme = new FormData(e.target);
  const vec = meme.getAll("file");

  file.set_blob(vec[0]);

  file.render();

  file.render_with_blurry(1000.0);
});

let file_reader = new FileReader();
