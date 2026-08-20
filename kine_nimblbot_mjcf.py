import placo
import numpy as np
from placo_utils.tf import tf
from ischedule import schedule, run_loop
from placo_utils.visualization import robot_viz, robot_frame_viz, frame_viz
import webbrowser
import meshcat.geometry as g
import meshcat.transformations as tfm

robot = placo.RobotWrapper(
    "/home/mtouri/Desktop/Maha_folder/robot_descriptions_maha/mjcf_description/nb-55-v7_motor.xml",
    placo.Flags.mjcf | placo.Flags.ignore_collisions
)


solver = placo.KinematicsSolver(robot)
solver.mask_fbase(True)
solver.dt = 0.01
solver.enable_velocity_limits(True)
solver.add_regularization_task(1e-4)

# TCP position task only
T0 = robot.get_T_world_frame("tcp").copy()
#tcp0 = T0[:3, 3].copy()
effector_task = solver.add_frame_task("tcp", np.eye(4))
#effector_task = solver.add_position_task("tcp", tcp0.copy())
effector_task.configure("tcp", "soft", 1.0,1.0)



viz = robot_viz(robot)
webbrowser.open(viz.viewer.url())


seed_task = solver.add_joints_task()
seed_task.configure("seed", "soft", 1.0)
seed_task.set_joints({
    "lower_ring_joint_0": 0.2,
    "upper_ring_joint_0": -0.2,
})

t=0
@schedule(interval=0.01)
def loop():
    global t
    t += 0.01

    # target_pos = np.array([0.17, 0.0, 0.4])
    # effector_task.target_world = target_pos
    #target = [0.25,0.1,  0.3]
    target = [0.25, 0.1 * np.cos(t),  0.3+0.1 * np.sin(t)]
    effector_task.T_world_frame = tf.translation_matrix(target)

    solver.solve(True)
    robot.update_kinematics()

    q = robot.state.q
    print(target)
    
    print("==============================")
    print("taille q =", len(q))
    print("q =", q)
    print()

    print("\n=== ANGLES DES JOINTS ===")
    print(f"{'Joint':40s} | {'Angle (rad)':>12s}")
    print("-" * 60)

    for joint_name in robot.joint_names():

        # on ignore les joints mimic
        if "middle" in joint_name:
            continue

        jid = robot.model.getJointId(joint_name)
        j = robot.model.joints[jid]

        iq = j.idx_q
        nq = j.nq
        nv = j.nv
        


        print(f"{joint_name:40s} | {q[iq]:12.6f}")
    
    viz.display(robot.state.q)
    robot_frame_viz(robot, "tcp")
    robot_frame_viz(robot, "lower_ring_0")
    frame_viz("target", effector_task.T_world_frame)
    # frame_viz("target", np.array([
    #     [1.0, 0.0, 0.0, target_pos[0]],
    #     [0.0, 1.0, 0.0, target_pos[1]],
    #     [0.0, 0.0, 1.0, target_pos[2]],
    #     [0.0, 0.0, 0.0, 1.0],
    # ]))

run_loop()
