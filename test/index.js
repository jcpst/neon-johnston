"use strict"

const assert = require("assert").strict
const lib = require("../lib")
const fixtures = require("./fixtures")

// TEST
function generateLattice_should_return_expected_result() {
  // arrange
  const p = (cents, ratio) => ({ cents, ratio })
  const dimensions = [3, 5]
  const steps = 3
  const expectedResult = fixtures.lattice()

  // act
  const result = lib.generateLattice(dimensions, steps)
  const resultWithTruncatedCents = result.map(({ limit, otonal, utonal }) => ({
    limit,
    otonal: otonal.map(truncateCents),
    utonal: utonal.map(truncateCents)
  }))

  // assert
  assert.deepEqual(resultWithTruncatedCents, expectedResult)
}

// TEST
function generateScale_should_return_expected_result() {
  // arrange

  // act
  const result = lib.generateScale([3, 5], 3)
  const resultWithTruncatedCents = result.map(truncateCents)

  // assert
  assert.deepEqual(resultWithTruncatedCents, fixtures.scale())
}

function truncateCents({ cents, ratio }) {
  return {
    cents: Math.trunc(cents),
    ratio
  }
}

// Takes an object with functions as the value for properties.
function testFramework(tests) {
  let passed = true
  let results = {}

  for (let name in tests) {
    try {
      tests[name]()
    } catch (err) {
      console.log("FAIL", name, "\n", err.name, "\n", err.message)
      passed = false
    }

    results[name] = { passed }
  }

  console.table(results)

  if (!passed) process.exit(1)
}

testFramework({
  generateLattice_should_return_expected_result,
  generateScale_should_return_expected_result
})
