import numpy as np
from vispy import app, scene
import fileinput


lines = [line.strip() for line in fileinput.input(files="inputs/9.txt")]

points = []
for line in lines:
    x, y = line.split(",")
    points.append((float(x), float(y)))

pts = np.array(points, dtype=np.float32)

# Normalize for GPU clip space
pts -= pts.min(axis=0)
pts /= pts.max(axis=0)

canvas = scene.SceneCanvas(
    keys='interactive',
    bgcolor='black',
    size=(1000, 1000),
    show=True
)

view = canvas.central_widget.add_view()
view.camera = scene.PanZoomCamera(rect=(0, 0, 1, 1))
view.camera.interactive = True

line = scene.visuals.Line(
    pts,
    color='lime',
    width=2,
    method='gl'
)
view.add(line)

if __name__ == "__main__":
    app.run()
