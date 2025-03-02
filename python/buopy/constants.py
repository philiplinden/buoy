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
