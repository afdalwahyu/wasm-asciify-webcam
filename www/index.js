import * as wasm from "mandel";


const getStream = async () => {
  return navigator.mediaDevices.getUserMedia({
    audio: false,
    video: {
      width: 80,
      height: 80
    }
  });
};

const snapshotVidToCanvas = () => {
  const vid = document.getElementById("vid");
  const canvas = document.getElementById("canvas");
  const ctx = canvas.getContext("2d");
  ctx.drawImage(vid, 0, 0, canvas.width, canvas.height);
  wasm.render();


  setTimeout(snapshotVidToCanvas, 100);
  // window.requestAnimationFrame(snapshotVidToCanvas);
};

const main = async () => {
  const stream = await getStream();
  const vid = document.getElementById("vid");
  vid.srcObject = stream;
  snapshotVidToCanvas();
};

main();
