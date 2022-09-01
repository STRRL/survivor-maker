import Phaser from "phaser";
import React from "react";
import ReactDOM from "react-dom/client";
import logoImg from "./assets/react.svg";
import App from "./App";
import "./style.css";

class DemoScene extends Phaser.Scene {
  constructor() {
    super("demo");
  }
  preload() {
    this.load.image("logo", logoImg);
  }

  create() {
    const logo = this.add.image(400, 150, "logo");
    this.tweens.add({
      targets: logo,
      y: 450,
      duration: 2000,
      ease: "Power2",
      yoyo: true,
      loop: -1,
    });
  }
}

const config: Phaser.Types.Core.GameConfig = {
  type: Phaser.AUTO,
  parent: "phaser-example",
  width: 1600,
  height: 900,
  scene: DemoScene,
};

const game = new Phaser.Game(config);
