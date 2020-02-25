"use strict"

const p = (cents, ratio) => ({ cents, ratio })

function lattice() {
  return [
    {
      limit: 3,
      otonal: [p(0, "1"), p(701, "3/2"), p(203, "9/8")],
      utonal: [p(0, "1"), p(498, "4/3"), p(996, "16/9")]
    },
    {
      limit: 5,
      otonal: [p(0, "1"), p(386, "5/4"), p(772, "25/16")],
      utonal: [p(0, "1"), p(813, "8/5"), p(427, "32/25")]
    }
  ]
}

function scale() {
  return [
    p(0, "1"),
    p(203, "9/8"),
    p(386, "5/4"),
    p(427, "32/25"),
    p(498, "4/3"),
    p(701, "3/2"),
    p(772, "25/16"),
    p(813, "8/5"),
    p(996, "16/9")
  ]
}

module.exports = {
  lattice,
  scale
}
