---
title: Python Quick Start
order: 1
---

## Installing Rerun

The Rerun SDK for Python requires a working installation of [Python-3.8+](https://www.python.org/).

You can install the Rerun SDK using the [rerun-sdk](https://pypi.org/project/rerun-sdk/) pypi package via pip:
```bash
$ pip3 install rerun-sdk
```

You are now ready to start logging and visualizing data.

## Trying out the viewer

The Rerun SDK comes packaged with a simple demo to quickly get a feel for the viewer. You can launch it with:
```bash
$ python3 -m rerun_demo
```

If everything is installed and working correctly, you should end up with a window like below.
Try looping the recording to see the fun animation.

<picture>
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/quickstart0_cube/770ffcd66ebc020bb0ff00ec123e19f1fcb0a3a4/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/quickstart0_cube/770ffcd66ebc020bb0ff00ec123e19f1fcb0a3a4/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/quickstart0_cube/770ffcd66ebc020bb0ff00ec123e19f1fcb0a3a4/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/quickstart0_cube/770ffcd66ebc020bb0ff00ec123e19f1fcb0a3a4/1200w.png">
  <img src="https://static.rerun.io/quickstart0_cube/770ffcd66ebc020bb0ff00ec123e19f1fcb0a3a4/full.png" alt="colored cube">
</picture>


*Note: If this is your first time launching Rerun you will see a notification in the terminal about the Rerun anonymous
data usage policy. Rerun collects anonymous usage data to help improve the project, though you may choose to opt out if you
would like.*

### If you're having problems
 * Checkout out our [troubleshooting guide](troubleshooting.md).
 * [open an issue](https://github.com/rerun-io/rerun/issues/new/choose).
 * Or [join the Discord server](https://discord.gg/PXtCgFBSmH).

## Using the viewer
Try out the following to interact with the viewer:
 * Click and drag in the main view to rotate the cube.
 * Zoom in and out with the scroll wheel.
 * Mouse over the "?" icons to find out about more controls.
 * Grab the time-slider and move it to see the cube at different time-points.
 * Click the "play" button to animate the cube.
 * Click on the cube to select all the points.
 * Hover and select individual points to see more information.

This is just a taste of some of what you can do with the viewer. We will cover other functionality in much
more detail later in the [Viewer Walkthrough](viewer-walkthrough.md)

## Logging your own data
Now instead of using a prepackaged demo, let's create some data ourselves. We will start with an
extremely simplified version of this dataset that just logs 1 dimension of points instead of 3.

Create a new python script with the following code:

```python
import rerun as rr  # NOTE: `rerun`, not `rerun-sdk`!
import numpy as np

rr.init("rerun_example_my_data", spawn=True)

positions = np.zeros((10, 3))
positions[:,0] = np.linspace(-10,10,10)

colors = np.zeros((10,3), dtype=np.uint8)
colors[:,0] = np.linspace(0,255,10)

rr.log(
    "my_points",
    rr.Points3D(positions, colors=colors, radii=0.5)
)
```

When you run this script you will again be greeted with the [Rerun Viewer](../reference/viewer/overview.md), this time
only showing a simple line of red points.

<picture>
  <img src="https://static.rerun.io/quickstart1_line/969a72007a8354bc2638d5794fa7d47176135d0c/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/quickstart1_line/969a72007a8354bc2638d5794fa7d47176135d0c/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/quickstart1_line/969a72007a8354bc2638d5794fa7d47176135d0c/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/quickstart1_line/969a72007a8354bc2638d5794fa7d47176135d0c/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/quickstart1_line/969a72007a8354bc2638d5794fa7d47176135d0c/1200w.png">
</picture>



The [`rr.log`](https://ref.rerun.io/docs/python/stable/common/logging_functions/#rerun.log) is the primary way to log data. It's a flexible
call that can accept a variety of correctly-formatted data—including your own. The easiest way to use it is to use an instance of
one of the built-in *archetype* class, such as [`rr.Points3D`](https://ref.rerun.io/docs/python/stable/common/archetypes/#rerun.archetypes.Points3D). Archetypes take care of gathering the various
components representing, in this case, a batch of 3D points such that it is recognized and correctly displayed by the Rerun viewer. The `rr.Points3D` archetype accepts any collection of positions that can be converted to a Nx3 Numpy array, along with other components such as colors, radii, etc.

Feel free to modify the code to log a different set of points. If you want to generate the colored cube from the
built-in demo, you can use the following numpy incantation.
```python
import rerun as rr
import numpy as np

rr.init("rerun_example_my_data", spawn=True)

SIZE = 10

pos_grid = np.meshgrid(*[np.linspace(-10, 10, SIZE)]*3)
positions = np.vstack([d.reshape(-1) for d in pos_grid]).T

col_grid = np.meshgrid(*[np.linspace(0, 255, SIZE)]*3)
colors = np.vstack([c.reshape(-1) for c in col_grid]).astype(np.uint8).T

rr.log(
    "my_points",
    rr.Points3D(positions, colors=colors, radii=0.5)
)
```

<picture>
  <img src="https://static.rerun.io/quickstart2_simple_cube/632a8f1c79f70a2355fad294fe085291fcf3a8ae/full.png" alt="">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/quickstart2_simple_cube/632a8f1c79f70a2355fad294fe085291fcf3a8ae/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/quickstart2_simple_cube/632a8f1c79f70a2355fad294fe085291fcf3a8ae/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/quickstart2_simple_cube/632a8f1c79f70a2355fad294fe085291fcf3a8ae/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/quickstart2_simple_cube/632a8f1c79f70a2355fad294fe085291fcf3a8ae/1200w.png">
</picture>

## What's next

If you're ready to move on to more advanced topics, check out the [Viewer Walkthrough](viewer-walkthrough.md) or our
more advanced guide for [Logging Data in Python](logging-python.md) where we will explore the core concepts that make
Rerun tick and log our first non-trivial dataset.

If you'd rather learn from examples, check out the [example gallery](/examples) for some more realistic examples, or browse the [Types](../reference/types.md) section for more simple examples of how to use the main  s.
