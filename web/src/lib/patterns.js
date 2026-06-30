export const PATTERNS = [
  {
    group: 'Methuselahs',
    desc: 'Tiny seeds with unexpectedly long evolutions',
    items: [
      {
        id: 'r-pentomino',
        name: 'R-pentomino',
        meta: '5 cells · stabilizes at gen 1,103',
        rle: 'boo$oob$bo!',
      },
      {
        id: 'acorn',
        name: 'Acorn',
        meta: '7 cells · stabilizes at gen 5,206',
        rle: 'bo$3bo$2o2b3o!',
      },
      {
        id: 'diehard',
        name: 'Diehard',
        meta: '7 cells · dies completely at gen 130',
        rle: '6bo$2o$bo3b3o!',
      },
    ],
  },
  {
    group: 'Guns',
    desc: 'Patterns that emit an infinite stream of spaceships',
    items: [
      {
        id: 'gosper-gun',
        name: 'Gosper Glider Gun',
        meta: '36 cells · fires a glider every 30 generations',
        rle: '24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4bobo$18bo5bo$11bo3bo$12b2o!',
      },
    ],
  },
  {
    group: 'Spaceships',
    desc: 'Patterns that translate across the grid',
    items: [
      {
        id: 'glider',
        name: 'Glider',
        meta: '5 cells · period 4 · moves diagonally',
        rle: 'bo$2bo$3o!',
      },
      {
        id: 'lwss',
        name: 'Lightweight Spaceship',
        meta: '9 cells · period 4 · moves horizontally',
        rle: 'bo2bo$o$o3bo$4o!',
      },
    ],
  },
  {
    group: 'Oscillators',
    desc: 'Patterns that cycle back to their initial state',
    items: [
      {
        id: 'blinker',
        name: 'Blinker',
        meta: '3 cells · period 2 · simplest oscillator',
        rle: '3o!',
      },
      {
        id: 'pulsar',
        name: 'Pulsar',
        meta: '48 cells · period 3',
        rle: '2b3o3b3o2b$4bobobob4o$boo3boob3oob$3boo3boob3o$boo3boob3oob$4bobobob4o$2b3o3b3o2b$7b7b$2b3o3b3o2b$4bobobob4o$boo3boob3oob$3boo3boob3o$boo3boob3oob$4bobobob4o$2b3o3b3o!',
      },
    ],
  },
]
