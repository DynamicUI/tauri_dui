import { Vec2 } from '$lib/class/vec2';

export enum BoxType {
	VARIABLE,
	FUNCTION
}

export type ConstBoxInfos = {
	// TODO faire un type color ? juste utiliser les couleurs de tailwind ?
	FIXED_COLOR: string; // Color when on the floor
	DRAGGING_COLOR: string; // Color when dragged
	SHADOW_COLOR: string; // Shadow color
	BACKGROUD_COLOR: string; // Color of the Box

	FIXED_SHADOW: number; // Shadow size and distance when on the floor
	DRAGGING_DELTA: number; // ... when dragged
	SHADOW_BLUR: number; //

	GAP: number; // Distance between two child
	BASIC_BOX_SIZE: Vec2;
};

/// Class qui contient tout les elements UI
export class Box {
	type: BoxType;
	name: string;
	id: string;
	child_count: number = 0;
	is_child: boolean = false; // if true, can't move
	shadow = { delta: 0, size: 0, blur: 0 };
	size: Vec2;
	deltaMouseBox: Vec2 = new Vec2(0, 0);
	border = { color: 'black', size: 1 };
	isDragging: boolean = false;
	css = { position: 'absolute', z_index: 0, cursor: 'default' };
	is_hover = false;

	constructor(name: string, type: BoxType, ui_info: ConstBoxInfos) {
		this.name = name;
		this.id = name; // TODO avoir un id autre que le name ? (avoir plusieurs fois le meme nom)
		this.type = type;
		this.size = ui_info.BASIC_BOX_SIZE;
		this.shadow.blur = ui_info.SHADOW_BLUR;
		if (this.child_count) {
			this.size.y = this.size.y * 1.2;
			this.size.x = this.size.x * this.child_count + ui_info.GAP * this.child_count;
		}
		if (this.is_child) {
			this.css.position = 'relative';
		}
	}
}
