import * as sim from "simulation-wasm";

const viewport = document.getElementById("viewport");
const simulation = new sim.Simulation();
let world = simulation.get_world();
const ctx = viewport.getContext('2d');

function draw_birds() {
    for (const bird of world.birds) {
        draw_triangle(
            bird.x * viewport.width,
            bird.y * viewport.height,
            bird.rotation,
            15)
    }
}

function draw_triangle(x, y, rotation, size) {
    ctx.beginPath();

    ctx.moveTo(x - Math.sin(rotation) * size * 1.5, y + Math.cos(rotation) * size * 1.5);
    ctx.lineTo(x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size, y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size);
    ctx.lineTo(x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size, y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size);
    ctx.lineTo(x - Math.sin(rotation) * size * 1.5, y + Math.cos(rotation) * size * 1.5);

    ctx.fillStyle = 'rgb(0,191,255)';
    ctx.fill();
}

function draw_foods() {
    for (const food of world.foods) {
        ctx.beginPath();
        ctx.arc(food.x * viewport.width, food.y * viewport.height, 8, 0, 2 * Math.PI);
        ctx.fillStyle = 'rgb(50,205,50)';
        ctx.fill();
    }
}

function redraw() {
    ctx.clearRect(0, 0, viewport.width, viewport.height);
    simulation.step();
    world = simulation.get_world();
    draw_birds();
    draw_foods();
    requestAnimationFrame(redraw);
}

redraw();