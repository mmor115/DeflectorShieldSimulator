import json
import os
from enum import Enum
from math import sqrt
from typing import List, Dict


def get_warp_defaults():
    return json.loads("""
{
  "snapshots": [
    {
      "global_config": {
        "physics_config": {
          "radius": 4.0,
          "sigma": 4.0,
          "u": 0.5,
          "u0": 0.5,
          "k0": 0.1,
          "x0": 0.0,
          "t0": 0.0,
          "gamma": 0.0,
          "epsilon": 1e-12,
          "deflector_back": 0.0,
          "deflector_sigma_factor": 1.1,
          "deflector_sigma_pushout": 1.0,
          "warp_drive_kind": "Ours"
        },
        "particle_settings": {
          "spawning_enabled": false,
          "y_position_variance": 150.0,
          "z_position_variance": 0.0,
          "x_velocity_variance": 0.0,
          "y_velocity_variance": 0.0,
          "z_velocity_variance": 0.0,
          "photon_chance": 0.0
        },
        "visual_settings": {
          "show_inner_bubble": true,
          "show_outer_bubble": true,
          "hide_untagged_particles": false
        },
        "shutdown_config": {
          "in_shutdown_state": false,
          "temporary_parameters": null
        },
        "validator_settings": {
          "nan_validator": "Die",
          "normalization_validator": "ExplodeParticle",
          "normalized_tolerance": 1e-6,
          "normalized_tolerance_power": -6
        }
      },
      "global_time": 0.0,
      "seeded_rng": {
        "rng": {
          "s": [
            1894788726570067626,
            14287494249828204766,
            9717952331376798956,
            6855634433582098613
          ]
        },
        "seed": [
          233,
          131,
          69,
          187,
          190,
          218,
          46,
          246,
          81,
          191,
          94,
          190,
          30,
          141,
          87,
          81,
          73,
          178,
          80,
          199,
          128,
          1,
          142,
          237,
          26,
          26,
          212,
          153,
          148,
          79,
          32,
          95
        ]
      },
      "ship_state": {
        "physics": [
          0.0,
          0.0,
          0.0,
          0.0,
          0.0,
          0.0,
          1.0
        ]
      },
      "particle_states": []
    }
  ],
  "tagged_particles": {
    "tagged_particles": []
  }
}
""")

class ParticleType(Enum):
    MASSIVE = 1
    PHOTON = 0

MASSIVE = ParticleType.MASSIVE
PHOTON = ParticleType.PHOTON

class PhysicsConfig:
    def __init__(self, physics_config_data):
        self.radius:float = physics_config_data["radius"]
        self.sigma:float = physics_config_data["sigma"]
        self.u:float = physics_config_data["u"]
        self.u0:float = physics_config_data["u0"]
        self.k0:float = physics_config_data["k0"]
        self.x0:float = physics_config_data["x0"]
        self.t0:float = physics_config_data["t0"]
        self.gamma:float = physics_config_data["gamma"]
        self.epsilon:float = physics_config_data["epsilon"]
        self.deflector_back:float = physics_config_data["deflector_back"]
        self.deflector_sigma_factor:float = physics_config_data["deflector_sigma_factor"]
        self.deflector_sigma_pushout:float = physics_config_data["deflector_sigma_pushout"]
        self.warp_drive_kind:str = physics_config_data.get("warp_drive_kind", "Ours")

    def assemble(self):
        return {
            "radius":self.radius,
            "sigma":self.sigma,
            "u":self.u,
            "u0":self.u0,
            "k0":self.k0,
            "x0":self.x0,
            "t0":self.t0,
            "gamma":self.gamma,
            "epsilon":self.epsilon,
            "deflector_back":self.deflector_back,
            "deflector_sigma_factor":self.deflector_sigma_factor,
            "deflector_sigma_pushout":self.deflector_sigma_pushout,
            "warp_drive_kind":self.warp_drive_kind,
        }


class ParticleSettings:
    def __init__(self, particle_settings_data):
        self.spawning_enabled:bool = particle_settings_data["spawning_enabled"]
        self.y_position_variance:float = particle_settings_data["y_position_variance"]
        self.z_position_variance:float = particle_settings_data["z_position_variance"]
        self.x_velocity_variance:float = particle_settings_data["x_velocity_variance"]
        self.y_velocity_variance:float = particle_settings_data["y_velocity_variance"]
        self.z_velocity_variance:float = particle_settings_data["z_velocity_variance"]
        self.photon_chance:float = particle_settings_data["photon_chance"]

    def assemble(self):
        return {
            "spawning_enabled":self.spawning_enabled,
            "y_position_variance":self.y_position_variance,
            "z_position_variance":self.z_position_variance,
            "x_velocity_variance":self.x_velocity_variance,
            "y_velocity_variance":self.y_velocity_variance,
            "z_velocity_variance":self.z_velocity_variance,
            "photon_chance":self.photon_chance
        }


class VisualSettings:
    def __init__(self, visualizer_settings):
        self.show_inner_bubble:bool = visualizer_settings["show_inner_bubble"]
        self.show_outer_bubble:bool = visualizer_settings["show_outer_bubble"]
        self.hide_untagged_particles:bool = visualizer_settings["hide_untagged_particles"]

    def assemble(self):
        return {
            "show_inner_bubble":self.show_inner_bubble,
            "show_outer_bubble":self.show_outer_bubble,
            "hide_untagged_particles":self.hide_untagged_particles
        }


class ShutdownConfig:
    def __init__(self, shutdown_settings):
        self.in_shutdown_state:bool = shutdown_settings["in_shutdown_state"]
        self.temporary_parameters = None

    def assemble(self):
        return {
            "in_shutdown_state":self.in_shutdown_state,
            "temporary_parameters":self.temporary_parameters
        }


class ValidatorSettings:
    def __init__(self, validator_settings):
        self.nan_validator:str = validator_settings["nan_validator"]
        self.normalization_validator:str = validator_settings["normalization_validator"]
        self.normalized_tolerance:float = validator_settings["normalized_tolerance"]
        self.normalized_tolerance_power:int = validator_settings["normalized_tolerance_power"]

    def assemble(self):
        return {
            "nan_validator":self.nan_validator,
            "normalization_validator":self.normalization_validator,
            "normalized_tolerance":self.normalized_tolerance,
            "normalized_tolerance_power":self.normalized_tolerance_power
        }


class GlobalConfig:
    def __init__(self, global_config_data):
        self.physics_config = PhysicsConfig(global_config_data["physics_config"])
        self.particle_settings = ParticleSettings(global_config_data["particle_settings"])
        self.visual_settings = VisualSettings(global_config_data["visual_settings"])
        self.validator_settings = ValidatorSettings(global_config_data["validator_settings"])
        self.shutdown_config = ShutdownConfig(global_config_data["shutdown_config"])

    def assemble(self):
        return {
            "physics_config": self.physics_config.assemble(),
            "particle_settings": self.particle_settings.assemble(),
            "visual_settings": self.visual_settings.assemble(),
            "validator_settings": self.validator_settings.assemble(),
            "shutdown_config": self.shutdown_config.assemble()
        }


class Physics:
    def __init__(self, physics_data):
        self.x = physics_data[0]
        self.y = physics_data[1]
        self.z = physics_data[2]
        self.vx = physics_data[3]
        self.vy = physics_data[4]
        self.vz = physics_data[5]
        self.e = physics_data[6]

    def assemble(self,x0=0):
        return [self.x+x0,self.y,self.z,self.vx,self.vy,self.vz,self.e]

    def normalize(self, ptype):
        v2 = self.vx ** 2 + self.vy ** 2 + self.vz ** 2
        if ptype == PHOTON:
            if v2 == 0:
                self.vx = -1
            else:
                f = 1/sqrt(v2)
                self.vx *= f
                self.vy *= f
                self.vz *= f
        elif ptype == MASSIVE:
            assert v2 < 1
            self.e = 1/sqrt(1-v2)

class ShipState:
    def __init__(self, ship_state):
        self.physics:Physics = Physics(ship_state["physics"])

    def assemble(self):
        return { "physics": self.physics.assemble() }


class Snapshot:
    def __init__(self, snapshot_data):
        self.global_config:GlobalConfig = GlobalConfig(snapshot_data["global_config"])
        self.global_time:float = snapshot_data["global_time"]
        self.seeded_rng:dict = snapshot_data["seeded_rng"]
        self.ship_state:ShipState = ShipState(snapshot_data["ship_state"])
        self.particle_states:List[ParticleState] = list()
        for particle_data in snapshot_data["particle_states"]:
            self.particle_states.append(ParticleState(particle_data))

    def assemble(self)->dict:
        output:Dict = dict()
        output["global_time"] = self.global_time
        output["global_config"] = self.global_config.assemble()
        output["ship_state"] = self.ship_state.assemble()
        particle_states = list()
        for particle_state in self.particle_states:
            particle_states.append(particle_state.assemble())
        output["particle_states"] = particle_states
        assert len(particle_states) == len(self.particle_states)
        output["seeded_rng"] = self.seeded_rng
        return output


class ParticleState:
    def __init__(self, particle_data):
        self.physics:Physics = Physics(particle_data["physics"])
        self.pid:str = particle_data["id"]
        self.ptype = PHOTON if particle_data["particle_type"] == "Photon" else MASSIVE

    def assemble(self):
        # self.physics.normalize(self.ptype)
        return {
            "physics": self.physics.assemble(),
            "id": self.pid,
            "particle_type": "Photon" if self.ptype == PHOTON else "Massive"
        }

    def __repr__(self):
        return json.dumps(self.assemble(), indent=2)

class WarpSim:
    snapshots: list[Snapshot]

    def __init__(self):
        self.idseq:int = 1
        self.jdata:dict = get_warp_defaults()
        self._init_from_jdata()

    @property
    def radius(self):
        return self.snapshots[0].global_config.physics_config.radius

    @radius.setter
    def radius(self,value):
        self.snapshots[0].global_config.physics_config.radius = value

    @property
    def sigma(self):
        return self.snapshots[0].global_config.physics_config.sigma

    @sigma.setter
    def sigma(self,value):
        self.snapshots[0].global_config.physics_config.sigma = value

    @property
    def deflector(self):
        return self.snapshots[0].global_config.physics_config.k0

    @deflector.setter
    def deflector(self, value):
        self.snapshots[0].global_config.physics_config.k0 = value

    @property
    def deflector_sigma_pushout(self):
        return self.snapshots[0].global_config.physics_config.deflector_sigma_pushout

    @deflector_sigma_pushout.setter
    def deflector_sigma_pushout(self, value):
        self.snapshots[0].global_config.physics_config.deflector_sigma_pushout = value

    @property
    def warp_drive_kind(self):
        return self.snapshots[0].global_config.physics_config.warp_drive_kind

    @warp_drive_kind.setter
    def warp_drive_kind(self, value:str):
        self.snapshots[0].global_config.physics_config.warp_drive_kind = value

    @property
    def deflector_sigma_factor(self):
        return self.snapshots[0].global_config.physics_config.deflector_sigma_factor

    @deflector_sigma_factor.setter
    def deflector_sigma_factor(self, value):
        self.snapshots[0].global_config.physics_config.deflector_sigma_factor = value

    @property
    def deflector_back(self):
        return self.snapshots[0].global_config.physics_config.deflector_back

    @deflector_back.setter
    def deflector_back(self, value):
        self.snapshots[0].global_config.physics_config.deflector_back = value

    @property
    def u_bubble(self):
        return self.snapshots[0].global_config.physics_config.u

    @u_bubble.setter
    def u_bubble(self, new_val):
        self.snapshots[0].global_config.physics_config.u = new_val

    @property
    def u_drag(self):
        return self.snapshots[0].global_config.physics_config.u0

    @u_drag.setter
    def u_drag(self, new_val):
        self.snapshots[0].global_config.physics_config.u0 = new_val

    @property
    def u_ship(self):
        return self.snapshots[0].ship_state.physics.vx

    @u_ship.setter
    def u_ship(self, new_val):
        self.snapshots[0].ship_state.physics.vx = new_val
        self.snapshots[0].ship_state.physics.normalize(MASSIVE)

    def add_particle(self, pos, vel, ty, trackme=False):
        pid = "%032d" % self.idseq
        self.idseq += 1
        particle_state = ParticleState({
            "physics":[*pos, *vel, 1],
            "id":pid,
            "particle_type":"Photon" if ty==ParticleType.PHOTON else "Massive"
        })
        if trackme:
            self.tagged_particles.append(pid)
        particle_state.physics.normalize(ty)
        self.snapshots[0].particle_states.append(particle_state)
        self.jdata["snapshots"][0]["particle_states"].append(particle_state.assemble())

    def load(self, fname="dump.json"):
        with open(fname, "r") as fd:
            self.jdata = json.load(fd)
        self._init_from_jdata()

    def _init_from_jdata(self):

        self.tagged_particles:List[str] = self.jdata['tagged_particles']['tagged_particles']

        self.snapshots:List[Snapshot] = list()

        for snapshot_data in self.jdata['snapshots']:
            self.snapshots.append(Snapshot(snapshot_data))

    def assemble(self)->dict:
        output:list = list()
        for snapshot in self.snapshots:
            output.append(snapshot.assemble())
        return {
            "snapshots": output,
            "tagged_particles": {
                "tagged_particles":self.tagged_particles
            }
        }

    def generate(self, fname="idump.json"):
        print("Generating",os.path.abspath(fname))
        jdata2 = self.assemble()

        assert self.u_bubble == self.u_ship + self.u_drag

        with open(fname, "w") as fd:
            fd.write(json.dumps(jdata2, indent=2))

#####

def check(jdata3):
    if type(jdata3) is dict:
        for k, v in jdata3.items():
            assert type(k) == str, "k type = {type(k)}"
            check(v)
    elif type(jdata3) is list:
        for item in jdata3:
            check(item)
    elif type(jdata3) in [str, int, float, bool, type(None)]:
        pass
    else:
        assert False, f"found type: {type(jdata3)}"


def cmp(jdata1, jdata2):
    assert type(jdata1) == type(jdata2), f"{type(jdata1)} != {type(jdata2)}, {jdata1}, {jdata2}"
    if type(jdata1) == list:
        assert len(jdata1) == len(jdata2), f"{len(jdata1)} != {len(jdata2)}, {jdata2[0]}"
        for j in range(len(jdata1)):
            cmp(jdata1[j], jdata2[j])
    elif type(jdata2) == dict:
        assert jdata1.keys() == jdata2.keys(), f"{jdata1.keys()} != {jdata2.keys()}"
        for k in jdata1.keys():
            assert type(k) == str, "k type = {type(k)}"
            cmp(jdata1[k], jdata2[k])
    elif type(jdata1) in [str, int, bool, float, type(None)]:
        assert jdata1 == jdata2, f"{jdata1} != {jdata2}"
    else:
        assert False, f"found type: {type(jdata1)}"

if __name__ == "__main__":

    import os
    visualizer_home = os.environ["VISUALIZER_HOME_DIR"]

    # Example
    sim = WarpSim()

    # generate the ship
    vx = 0.01
    sim.u_ship = 0.0
    sim.u_bubble = 0.5
    sim.u_drag = sim.u_bubble - sim.u_ship

    sim.deflector = 0.9
    sim.deflector_sigma_pushout = 1
    sim.deflector_sigma_factor = 1
    sim.deflector_back = 1

    # Generate a wall of particles
    NP = 101 
    for i in range(NP):
        sim.add_particle((24,(i-NP/2)*24/NP+1e-6,0), (vx,0,0), MASSIVE)
    sim.generate(f"{visualizer_home}/idump.json")
    print("simulation generated")
