import { set_panic_hook, Config, House, Waves } from "canvas";

const animation = House;

const getDims = () => ({ width: window.innerWidth, height: window.innerHeight });

const renderLoop = () => {
  instance.update(config);
  instance.render(context);
  requestAnimationFrame(renderLoop);
};

const resize = () => {
  const dims = getDims();
  config = { ...config, dims };
  element.width = dims.width;
  element.height = dims.height;
};

///
// main
//

set_panic_hook();

const element = document.getElementById("canvas");
const context = element.getContext("2d");

let config = { dims: getDims() };

const instance = animation.new(config);

addEventListener("resize", resize);
resize();
renderLoop();
