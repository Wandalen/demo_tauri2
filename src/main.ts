import { invoke } from "@tauri-apps/api/core";

async function start_fibonacci() {
  await invoke("start_fibonacci");
}

window.addEventListener("DOMContentLoaded", () => {
  document.getElementById("FibonacciSequence")?.addEventListener("click", () => {
    start_fibonacci() .then(() => console.log("Fibonacci notifications started")).catch((err) => console.error("Error starting Fibonacci:", err));
  });
});