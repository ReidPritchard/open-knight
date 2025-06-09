// https://en.wikipedia.org/wiki/Portable_Game_Notation#Numeric_Annotation_Glyphs

// TODO: Either complete this file or move it to the backend/database

export enum NAGCategories {
	None = 0,
	MoveAssessments = 1, // $1-$9
	PositionalAssessments = 10, // $10-$135
	TimePressureCommentaries = 136, // $136-$139
	NotDefined = 140, // $140+
}

export const NAGs = {
	[NAGCategories.MoveAssessments]: {
		[1]: "!",
		[2]: "?",
		[3]: "!!",
		[4]: "??",
		[5]: "!!!",
		[6]: "???",
	},
};
