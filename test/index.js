'use strict'

const assert = require('assert').strict
const lib = require('../lib')

function generate_lattice_should_return_expected_result() {
  // arrange
  const p = (cents, ratio) => ({ cents, ratio })
  const dimensions = [3, 5]
  const steps = 3
  const expectedResult = [
    {
      limit: dimensions[0],
      otonal: [p(0, '1'), p(701, '3/2'), p(203, '9/8')],
      utonal: [p(0, '1'), p(498, '4/3'), p(996, '16/9')]
    },
    {
      limit: dimensions[1],
      otonal: [p(0, '1'), p(386, '5/4'), p(772, '25/16')],
      utonal: [p(0, '1'), p(813, '8/5'), p(427, '32/25')]
    }
  ]

  // act
  const result = lib.generateLattice(dimensions, steps)
  const truncateCents = ({ cents, ratio }) => ({
    cents: Math.trunc(cents),
    ratio
  })
  const resultWithTruncatedCents = result.map(({ limit, otonal, utonal }) => ({
    limit,
    otonal: otonal.map(truncateCents),
    utonal: utonal.map(truncateCents)
  }))

  // assert
  assert.deepEqual(resultWithTruncatedCents, expectedResult)
}

// Takes an object with functions as the value for properties.
function testFramework(tests) {
  let passed = true
  let results = {}

  for (let name in tests) {
    try {
      tests[name]()
    } catch (err) {
      console.log('FAIL', name, '\n', err.name, '\n', err.message)
      passed = false
    }

    results[name] = { passed }
  }

  console.table(results)

  if (!passed) process.exit(1)
}

testFramework({
  generate_lattice_should_return_expected_result
})
