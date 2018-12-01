
export default class GigaSecond {
    age: Date
    constructor(age: Date) {
        this.age = age
    }

    date() {
        return new Date(this.age.getTime() + 1000000000000)
    }
}