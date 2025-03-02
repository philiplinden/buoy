from collections import namedtuple
from . import units as u
from . import constants as c
from .atmosphere import Atmosphere


State = namedtuple("State", ["time", "position", "velocity", "acceleration"])


class RigidBody:
    """A rigid body with mass properties and state variables for 3D motion."""

    def __init__(
        self,
        mass: float,
        position: float = 0.0,
        velocity: float = 0.0,
        drag_coefficient: float = 0.5,
        drag_area: float = 1.0,
        displaced_volume: float = 1.0,
    ):
        """
        Initialize a rigid body.

        Args:
            mass: Mass in kg
            position: Initial position in m (vertical, positive up)
            velocity: Initial velocity in m/s (vertical, positive up)
            drag_coefficient: Aerodynamic drag coefficient
            drag_area: Reference area for drag calculation in m²
            displaced_volume: Volume of fluid displaced by body in m³
        """
        if mass <= 0:
            raise ValueError("Mass must be positive")
        self.mass = u.mass(mass)
        self.position = u.length(position)
        self.velocity = u.velocity(velocity)
        if drag_coefficient <= 0:
            raise ValueError("Drag coefficient must be positive")
        self.drag_coefficient = drag_coefficient
        if drag_area <= 0:
            raise ValueError("Drag area must be positive")
        self.drag_area = u.area(drag_area)
        if displaced_volume is not None and displaced_volume <= 0:
            raise ValueError("Displaced volume must be positive")
        self.displaced_volume = u.volume(displaced_volume)

        # Initialize atmosphere model
        self.atmosphere = Atmosphere()

    def net_force(self, position, velocity):
        """
        Calculate net force at a given state.

        Args:
            position: Position in m
            velocity: Velocity in m/s

        Returns:
            Net force in Newtons
        """
        # Get ambient conditions at current altitude
        altitude = position  # y-component is altitude
        ambient_density = self.atmosphere.density(altitude)

        # Calculate individual forces
        weight = gravity(altitude) * self.mass
        drag_force = drag(
            velocity, ambient_density, self.drag_area, self.drag_coefficient
        )
        buoyant_force = buoyancy(
            altitude, self.displaced_volume, ambient_density
        )

        # Sum all forces and get acceleration
        total_force = weight + drag_force + buoyant_force
        return total_force

    def acceleration(self, position, velocity):
        """
        Calculate the net force acting on the rigid body.

        Returns:
            Acceleration in m/s²
        """
        return self.net_force(position, velocity) / self.mass

    def step(self, t, dt):
        """
        Update state by stepping forward in time using RK4 integration.

        Args:
            dt: Time step in seconds
        """
        dt = u.time(dt)

        # RK4 integration
        k1_v = self.acceleration(self.position, self.velocity) * dt
        k1_x = self.velocity * dt

        k2_v = (
            self.acceleration(
                self.position + k1_x / 2, self.velocity + k1_v / 2
            )
            * dt
        )
        k2_x = (self.velocity + k1_v / 2) * dt

        k3_v = (
            self.acceleration(
                self.position + k2_x / 2, self.velocity + k2_v / 2
            )
            * dt
        )
        k3_x = (self.velocity + k2_v / 2) * dt

        k4_v = (
            self.acceleration(self.position + k3_x, self.velocity + k3_v) * dt
        )
        k4_x = (self.velocity + k3_v) * dt

        # Update state
        self.velocity = self.velocity + (k1_v + 2 * k2_v + 2 * k3_v + k4_v) / 6
        self.position = self.position + (k1_x + 2 * k2_x + 2 * k3_x + k4_x) / 6

        return State(
            time=t + dt,
            position=self.position,
            velocity=self.velocity,
            acceleration=self.acceleration(self.position, self.velocity),
        )


def drag(
    velocity,
    ambient_density,
    drag_area,
    drag_coefficient,
):
    """
    Calculate force due to drag as a solid body moves through a fluid.

    Args:
        velocity: Velocity in m/s (positive up).
        ambient_density: Density of the surrounding fluid in kg/m³.
        drag_area: Reference area for drag in m².
        drag_coefficient: Drag coefficient (dimensionless).

    Returns:
        Vertical force in Newtons, opposing motion.
    """
    # Handle zero velocity case
    velocity = u.velocity(velocity)
    if abs(velocity) < 1e-10 * u.velocity(1.0):
        return u.force(0.0)

    # Calculate drag force opposing motion
    return (
        -0.5
        * drag_coefficient
        * u.density(ambient_density)
        * u.area(drag_area)
        * abs(velocity)
        * velocity  # This gives correct sign and magnitude
    )


def buoyancy(
    altitude,
    displaced_volume,
    ambient_density,
):
    """
    Calculate upward force vector due to atmosphere displaced by a gas volume.
    The direction of this force is always world-space up (it opposes gravity).

    Args:
        altitude: Geometric altitude in meters.
        displaced_volume: Volume of displaced fluid in m³.
        ambient_density: Density of the surrounding fluid in kg/m³.

    Returns:
        Vertical force in Newtons, positive upward.
    """
    gravity_acceleration = gravity(altitude)
    displaced_volume = u.volume(displaced_volume)
    ambient_density = u.density(ambient_density)

    return displaced_volume * ambient_density * -gravity_acceleration


def simulate_trajectory(
    body: RigidBody, duration: float, dt: float = 0.1
) -> tuple:
    """
    Simulate the trajectory of a rigid body over time.

    Args:
        body: RigidBody instance to simulate
        duration: Total simulation time in seconds
        dt: Time step in seconds

    Returns:
        Tuple of (positions, velocities, forces) arrays containing state history
    """
    states = []
    state = State(
        0.0,
        body.position,
        body.velocity,
        body.acceleration(body.position, body.velocity),
    )
    while state.time < duration:
        states.append(state)
        state = body.step(state.time, dt)

    return states


def gravity(altitude):
    """
    Calculate the fraction of standard gravity at an altitude above mean sea level.

    Args:
        altitude: Geometric altitude in meters (float or quantity).

    Returns:
        Vertical acceleration in m/s², positive upwards.
    """
    # Calculate the gravity scale
    scale = c.EARTH_RADIUS / (c.EARTH_RADIUS + u.length(altitude))
    return u.acceleration(scale * -c.STANDARD_GRAVITY)


# Optional: Testing the module when run as a script.
if __name__ == "__main__":
    # Example simulation of a balloon
    balloon = RigidBody(
        mass=1.0,  # 1 kg
        position=0.0,  # start at origin
        velocity=0.0,  # start from rest
        drag_coefficient=0.47,  # sphere
        drag_area=1.0,  # 1 m²
        displaced_volume=2.0,  # 2 m³ (will create positive buoyancy)
    )

    # Simulate for 60 seconds
    states = simulate_trajectory(
        balloon, duration= 1000.0, dt=0.1
    )

    # Print final state
    print(f"Final position: {states[-1].position} m")
    print(f"Final velocity: {states[-1].velocity} m/s")
    print(f"Final acceleration: {states[-1].acceleration} m/s²")

    # Simple tests for gravity scaling
    altitudes = [0, 5000, 10000, 20000, 40000]  # in meters
    atm = Atmosphere()

    print("\nGravity at different altitudes:")
    print("Altitude (m) | Gravity (m/s²)")
    print("-" * 30)
    for alt in altitudes:
        print(f"{alt:11.0f} | {gravity(alt):0.6f}")

    # Test for drag force
    velocity = u.velocity(10.0)  # m/s upward
    density = u.density(1.225)  # sea level density
    area = u.area(1.0)  # 1 square meter
    cd = 0.5  # typical drag coefficient

    drag_force = drag(velocity, density, area, cd)

    print("\nDrag force test:")
    print(f"Velocity: {velocity} m/s")
    print(f"Air density: {density}")
    print(f"Reference area: {area}")
    print(f"Drag coefficient: {cd}")
    print(f"Resulting drag force: {drag_force} N")

    # Test for buoyancy
    volume = u.volume(10.0)
    sea_level_density = atm.standard_density()

    buoyancy_force = buoyancy(0, volume, sea_level_density)

    print("\nBuoyancy force test:")
    print(f"Gravity: {gravity(0)}")
    print(f"Displaced volume: {volume}")
    print(f"Air density: {sea_level_density}")
    print(f"Resulting buoyancy force: {buoyancy_force} N")
