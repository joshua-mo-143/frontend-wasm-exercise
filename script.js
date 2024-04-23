import init, { FileHolder } from "./pkg/wasm_blur.js";

let form = document.querySelector("form");

form.addEventListener("submit", async function (e) {
  e.preventDefault();
  await init();

  const meme = new FormData(e.target);
  const vec = meme.getAll("file");

  let buf = vec[0];

  let file = new FileHolder(buf);

  file.render();
});
