export class Queue {
    constructor() {
        this.queue = Promise.resolve();
    }

    push(fn) {
        this.queue = this.queue.then(fn);

        return this.queue;
    }
}
