export class Vec2 {
	x: number;
	y: number;

	constructor(x: number, y: number) {
		this.x = x;
		this.y = y;
	}
}

// TODO comment le mettre dans la class ?
//Vec2.prototype['+'] = function (operand: Vec2) {
//	return new Vec2(this.x + operand.x, this.y + operand.y);
//};
