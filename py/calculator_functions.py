

import math

# some functions for use in a repl

__bold__ = "\033[1m"
__end__ = "\033[0m"

__help__ = f"""
{__bold__}syntax:{__end__} name_of_function: args

##################################

{__bold__}area_square{__end__}: x
{__bold__}area_rect{__end__}: x y
{__bold__}area_trapezium{__end__}: top bottom height
{__bold__}area_triangle{__end__}: base height
{__bold__}distance_graph{__end__}: a b
{__bold__}hyp{__end__}: a b
{__bold__}reverse_hyp{__end__}: c b
{__bold__}area_circle{__end__}: r
{__bold__}area_circle_diameter{__end__}: d
{__bold__}area_circle_pro{__end__}: pi r
{__bold__}area_circle_diameter{__end__}: pi d
{__bold__}interior_angles_polygon{__end__}: polygon_sides
{__bold__}one_interior_angle_polygon{__end__}: polygon_sides
{__bold__}one_exterior_angle_polygon{__end__}: polygon_sides

"""

print(__help__)

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
interior_angles_polygon = lambda polygon_sides: 180*(polygon_sides-2)
one_interior_angle_polygon = lambda polygon_sides: interior_angles(polygon_sides)/polygon_sides
one_exterior_angle_polygon = lambda polygon_sides: 180-one_interior_angle(polygon_sides)
