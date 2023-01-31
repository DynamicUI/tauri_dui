export class Input {
	target: any;
	isWriteMode: boolean;
	isFocus: boolean = false;
	name: string; // TODO a quoi sert le name bon sang ?
	value: any = '';

	constructor(target: any, isWriteMode: boolean, name: string) {
		this.target = target;
		this.isWriteMode = isWriteMode;
		this.name = name;
	}
}
