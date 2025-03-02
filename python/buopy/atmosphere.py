import numpy as np
import forallpeople as si
from . import constants as c
from . import units as u
from .ideal_gas import speed_of_sound, Gas, get_gas_properties


class Atmosphere:
    """US Standard Atmosphere, 1976

    Model validity boundaries:
    - MAX_ALTITUDE: Maximum altitude defined in the US Standard Atmosphere 1976
    - MIN_ALTITUDE: Minimum altitude (approximates Dead Sea elevation, lowest
                    land point on Earth)
    """

    MAX_ALTITUDE = u.length(84852.0)
    MIN_ALTITUDE = u.length(-56.0)

    # Sea level conditions (US Standard Atmosphere 1976)
    T0 = u.temperature(288.15)
    P0 = u.pressure(101325.0)

    # Define the atmospheric layers as (base altitude, top altitude, lapse rate).
    # Altitudes are given as geopotential altitudes.
    layers = [
        (0 * si.m, 11000 * si.m, -0.0065 * si.K / si.m),
        (11000 * si.m, 20000 * si.m, 0.0 * si.K / si.m),
        (20000 * si.m, 32000 * si.m, 0.0010 * si.K / si.m),
        (32000 * si.m, 47000 * si.m, 0.0028 * si.K / si.m),
        (47000 * si.m, 51000 * si.m, 0.0 * si.K / si.m),
        (51000 * si.m, 71000 * si.m, -0.0028 * si.K / si.m),
        (71000 * si.m, 84852 * si.m, -0.0020 * si.K / si.m),
    ]

    def __init__(self):
        pass

    def temperature(self, altitude):
        """
        Temperature of the atmosphere at a given altitude.

        Args:
            altitude: Geometric altitude in meters (float or quantity).

        Returns:
            Temperature as a quantity in Kelvin.
        """
        T, _ = pressure_temperature_at_altitude(altitude)
        return T

    def pressure(self, altitude):
        """
        Pressure of the atmosphere at a given altitude.

        Args:
            altitude: Geometric altitude in meters (float or quantity).

        Returns:
            Pressure as a quantity in Pascals.
        """
        _, P = pressure_temperature_at_altitude(altitude)
        return P

    def density(self, altitude):
        """
        Density of the atmosphere at a given altitude.

        Args:
            altitude: Geometric altitude in meters (float or quantity).

        Returns:
            Density as a quantity in kg/m³.
        """
        T, P = pressure_temperature_at_altitude(altitude)
        return u.density(T, P, Gas.AIR)

    def speed_of_sound(self, altitude):
        """
        Speed of sound in the atmosphere at a given altitude.

        Args:
            altitude: Geometric altitude in meters (float or quantity).

        Returns:
            Speed of sound as a quantity in m/s.
        """
        T, _ = pressure_temperature_at_altitude(altitude)
        return speed_of_sound(T, Gas.AIR)

    def properties(self, altitude):
        """
        Get all atmospheric properties at a given altitude.

        Args:
            altitude: Geometric altitude in meters (float or quantity).

        Returns:
            A dictionary with keys:
                - temperature: Temperature in Kelvin
                - pressure: Pressure in Pascals
                - density: Density in kg/m³
                - speed_of_sound: Speed of sound in m/s
        """
        return us_standard_atmosphere(altitude)

    # Standard atmosphere conditions
    @staticmethod
    def standard_temperature():
        """Standard sea level temperature."""
        return Atmosphere.T0

    @staticmethod
    def standard_pressure():
        """Standard sea level pressure."""
        return Atmosphere.P0

    @staticmethod
    def standard_density():
        """Standard sea level density."""
        return u.density(Atmosphere.T0, Atmosphere.P0, Gas.AIR)

    @staticmethod
    def standard_speed_of_sound():
        """Standard sea level speed of sound."""
        return speed_of_sound(Atmosphere.T0, Gas.AIR)


def geometric_to_geopotential(h):
    """
    Convert a geometric altitude h (given as a float in meters or as a quantity)
    to geopotential altitude. Returns a quantity in meters.
    """
    return (c.EARTH_RADIUS * h) / (c.EARTH_RADIUS + u.length(h))


def pressure_temperature_at_altitude(h):
    """
    Compute temperature (in K) and pressure (in Pa) at a given geometric
    altitude h using the US Standard Atmosphere 1976 model.

    The input h can be a float (meters) or a quantity. Returns T and P as
    quantities with units.
    """
    # Convert h to a quantity in meters if needed.
    h = u.length(h)

    if h < Atmosphere.MIN_ALTITUDE or h > Atmosphere.MAX_ALTITUDE:
        raise ValueError(f"Altitude out of bounds: {h}")

    # Convert geometric altitude to geopotential altitude.
    H = geometric_to_geopotential(h)

    T = Atmosphere.T0
    P = Atmosphere.P0

    # Get the specific gas constant for air
    R_air = Gas.AIR.value.gas_constant

    # Process each atmospheric layer sequentially.
    for h_base, h_top, L in Atmosphere.layers:
        # Determine the altitude increment (delta_h) for this layer.
        if H > h_top:
            delta_h = h_top - h_base
        else:
            delta_h = H - h_base

        if delta_h < 0 * si.m:
            break

        # Convert L to a float value for the comparison
        if np.isclose(L, 0.0):
            # Isothermal layer: temperature remains constant.
            exponent = float(-c.STANDARD_GRAVITY * delta_h / (R_air * T))
            P = P * np.exp(exponent)
            # T remains unchanged.
        else:
            T_end = T + L * delta_h
            pressure_ratio = T_end / T
            exponent = -c.STANDARD_GRAVITY / (L * R_air)
            P = P * np.power(pressure_ratio, exponent)
            T = T_end

        if H <= h_top:
            return T, P

    # For altitudes above the last layer, return the last computed values.
    return T, P


def us_standard_atmosphere(h):
    """
    Return a dictionary with temperature (K), pressure (Pa), density (kg/m³),
    and speed of sound (m/s) at a given geometric altitude h (in meters)
    based on the US Standard Atmosphere 1976.

    The outputs are provided as quantities with appropriate units.
    """
    T, P = pressure_temperature_at_altitude(h)
    rho = u.density(T, P, Gas.AIR)
    a = u.speed_of_sound(T, Gas.AIR)
    return {
        "temperature": T,
        "pressure": P,
        "density": rho,
        "speed_of_sound": a,
    }


# Optional: Testing the module when run as a script.
if __name__ == "__main__":
    atm = Atmosphere()
    altitudes = [0, 5000, 10000, 15000, 20000, 25000, 30000]  # in meters

    print("Using atmosphere object methods:")
    print("Altitude (m) | Temperature | Pressure | Density | Speed of Sound")
    print("-" * 75)
    for h in altitudes:
        props = atm.properties(h)
        print(
            " | ".join(
                [
                    f"{h:11.0f}",
                    f"{props['temperature']}",
                    f"{props['pressure']}",
                    f"{props['density']}",
                    f"{props['speed_of_sound']}",
                ]
            )
        )
    print("\nUsing direct functions:")
    for h in altitudes:
        props = us_standard_atmosphere(h)
        print(
            f"Altitude: {h} m -> Temperature: {props['temperature']}, "
            f"Pressure: {props['pressure']}, Density: {props['density']}, "
            f"Speed of sound: {props['speed_of_sound']}"
        )

    # Compare air and helium properties
    print("\nComparing Air vs Helium at sea level:")
    T0 = Atmosphere.T0
    P0 = Atmosphere.P0

    air_props = get_gas_properties(Gas.AIR)
    helium_props = get_gas_properties(Gas.HELIUM)

    air_density = u.density(T0, P0, Gas.AIR)
    helium_density = u.density(T0, P0, Gas.HELIUM)

    air_speed = speed_of_sound(T0, Gas.AIR)
    helium_speed = speed_of_sound(T0, Gas.HELIUM)

    print(
        ", ".join(
            [
                f"Air name: {air_props.name}",
                f"molar mass: {air_props.molar_mass_quantity}",
                f"gamma: {air_props.gamma}",
            ]
        )
    )
    print(
        ", ".join(
            [
                f"Helium name: {helium_props.name}",
                f"molar mass: {helium_props.molar_mass_quantity}",
                f"gamma: {helium_props.gamma}",
            ]
        )
    )
    print(f"Air density: {air_density}, speed of sound: {air_speed}")
    print(f"Helium density: {helium_density}, speed of sound: {helium_speed}")
    print(f"Density ratio (He/Air): {helium_density / air_density}")
    print(f"Speed of sound ratio (He/Air): {helium_speed / air_speed}")
