export class Input {
	/* Data */
	value: any = '';

	/* UI */
	target: any;
	isWriteMode: boolean;
	isFocus: boolean = false;

	constructor(target: any, isWriteMode: boolean) {
		this.target = target;
		this.isWriteMode = isWriteMode;
	}
}
