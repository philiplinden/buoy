"""Command line interface for Buopy."""

import typer
import numpy as np
import forallpeople as si
from rich.console import Console
from rich.table import Table
from rich.panel import Panel
from typing import Optional, List
from . import __version__
from . import constants as c
from . import units as u
from .atmosphere import Atmosphere
from .ideal_gas import Gas, get_gas_properties, density, speed_of_sound
from .dynamics import RigidBody, simulate_trajectory, gravity, drag, buoyancy

console = Console()
app = typer.Typer(
    help="Buopy: Quick math estimations for buoy-core models",
)

# Create subcommands for different modules
atmosphere_app = typer.Typer(help="Atmosphere related commands")
forces_app = typer.Typer(help="Forces related commands")
gas_app = typer.Typer(help="Ideal gas related commands")
dynamics_app = typer.Typer(help="Rigid body dynamics commands")

app.add_typer(atmosphere_app, name="atmo")
app.add_typer(forces_app, name="force")
app.add_typer(gas_app, name="gas")
app.add_typer(dynamics_app, name="dynamics")


def version_callback(value: bool):
    if value:
        console.print(f"Buopy CLI {__version__}")
        raise typer.Exit()


@app.callback()
def callback(
    version: Optional[bool] = typer.Option(
        None,
        "--version",
        "-v",
        help="Show the version and exit.",
        callback=version_callback,
        is_flag=True,
    ),
):
    """Buopy CLI for quick math estimations."""


# Atmosphere commands
@atmosphere_app.command("profile")
def atmosphere_profile(
    altitudes: List[float] = typer.Option(
        [0, 5000, 10000, 15000, 20000, 25000, 30000],
        help="List of altitudes in meters to calculate properties for",
    ),
):
    """Show atmospheric properties at different altitudes."""
    atm = Atmosphere()
    
    table = Table(title="Atmospheric Properties at Different Altitudes")
    table.add_column("Altitude  ", justify="right")
    table.add_column("Temperature", justify="right")
    table.add_column("Pressure", justify="right")
    table.add_column("Density", justify="right")
    table.add_column("Speed of Sound", justify="right")
    
    for altitude in altitudes:
        props = atm.properties(altitude)
        table.add_row(
            f"{altitude:,.0f}",
            f"{props['temperature']}",
            f"{props['pressure']}",
            f"{props['density']}",
            f"{props['speed_of_sound']}",
        )
    
    console.print(table)


@atmosphere_app.command("compare-gases")
def compare_gases():
    """Compare properties of Air vs Helium at sea level."""
    T0 = Atmosphere.T0
    P0 = Atmosphere.P0

    air_props = get_gas_properties(Gas.AIR)
    helium_props = get_gas_properties(Gas.HELIUM)

    air_density = density(T0, P0, Gas.AIR)
    helium_density = density(T0, P0, Gas.HELIUM)

    air_speed = speed_of_sound(T0, Gas.AIR)
    helium_speed = speed_of_sound(T0, Gas.HELIUM)
    
    table = Table(title="Air vs Helium Properties at Sea Level")
    table.add_column("Property", justify="left")
    table.add_column("Air", justify="right")
    table.add_column("Helium", justify="right")
    table.add_column("Ratio (He/Air)", justify="right")
    
    table.add_row(
        "Name", 
        air_props.name, 
        helium_props.name,
        ""
    )
    table.add_row(
        "Molar Mass", 
        f"{air_props.molar_mass_quantity}", 
        f"{helium_props.molar_mass_quantity}",
        f"{float(helium_props.molar_mass_quantity.to(si.kg/si.mol)) / float(air_props.molar_mass_quantity.to(si.kg/si.mol)):.4f}"
    )
    table.add_row(
        "Gamma (cp/cv)", 
        f"{air_props.gamma}", 
        f"{helium_props.gamma}",
        f"{helium_props.gamma / air_props.gamma:.4f}"
    )
    table.add_row(
        "Density", 
        f"{air_density}", 
        f"{helium_density}",
        f"{float(helium_density.to(si.kg/si.m**3)) / float(air_density.to(si.kg/si.m**3)):.4f}"
    )
    table.add_row(
        "Speed of Sound", 
        f"{air_speed}", 
        f"{helium_speed}",
        f"{float(helium_speed.to(si.m/si.s)) / float(air_speed.to(si.m/si.s)):.4f}"
    )
    
    console.print(table)


# Forces commands
@forces_app.command("gravity")
def show_gravity(
    altitudes: List[float] = typer.Option(
        [0, 5000, 10000, 20000, 40000],
        help="List of altitudes in meters to calculate gravity for",
    ),
):
    """Show gravity at different altitudes."""
    table = Table(title="Gravity at Different Altitudes")
    table.add_column("Altitude (m)", justify="right")
    table.add_column("Gravity", justify="right")
    
    for alt in altitudes:
        g = gravity(alt)
        table.add_row(
            f"{alt:,.0f}",
            f"{g}"
        )
    
    console.print(table)


@forces_app.command("drag")
def show_drag(
    velocity_x: float = typer.Option(
        10.0, help="X component of velocity (m/s)"
    ),
    velocity_y: float = typer.Option(0.0, help="Y component of velocity (m/s)"),
    velocity_z: float = typer.Option(0.0, help="Z component of velocity (m/s)"),
    density: float = typer.Option(1.225, help="Ambient density (kg/m³)"),
    area: float = typer.Option(1.0, help="Reference area (m²)"),
    drag_coefficient: float = typer.Option(0.5, help="Drag coefficient"),
):
    """Calculate drag force for given parameters."""
    # Create velocity vector
    velocity = np.array([velocity_x, velocity_y, velocity_z])
    
    # Convert to proper units
    density_qty = density * si.kg / si.m**3
    area_qty = area * si.m**2
    
    # Calculate drag force
    drag_force = drag(velocity, density_qty, area_qty, drag_coefficient)
    
    # Create rich panel to display results
    velocity_mag = np.linalg.norm(velocity)
    drag_mag = np.linalg.norm(drag_force)
    
    result = "Input Parameters:\n"
    result += f"  Velocity: [{velocity_x}, {velocity_y}, {velocity_z}] m/s (magnitude: {velocity_mag:.2f} m/s)\n"
    result += f"  Air density: {density_qty}\n"
    result += f"  Reference area: {area_qty}\n"
    result += f"  Drag coefficient: {drag_coefficient}\n\n"
    result += "Results:\n"
    result += f"  Drag force vector: [{drag_force[0]:.4f}, {drag_force[1]:.4f}, {drag_force[2]:.4f}] N\n"
    result += f"  Drag force magnitude: {drag_mag:.4f} N"
    
    console.print(Panel(result, title="Drag Force Calculation", expand=False))


@forces_app.command("buoyancy")
def show_buoyancy(
    volume: float = typer.Option(10.0, help="Displaced volume (m³)"),
    altitude: float = typer.Option(
        0.0, help="Altitude for density calculation (m)"
    ),
    custom_density: Optional[float] = typer.Option(
        None, help="Custom ambient density (kg/m³), overrides altitude"
    ),
):
    """Calculate buoyancy force for given parameters."""
    # Set up atmosphere
    atm = Atmosphere()
    
    # Get gravity
    g = c.STANDARD_GRAVITY
    
    # Get density based on altitude or custom value
    if custom_density is not None:
        density = custom_density * si.kg / si.m**3
    else:
        density = atm.density(altitude)
    
    # Convert volume to proper units
    volume_qty = volume * si.m**3
    
    # Calculate buoyancy force
    buoyancy_force = buoyancy(altitude, volume_qty, density)
    
    # Create rich panel to display results
    buoyancy_mag = np.linalg.norm(buoyancy_force)
    
    result = "Input Parameters:\n"
    result += f"  Gravity: {g}\n"
    result += f"  Displaced volume: {volume_qty}\n"
    if custom_density is not None:
        result += f"  Custom density: {density}\n"
    else:
        result += f"  Altitude: {altitude} m\n"
        result += f"  Air density at altitude: {density}\n"
    
    result += "\nResults:\n"
    result += f"  Buoyancy force vector: [{buoyancy_force[0]:.4f}, {buoyancy_force[1]:.4f}, {buoyancy_force[2]:.4f}] N\n"
    result += f"  Buoyancy force magnitude: {buoyancy_mag:.4f} N"
    
    console.print(Panel(result, title="Buoyancy Force Calculation", expand=False))


# Gas commands
@gas_app.command("properties")
def gas_properties(
    gas_name: str = typer.Option("AIR", help="Gas name (AIR, HELIUM, etc)"),
):
    """Show properties of a specific gas."""
    try:
        gas = Gas[gas_name.upper()]
        props = get_gas_properties(gas)
        
        table = Table(title=f"Properties of {props.name}")
        table.add_column("Property", justify="left")
        table.add_column("Value", justify="right")
        
        table.add_row("Name", props.name)
        table.add_row("Molar Mass", f"{props.molar_mass_quantity}")
        table.add_row("Gamma (cp/cv)", f"{props.gamma}")
        table.add_row("Gas Constant", f"{props.gas_constant}")
        
        console.print(table)
        
    except KeyError:
        console.print(f"[bold red]Error:[/bold red] Gas '{gas_name}' not found. Available gases: {', '.join([g.name for g in Gas])}")


@gas_app.command("density")
def calculate_density(
    temperature: float = typer.Option(288.15, help="Temperature (K)"),
    pressure: float = typer.Option(101325.0, help="Pressure (Pa)"),
    gas_name: str = typer.Option("AIR", help="Gas name (AIR, HELIUM, etc)"),
):
    """Calculate density for given gas, temperature and pressure."""
    try:
        gas = Gas[gas_name.upper()]
        T = temperature * si.K
        P = pressure * si.Pa
        
        rho = density(T, P, gas)
        
        result = "Input Parameters:\n"
        result += f"  Gas: {gas.value.name}\n"
        result += f"  Temperature: {T}\n"
        result += f"  Pressure: {P}\n\n"
        result += "Result:\n"
        result += f"  Density: {rho}"
        
        console.print(Panel(result, title="Gas Density Calculation", expand=False))
        
    except KeyError:
        console.print(f"[bold red]Error:[/bold red] Gas '{gas_name}' not found. Available gases: {', '.join([g.name for g in Gas])}")


@gas_app.command("sound-speed")
def calculate_sound_speed(
    temperature: float = typer.Option(288.15, help="Temperature (K)"),
    gas_name: str = typer.Option("AIR", help="Gas name (AIR, HELIUM, etc)"),
):
    """Calculate speed of sound for given gas and temperature."""
    try:
        gas = Gas[gas_name.upper()]
        T = temperature * si.K
        
        sound_speed = speed_of_sound(T, gas)
        
        result = "Input Parameters:\n"
        result += f"  Gas: {gas.value.name}\n"
        result += f"  Temperature: {T}\n\n"
        result += "Result:\n"
        result += f"  Speed of Sound: {sound_speed}"
        
        console.print(Panel(result, title="Speed of Sound Calculation", expand=False))
        
    except KeyError:
        console.print(f"[bold red]Error:[/bold red] Gas '{gas_name}' not found. Available gases: {', '.join([g.name for g in Gas])}")


# Add dynamics commands
@dynamics_app.command("simulate")
def simulate_body(
    mass: float = typer.Option(1.0, help="Mass of the body (kg)"),
    init_y: float = typer.Option(0.0, help="Initial y position (m)"),
    init_vy: float = typer.Option(0.0, help="Initial y velocity (m/s)"),
    drag_coef: float = typer.Option(0.47, help="Drag coefficient"),
    drag_area: float = typer.Option(1.0, help="Reference area for drag (m²)"),
    displaced_volume: float = typer.Option(1.0, help="Volume of displaced fluid (m³)"),
    duration: float = typer.Option(60.0, help="Simulation duration (s)"),
    dt: float = typer.Option(0.1, help="Time step (s)"),
    show_plot: bool = typer.Option(False, help="Show plot of results"),
):
    """Simulate rigid body motion with drag and buoyancy."""
    # Create the rigid body
    body = RigidBody(
        mass=mass,
        position=init_y,
        velocity=init_vy,
        drag_coefficient=drag_coef,
        drag_area=drag_area,
        displaced_volume=displaced_volume,
    )
    dt = u.time(dt)
    
    # Run simulation
    states = simulate_trajectory(body, duration, dt)
    
    # Create results table
    table = Table(title=f"Rigid Body Simulation Results (dt={dt})")
    table.add_column("Time", justify="right")
    table.add_column("Position", justify="right")
    table.add_column("Velocity", justify="right")
    table.add_column("Acceleration", justify="right")
    
    # Print simulation parameters
    params = "Simulation Parameters:\n"
    params += f"  Mass: {mass} kg\n"
    params += f"  Initial position: {init_y} m\n"
    params += f"  Initial velocity: {init_vy} m/s\n"
    params += f"  Drag coefficient: {drag_coef}\n"
    params += f"  Drag area: {drag_area} m²\n"
    params += f"  Displaced volume: {displaced_volume} m³\n"
    params += f"  Duration: {duration} s"

    console.print(Panel(params, title="Simulation Setup"))
    
    # Add rows for first 5 and last 5 indices
    total_samples = len(states)
    
    # First 5 indices
    for i in range(min(5, total_samples)):
        table.add_row(
            f"{states[i].time}",
            f"{states[i].position}",
            f"{states[i].velocity}",
            f"{states[i].acceleration}"
        )
    
    # Add separator row if we have more than 5 samples
    if total_samples > 10:
        table.add_row("...", "...", "...", "...")
    
    # Last 5 indices (if we have more than 5 samples)
    if total_samples > 5:
        start_idx = max(5, total_samples - 5)
        for i in range(start_idx, total_samples):
            table.add_row(
                f"{states[i].time}",
                f"{states[i].position}",
                f"{states[i].velocity}",
                f"{states[i].acceleration}"
            )

    console.print(table)
    
    # Print final state summary
    final = Panel(
        f"Final State:\n"
        f"  Time: {states[-1].time}\n"
        f"  Position: {states[-1].position}\n"
        f"  Velocity: {states[-1].velocity}\n"
        f"  Acceleration: {states[-1].acceleration}",
        title="Simulation Results"
    )
    console.print(final)
    
    if show_plot:
        import matplotlib.pyplot as plt

        # Extract data for plotting
        times = [state.time.value for state in states]
        positions = [state.position.value for state in states]
        velocities = [state.velocity.value for state in states]
        accelerations = [state.acceleration.value for state in states]

        # Create subplots
        _, (ax1, ax2, ax3) = plt.subplots(3, 1, figsize=(10, 15))
        
        # Plot position
        ax1.scatter(times, positions)
        ax1.set_title("Position")
        ax1.set_xlabel("Time (s)")
        ax1.set_ylabel("Position (m)")
        
        # Plot velocity
        ax2.scatter(times, velocities)
        ax2.set_title("Velocity")
        ax2.set_xlabel("Time (s)")
        ax2.set_ylabel("Velocity (m/s)")
        
        # Plot acceleration
        ax3.scatter(times, accelerations)
        ax3.set_title("Acceleration")
        ax3.set_xlabel("Time (s)")
        ax3.set_ylabel("Acceleration (m/s²)")
        
        # Adjust layout and show plot
        plt.tight_layout()
        plt.savefig("plot.png")


if __name__ == "__main__":
    app()
