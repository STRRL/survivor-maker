import Phaser from "phaser";
import adhocCharacter from "/characters/adhoc-character.png";

export default class Hero extends Phaser.GameObjects.Container {
  private cursors: Phaser.Types.Input.Keyboard.CursorKeys;

  private speed: number;
  constructor(scene: Phaser.Scene, x: number, y: number) {
    super(scene, x, y);
    this.speed = 5;
    this.cursors = scene.input.keyboard.createCursorKeys();
    const theSprite = scene.add.sprite(0, 0, "character");
    this.add(theSprite);
  }

  preUpdate() {
    if (this.cursors.left.isDown) {
      this.x -= this.speed;
    }
    if (this.cursors.right.isDown) {
      this.x += this.speed;
    }
    if (this.cursors.up.isDown) {
      this.y -= this.speed;
    }
    if (this.cursors.down.isDown) {
      this.y += this.speed;
    }
  }
}
