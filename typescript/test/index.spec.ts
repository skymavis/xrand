import { beforeEach, describe, expect, test } from '@jest/globals'
import { TGFSR } from '../src/index'

describe('TGFSR Random Number Generator', () => {
  let tgfsr: TGFSR

  beforeEach(() => {
    tgfsr = new TGFSR(12345)
  })

  test('should initialize with a given seed', () => {
    expect(tgfsr).toBeDefined()
  })

  test('should generate a random number', () => {
    const randomNumber = tgfsr.random()
    expect(randomNumber).toBeGreaterThanOrEqual(0)
    expect(randomNumber).toBeLessThanOrEqual(0x7fffffff)
  })

  test('should generate different random numbers on consecutive calls', () => {
    const firstRandom = tgfsr.random()
    const secondRandom = tgfsr.random()
    expect(firstRandom).not.toBe(secondRandom)
  })

  test('should produce consistent results for the same seed', () => {
    const tgfsr1 = new TGFSR(12345)
    const tgfsr2 = new TGFSR(12345)

    const randomNumbers1 = Array.from({ length: 10 }, () => tgfsr1.random())
    const randomNumbers2 = Array.from({ length: 10 }, () => tgfsr2.random())

    expect(randomNumbers1).toEqual(randomNumbers2)
  })

  test('should handle multiple iterations correctly', () => {
    const numbers = Array.from({ length: 100 }, () => tgfsr.random())
    expect(numbers).toHaveLength(100)
  })
})
