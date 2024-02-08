import * as wasm from "hello-wasm-pack";
import * as sim from "simulation-wasm";

const viewport = document.getElementById("viewport");
const world = sim.create_new_world();
const ctxt = viewport.getContext('2d');

draw_birds();
draw_foods();
function draw_birds() {
    for (const bird of world.birds) {
        console.log(bird.rotation)
        draw_triangle(
            bird.position[0] * viewport.width,
            bird.position[1] * viewport.height,
            bird.rotation,
            15)
    }
}

function draw_triangle(x, y, rotation, size) {
    ctxt.beginPath();

    ctxt.moveTo(x - Math.sin(rotation) * size * 1.5, y + Math.cos(rotation) * size * 1.5);
    ctxt.lineTo(x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size, y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size);
    ctxt.lineTo(x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size, y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size);
    ctxt.lineTo(x - Math.sin(rotation) * size * 1.5, y + Math.cos(rotation) * size * 1.5);

    ctxt.fill();
}

function draw_foods() {
    for (const food of world.foods) {
        ctxt.beginPath();
        ctxt.arc(food.position[0] * viewport.width, food.position[1] * viewport.height, 4, 0, 2 * Math.PI);
        ctxt.fill();
    }
}

function redraw() {

}