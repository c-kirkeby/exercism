export default class ArmstrongNumbers {
  public static isArmstrongNumber(input: number): boolean {
    return `${input}`.split('')
                     .reduce((accumulator, item, _index, initial) => {
                       return accumulator += parseInt(item, 0) ** initial.length
                     }, 0) === input
  }
}
