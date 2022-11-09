export class MissingFieldError extends Error {
    constructor(field: string) {
        super(`ERROR: Item is missing field ${field}`);
    }
}
