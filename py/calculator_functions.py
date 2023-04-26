

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
area_circle_with_diameter = lambda d: (d/2)**2*3.14 
area_circle_pro = lambda pi, r: pi*r**2
area_circle_pro_with_diameter = lambda pi, d: (d/2)**2*pi
