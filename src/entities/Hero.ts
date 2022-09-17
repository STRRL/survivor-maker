import Phaser from "phaser";

export default class Hero extends Phaser.GameObjects.Container {
  private cursors: Phaser.Types.Input.Keyboard.CursorKeys;
  private RKey: Phaser.Input.Keyboard.Key;

  private speed: number = 5;
  private unbreakableSprite: Phaser.GameObjects.Sprite;
  public unbreakable: boolean = false;

  constructor(scene: Phaser.Scene, x: number, y: number) {
    super(scene, x, y);
    this.cursors = scene.input.keyboard.createCursorKeys();
    this.RKey = scene.input.keyboard.addKey(Phaser.Input.Keyboard.KeyCodes.R);
    const theSprite = scene.add.sprite(0, 0, "character");
    this.add(theSprite);

    this.unbreakableSprite = scene.add.sprite(0, 0, "character-unbreakable");
    this.add(this.unbreakableSprite);
  }

  private setUnbreakable(v: boolean) {
    this.unbreakable = v;
  }

  preUpdate() {
    // data
    if (this.cursors.left.isDown) {
      this.body.position.x -= this.speed;
      this.x -= this.speed;
    }
    if (this.cursors.right.isDown) {
      this.body.position.x += this.speed;
      this.x += this.speed;
    }
    if (this.cursors.up.isDown) {
      this.body.position.y -= this.speed;
      this.y -= this.speed;
    }
    if (this.cursors.down.isDown) {
      this.body.position.y += this.speed;
      this.y += this.speed;
    }
    if (this.RKey.isDown) {
      this.setUnbreakable(true);
    } else {
      this.setUnbreakable(false);
    }

    // fx
    this.unbreakableSprite.visible = this.unbreakable;
  }
}
