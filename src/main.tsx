import Phaser from "phaser";
import logoImg from "/vite.svg";
import adhocTile from "/tiles/adhoc-tile.png";
import adhocCharacter from "/characters/adhoc-character.png";
import Hero from "./assets/entities/Hero";

class DemoScene extends Phaser.Scene {
  constructor() {
    super("demo");
  }
  preload() {
    this.load.image("logo", logoImg);
    this.load.image("tile", adhocTile);
    this.load.image("character", adhocCharacter);
  }

  create() {
    const width = this.scale.width;
    const height = this.scale.height;
    this.add.tileSprite(0, 0, width, height, "tile").setOrigin(0, 0);
    const logo = this.add.image(400, 150, "logo");
    this.tweens.add({
      targets: logo,
      y: 450,
      duration: 2000,
      ease: "Power2",
      yoyo: true,
      loop: -1,
    });

    const theHero = new Hero(this, width / 2, height / 2);
    this.add.existing(theHero);
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
