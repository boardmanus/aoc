import { WebTime, WebBluePrint, init } from "day19";
init();

// DOM is already loaded, the `<script>` tag is at the bottom of the page
let content = document.getElementById("content");
let status = document.getElementById("status");
let bar = document.getElementById("bar");
let reset_button = document.getElementById("reset");
let step_button = document.getElementById("step");
let step10_button = document.getElementById("step10");
let playpause_button = document.getElementById("playpause");
let file_input = document.querySelector("input[type=file]");

let input = `Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
`;

let state = { input, playing: false };

let render = () => {
  let { blueprint, time } = state;
  let svg = time.to_svg(blueprint);
  content.innerHTML = svg;
};

let reset = () => {
  state.blueprint = WebBluePrint.default();
  state.time = new WebTime();
  state.playing = false;
  playpause_button.innerText = "Play";
  render();
};

file_input.onchange = (ev) => {
  let input = ev.currentTarget;
  if (input.files.length == 0) {
    return;
  }
  let reader = new FileReader();
  reader.onload = (ev) => {
    state.input = ev.target.result;
    reset();
  };
  reader.readAsText(input.files[0]);
};

reset_button.onclick = reset;
step_button.onclick = () => {
  state.time = state.time.update(state.blueprint);
  render();
};
step10_button.onclick = () => {
  for (let i = 0; i < 10; i++) {
    state.grid.step();
  }
  render();
};
playpause_button.onclick = () => {
  state.playing = !state.playing;
  if (state.playing) {
    playpause_button.innerText = "Pause";
    step_button.onclick();
  } else {
    playpause_button.innerText = "Play";
  }
};
reset();
