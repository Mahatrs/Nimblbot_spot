import argparse
import json
import numpy as np


# ============================================================
# ARGUMENTS
# ============================================================

parser = argparse.ArgumentParser()

parser.add_argument(
    "--Xtarget",
    default=1.0,
    type=float,
    help="Position X de la vanne"
)

parser.add_argument(
    "--Ytarget",
    default=0.0,
    type=float,
    help="Position Y de la vanne"
)

parser.add_argument(
    "--Ztarget",
    default=0.6,
    type=float,
    help="Position Z de la vanne"
)

parser.add_argument(
    "--output_json",
    default="/tmp/spot_best_pose.json",
    type=str,
    help="Chemin du fichier JSON de sortie"
)

args = parser.parse_args()


# ============================================================
# POSITION DE LA VANNE
# ============================================================

target = np.array([
    args.Xtarget,
    args.Ytarget,
    args.Ztarget
], dtype=float)

print("==========================================")
print("Position cible de la vanne")
print("X =", target[0])
print("Y =", target[1])
print("Z =", target[2])
print("==========================================")


# ============================================================
# CALCUL HAUTEUR / PITCH DE SPOT
# ============================================================

def spot_z_pitch_from_target_z(z_target):
    """
    Calcule la hauteur et le pitch de Spot
    en fonction de la hauteur de la vanne.

    Version actuelle :
        Ztarget = 0.0 -> Zspot = 0.4
        Ztarget = 1.0 -> Zspot = 0.6

    Le pitch est actuellement forcé à 0.
    """

    z_min = 0.0
    z_max = 1.0

    alpha = np.clip(
        (z_target - z_min) / (z_max - z_min),
        0.0,
        1.0
    )

    z_spot = 0.4 + alpha * (0.6 - 0.4)

    # Actuellement pas d'inclinaison de Spot
    pitch_spot = 0.0

    return z_spot, pitch_spot


z_spot, pitch_spot = spot_z_pitch_from_target_z(target[2])


# ============================================================
# CALCUL DE LA POSE DE SPOT
# ============================================================

# Spot doit se placer :
#   - 0.85 m avant la vanne sur X
#   - 0.05 m décalé sur Y

X_GOAL = target[0] - 0.85
Y_GOAL = target[1] - 0.05
Z_GOAL = z_spot

ROLL_GOAL = 0.0
PITCH_GOAL = pitch_spot
YAW_GOAL = 0.0


best_base_spot = [
    X_GOAL,
    Y_GOAL,
    Z_GOAL,
    ROLL_GOAL,
    PITCH_GOAL,
    YAW_GOAL
]


# ============================================================
# CALCUL DE LA BASE DU BRAS
# ============================================================

# Le bras est monté par rapport à la base de Spot à :
#
#   +0.30 m sur X
#   +0.00 m sur Y
#   +0.08 m sur Z

ARM_OFFSET_X = 0.30
ARM_OFFSET_Y = 0.00
ARM_OFFSET_Z = 0.08


ARM_X = X_GOAL + ARM_OFFSET_X
ARM_Y = Y_GOAL + ARM_OFFSET_Y
ARM_Z = Z_GOAL + ARM_OFFSET_Z

ARM_ROLL = ROLL_GOAL
ARM_PITCH = PITCH_GOAL
ARM_YAW = YAW_GOAL


best_base_arm = [
    ARM_X,
    ARM_Y,
    ARM_Z,
    ARM_ROLL,
    ARM_PITCH,
    ARM_YAW
]


# ============================================================
# AFFICHAGE
# ============================================================

print()
print("------------------------------------------")
print("Meilleure base Spot")
print("------------------------------------------")

print("X     =", X_GOAL)
print("Y     =", Y_GOAL)
print("Z     =", Z_GOAL)
print("Roll  =", ROLL_GOAL)
print("Pitch =", PITCH_GOAL)
print("Yaw   =", YAW_GOAL)

print()
print("------------------------------------------")
print("Base du bras")
print("------------------------------------------")

print("X     =", ARM_X)
print("Y     =", ARM_Y)
print("Z     =", ARM_Z)
print("Roll  =", ARM_ROLL)
print("Pitch =", ARM_PITCH)
print("Yaw   =", ARM_YAW)


# ============================================================
# CONSTRUCTION DU JSON
# ============================================================

result = {

    "real_spot_cmd": {

        # Déplacement horizontal de Spot
        "dx": float(X_GOAL),
        "dy": float(Y_GOAL),

        # Rotation utilisée par ton node Spot
        "dyaw": 0.0,

        # Spot part d'une hauteur nominale de 0.6 m
        "dz": float(Z_GOAL - 0.6),

        "roll": float(ROLL_GOAL),
        "pitch": float(PITCH_GOAL),
        "yaw": float(YAW_GOAL),
    },

    "real_arm_cmd": {

        "x": float(ARM_X),
        "y": float(ARM_Y),
        "z": float(ARM_Z),

        "roll": float(ARM_ROLL),
        "pitch": float(ARM_PITCH),
        "yaw": float(ARM_YAW),
    }
}


# ============================================================
# ÉCRITURE DU JSON
# ============================================================

with open(args.output_json, "w") as f:
    json.dump(result, f, indent=2)


print()
print("==========================================")
print("JSON créé :", args.output_json)
print("==========================================")

print(json.dumps(result, indent=2))