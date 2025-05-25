export class DebuffManager {
    #debufs;
    #maxSize;
    constructor(maxSize = Number.MAX_SAFE_INTEGER) {
        this.#debufs = [];
        this.#maxSize = maxSize;
    }
    debuffed(a) {
        let q = a.searchParams.get("debuff");
        if (q === null) {
            return true;
        }
        if (this.#debufs.includes(q)) {
            return true;
        }
        this.#debufs.push(q);
        if (this.#debufs.length > this.#maxSize) {
            this.#debufs.shift();
        }
        return false;
    }
}
