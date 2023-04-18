const { invoke } = window.__TAURI__.tauri;

let polynomialInputEl;
let polynomialSolutionArea;
let horizontalLines;
let verticalLines;

async function getResults() {
  let val = await invoke("get_results", { name: polynomialInputEl.value });
  if (val.length !== 0) {
    polynomialSolutionArea.innerHTML = "";
    var scale = 0;
    for (const key in val) {
      if (Object.hasOwnProperty.call(val, key)) {
        const elem = val[key];
        scale = Math.max(scale, Math.abs(elem.re), Math.abs(elem.im))
      }
    }
    if (scale == 0) scale = 1;
    var i = 1;
    for (const line in horizontalLines) {
      if (Object.hasOwnProperty.call(horizontalLines, line)) {
        var element = horizontalLines[line];
        if (!element.classList.contains("main"))
          element.innerHTML = "<p>" + Math.round(scale * i * 100) / 100 + "i</p>";
        i -= 0.25
      }
    }
    i = -1;
    for (const line in verticalLines) {
      if (Object.hasOwnProperty.call(verticalLines, line)) {
        var element = verticalLines[line];
        if (!element.classList.contains("main"))
          element.innerHTML = "<p>" + Math.round(scale * i * 100) / 100 + "</p>";
        i += 0.25
      }
    }
    for (const key in val) {
      if (Object.hasOwnProperty.call(val, key)) {
        const elem = val[key];
        polynomialSolutionArea.innerHTML += '<div class="result" style="top:' + (50 - 50 * elem.im / scale) + '%;left: ' + (50 + 50 * elem.re / scale) + '%;"><p>' + elem.string + '</p></div>';
      }
    }
  } else {
    polynomialSolutionArea.innerHTML = '<p class="error">Invalid polynomial</p';
  }
}

window.addEventListener("DOMContentLoaded", () => {
  polynomialInputEl = document.querySelector("#polynomial-input");
  polynomialSolutionArea = document.querySelector("#safe-area");
  horizontalLines = document.getElementsByClassName("horizontal-line");
  verticalLines = document.getElementsByClassName("vertical-line");
  document
    .querySelector("#polynomial-button")
    .addEventListener("click", () => getResults());
});