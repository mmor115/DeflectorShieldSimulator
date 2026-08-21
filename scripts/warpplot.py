# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches and Steven Robert Brandt

import os
import sys
from typing import List

import matplotlib.image as mpimg
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from matplotlib.offsetbox import OffsetImage, AnnotationBbox
from subprocess import run
import json

from danger import visualizer_home, simulator_home
from warpsim import WarpSim
import platform

is_linux = platform.system() == "Linux"

if is_linux:
    import matplotlib as mpl
    mpl.rcParams["font.family"] = "cm"
    mpl.rcParams["font.size"] = 20
    mpl.rcParams["text.usetex"] = True
    mpl.rcParams["text.latex.preamble"] = r"\usepackage{amsmath}"
    mpl.rcParams["lines.linewidth"] = 2.0

home = os.path.expanduser("~")
import argparse as ap
parser = ap.ArgumentParser(prog="WarpPlot",
                           description="Utility for plotting trajectories in WarpDrive spacetimes")
parser.add_argument("-v","--visualizer-home",type=str,
                    help="The directory where https://github.com/mmor115/DeflectorShieldSimulator.git is located",
                    default=os.environ.get("VISUALIZER",f"{home}/repos/DeflectorShieldSimulator"))
parser.add_argument("-s","--simulator-home",
                    help="The directory where https://github.com/lucass-carneiro/DeflectorShields.git is located",
                    default=os.environ.get("SIMULATOR",f"{home}/repos/DeflectorShields"))
parser.add_argument("-o","--output",type=str,
                    help="The output file name",
                    default=os.environ.get("OUTPUT",f"{home}/Downloads/warp.pdf"))
parser.add_argument("-i","--input", default=None, help="The input file name")

pres = parser.parse_args(sys.argv[1:])

simulator_home = pres.simulator_home
visualizer_home = pres.visualizer_home
output_dir = os.path.dirname(pres.output)

def run_field_plot(ph):
    print(json.dumps(ph.assemble(), indent=2))
    code = run([
        f"{simulator_home}/target/debug/deflector-field-plot.exe",
        str(ph.warp_drive_kind),
        str(ph.radius),
        str(ph.sigma),
        str(ph.u),
        str(ph.u0),
        str(ph.k0),
        str(ph.deflector_sigma_pushout),
        str(ph.deflector_sigma_factor),
        str(ph.deflector_back)
    ])
    print(f"Ran field plotter {code}")

ship_img = mpimg.imread(f'{visualizer_home}/assets/images/ship.png')

def mk_warp_plot(src_file_name, out_file_name,
                       time_to_display:float = 1000,
                       save_fig:bool = True,
                       show:bool = True,
                       show_scatter:bool = False):
    ws = WarpSim()
    print("Reading:", src_file_name)
    ws.load(src_file_name)

    tagged_particles = ws.tagged_particles
    if show_scatter:
        tagged_particles = list()

    tagged_trajectories = dict()
    for tagged_particle in tagged_particles:
        tagged_trajectories[tagged_particle] = {
            "x":list(),
            "y":list()
        }

    x_vals:List[float] = list()
    y_vals:List[float] = list()
    print("snapshots:",len(ws.snapshots))
    time_to_display = max(time_to_display, ws.snapshots[0].global_time)
    time_to_display = min(time_to_display, ws.snapshots[-1].global_time)
    global_time : float = -1
    for s in ws.snapshots:
        ship_state = s.ship_state
        ship_x = ship_state.physics.x
        global_time = s.global_time
        if len(tagged_particles)==0 and global_time < time_to_display:
            continue
        for particle_state in s.particle_states:
            physics = particle_state.physics
            pid = particle_state.pid
            ptype = particle_state.ptype
            x = physics.x - ship_x
            y = physics.y
            x_vals.append(x)
            y_vals.append(y)
            if pid in tagged_trajectories:
                tagged_trajectories[pid]["x"].append(x)
                tagged_trajectories[pid]["y"].append(y)
        if len(tagged_particles)==0:
            break

    if s is not None:
        run_field_plot(s.global_config.physics_config)

    csv_base = pd.read_csv(f"plot_base.csv")
    df_base = csv_base.pivot(columns='x', index='y', values='val')

    csv_vx = pd.read_csv(f"plot_vx.csv")
    df_vx = csv_vx.pivot(columns='x', index='y', values='val')

    mx_base = max(abs(csv_base["val"]))
    mx_vx = max(csv_vx["val"])
    mn_vx = min(csv_vx["val"])
    print("max base:", mx_base)
    print("vx:", mn_vx, mx_vx)
    lo_vx = .01*mx_vx #.99*mn_vx + .01*mx_vx
    hi_vx = .99*mx_vx #.01*mn_vx + .99*mx_vx

    frac=.1
    fig, ax = plt.subplots()
    if ws.warp_drive_kind == "Ours":
        ax.contour(df_base.columns, df_base.index, df_base.values, levels=[frac * mx_base], linestyles=[':'], colors='black')
    ax.contour(df_vx.columns, df_vx.index, df_vx.values, levels=[-lo_vx, lo_vx, hi_vx], linestyles=['dotted'], colors='black')

    if len(tagged_particles) > 0:
        for tagged_particle in tagged_trajectories:
            px = np.array(tagged_trajectories[tagged_particle]["x"])
            py = np.array(tagged_trajectories[tagged_particle]["y"])
            print("Plotting particle", tagged_particle, px.shape, py.shape)
            ax.plot(px, py, linestyle="dashdot", color='black')
    else:
        ax.scatter(np.array(x_vals), np.array(y_vals),s=1,c='black')

    imagebox = OffsetImage(ship_img,zoom=0.03)
    ab = AnnotationBbox(imagebox,(0,0),frameon=False)
    ax.add_artist(ab)

    #if len(tagged_particles) > 0:
    #    ax.set_title("Trajectories")
    #else:
    #    ax.set_title(f"Time: {global_time}")

    ax.set_xbound(-12,12)
    ax.set_ybound(-12,12)
    ax.set_aspect(1)
    ax.set_xlabel(r"$x$")
    ax.set_ylabel(r"$y$")

    #plt.savefig(f"{outdir}/warp.png", bbox_inches="tight")
    if save_fig:
        print("Writing:", out_file_name)
        plt.savefig(out_file_name, bbox_inches="tight")
    if show:
        plt.show()

if __name__ == "__main__":
    in_file = pres.input
    if in_file is None:
        in_file = f"{visualizer_home}/idump.json"
    out_file = pres.output
    mk_warp_plot(in_file, out_file)