# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches and Steven Robert Brandt

import os
import sys
from math import pi, cos, sin

from warpsim import WarpSim, MASSIVE, PHOTON

ws = WarpSim()
ws.u_ship = 0.0
ws.u_bubble = 0.5
ws.u_drag = ws.u_bubble - ws.u_ship
ws.deflector = 0
ws.warp_drive_kind = "Natario"

# Add a group of particles in a line
N=3
for i in range(N):
    ws.add_particle((ws.radius*3,2*(i+.5),0),(0,0,0),MASSIVE,trackme=True)

# Add a triad of stationary particles
ws.add_particle((ws.radius + ws.sigma,0,0),(0.5,0,0),MASSIVE)

q=2*pi/3
x = (ws.radius+ws.sigma)*cos(q)
y = (ws.radius+ws.sigma)*sin(q)
ws.add_particle((x,y,0),(0.5,0,0),MASSIVE)

q=4*pi/3
x = (ws.radius+ws.sigma)*cos(q)
y = (ws.radius+ws.sigma)*sin(q)
ws.add_particle((x,y,0),(0.5,0,0),MASSIVE)

import argparse as ap
parser = ap.ArgumentParser(prog="WarpGen",
                           description="Example for setting up WarpDrive spacetimes and particles")
parser.add_argument("-o","--output",type=str,
                    help="The output file name",
                    default=os.environ.get("OUTPUT","idump.json"))
pres = parser.parse_args(sys.argv[1:])
ws.generate(pres.output)
