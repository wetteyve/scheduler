import test from 'ava'

import { helloNapi } from '../index'

const defaultValue = 'Hello, napi-rs!'
test('helloNapi', (t) => {
  t.is(helloNapi(), defaultValue)
})

test('helloNapi with string input', (t) => {
  const input = 'rusty'
  t.is(helloNapi(input), `Hello, ${input}!`)
})

test('helloNapi throws with non-string input', (t) => {
  const input = 4 as unknown as string
  t.throws(() => helloNapi(input))
})
