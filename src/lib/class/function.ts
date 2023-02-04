//import { builtins, getParamNames } from '$lib/builtins';
import { Vec2 } from './vec2';
import type { Input } from './input';

// specifique a la class Variable

export enum VarType {
	INT,
	STRING,
	FLOAT,
	OTHER
}

/// Class specifique qui contient tout ce qui concerne les Box Varibles
export class Function_ {
	id: string;
	name: string;
	args: {
		name: string;
		type: string;
		value: any;
		draggover: boolean;
		input: Input;
	}[] = [];
	body: any[] = [];

	constructor(name: string) {
		this.id =
			`function_${Function_.quantity}_` +
			(Date.now() / 1000000 - Function_.quantity).toFixed().toString();
		this.name = name;
		Function_.quantity += 1;
	}

	/*
	static fromBuiltIn(builtinName: string) {
		//let fn: Function | undefined = builtins.get(builtinName);
		//if (!fn) return undefined;
		if (!builtins.has(builtinName)) return [];

		return getParamNames(builtins.get(builtinName)).map((arg) => {
			return {
				name: arg?.name || '',
				type: arg?.type || 'any',
				value: null,
				draggover: false,
				input: Input.default(arg?.name || '')
			};
		});
	}
	*/

	static CONST_BOX_INFOS: any = {
		//FIXED_COLOR: 'black',
		//DRAGGING_COLOR: 'red',
		//BACKGROUD_COLOR: 'bg-teal-100',
		//SHADOW_COLOR: 'grey',
		//SHADOW_BLUR: 5,
		//FIXED_SHADOW: 0,
		//DRAGGING_DELTA: 4,
		BASIC_BOX_SIZE: new Vec2(300, 100)
		//GAP: 20
	};

	static quantity = 0;
}

/* reflection sur comment s'occuper de tous ces blocks:
- avoir une AST direct ? 
- avoir un tableau d'un class block de base ?

- avoir une class pour chaque type de boite 
	et une class box qui compose les autres (ou heritage)
	+ avoir un lien vers le parent ?


- Ce qu'il faut se dire c'est qu'a un moment ou a un autre, on va devoir creer
	un interpreteur/compilateur et qu'un des premieres etapes,
	la tokenisation (et le parsing) va etre grandement simplifiee
	si on a une structure de donnee qui est deja "simple" et qui
	va dans le sens ou qui est directement la structure de donnee qu'on
	veut pour le compilateur
 
*/
