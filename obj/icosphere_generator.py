# icosphere.py
import math

def normalize(v, radius=1.0):
    x,y,z = v
    mag = math.sqrt(x*x + y*y + z*z)
    if mag == 0:
        return (0.0, 0.0, 0.0)
    k = radius / mag
    return (x*k, y*k, z*k)

def create_icosahedron(radius=1.0):
    t = (1.0 + math.sqrt(5.0)) / 2.0
    verts = [
        (-1,  t,  0), ( 1,  t,  0), (-1, -t,  0), ( 1, -t,  0),
        ( 0, -1,  t), ( 0,  1,  t), ( 0, -1, -t), ( 0,  1, -t),
        ( t,  0, -1), ( t,  0,  1), (-t,  0, -1), (-t,  0,  1),
    ]
    verts = [normalize(v, radius) for v in verts]
    faces = [
        (0, 11, 5), (0, 5, 1), (0, 1, 7), (0, 7, 10), (0, 10, 11),
        (1, 5, 9), (5, 11, 4), (11, 10, 2), (10, 7, 6), (7, 1, 8),
        (3, 9, 4), (3, 4, 2), (3, 2, 6), (3, 6, 8), (3, 8, 9),
        (4, 9, 5), (2, 4, 11), (6, 2, 10), (8, 6, 7), (9, 8, 1),
    ]
    return verts, faces

def midpoint(a, b):
    return ((a[0]+b[0])/2.0, (a[1]+b[1])/2.0, (a[2]+b[2])/2.0)

def subdivide(verts, faces, radius, recursion_level):
    # cache to avoid creating duplicate midpoints
    midpoint_cache = {}
    def vertex_for_edge(i1, i2):
        key = (min(i1,i2), max(i1,i2))
        if key in midpoint_cache:
            return midpoint_cache[key]
        v1 = verts[i1]
        v2 = verts[i2]
        mid = midpoint(v1, v2)
        mid_n = normalize(mid, radius)
        verts.append(mid_n)
        idx = len(verts) - 1
        midpoint_cache[key] = idx
        return idx

    for _ in range(recursion_level):
        new_faces = []
        for tri in faces:
            i0, i1, i2 = tri
            a = vertex_for_edge(i0, i1)
            b = vertex_for_edge(i1, i2)
            c = vertex_for_edge(i2, i0)
            new_faces.append((i0, a, c))
            new_faces.append((i1, b, a))
            new_faces.append((i2, c, b))
            new_faces.append((a, b, c))
        faces = new_faces
        midpoint_cache.clear()  # optional: reduce memory per subdivision
    return verts, faces

def write_obj(filename, verts, faces):
    with open(filename, "w") as f:
        for v in verts:
            f.write(f"v {v[0]:.6f} {v[1]:.6f} {v[2]:.6f}\n")
        for tri in faces:
            # OBJ format is 1-based
            f.write(f"f {tri[0]+1} {tri[1]+1} {tri[2]+1}\n")

def generate_icosphere(filename="icosphere.obj", radius=1.0, recursion_level=2):
    verts, faces = create_icosahedron(radius)
    verts, faces = subdivide(verts, faces, radius, recursion_level)
    write_obj(filename, verts, faces)
    return filename

if __name__ == "__main__":
    # Example usage. Adjust recursion_level for more detail.
    out = generate_icosphere("icosphere.obj", radius=1.0, recursion_level=3)
    print("Wrote", out)
