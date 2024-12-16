import type { Config } from 'jest'

const config: Config = {
  verbose: true,
  transform: {
    '^.+.tsx?$': ['ts-jest', {}],
  },
  testEnvironment: 'node',
  testMatch: ['**/?(*.)+(spec|test).ts?(x)'],
}

export default config
