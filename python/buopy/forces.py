import numpy as np
import forallpeople as si
from . import constants as c
from . import units as u
from .atmosphere import Atmosphere


def gravity(altitude):
    """
    Calculate the fraction of standard gravity at an altitude above mean sea level.
    
    Args:
        altitude: Geometric altitude in meters (float or quantity).
        
    Returns:
        Fraction of standard gravity (dimensionless float).
    """
    # Calculate the gravity scale
    scale = c.EARTH_RADIUS / (c.EARTH_RADIUS + u.length(altitude))
    return scale * c.STANDARD_GRAVITY


def drag(
    velocity: np.ndarray,
    ambient_density,
    drag_area,
    drag_coefficient: float,
) -> np.ndarray:
    """
    Calculate force due to drag as a solid body moves through a fluid.
    
    Args:
        velocity: 3D velocity vector in m/s.
        ambient_density: Density of the surrounding fluid in kg/m³.
        drag_area: Reference area for drag in m².
        drag_coefficient: Drag coefficient (dimensionless).
        
    Returns:
        3D force vector in Newtons.
    """
    # Calculate velocity magnitude
    velocity_magnitude = np.linalg.norm(velocity)
    
    # Calculate drag force
    force_magnitude = -0.5 * drag_coefficient * ambient_density * drag_area * velocity_magnitude
    
    # Force direction is opposite to velocity
    if velocity_magnitude > 0:
        force = force_magnitude * velocity / velocity_magnitude
    else:
        force = np.zeros(3)
        
    return force


def buoyancy(
    gravity_acceleration,
    displaced_volume,
    ambient_density,
) -> np.ndarray:
    """
    Calculate upward force vector due to atmosphere displaced by a gas volume.
    The direction of this force is always world-space up (it opposes gravity).
    
    Args:
        gravity_acceleration: Gravity acceleration in m/s².
        displaced_volume: Volume of displaced fluid in m³.
        ambient_density: Density of the surrounding fluid in kg/m³.
        
    Returns:
        3D force vector in Newtons, pointing upward.
    """
    gravity_acceleration = u.acceleration(gravity_acceleration)
    displaced_volume = u.volume(displaced_volume)
    ambient_density = u.density(ambient_density)

    # Calculate buoyancy force magnitude
    force_magnitude = displaced_volume * ambient_density * gravity_acceleration
    
    # Force direction is upward
    force = np.array([0, force_magnitude, 0])
    
    return force


# Optional: Testing the module when run as a script.
if __name__ == "__main__":
    # Simple tests for gravity scaling
    altitudes = [0, 5000, 10000, 20000, 40000]  # in meters
    atm = Atmosphere()
    
    print("Gravity at different altitudes:")
    print("Altitude (m) | Gravity")
    print("-" * 30)
    for alt in altitudes:
        print(f"{alt:11.0f} | {gravity(alt):0.6f}")
    
    # Test for drag force
    velocity = np.array([10.0, 0.0, 0.0])  # m/s in x-direction
    density = 1.225 * si.kg / si.m**3     # sea level density
    area = 1.0 * si.m**2                  # 1 square meter
    cd = 0.5                             # typical drag coefficient
    
    drag_force = drag(velocity, density, area, cd)
    
    print("\nDrag force test:")
    print(f"Velocity: {velocity} m/s")
    print(f"Air density: {density}")
    print(f"Reference area: {area}")
    print(f"Drag coefficient: {cd}")
    print(f"Resulting drag force: {drag_force} N")
    
    # Test for buoyancy
    gravity = c.STANDARD_GRAVITY
    volume = 10.0 * si.m**3
    sea_level_density = atm.standard_density()
    
    buoyancy_force = buoyancy(gravity, volume, sea_level_density)
    
    print("\nBuoyancy force test:")
    print(f"Gravity: {gravity}")
    print(f"Displaced volume: {volume}")
    print(f"Air density: {sea_level_density}")
    print(f"Resulting buoyancy force: {buoyancy_force} N")
