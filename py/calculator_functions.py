

import math

# some functions for use in a repl

area_square = lambda x: x**2
area_rect = lambda x, y: x*y
area_trapezium = lambda top, bottom, height: ((top+bottom)/2)*height
area_triangle = lambda base, height: (base*height)/2
distance_graph = lambda a, b: abs(a)+abs(b)
hyp = lambda a, b: math.sqrt(a**2+b**2)
reverse_hyp = lambda c, b: math.sqrt(c**2-b**2)
area_circle = lambda r: r**2*3.14
area_circle_diameter = lambda d: (d/2)**2*3.14 
area_circle_pro = lambda pi, r: pi*r**2
area_circle_pro_diameter = lambda pi, d: (d/2)**2*pi
interior_angles = lambda polygon_sides: 180*(polygon_sides-2)
one_interior_angle = lambda polygon_sides: interior_angles(polygon_sides)/polygon_sides
one_exterior_angle = lambda polygon_sides: 180-one_interior_angle(polygon_sides)
