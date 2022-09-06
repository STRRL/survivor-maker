export default class Monster extends Phaser.GameObjects.Container {
  private speed: number;

  constructor(scene: Phaser.Scene, x: number, y: number) {
    super(scene, x, y);
    this.speed = 5;
    const theSprite = scene.add.sprite(0, 0, "monster");
    this.add(theSprite);
  }
}
