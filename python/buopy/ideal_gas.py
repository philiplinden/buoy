"""
Ideal gas equations and related thermodynamic functions.

This module provides calculations for properties of gases using the ideal gas law
and related thermodynamic equations.
"""

from dataclasses import dataclass
from enum import Enum
import numpy as np
import forallpeople as si
from . import constants as c
from .constants import kelvin, pascal


@dataclass
class GasProperties:
    """Properties of a gas species."""
    name: str
    molar_mass: float  # kg/mol (as a float, will be converted to quantity when needed)
    gamma: float  # Ratio of specific heats
    
    @property
    def molar_mass_quantity(self):
        """Get molar mass as a quantity with units."""
        return self.molar_mass * (si.kg / si.mol)
    
    @property
    def gas_constant(self):
        """Get specific gas constant as a quantity with units."""
        return specific_gas_constant(self.molar_mass)


# Gas properties constants
class Gas(Enum):
    """Enumeration of gas species with their properties."""
    AIR = GasProperties(
        name="Air",
        molar_mass=0.0289644,  # kg/mol
        gamma=1.4,  # Ratio of specific heats
    )
    HELIUM = GasProperties(
        name="Helium",
        molar_mass=0.004002602,  # kg/mol
        gamma=1.66,  # Ratio of specific heats (monatomic gas)
    )


# Function to calculate specific gas constant from molar mass
def specific_gas_constant(molar_mass):
    """
    Calculate the specific gas constant for a gas with given molar mass.
    
    Args:
        molar_mass: Molar mass in kg/mol
        
    Returns:
        Specific gas constant in J/(kg·K)
    """
    return c.GAS_CONSTANT / (molar_mass * si.kg / si.mol)


def density(T, P, gas: Gas = Gas.AIR):
    """
    Compute gas density using the ideal gas law.
    
    Args:
        T: Temperature in Kelvin
        P: Pressure in Pascals
        gas: Gas type (default: Air)
        
    Returns:
        Density as a quantity in kg/m³
    """
    T = kelvin(T)
    P = pascal(P)
    R = gas.value.gas_constant
    return P / (R * T)


def volume(T, P, mass, gas: Gas = Gas.AIR):
    """
    Compute gas volume using the ideal gas law.
    
    Args:
        T: Temperature in Kelvin
        P: Pressure in Pascals
        mass: Mass in kg
        gas: Gas type (default: Air)
        
    Returns:
        Volume as a quantity in m³
    """
    T = kelvin(T)
    P = pascal(P)
    mass = mass * si.kg if not hasattr(mass, "to") else mass.to(si.kg)
    R = gas.value.gas_constant
    
    return (mass * R * T) / P


def pressure(T, density, gas: Gas = Gas.AIR):
    """
    Compute gas pressure using the ideal gas law.
    
    Args:
        T: Temperature in Kelvin
        density: Density in kg/m³
        gas: Gas type (default: Air)
        
    Returns:
        Pressure as a quantity in Pa
    """
    T = kelvin(T)
    density = density * si.kg/si.m**3 if not hasattr(density, "to") else density.to(si.kg/si.m**3)
    R = gas.value.gas_constant
    
    return density * R * T


def temperature(P, density, gas: Gas = Gas.AIR):
    """
    Compute gas temperature using the ideal gas law.
    
    Args:
        P: Pressure in Pascals
        density: Density in kg/m³
        gas: Gas type (default: Air)
        
    Returns:
        Temperature as a quantity in K
    """
    P = pascal(P)
    density = density * si.kg/si.m**3 if not hasattr(density, "to") else density.to(si.kg/si.m**3)
    R = gas.value.gas_constant
    
    return P / (density * R)


def speed_of_sound(T, gas: Gas = Gas.AIR):
    """
    Compute the speed of sound in a gas.
    
    Args:
        T: Temperature in Kelvin
        gas: Gas type (default: Air with gamma = 1.4)
        
    Returns:
        Speed of sound as a quantity in m/s
    """
    T = kelvin(T)
    gamma = gas.value.gamma
    R = gas.value.gas_constant
    a_squared = gamma * R * T
    # Convert to m²/s² then take the square root; reattach m/s unit.
    return np.sqrt(a_squared) * (si.m / si.s)


def get_gas_properties(gas: Gas = Gas.AIR) -> GasProperties:
    """
    Get properties of a specified gas.
    
    Args:
        gas: The gas to get properties for (default: Air)
    
    Returns:
        GasProperties object containing the gas properties
    """
    return gas.value
