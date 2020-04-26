export interface IAuthor {
    createdAt: Date;
    updatedAt: Date;
    firstname: string;
    lastname: string;
    birth: Date;
    death: Date;
    nationality: string;
    biography: string;
    movements: string[];
    works: string[];
}