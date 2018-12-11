export default class Squares {
  raw: number
  sumOfSquares: number
  squareOfSum: number
  difference: number
  constructor(raw: number) {
    this.raw = raw
    this.sumOfSquares = this.calculateSumOfSquares()
    this.squareOfSum = this.calculateSquareOfSum()
    this.difference = this.squareOfSum - this.sumOfSquares
  }

  private calculateSumOfSquares(): number {
    let result: number = 0
    for (let x = this.raw; x > 0; x--) {
      result += x ** 2
    }
    return result
  }

  private calculateSquareOfSum(): number {
    let result: number = 0
    for (let x = this.raw; x > 0; x--) {
      result += x
    }
    return result ** 2
  }
}