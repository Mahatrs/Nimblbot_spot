import placo
import argparse
import numpy as np
from ischedule import schedule, run_loop
from placo_utils.visualization import robot_viz, robot_frame_viz, frame_viz, point_viz
import webbrowser
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D



# chemin

URDF_PATH = "/home/mtouri/Desktop/Maha_folder/robot_descriptions_maha/mjcf_description/nb-55-v7_motor.xml"
ARM_BASE_FRAME = "lower_ring_0"
ARM_TCP         = "tcp"

SPOT_URDF = "/home/mtouri/Desktop/Maha folder/spot_descritpion/model.urdf"
ANYMAL_URDF = "/home/mtouri/Desktop/Maha folder/anymal_c_simple_description-master/urdf/anymal.urdf"


parser = argparse.ArgumentParser(description="Choisis ta monture")
parser.add_argument("monture", type=str, choices=["spot", "anymal"], help="monture type")
parser.add_argument("distance", type=float, help ="Distance du bout")
args = parser.parse_args()


# paramètres

Xw, Yw, Zw = 0.4, 0.4, 0.6

if args.monture == "spot":
    Zp_min, Zp_max = 0.52, 0.70
    MOUNT_URDF = SPOT_URDF
    MOUNT_BASE_FRAME = "base"

elif args.monture == "anymal":
    Zp_min, Zp_max = 0.50, 0.80
    MOUNT_URDF = ANYMAL_URDF
    MOUNT_BASE_FRAME = "base" 


target = np.array([1.5, -0.1, 0.8])


start_base = np.array([
   target[0] - Xw,
   target[1],
   Zp_min + (Zp_max - Zp_min) / 2,
   0.0, 
   0.0,  
   0.0   
])




# robot+solver

robot = placo.RobotWrapper(URDF_PATH, placo.Flags.mjcf | placo.Flags.ignore_collisions)
solver = placo.KinematicsSolver(robot)
solver.mask_fbase(True)
solver.dt = 0.01
solver.enable_velocity_limits(False)
solver.add_regularization_task(1e-4)

T0 = robot.get_T_world_frame(ARM_TCP).copy()
tcp_task = solver.add_position_task(ARM_TCP, T0[:3, 3].copy())
tcp_task.configure(ARM_TCP, "soft", 10.0)

q_init = robot.state.q.copy()



# outils

def rpy_to_quat(roll, pitch, yaw):
   cr = np.cos(roll/2)
   sr = np.sin(roll/2)
   cp = np.cos(pitch/2)
   sp = np.sin(pitch/2)
   cy = np.cos(yaw/2)
   sy = np.sin(yaw/2)

   return np.array([
       sr*cp*cy - cr*sp*sy,
       cr*sp*cy + sr*cp*sy,
       cr*cp*sy - sr*sp*cy,
       cr*cp*cy + sr*sp*sy
   ])


def rpy_to_rot(roll, pitch, yaw):
   cr, sr = np.cos(roll), np.sin(roll)
   cp, sp = np.cos(pitch), np.sin(pitch)
   cy, sy = np.cos(yaw), np.sin(yaw)

   return np.array([
       [cy*cp, cy*sp*sr - sy*cr, cy*sp*cr + sy*sr],
       [sy*cp, sy*sp*sr + cy*cr, sy*sp*cr - cy*sr],
       [-sp,   cp*sr,            cp*cr]
   ])



def manipulability(robot, tcp=ARM_TCP):
    robot.update_kinematics()
    J = robot.frame_jacobian(tcp, "local_world_aligned")
    J = J[:3, 6:]
    return np.sqrt(max(np.linalg.det(J @ J.T), 0.0))


def in_zone(base):
    x, y, z, _,_,_ = base

    if z < Zp_min or z > Zp_max:
        return False

    return (((x - target[0]) / Xw) ** 2 +
            ((y - target[1]) / Yw) ** 2 +
            ((z - target[2]) / Zw) ** 2) <= 1.0


def test_base(base, ik_iters=80, tol=1e-2):
    if not in_zone(base):
        return False, 0.0, None

    x, y, z, roll, pitch, yaw = base

    q = q_init.copy()
    q[0:3] = [x, y, z]
    q[3:7] = rpy_to_quat(roll, pitch, yaw)

    robot.state.q = q
    robot.update_kinematics()

    tcp_task.target_world = target

    for _ in range(ik_iters):
        solver.solve(True)
        robot.update_kinematics()

    tcp_pos = robot.get_T_world_frame(ARM_TCP)[:3, 3]
    err = np.linalg.norm(target - tcp_pos)

    if err > tol:
        return False, 0.0, err

    z_mid = (Zp_min + Zp_max) / 2
    penalty = ((z - z_mid) / (Zp_max - Zp_min)) ** 2

    score = manipulability(robot) - 0.2 * penalty
    return True, score, err



# algo simulated annealing
def simulated_annealing(start, N=200):
    current = start.copy()
    feasible, current_score, _ = test_base(current)

    best = None
    best_score = -np.inf

    tested_bases = []
    tested_scores = []

    if feasible:
        best = current.copy()
        best_score = current_score
        tested_bases.append(current.copy())
        tested_scores.append(current_score)

    T = 0.2

    for _ in range(N):
        step = np.array([0.05, 0.05, 0.03, 0.2, 0.2, 0.5]) * np.sqrt(T)
        proposal = current + np.random.normal(size=6) * step

        feasible_p, score_p, _ = test_base(proposal)

        if feasible_p:
            tested_bases.append(proposal.copy())
            tested_scores.append(score_p)

        if not feasible_p:
            T *= 0.95
            continue

        if best is None:
            best = proposal.copy()
            best_score = score_p
            current = proposal.copy()
            current_score = score_p
            T *= 0.95
            continue

        if (score_p > current_score) or (np.random.rand() < np.exp((score_p - current_score) / max(T, 1e-6))):
            current = proposal.copy()
            current_score = score_p

        if score_p > best_score:
            best = proposal.copy()
            best_score = score_p

        T *= 0.95

    return best, best_score, np.array(tested_bases).reshape(-1, 6), np.array(tested_scores)



# les prints/plots

best_base, best_score, tested_bases, tested_scores = simulated_annealing(start_base)

if best_base is None:
    print("Cible inatteignable :( ")
    raise SystemExit

print("Meilleure pose de base =", best_base)
print("Meilleur score =", best_score)



if len(tested_bases) > 0:
    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')

    sc = ax.scatter(
        tested_bases[:, 0],
        tested_bases[:, 1],
        tested_bases[:, 2],
        c=tested_scores,
        cmap='viridis',
        s=20,
        alpha=0.8
    )

    ax.scatter(best_base[0], best_base[1], best_base[2], color='red', s=100, label='best base')
    ax.scatter(target[0], target[1], target[2], color='blue', s=100, label='target')

    ax.set_xlabel("X")
    ax.set_ylabel("Y")
    ax.set_zlabel("Z")
    ax.set_title("Tested base positions colored by score")
    ax.legend()

    cbar = plt.colorbar(sc, ax=ax)
    cbar.set_label("score")

    plt.show()



# visualisation bras

robot_visu = placo.RobotWrapper(URDF_PATH,placo.Flags.mjcf |placo.Flags.ignore_collisions)
solver_visu = placo.KinematicsSolver(robot_visu)
solver_visu.mask_fbase(True)
solver_visu.dt = 0.01
solver_visu.enable_velocity_limits(False)
solver_visu.add_regularization_task(1e-4)

x, y, z, roll, pitch, yaw = best_base

qv = robot_visu.state.q.copy()
qv[0:3] = [x, y, z]
qv[3:7] = rpy_to_quat(roll, pitch, yaw)
robot_visu.state.q = qv
robot_visu.update_kinematics()

Ttcp = robot_visu.get_T_world_frame(ARM_TCP).copy()
tcp_task_visu = solver_visu.add_position_task(ARM_TCP, Ttcp[:3, 3].copy())
tcp_task_visu.configure(ARM_TCP, "soft", 10.0)
tcp_task_visu.target_world = target


# visualisation monture

mount = placo.RobotWrapper(MOUNT_URDF, placo.Flags.ignore_collisions)

T_world_mount = np.eye(4)
T_world_mount[:3, :3] = rpy_to_rot(roll, pitch, yaw)
T_world_mount[:3, 3] = [x+args.distance, y, z-0.09]

mount.set_T_world_frame(MOUNT_BASE_FRAME, T_world_mount)
mount.update_kinematics()



viz = robot_viz(robot_visu, "arm")
viz_mount = robot_viz(mount, args.monture)

webbrowser.open(viz.viewer.url())


# =========================
# LOOP
# =========================
@schedule(interval=0.01)
def loop():
    solver_visu.solve(True)
    robot_visu.update_kinematics()

    viz.display(robot_visu.state.q)
    viz_mount.display(mount.state.q)

    robot_frame_viz(robot_visu, ARM_TCP)

    frame_viz("best_base", np.array([
        [1, 0, 0, x],
        [0, 1, 0, y],
        [0, 0, 1, z],
        [0, 0, 0, 1],
    ]))

    point_viz("target_point", target, radius=0.04, color=0x0000FF)


run_loop()
