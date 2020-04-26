import { Document, Schema, Model, model, Types } from "mongoose";
import { IAuthor } from "../interfaces/author";


export interface IAuthorModel extends Document, IAuthor {
    fullname(): String;
}

export const AuthorSchema: Schema = new Schema({
    createdAt: Date,
    updatedAt: Date,
    firstname: {
        type: String,
        trim: true,
        lowercase: true,
        required: true
    },
    lastname: {
        type: String,
        trim: true,
        lowercase: true,
        required: true
    },
    birth: {
        type: Date,
        required: false
    },
    death: {
        type: Date,
        required: false
    },
    nationality: {
        type: String,
        required: false
    },
    biography: {
        type: String,
        required: false
    },
    movements: {
        type: [Types.ObjectId],
        required: false
    },
    works: {
        type: [Types.ObjectId],
        required: false
    }
})

AuthorSchema.pre<IAuthorModel>("save", function (next: any) {
    if (!this.createdAt) {
        this.createdAt = new Date();
    }
    this.updatedAt = new Date();
    next();
});

AuthorSchema.methods.fullname = function (): string {
    return (this.firstname.trim() + " " + this.lastname.trim());
}

export const Author: Model<IAuthorModel> = model<IAuthorModel>("Author", AuthorSchema);