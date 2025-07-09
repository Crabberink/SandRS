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

var mouseButtons = 0;
canvas.addEventListener('mousedown', function(evt) {
    mouseButtons = evt.buttons;
});
canvas.addEventListener('mouseup', function(evt) {
    mouseButtons = 0;
});
canvas.addEventListener('mouseleave', function(evt) {
    mouseButtons = 0;
});

document.addEventListener('keydown', function(e) {
    console.log(e.code);
    if (e.code == "KeyX") { placeType = 0; }
    if (e.code == "Digit1") { placeType = 1; }
    if (e.code == "Digit2") { placeType = 2; }
});

let placeType = 0;


function isMousePressed(button) {
    let mask = 1 << button;
    return mouseButtons & mask > 0;
}


let lastTime;

let size = 100;

function getPixelToPlace() {
    switch (placeType) {
        case 0:
            return Pixel.empty();
        case 1:
            return Pixel.sand();
        case 2:
            return Pixel.water();
        default:
            return Pixel.empty();
    }
}

function renderFrame(timeStamp) {
    if(lastTime == undefined) {
        lastTime = timeStamp;
        requestAnimationFrame(renderFrame);
        return;
    }
    let dt = timeStamp - lastTime; 

    let start = performance.now()

    if (isMousePressed(0)) {
        for (let ox = -size; ox < size; ox++) {
            for (let oy = -size; oy < size; oy++) {
                if (mousePos.x + ox < 0 || mousePos.x + ox >= width || mousePos.y + oy < 0 || mousePos.y + oy >= height) { continue; }
                world.set_pixel(mousePos.x + ox, mousePos.y + oy, getPixelToPlace());
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