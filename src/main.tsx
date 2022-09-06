import Phaser from "phaser";
import logoImg from "/vite.svg";
import adhocTile from "/tiles/adhoc-tile.png";
import adhocCharacter from "/characters/adhoc-character.png";
import adhocMonster from "/monster/adhoc-monster.png";
import Hero from "./entities/Hero";
import Monster from "./entities/Monster";

class DemoScene extends Phaser.Scene {
  constructor() {
    super("demo");
  }

  preload() {
    this.load.image("logo", logoImg);
    this.load.image("tile", adhocTile);
    this.load.image("character", adhocCharacter);
    this.load.image("monster", adhocMonster);
  }

  create() {
    const width = this.scale.width;
    const height = this.scale.height;
    this.add.tileSprite(0, 0, width, height, "tile").setOrigin(0, 0);

    const theHero = new Hero(this, width / 2, height / 2);
    this.physics.add.existing(theHero, true);
    this.add.existing(theHero);

    const theMonster = new Monster(this, width / 2 + 300, height / 2 + 300);

    this.add.existing(theMonster);
    this.physics.add.existing(theMonster, false);

    const monsterBody = theMonster.body as Phaser.Physics.Arcade.Body;
    monsterBody.setOffset(-32, -32);

    this.physics.moveToObject(theMonster, theHero, 200);

    this.physics.add.overlap(
      theMonster,
      theHero,
      this.handleHeroHitLogo,
      undefined,
      this
    );
  }

  private handleHeroHitLogo(
    logo: Phaser.GameObjects.GameObject,
    hero: Phaser.GameObjects.GameObject
  ) {
    this.scene.restart();
    console.log("Hero Died");
  }
}

const config: Phaser.Types.Core.GameConfig = {
  type: Phaser.AUTO,
  parent: "phaser-example",
  width: 1600,
  height: 900,
  scene: DemoScene,
  physics: {
    default: "arcade",
    arcade: {
      debug: true,
    },
  },
};

const game = new Phaser.Game(config);
