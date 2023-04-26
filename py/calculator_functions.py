

import math

# some functions for use in a repl

area_square = lambda x: x**2
area_rect = lambda x, y: x*y
area_trapezium = lambda top, bottom, height: ((top+bottom)/2)*height
area_triangle = lambda base, height: (base*height)/2
distance_graph = lambda a, b: abs(a)+abs(b)
hyp = lambda a, b: math.sqrt(a**2+b**2)
reverse_hyp = lambda c, b: math.sqrt(c**2-b**2)
