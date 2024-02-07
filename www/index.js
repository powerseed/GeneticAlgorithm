import * as wasm from "hello-wasm-pack";
import * as sim from "simulation-wasm";

wasm.greet();
alert("who's that dog? " + sim.whos_that_dog() + "!");