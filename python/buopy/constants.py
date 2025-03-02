from scipy import constants
import forallpeople as si

PI = constants.pi

BOLTZMANN_CONSTANT = constants.Boltzmann * si.J * si.K**-1
GAS_CONSTANT = constants.gas_constant * si.J * si.mol**-1 * si.K**-1
STANDARD_PRESSURE = 101325 * si.Pa
STANDARD_TEMPERATURE = 273.15 * si.K

EARTH_RADIUS = 6371007.2 * si.m
GRAVITATIONAL_CONSTANT = constants.gravitational_constant * si.m**3 * si.kg**-1 * si.s**-2
STANDARD_GRAVITY = constants.g * si.m * si.s**-2


def kelvin(T):
    """Convert a quantity or float to a quantity with units of kelvin."""
    if not hasattr(T, "to"):
        return T * si.K
    return T


def pascal(P):
    """Convert a quantity or float to a quantity with units of pascal."""
    if not hasattr(P, "to"):
        return P * si.Pa
    return P


def kg_per_m3(rho):
    """Convert a quantity or float to a quantity with units of kg/m³."""
    if not hasattr(rho, "to"):
        return rho * si.kg / si.m**3
    return rho


def meters(h):
    """Convert a quantity or float to a quantity with units of meters."""
    if not hasattr(h, "to"):
        return h * si.m
    return h


def m2(A):
    """Convert a quantity or float to a quantity with units of square meters."""
    if not hasattr(A, "to"):
        return A * si.m**2
    return A


def m3(V):
    """Convert a quantity or float to a quantity with units of cubic meters."""
    if not hasattr(V, "to"):
        return V * si.m**3
    return V


def m_per_s(v):
    """Convert a quantity or float to a quantity with units of meters per second."""
    if not hasattr(v, "to"):
        return v * si.m / si.s
    return v


def m_per_s2(a):
    """Convert a quantity or float to a quantity with units of meters per second squared."""
    if not hasattr(a, "to"):
        return a * si.m / si.s**2
    return a


def newtons(F):
    """Convert a quantity or float to a quantity with units of newtons."""
    if not hasattr(F, "to"):
        return F * si.N
    return F
