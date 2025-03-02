import forallpeople as si


def temperature(T):
    """Convert a float to a quantity with units of kelvin, or preserve the
    existing units."""
    if not hasattr(T, "to"):
        return T * si.K
    return T


def pressure(P):
    """Convert a float to a quantity with units of pascal, or preserve the
    existing units."""
    if not hasattr(P, "to"):
        return P * si.Pa
    return P


def density(rho):
    """Convert a float to a quantity with units of kg/m³, or preserve the
    existing units."""
    if not hasattr(rho, "to"):
        return rho * si.kg / si.m**3
    return rho


def length(h):
    """Convert a float to a quantity with units of meters, or preserve the
    existing units."""
    if not hasattr(h, "to"):
        return h * si.m
    return h


def area(A):
    """Convert a float to a quantity with units of square meters, or preserve
    the existing units."""
    if not hasattr(A, "to"):
        return A * si.m**2
    return A


def volume(V):
    """Convert a float to a quantity with units of cubic meters, or preserve the
    existing units."""
    if not hasattr(V, "to"):
        return V * si.m**3
    return V


def velocity(v):
    """Convert a float to a quantity with units of meters per second, or
    preserve the existing units."""
    if not hasattr(v, "to"):
        return v * si.m / si.s
    return v


def acceleration(a):
    """Convert a float to a quantity with units of meters per second squared, or
    preserve the existing units."""
    if not hasattr(a, "to"):
        return a * si.m / si.s**2
    return a


def force(F):
    """Convert a float to a quantity with units of newtons, or preserve the
    existing units."""
    if not hasattr(F, "to"):
        return F * si.N
    return F
