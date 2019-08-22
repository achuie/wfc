#!/bin/env python2

import wavefunctioncollapse as wfc

r = wfc.Resolver(1)
r.generate_image("wfc-image/examples/bricks.png", 3, [48, 48], wfc.Orient().all(), 10,
        "./generated_bricks.png")
