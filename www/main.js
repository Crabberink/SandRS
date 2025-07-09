import init from "/pkg/sand_rs.js";
await init();

import { World, wasm_memory, Pixel } from "/pkg/sand_rs.js";

console.log("Initialized wasm!");

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const width = 800;
const height = 800;

canvas.width = width;
canvas.height = height;


const world = World.new(width, height);
const imageData = new ImageData(width, height);

var mousePos = {x:0, y:0};
function getMousePos(canvas, evt) {
    var rect = canvas.getBoundingClientRect();
    return {
        x: evt.clientX - rect.left,
        y: evt.clientY - rect.top
    };
}
  
canvas.addEventListener('mousemove', function(evt) {
    mousePos = getMousePos(canvas, evt);
}, false);

var mousePressed = false;
canvas.addEventListener('mousedown', function(evt) {
    mousePressed = true;
});
canvas.addEventListener('mouseup', function(evt) {
    mousePressed = false;
});
canvas.addEventListener('mouseleave', function(evt) {
    mousePressed = false;
});


let lastTime;

function renderFrame(timeStamp) {
    if(lastTime == undefined) {
        lastTime = timeStamp;
        requestAnimationFrame(renderFrame);
        return;
    }
    let dt = timeStamp - lastTime; 

    let start = performance.now()

    if (mousePressed) {
        for (let ox = -2; ox < 2; ox++) {
            for (let oy = -2; oy < 2; oy++) {
                world.set_pixel(mousePos.x + ox, mousePos.y + oy, Pixel.empty());
            }
        }
    }

    world.update();
    world.generate_texture();
    let dataBuffer = new Uint8Array(wasm_memory().buffer, world.texture_buffer(), width * height * 4);
    imageData.data.set(dataBuffer);
    ctx.putImageData(imageData, 0, 0);
    let end = performance.now();

    console.log(`Ran in ${end-start}`);

    requestAnimationFrame(renderFrame);
}



requestAnimationFrame(renderFrame);